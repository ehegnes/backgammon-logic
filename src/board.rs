use constants::*;
use moves::Submove;
use player::Player;
use std::str::FromStr;

const INITIAL_BOARD: _Board = [
    Some((2, Player::Black)), // 1
    None, // 2
    None, // 3
    None, // 4
    None, // 5
    Some((5, Player::White)), // 6
    None, // 7
    Some((3, Player:: White)), // 8
    None, // 9
    None, // 10
    None, // 11
    Some((5, Player::Black)), // 12
    Some((5, Player::White)), // 13
    None, // 14
    None, // 15
    None, // 16
    Some((3, Player::Black)), // 17
    None, // 18
    Some((5, Player::Black)), // 19
    None, // 20
    None, // 21
    None, // 22
    None, // 23
    Some((2, Player::White)), // 24
];

pub type Position = usize;
type _Point = (u8, Player);
pub type Point  = Option<(_Point)>;
type _Board = [Point; BOARD_SIZE];

#[derive(Clone, Copy, Default)]
pub struct Board {
    board: _Board,
    bar_black: u8,
    bar_white: u8,
}

impl Board {
    /// Return a board setup as such:
    /// ```
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
    pub fn board(&self, p: Player) -> _Board {
        match p {
            Player::White => {
                let mut reversed_board = self.board;
                reversed_board.reverse();
                reversed_board
            },
            _ => self.board,
        }
    }

    pub fn apply_submove(&mut self, _s: Submove) -> () {
        /* XXX: Fuck. How do we handle both bars without muddying the current data structures?
         * `gnubg` uses two copies of the board. It would be nice to avoid that.
         * `mmakowski/backgammon-model` uses dedicated counters for each bar.
         */
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
        let owns_chequer = |x: Position| self.board(p)[x].unwrap().1 == p;

        // Check bar
        let check_bar = self.bar(p) > 0;

        // Check not moving onto an opponent's point
        let check_moving_to_point = |t: Position|
            self.board(p)[t].is_some() &&
            self.board(p)[t].unwrap().1 != p &&
            self.board(p)[t].unwrap().0 >= 2;

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
    pub fn pips(&self, p: Player) -> u16 {
        let mut count: u16 = 0;
        for (i, x) in self.board(p).iter().enumerate() {
            let i = BOARD_SIZE - i;
            let x = x.unwrap_or((0, p.switch()));
            if x.1 == p { count += (x.0 as u16) * i as u16 }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
