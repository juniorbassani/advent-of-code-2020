#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

fn get_buffered_input(path: &str) -> impl BufRead {
    BufReader::new(File::open(path).expect("Couldn't open file"))
}
