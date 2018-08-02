#![allow(dead_code)]

//! A library for playing and checking backgammon games.
//!
//! `backgammon_logic` handles game logic such as setting up boards, checking move legality,
//! listing possible moves, and tracking game progress.

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
