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
    position: Option<(isize, isize, isize)>,
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
            position: None,
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

    fn identify_overlaping_scanner(&self, other: &Scanner) -> Option<Scanner> {
        for other_orientation in other.orientations() {
            for self_bv in &self.beacon_vectors {
                for other_bv in &other_orientation.beacon_vectors {
                    let (ax, ay, az) = self_bv;
                    let (bx, by, bz) = other_bv;
                    let (dx, dy, dz) = (ax - bx, ay - by, az - bz);

                    let translated_other =
                        other_orientation.translated_beacon_vectors(&(dx, dy, dz));

                    let intersection: HashSet<_> = self
                        .beacon_vectors
                        .intersection(&translated_other)
                        .cloned()
                        .collect();

                    if intersection.len() >= 12 {
                        return Some(Scanner {
                            number: other.number,
                            beacon_vectors: translated_other,
                            position: Some((dx, dy, dz)),
                        });
                    }
                }
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

fn identify_next_scanner(
    identified_scanners: &mut Vec<Scanner>,
    unidentified_scanners: &mut Vec<Scanner>,
) {
    for i in 0..identified_scanners.len() {
        for u in 0..unidentified_scanners.len() {
            if let Some(scanner) =
                identified_scanners[i].identify_overlaping_scanner(&unidentified_scanners[u])
            {
                identified_scanners.push(scanner);
                unidentified_scanners.swap_remove(u);
                return;
            }
        }
    }
}

fn main() {
    let filename = env::args()
        .nth(1)
        .expect("please specify the input filename");

    let mut unidentified_scanners = parse_input(&filename);
    let mut identified_scanners: Vec<Scanner> = Vec::with_capacity(unidentified_scanners.len());

    // we start with one identified and use its position as the origin
    identified_scanners.push(Scanner {
        number: unidentified_scanners[0].number,
        beacon_vectors: unidentified_scanners[0].beacon_vectors.clone(),
        position: Some((0, 0, 0)),
    });
    unidentified_scanners.swap_remove(0);

    while !unidentified_scanners.is_empty() {
        identify_next_scanner(&mut identified_scanners, &mut unidentified_scanners);
    }

    let mut beacon_vectors = HashSet::new();

    for scanner in identified_scanners {
        println!(
            "scanner {} is at position {:?}",
            scanner.number,
            scanner.position.unwrap()
        );
        for bv in scanner.beacon_vectors {
            beacon_vectors.insert(bv);
        }
    }

    println!("The total number of beacons is: {}", beacon_vectors.len());
}
