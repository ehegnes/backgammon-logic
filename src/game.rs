use board::Board;
use player::Player;

use rand::{thread_rng, Rng};

pub const DIE_MAX: u8 = 6;

pub type Die = u8;

pub type Dice = (Die, Die);

pub fn roll_die() -> u8 {
    thread_rng().gen_range::<u8>(0, DIE_MAX) + 1
}

pub fn roll_dice() -> (u8, u8) {
    (roll_die(), roll_die())
}

#[repr(C)]
#[derive(Clone)]
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

