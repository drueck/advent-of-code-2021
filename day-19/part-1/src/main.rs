// Advent of Code 2021: Day 19, Part 1
// https://adventofcode.com/2021/day/19
// Usage `cargo run <input-file>

use itertools::Itertools;
use std::collections::HashSet;
use std::{env, fs::File, io::BufRead, io::BufReader};

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

fn parse_input(filename: &str) -> Vec<Scanner> {
    let file = File::open(filename).expect("no such file");
    let reader = BufReader::new(file);
    let mut lines_iter = reader.lines().map(|l| l.unwrap());

    let mut scanners = vec![];

    let mut scanner = 0;
    let mut beacon_vectors = vec![];
    lines_iter.next(); // skip the first line

    while let Some(line) = lines_iter.next() {
        if line.trim().is_empty() {
            continue;
        }
        if line.starts_with("--") {
            scanners.push(Scanner {
                number: scanner,
                beacon_vectors: beacon_vectors.clone(),
            });
            scanner += 1;
            beacon_vectors = vec![];
        } else {
            if let Some((x, y, z)) = line
                .split(",")
                .map(|num| num.parse::<isize>().unwrap())
                .collect_tuple()
            {
                beacon_vectors.push((x, y, z));
            } else {
                panic!("programming error in input parsing");
            }
        }
    }
    scanners.push(Scanner {
        number: scanner,
        beacon_vectors: beacon_vectors.clone(),
    });

    scanners
}

fn main() {
    let filename = env::args()
        .nth(1)
        .expect("please specify the input filename");

    let scanners = parse_input(&filename);

    println!("scanner 0: {:?}", scanners[0]);
}
