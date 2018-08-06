#![allow(dead_code)]

//! A library for playing and checking backgammon games.
//!
//! `backgammon_logic` handles game logic such as setting up boards, checking move legality,
//! listing possible moves, and tracking game progress.
//!
//! # Assumptions
//!
//! All functions assume the board is ordered in a counter-clockwise manner from the current
//! player.

extern crate rand;

pub mod board;
pub mod constants;
pub mod game;
pub mod moves;
pub mod player;

#[no_mangle]
pub extern fn rust_test_ffi(x: i32) -> i32 {
    x+1
}

#[no_mangle]
pub extern fn init_game() -> game::Game {
    game::Game::init()
}

#[no_mangle]
pub extern fn init_player() -> player::Player {
    player::Player::Black
}
