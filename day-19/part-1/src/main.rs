// Advent of Code 2021: Day 19, Part 1
// https://adventofcode.com/2021/day/19
// Usage `cargo run <input-file>

use itertools::Itertools;
use std::collections::HashSet;
use std::{env, fs};

type TransformFunction<'a> = &'a dyn Fn((isize, isize, isize)) -> (isize, isize, isize);

const TRANSFORMATIONS: [TransformFunction; 24] = [
    // positive z
    &|(x, y, z)| (x, y, z),
    &|(x, y, z)| (y, -x, z),
    &|(x, y, z)| (-x, -y, z),
    &|(x, y, z)| (-y, x, z),
    // negative z
    &|(x, y, z)| (-x, y, -z),
    &|(x, y, z)| (y, x, -z),
    &|(x, y, z)| (x, -y, -z),
    &|(x, y, z)| (-y, -x, -z),
    // positive x
    &|(x, y, z)| (-z, y, x),
    &|(x, y, z)| (y, z, x),
    &|(x, y, z)| (z, -y, x),
    &|(x, y, z)| (-y, -z, x),
    // negative x
    &|(x, y, z)| (-z, y, -x),
    &|(x, y, z)| (y, z, -x),
    &|(x, y, z)| (z, -y, -x),
    &|(x, y, z)| (-y, -z, -x),
    // positive y
    &|(x, y, z)| (x, -z, y),
    &|(x, y, z)| (z, -x, y),
    &|(x, y, z)| (-x, z, y),
    &|(x, y, z)| (z, x, y),
    // negative y
    &|(x, y, z)| (x, z, -y),
    &|(x, y, z)| (z, -x, -y),
    &|(x, y, z)| (-x, -z, -y),
    &|(x, y, z)| (-z, x, -y),
];

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

struct ScannerOrientations<'a> {
    scanner: &'a Scanner,
    transformation_index: usize,
}

impl<'a> ScannerOrientations<'a> {
    fn new(scanner: &'a Scanner) -> Self {
        ScannerOrientations {
            scanner,
            transformation_index: 0,
        }
    }
}

impl Iterator for ScannerOrientations<'_> {
    type Item = Scanner;

    fn next(&mut self) -> Option<Self::Item> {
        if self.transformation_index > TRANSFORMATIONS.len() - 1 {
            return None;
        }

        let transform = TRANSFORMATIONS[self.transformation_index];
        let transformed_vectors: Vec<(isize, isize, isize)> = self
            .scanner
            .beacon_vectors
            .iter()
            .map(|vector| transform(*vector))
            .collect();

        self.transformation_index += 1;

        Some(Scanner::new(self.scanner.number, &transformed_vectors))
    }
}

impl Scanner {
    fn new(number: usize, beacon_vectors: &Vec<(isize, isize, isize)>) -> Self {
        Scanner {
            number,
            beacon_vectors: beacon_vectors.clone(),
        }
    }

    fn orientations(&self) -> ScannerOrientations {
        ScannerOrientations::new(&self)
    }

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
        .map(|(number, beacon_vectors)| Scanner::new(number, &beacon_vectors))
        .collect()
}

fn relative_vector(
    beacon_a: &(isize, isize, isize),
    beacon_b: &(isize, isize, isize),
) -> (isize, isize, isize) {
    let (ax, ay, az) = beacon_a;
    let (bx, by, bz) = beacon_b;
    (ax - bx, ay - by, az - bz)
}

fn intersecting_beacons(scanner_a: &Scanner, scanner_b: &Scanner) -> Option<usize> {
    for sa_orientation in scanner_a.orientations() {
        for sb_orientation in scanner_b.orientations() {
            let mut matching_beacons = HashSet::new();

            let sa_relative_vectors = sa_orientation.relative_vectors();
            let sb_relative_vectors = sb_orientation.relative_vectors();
            if sa_relative_vectors.is_disjoint(&sb_relative_vectors) {
                continue;
            }

            for sa_combo in sa_orientation.beacon_vectors.iter().combinations(2) {
                for sb_combo in sb_orientation.beacon_vectors.iter().combinations(2) {
                    let rv_a = relative_vector(sa_combo[0], sa_combo[1]);
                    let rv_b = relative_vector(sb_combo[0], sb_combo[1]);

                    if rv_a == rv_b {
                        matching_beacons.insert(*sa_combo[0]);
                        matching_beacons.insert(*sa_combo[1]);
                    }
                }
            }
            let count = matching_beacons.len();
            if count >= 12 {
                return Some(count);
            }
        }
    }
    None
}

fn main() {
    let filename = env::args()
        .nth(1)
        .expect("please specify the input filename");

    let scanners = parse_input(&filename);

    let scanner_0 = &scanners[0];
    let scanner_1 = &scanners[1];
    // let scanner_4 = &scanners[4];

    if let Some(count) = intersecting_beacons(&scanner_0, &scanner_1) {
        println!(
            "Found {} intersecting beacons between these two scanners",
            count
        );
    } else {
        println!("There was no intersection found between these two scanners");
    }

    let combos_of_twelve = (1..=12).combinations(2).count();

    println!(
        "the number of unique combinations of twelve things are: {}",
        combos_of_twelve
    );
}
