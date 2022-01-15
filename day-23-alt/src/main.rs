// Advent of Code 2021: Day 23
// https://adventofcode.com/2021/day/23
// Usage `cargo run <input-file>

use day_23_alt::Burrow;
use std::env;
use std::fs;

fn main() {
    let filename = env::args().nth(1).expect("please provide an input file");
    let input = fs::read_to_string(filename).expect("no such file");
    let burrow = Burrow::new(&input);

    println!("{}", burrow);
}
