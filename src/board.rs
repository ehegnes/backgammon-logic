use player::Player;
use moves::{Move, Submove};
use constants::*;

const INITIAL_BOARD: _Board = [
    None, // BAR_IDX
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
    None, // BEAR_OFF_IDX
];

pub type Position = usize;
type Point  = Option<(u8, Player)>;
type _Board = [Point; BOARD_SIZE];

#[derive(Clone, Copy, Default)]
pub struct Board {
    board: _Board,
}

impl Board {
    /// Return a board setup as such:
    /// ```
    /// +12-11-10--9--8--7-----6--5--4--3--2--1---0+
    /// | B  .  .  .  W  . | | W  .  .  .  .  B |  |
    /// | B  .  .  .  W  . | | W  .  .  .  .  B |  |
    /// | B  .  .  .  W  . | | W  .  .  .  .  . |  |
    /// | B  .  .  .  .  . | | W  .  .  .  .  . |  |
    /// | B  .  .  .  .  . | | W  .  .  .  .  . |  |
    /// |                  | |                  |--|
    /// | W  .  .  .  .  . | | B  .  .  .  .  . |  |
    /// | W  .  .  .  .  . | | B  .  .  .  .  . |  |
    /// | W  .  .  .  B  . | | B  .  .  .  .  . |  |
    /// | W  .  .  .  B  . | | B  .  .  .  .  W |  |
    /// | W  .  .  .  B  . | | B  .  .  .  .  W |  |
    /// +13-14-15-16-17-18----19-20-21-22-23-24--25+
    /// ```
    pub fn init() -> Board {
        Board {
            board: INITIAL_BOARD,
        }
    }

    /// Returns a board that is counter-clockwise from the [`Player`](player/enum.Player.html).
    pub fn get(&self, p: Player) -> Board {
        Board {
            board: match p {
                Player::White => {
                    let mut reversed_board = self.board;
                    reversed_board.reverse();
                    reversed_board
                },
                _ => self.board,
            }
        }
    }

    pub fn apply_move(&self, _m: Move) -> Board {
        Board::init()
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
        let from_chequer = match self.board[s.from] {
            Some(v) => v,
            None    => return Err("Chequer does not exist."),
        };

        let bar_point = self.board[BAR_IDX];
        let to_chequer = self.board[s.to];

        Ok([
           // Check chequer ownership
           from_chequer.1 == p,
           // Check bar
           bar_point.is_some() && (bar_point.unwrap().1 != p || s.from != BAR_IDX),
           // Check not moving onto an opponent's point
           to_chequer.is_some() && to_chequer.unwrap().1 == p.switch() && to_chequer.unwrap().0 <= 1,
        ].iter().all(|x| *x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_invalid_submoves() {
        let b = Board::init();
        let p= Player::Black;
        // Chequer existence
        assert_eq!(b.validate_submove(&Submove::new(4, 3), p), Err("Chequer does not exist."));
        // Chequer ownership
        assert_eq!(b.validate_submove(&Submove::new(1, 3), p), Ok(false));
        // TODO: Check bar
        // TODO: Check moving onto an opponent's point
    }
}
