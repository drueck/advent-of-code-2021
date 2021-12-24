// Advent of Code 2021: Day 19, Part 1
// https://adventofcode.com/2021/day/19
// Usage `cargo run <input-file>

use itertools::Itertools;
use std::collections::HashSet;
use std::{env, fs};

// fn relative_vectors(beacons_from_scanner: &Vec<(i32, i32)>) -> HashSet<(i32, i32)> {
//     let mut relative_vectors = HashSet::new();

//     for pair in beacons_from_scanner.into_iter().combinations(2) {
//         let (ax, ay) = pair[0];
//         let (bx, by) = pair[1];
//         relative_vectors.insert((ax - bx, ay - by));
//     }

//     relative_vectors
// }

#[derive(Debug)]
struct Scanner {
    number: usize,
    beacon_vectors: Vec<(isize, isize, isize)>,
}

impl Scanner {
    fn relative_vectors(&self) -> HashSet<(isize, isize, isize)> {
        self.beacon_vectors
            .iter()
            .combinations(2)
            .map(|pair| {
                let (ax, ay, az) = pair[0];
                let (bx, by, bz) = pair[1];
                (ax - bx, ay - by, az - bz)
            })
            .collect()
    }
}

fn parse_input(filename: &str) -> Vec<Scanner> {
    fs::read_to_string(filename)
        .expect("error reading input file")
        .split("\n\n")
        .map(|scanner_data| {
            scanner_data
                .trim()
                .split("\n")
                .skip(1)
                .map(|coords| {
                    coords
                        .split(",")
                        .map(|num| num.parse::<isize>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect()
        })
        .enumerate()
        .map(|(number, beacon_vectors)| Scanner {
            number,
            beacon_vectors,
        })
        .collect()
}

fn main() {
    let filename = env::args()
        .nth(1)
        .expect("please specify the input filename");

    let scanners = parse_input(&filename);

    for scanner in &scanners {
        println!(
            "scanner: {}, beacons: {}, relative vectors length: {}",
            scanner.number,
            scanner.beacon_vectors.len(),
            scanner.relative_vectors().len(),
        );
    }

    for pair in scanners.windows(2) {
        let a_relative_vectors = pair[0].relative_vectors();
        let b_relative_vectors = pair[1].relative_vectors();
        let intersection = a_relative_vectors.intersection(&b_relative_vectors);
        println!(
            "number of common beacons without rotation: {}",
            intersection.count()
        );
    }
}
