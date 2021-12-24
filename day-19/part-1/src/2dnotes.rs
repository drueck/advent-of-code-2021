// Advent of Code 2021: Day 19, Part 1
// https://adventofcode.com/2021/day/19
// Usage `cargo run <input-file>

use itertools::Itertools;
use std::collections::HashSet;

fn relative_vectors(beacons_from_scanner: &Vec<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut relative_vectors = HashSet::new();

    for pair in beacons_from_scanner.into_iter().combinations(2) {
        let (ax, ay) = pair[0];
        let (bx, by) = pair[1];
        relative_vectors.insert((ax - bx, ay - by));
    }

    relative_vectors
}

fn main() {
    let mut scanner_0 = vec![(0, 2), (4, 1), (3, 3)];
    let mut scanner_1 = vec![(-1, -1), (-5, 0), (-2, 1)];

    scanner_0.sort();
    scanner_1.sort();

    println!(
        "Relative vectors for scanner 0: {:?}",
        relative_vectors(&scanner_0)
    );

    println!(
        "Relative vectors for scanner 1: {:?}",
        relative_vectors(&scanner_1)
    );

    println!(
        "Common relative vectors between scanners 0 and 1: {:?}",
        relative_vectors(&scanner_0).intersection(&relative_vectors(&scanner_1))
    )
}
