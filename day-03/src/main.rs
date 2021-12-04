// Advent of Code 2021: Day 3
// https://adventofcode.com/2021/day/3
// Usage `cargo run <input-file>

use std::{env, fs::File, io::BufRead, io::BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file: &String = &args[1];

    let file = File::open(input_file).expect("no such file");
    let buf = BufReader::new(file);
    let instructions: Vec<Vec<char>> = buf
        .lines()
        .map(|l| l.expect("could not parse line"))
        .map(|s| s.chars().collect())
        .collect();

    let instruction_length = instructions[0].len();
    let half_of_instructions = instructions.len() / 2;

    let mut gamma_rate_string: String = "".to_string();
    let mut epsilon_rate_string: String = "".to_string();

    for i in 0..instruction_length {
        let ones_count = instructions
            .iter()
            .filter(|instruction| instruction[i] == '1')
            .count();

        match ones_count > half_of_instructions {
            true => {
                gamma_rate_string.push('1');
                epsilon_rate_string.push('0');
            }
            false => {
                gamma_rate_string.push('0');
                epsilon_rate_string.push('1');
            }
        }
    }

    let gamma_rate = u32::from_str_radix(&gamma_rate_string, 2).unwrap();
    let epsilon_rate = u32::from_str_radix(&epsilon_rate_string, 2).unwrap();

    println!("The answer is {}", gamma_rate * epsilon_rate);
}
