// Advent of Code 2021: Day 7, Part 2
// https://adventofcode.com/2021/day/7
// Usage `cargo run <input-file>

use std::{env, fs::File, io::BufRead, io::BufReader};

// fuel required to go the given distance
// 1 + 2 + ... + n  =  n(n+1)/2
// https://en.wikipedia.org/wiki/1_%2B_2_%2B_3_%2B_4_%2B_%E2%8B%AF
fn fuel(distance: usize) -> usize {
    (distance * (distance + 1)) / 2
}

fn total_fuel(positions: &Vec<usize>, meeting_point: isize) -> usize {
    positions
        .iter()
        .map(|position| fuel((*position as isize - meeting_point).abs() as usize))
        .sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file: &String = &args[1];

    let file = File::open(input_file).expect("no such file");
    let mut buf = BufReader::new(file);

    let mut input = String::new();
    buf.read_line(&mut input)
        .expect("failed to read input line");

    let positions: Vec<usize> = input
        .trim()
        .split(",")
        .map(|position| position.parse().expect("not an int"))
        .collect();

    let sum: usize = positions.iter().sum();
    let length: usize = positions.len();
    let mean: f32 = sum as f32 / length as f32;

    // not sure how to predict which of these two will be better
    let fuel_left: usize = total_fuel(&positions, mean.floor() as isize);
    let fuel_right: usize = total_fuel(&positions, mean.ceil() as isize);

    let optimal_fuel = if fuel_left < fuel_right {
        fuel_left
    } else {
        fuel_right
    };

    println!("The optimal fuel consumption is {}", optimal_fuel);
}
