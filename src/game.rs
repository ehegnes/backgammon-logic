use rand::{ ThreadRng, thread_rng, Rng };
use player::Player;
use board::Board;

const DIE_MAX: u8 = 6;

type Die = u8;
type Dice = (Die, Die);

pub fn roll_die(g: &mut Game) -> u8 {
    g.rng.gen_range::<u8>(0, DIE_MAX) + 1
}

pub fn roll_dice(g: &mut Game) -> (u8, u8) {
    (roll_die(g), roll_die(g))
}

#[repr(C)]
pub struct Game {
    board: Board,
    dice: Dice,
    turn: Player,
    rng: ThreadRng,
}

impl Game {
    pub fn init() -> Game {
        Game {
            board: Board::init(),
            dice: (0, 0),
            turn: Player::White,
            rng: thread_rng(),
        }
    }
}

