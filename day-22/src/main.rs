// Advent of Code 2021: Day 22
// https://adventofcode.com/2021/day/22
// Usage `cargo run <input-file>

use day_22::cube_grid::CubeGrid;
use day_22::instruction::{Instruction, Operation};

use std::{env, fs};

fn parse_input(filename: &str) -> Vec<Instruction> {
    let input = fs::read_to_string(&filename).unwrap();
    input
        .trim()
        .lines()
        .map(|line| Instruction::new(&line))
        .collect()
}

fn main() {
    let filename = env::args().nth(1).expect("please supply an input filename");
    let instructions = parse_input(&filename);

    let mut grid = CubeGrid::new();

    for instruction in instructions {
        match instruction.operation {
            Operation::On => {
                grid.add(instruction.cube);
            }
            Operation::Off => {
                grid.subtract(instruction.cube);
            }
        }
    }

    println!("The number of lit cuboids is: {}", grid.volume());
}
