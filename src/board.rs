use constants::*;
use moves::Submove;
use player::Player;

pub type Position = usize;
pub type MaybePoint = Option<Point>;
pub type InternalBoard = [MaybePoint; BOARD_SIZE];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub owner: Player,
    pub count: u8,
}

impl Point {
    pub fn new(owner: Player, count: u8) -> Point {
        Point {
            owner,
            count,
        }
    }
}

pub const INITIAL_BOARD: [MaybePoint; BOARD_SIZE] = [
    Some(Point { owner: Player::Black, count: 2 }), // 1
    None, // 2
    None, // 3
    None, // 4
    None, // 5
    Some(Point { owner: Player::White, count: 5 }), // 6
    None, // 7
    Some(Point { owner: Player::White, count: 3 }), // 8
    None, // 9
    None, // 10
    None, // 11
    Some(Point { owner: Player::Black, count: 5 }), // 12
    Some(Point { owner: Player::White, count: 5 }), // 13
    None, // 14
    None, // 15
    None, // 16
    Some(Point { owner: Player::Black, count: 3 }), // 17
    None, // 18
    Some(Point { owner: Player::Black, count: 5 }), // 19
    None, // 20
    None, // 21
    None, // 22
    None, // 23
    Some(Point { owner: Player::White, count: 2 }), // 24
    ];

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub board: InternalBoard,
    pub bar_black: u8,
    pub bar_white: u8,
}

impl Board {
    /// Return a board setup as such:
    /// ```markdown
    /// +11-10--9--8--7--6-----5--4--3--2--1--0-+
    /// | B  .  .  .  W  . | | W  .  .  .  .  B |
    /// | B  .  .  .  W  . | | W  .  .  .  .  B |
    /// | B  .  .  .  W  . | | W  .  .  .  .  . |
    /// | B  .  .  .  .  . | | W  .  .  .  .  . |
    /// | B  .  .  .  .  . | | W  .  .  .  .  . |
    /// |                  | |                  |
    /// | W  .  .  .  .  . | | B  .  .  .  .  . |
    /// | W  .  .  .  .  . | | B  .  .  .  .  . |
    /// | W  .  .  .  B  . | | B  .  .  .  .  . |
    /// | W  .  .  .  B  . | | B  .  .  .  .  W |
    /// | W  .  .  .  B  . | | B  .  .  .  .  W |
    /// +12-13-14-15-16-17----18-19-20-21-22-23-+
    /// ```
    pub fn init() -> Board {
        Board {
            board: INITIAL_BOARD,
            bar_black: 0,
            bar_white: 0,
        }
    }

    pub fn bar(&self, p: Player) -> u8 {
        match p {
            Player::Black => self.bar_black,
            _ => self.bar_white,
        }
    }

    /// Returns a board that is counter-clockwise from the [`Player`](../player/enum.Player.html).
    pub fn board(&self, p: Player) -> InternalBoard {
        match p {
            Player::White => {
                let mut reversed_board = self.board;
                reversed_board.reverse();
                reversed_board
            },
            _ => self.board.clone(),
        }
    }

    /// Check if a submove is legal. Returns `true` if valid or `false` if invalid.
    ///
    /// Checks (in order):
    /// - Ensure chequer exists
    /// - Check chequer ownership
    /// - Check bar
    /// - Check not moving onto an opponent's point
    pub fn validate_submove(&self, s: &Submove, p: Player) -> Result<bool, &str> {
        // Ensure chequer exists
        let chequer_exists = |x: Position| match self.board(p)[x] {
            Some(_) => Ok(true),
            None    => return Err("Chequer does not exist."),
        };

        // Check chequer ownership
        let owns_chequer = |x: Position| self.board(p)[x].unwrap().owner == p;

        // Check bar
        let check_bar = self.bar(p) > 0;

        // Check not moving onto an opponent's point
        let check_moving_to_point = |t: Position|
            self.board(p)[t].is_some() &&
            self.board(p)[t].unwrap().owner != p &&
            self.board(p)[t].unwrap().count >= 2;

        match s {
            Submove::Enter { to } => {
                Ok(check_moving_to_point(*to))
            },
            Submove::Move { from, to } => {
                Ok(vec![
                   chequer_exists(*from)?,
                   owns_chequer(*from),
                   check_bar,
                   check_moving_to_point(*to),
                ].iter().all(|x| *x))
            },
            Submove::BearOff { from } => {
                Ok(vec![
                   chequer_exists(*from)?,
                   owns_chequer(*from),
                   check_bar,
                ].iter().all(|x| *x))
            },
        }
    }

    /// Return the pip count for the `Player`.
    ///
    /// TODO: it would be nice to perform this functionally with `zip()` and `fold()`.
    pub fn pips(&mut self, p: Player) -> u16 {
        let mut count: u16 = 0;
        for (i, x) in self.board(p).iter().enumerate() {
            let i = BOARD_SIZE - i;
            let or_point = Point::new(p.switch(), 0).clone();
            let x = x.unwrap_or(or_point);
            if x.owner == p { count += (x.count as u16) * i as u16 }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn check_invalid_submoves() {
        let p = Player::Black;
        let b = Board::init();
        // Chequer existence
        assert_eq!(b.validate_submove(&Submove::from_str("4/3").unwrap(), p), Err("Chequer does not exist."));
        // Chequer ownership
        assert_eq!(b.validate_submove(&Submove::from_str("1/3").unwrap(), p), Ok(false));
        // TODO: Check bar
        // TODO: Check moving onto an opponent's point
    }

    #[test]
    fn check_pips() {
        assert_eq!(Board::init().pips(Player::Black), 167);
        assert_eq!(Board::init().pips(Player::White), 167);
    }
}
