// Advent of Code 2021: Day 10, Part 2
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
    let opener_for = HashMap::from([(']', '['), (')', '('), ('}', '{'), ('>', '<')]);
    let closer_for = HashMap::from([('[', ']'), ('(', ')'), ('{', '}'), ('<', '>')]);
    let openers: HashSet<&char> = HashSet::from_iter(opener_for.values());
    let closers: HashSet<&char> = HashSet::from_iter(closer_for.keys());

    let incomplete_lines: Vec<&Vec<char>> = lines
        .iter()
        .filter(|&line| {
            for char in line {
                if openers.contains(&char) {
                    stack.push(*char);
                } else {
                    match stack.pop() {
                        Some(popped) => {
                            if popped != opener_for[&char] {
                                return false;
                            }
                        }
                        None => {
                            return false;
                        }
                    }
                }
            }
            true
        })
        .collect();

    let mut completions: Vec<Vec<char>> = vec![];

    for line in incomplete_lines {
        stack = vec![];
        let mut line_completions: Vec<char> = vec![];
        let mut line_chars = line.clone();
        while let Some(char) = line_chars.pop() {
            if closers.contains(&char) {
                stack.push(char);
            } else {
                if let None = stack.pop() {
                    line_completions.push(closer_for[&char]);
                }
            }
        }
        completions.push(line_completions);
    }

    let closer_bounties = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);

    let mut scores: Vec<usize> = completions
        .iter()
        .map(|line_completions| {
            let mut line_score = 0;
            for char in line_completions {
                line_score = line_score * 5 + closer_bounties[&char];
            }
            line_score
        })
        .collect();

    scores.sort();

    let middle_score = scores[scores.len() / 2];

    println!("middle score: {}", middle_score);
}
