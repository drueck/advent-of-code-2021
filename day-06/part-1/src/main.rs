// Advent of Code 2021: Day 5, Part 1
// https://adventofcode.com/2021/day/5
// Usage `cargo run <input-file>

use std::{collections::HashMap, env, fs::File, io::BufRead, io::BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file: &String = &args[1];

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

    let mut timer_counts: HashMap<usize, usize> = HashMap::new();
    for days in lanternfish_timers {
        *timer_counts.entry(days).or_insert(0) += 1;
    }

    for _day in 1..=80 {
        let spawners = timer_counts.get(&0).unwrap_or(&0).clone();
        for days in 0..8 {
            let next_day_count = timer_counts.get(&(days + 1)).unwrap_or(&0).clone();
            *timer_counts.entry(days).or_insert(0) = next_day_count;
        }
        *timer_counts.entry(6).or_insert(0) += spawners;
        *timer_counts.entry(8).or_insert(0) = spawners;
    }

    let total_population: usize = timer_counts.values().sum();

    println!(
        "The total population after 80 days would be {}",
        total_population
    );
}
