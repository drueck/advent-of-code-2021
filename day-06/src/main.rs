// Advent of Code 2021: Day 5
// https://adventofcode.com/2021/day/5
// Usage `cargo run <input-file> <days>

use std::{env, fs::File, io::BufRead, io::BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file: &String = &args[1];
    let simulation_days: usize = args[2].parse().expect("please specify the number of days");

    let file = File::open(input_file).expect("no such file");
    let mut buf = BufReader::new(file);

    let mut timers_string = String::new();
    buf.read_line(&mut timers_string)
        .expect("failed to read input line");

    let lanternfish_timers: Vec<usize> = timers_string
        .trim()
        .split(",")
        .map(|days| days.parse().expect("not an int"))
        .collect();

    let mut timer_counts: Vec<usize> = vec![0; 9];

    for days in lanternfish_timers {
        timer_counts[days] += 1;
    }

    for _day in 1..=simulation_days {
        let spawners = timer_counts[0];
        for days in 0..8 {
            timer_counts[days] = timer_counts[days + 1];
        }
        timer_counts[6] += spawners;
        timer_counts[8] = spawners;
    }

    let total_population: usize = timer_counts.iter().sum();

    println!(
        "The total population after {} days would be {}",
        simulation_days, total_population
    );
}
