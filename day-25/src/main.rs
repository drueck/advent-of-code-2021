// Advent of Code 2021: Day 25
// https://adventofcode.com/2021/day/25
// Usage `cargo run <input-file>

use std::env;
use std::fs;

use day_25::SeaCucumberMap;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input");

    let mut map = SeaCucumberMap::new(&input);

    let mut steps = 1;
    while map.move_both_herds() > 0 {
        steps += 1;
    }

    println!("The herds stopped moving after {} steps", steps);
}
