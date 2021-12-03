// Advent of Code 2021: Day 2
// https://adventofcode.com/2021/day/2
// Usage `cargo run <input-file>

use std::{env, fs::File, io::BufRead, io::BufReader};

enum Instruction {
    Up(i32),
    Down(i32),
    Forward(i32),
}

impl Instruction {
    fn from_str(instruction: &str) -> Instruction {
        let parts: Vec<&str> = instruction.split(" ").collect();
        let direction = parts[0];
        let units: i32 = parts[1].parse().expect("invalid units!");
        match direction {
            "up" => Instruction::Up(units),
            "down" => Instruction::Down(units),
            "forward" => Instruction::Forward(units),
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Debug)]
struct SubmarineState {
    aim: i32,
    depth: i32,
    position: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file: &String = &args[1];

    let file = File::open(input_file).expect("no such file");
    let buf = BufReader::new(file);
    let instructions: Vec<Instruction> = buf
        .lines()
        .map(|l| l.expect("could not parse line"))
        .map(|s| Instruction::from_str(&s))
        .collect();

    let mut state = SubmarineState {
        aim: 0,
        depth: 0,
        position: 0,
    };

    for instruction in instructions {
        match instruction {
            Instruction::Up(units) => state.aim -= units,
            Instruction::Down(units) => state.aim += units,
            Instruction::Forward(units) => {
                state.position += units;
                state.depth += state.aim * units;
            }
        }
    }

    println!("The final {:?}", state);
    println!("The answer is {}", state.depth * state.position);
}
