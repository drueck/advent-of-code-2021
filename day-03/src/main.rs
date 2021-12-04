// Advent of Code 2021: Day 3
// https://adventofcode.com/2021/day/3
// Usage `cargo run <input-file>

use std::{env, fs::File, io::BufRead, io::BufReader};

fn counts(diagnostics: &Vec<Vec<char>>, index: usize) -> (usize, usize) {
    let total_diagnostics = diagnostics.len();
    let ones = diagnostics
        .iter()
        .filter(|instruction| instruction[index] == '1')
        .count();
    let zeros = total_diagnostics - ones;

    (ones, zeros)
}

fn find_rating(diagnostics: &Vec<Vec<char>>, criteria_fn: &dyn Fn(usize, usize) -> char) -> usize {
    let mut candidates = diagnostics.to_vec();
    let diagnostic_length = candidates[0].len();
    let mut index = 0;

    while candidates.len() > 1 && index < diagnostic_length {
        let (ones, zeros) = counts(&candidates, index);
        candidates.retain(|candidate| candidate[index] == criteria_fn(ones, zeros));
        index += 1;
    }

    if candidates.len() > 1 {
        panic!("Failed to find a single rating that matched the criteria!");
    }

    let rating: String = candidates[0].iter().collect();

    usize::from_str_radix(&rating, 2).unwrap()
}

fn find_oxygen_generator_rating(diagnostics: &Vec<Vec<char>>) -> usize {
    find_rating(&diagnostics, &|ones, zeros| {
        if ones >= zeros {
            '1'
        } else {
            '0'
        }
    })
}

fn find_c02_scrubber_rating(diagnostics: &Vec<Vec<char>>) -> usize {
    find_rating(&diagnostics, &|ones, zeros| {
        if ones < zeros {
            '1'
        } else {
            '0'
        }
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file: &String = &args[1];

    let file = File::open(input_file).expect("no such file");
    let buf = BufReader::new(file);
    let diagnostics: Vec<Vec<char>> = buf
        .lines()
        .map(|l| l.expect("could not parse line"))
        .map(|s| s.chars().collect())
        .collect();

    let oxygen_generator_rating = find_oxygen_generator_rating(&diagnostics);
    let c02_scrubber_rating = find_c02_scrubber_rating(&diagnostics);

    println!("Oxygen generator rating: {}", oxygen_generator_rating);
    println!("C02 scrubber rating: {}", c02_scrubber_rating);
    println!(
        "Life support rating: {}",
        oxygen_generator_rating * c02_scrubber_rating
    );
}
