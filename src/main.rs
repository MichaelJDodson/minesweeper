mod board;
mod cell;
mod input;
mod game;

use ::rand::{thread_rng, Rng};
use std::fmt::Display;
use macroquad::prelude::*;

fn main() {
   
    let minesweeper_board = board::Board::<5, 5>::initialize_random();
    println!("{}", minesweeper_board);

}


