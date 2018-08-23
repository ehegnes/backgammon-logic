use board::Board;
use player::Player;

use rand::{thread_rng, Rng};

pub const DIE_MAX: u8 = 6;

pub type Die = u8;

pub type Dice = (Die, Die);

#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    board: Board,
    dice: Dice,
    turn: Player,
}

impl Game {
    pub fn init() -> Game {
        Game {
            board: Board::init(),
            dice: (0, 0).into(),
            turn: Player::White,
        }
    }
}

pub fn roll_die() -> Die {
    thread_rng().gen_range::<u8>(0, DIE_MAX) + 1
}

pub fn roll_dice() -> Dice {
    (roll_die(), roll_die())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        let game = Game::init();
        assert_eq!(game, Game {
            board: Board::init(),
            dice: (0, 0).into(),
            turn: Player::White,
        })
    }
    #[test]
    fn test_roll_die() {
        let die = roll_die();
        assert!(die <= 6);
    }

    #[test]
    fn test_roll_dice() {
        let dice = roll_dice();
        assert!(dice.0 <= 6 && dice.1 <= 6);
    }
}
