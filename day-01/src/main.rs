// Advent of Code 2021: Day 1
//
// https://adventofcode.com/2021/day/1
//
// Usage `cargo run <input-file>

use std::{env, fs::File, io::BufRead, io::BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file: &String = &args[1];

    let file = File::open(input_file).expect("no such file");
    let buf = BufReader::new(file);
    let entries: Vec<u32> = buf
        .lines()
        .map(|l| l.expect("could not parse line"))
        .map(|s| s.parse::<u32>().expect("entry was not an integer"))
        .collect::<Vec<u32>>();

    let result: u32 =
        entries.windows(2)
        .map(|pair| if pair[1] > pair[0] { 1 } else { 0 })
        .sum();

    println!("The answer is {}", result);
}
