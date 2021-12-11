// Advent of Code 2021: Day 10, Part 1
// https://adventofcode.com/2021/day/10
// Usage `cargo run <input-file>

use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::BufRead,
    io::BufReader,
    iter::FromIterator,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file: &String = &args[1];

    let file = File::open(input_file).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<Vec<char>> = buf
        .lines()
        .map(|l| l.expect("could not parse line"))
        .map(|s| s.chars().collect())
        .collect();

    let line_length = lines[0].len();
    let mut stack: Vec<char> = Vec::with_capacity(line_length);
    let pairs = HashMap::from([(']', '['), (')', '('), ('}', '{'), ('>', '<')]);
    let openers: HashSet<&char> = HashSet::from_iter(pairs.values());
    let closers: HashSet<&char> = HashSet::from_iter(pairs.keys());

    let mut invalid_chars: Vec<char> = vec![];

    for line in lines {
        for char in line {
            if openers.contains(&char) {
                stack.push(char);
            } else if closers.contains(&char) {
                match stack.pop() {
                    Some(popped) => {
                        if popped != pairs[&char] {
                            invalid_chars.push(char);
                        }
                    }
                    None => {
                        invalid_chars.push(char);
                    }
                }
            }
        }
    }

    let bounties = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let points: u32 = invalid_chars.iter().map(|char| bounties[char]).sum();

    println!("Total points: {}", points);
}
