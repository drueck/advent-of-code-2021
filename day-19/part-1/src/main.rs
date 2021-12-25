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

#[derive(Debug)]
struct Scanner {
    number: usize,
    beacon_vectors: HashSet<(isize, isize, isize)>,
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
        let transformed_vectors: HashSet<(isize, isize, isize)> = self
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
    fn new(number: usize, beacon_vectors: &HashSet<(isize, isize, isize)>) -> Self {
        Scanner {
            number,
            beacon_vectors: beacon_vectors.clone(),
        }
    }

    fn orientations(&self) -> ScannerOrientations {
        ScannerOrientations::new(&self)
    }

    fn translated_beacon_vectors(
        &self,
        (dx, dy, dz): &(isize, isize, isize),
    ) -> HashSet<(isize, isize, isize)> {
        self.beacon_vectors
            .clone()
            .iter()
            .map(|(x, y, z)| (x + dx, y + dy, z + dz))
            .collect()
    }

    fn common_beacons(&self, other: &Scanner) -> Option<HashSet<(isize, isize, isize)>> {
        for a in self.orientations() {
            for b in other.orientations() {
                let a_bv = a.beacon_vectors.iter().next().unwrap();
                for b_bv in &b.beacon_vectors {
                    let (ax, ay, az) = a_bv;
                    let (bx, by, bz) = b_bv;
                    let (dx, dy, dz) = (ax - bx, ay - by, az - bz);

                    let translated_other = b.translated_beacon_vectors(&(dx, dy, dz));

                    let intersection: HashSet<_> = a
                        .beacon_vectors
                        .intersection(&translated_other)
                        .cloned()
                        .collect();

                    if intersection.len() >= 12 {
                        return Some(intersection);
                    }
                }
                // }
            }
        }
        None
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

fn main() {
    let filename = env::args()
        .nth(1)
        .expect("please specify the input filename");

    let scanners = parse_input(&filename);

    let scanner_0 = &scanners[0];
    // let scanner_1 = &scanners[1];
    let scanner_2 = &scanners[2];
    // let scanner_4 = &scanners[4];

    if let Some(intersection) = scanner_0.common_beacons(&scanner_2) {
        println!(
            "Found {} common beacons. They were: {:?}",
            intersection.len(),
            intersection
        );
    }
}
