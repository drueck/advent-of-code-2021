// Advent of Code 2021: Day 14
// https://adventofcode.com/2021/day/14
// Usage `cargo run <input-file> <steps>

use std::{collections::HashMap, env, fs::File, io::BufRead, io::BufReader};

fn main() {
    let input_file = env::args().nth(1).expect("please supply the input file");
    let steps = env::args()
        .nth(2)
        .expect("please supply the number of steps")
        .parse::<usize>()
        .expect("steps must be a positive integer");
    let file = File::open(input_file).expect("no such file");
    let reader = BufReader::new(file);
    let mut lines_iter = reader.lines().map(|l| l.expect("could not parse line"));

    let polymer_template: Vec<char> = lines_iter
        .next()
        .expect("failed to extract the polymer template")
        .chars()
        .collect();

    lines_iter.next(); // skip the blank line

    let insertion_rules_vec: Vec<(Vec<char>, char)> = lines_iter
        .map(|s| {
            let parts: Vec<&str> = s.split(" -> ").collect();
            (
                parts[0].chars().collect(),
                parts[1]
                    .chars()
                    .nth(0)
                    .expect("failed to grab the char to insert"),
            )
        })
        .collect();

    let mut pair_counts: HashMap<(char, char), usize> = HashMap::new();
    for pair in polymer_template.windows(2) {
        *pair_counts.entry((pair[0], pair[1])).or_default() += 1;
    }

    let mut insertion_rules: HashMap<(char, char), char> = HashMap::new();
    for (pair, char) in insertion_rules_vec {
        insertion_rules.insert((pair[0], pair[1]), char);
    }

    let mut element_counts: HashMap<char, usize> = HashMap::new();
    for element in polymer_template {
        *element_counts.entry(element).or_default() += 1;
    }

    for _ in 0..steps {
        let mut changes: HashMap<(char, char), isize> = HashMap::new();

        for pair in pair_counts.keys() {
            let (first, second) = *pair;
            if let Some(element) = insertion_rules.get(&(first, second)) {
                let pair_count = *pair_counts.get(&(first, second)).unwrap();

                *changes.entry((first, *element)).or_default() += pair_count as isize;
                *changes.entry((*element, second)).or_default() += pair_count as isize;
                *changes.entry((first, second)).or_default() -= pair_count as isize;

                *element_counts.entry(*element).or_default() += pair_count;
            }
        }

        for ((first, second), change) in changes.iter() {
            let initial_count = *pair_counts.get(&(*first, *second)).unwrap_or(&0) as isize;
            let new_count = (initial_count + change) as usize;
            pair_counts.insert((*first, *second), new_count);
        }
    }

    let max = element_counts.values().max().unwrap();
    let min = element_counts.values().min().unwrap();

    println!("result: {}", max - min);
}
