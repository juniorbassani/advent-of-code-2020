#[macro_use]
extern crate lazy_static;

use std::fs::{self, File};
use std::io::{BufRead, BufReader};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;

fn get_buffered_input(path: &str) -> impl BufRead {
    BufReader::new(File::open(path).expect("Couldn't open file"))
}

fn get_input_as_string(path: &str) -> String {
    fs::read_to_string(path).expect("Couldn't open file")
}
