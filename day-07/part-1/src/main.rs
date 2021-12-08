// Advent of Code 2021: Day 7, Part 1
// https://adventofcode.com/2021/day/7
// Usage `cargo run <input-file>

use std::{env, fs::File, io::BufRead, io::BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file: &String = &args[1];

    let file = File::open(input_file).expect("no such file");
    let mut buf = BufReader::new(file);

    let mut input = String::new();
    buf.read_line(&mut input)
        .expect("failed to read input line");

    let mut positions: Vec<isize> = input
        .trim()
        .split(",")
        .map(|position| position.parse().expect("not an int"))
        .collect();

    positions.sort();

    let median = positions[positions.len() / 2];

    let fuel: isize = positions
        .iter()
        .map(|position| (position - median).abs())
        .sum();

    println!("The fuel required is {}", fuel);
}
