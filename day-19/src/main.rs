// Advent of Code 2021: Day 19
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
    &|(x, y, z)| (z, y, -x),
    &|(x, y, z)| (y, -z, -x),
    &|(x, y, z)| (-z, -y, -x),
    &|(x, y, z)| (-y, z, -x),
    // positive y
    &|(x, y, z)| (x, -z, y),
    &|(x, y, z)| (-z, -x, y),
    &|(x, y, z)| (-x, z, y),
    &|(x, y, z)| (z, x, y),
    // negative y
    &|(x, y, z)| (-x, -z, -y),
    &|(x, y, z)| (-z, x, -y),
    &|(x, y, z)| (x, z, -y),
    &|(x, y, z)| (z, -x, -y),
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
            // we want to try lining up a point in our beacons with each point
            // in the other scanner's beacons and then see if at least 12 beacons line up
            // we need to try enough to make sure we've tried at least one potentially overlapping
            // beacon, thus the beacon_vectors.len() - 11
            // for example, if our scanner had 15 beacons, we'd need to line up at most 4 of them
            // to be sure we checked one of the potentially overlapping ones
            for self_bv in self
                .beacon_vectors
                .iter()
                .take(self.beacon_vectors.len() - 11)
            {
                for other_bv in other_orientation
                    .beacon_vectors
                    .iter()
                    .take(other.beacon_vectors.len() - 11)
                {
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

    fn manhattan_distance(&self, other: &Scanner) -> usize {
        let (ax, ay, az) = self.position.expect("missing position!");
        let (bx, by, bz) = other.position.expect("missing position!");

        ((ax - bx).abs() + (ay - by).abs() + (az - bz).abs()) as usize
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

    let mut scanners = parse_input(&filename);
    let mut unidentified_scanners: HashSet<usize> = HashSet::new();
    let mut identified_scanners: HashSet<usize> = HashSet::new();
    let mut scanners_to_check: Vec<usize> = vec![];

    scanners[0].position = Some((0, 0, 0));

    identified_scanners.insert(0);
    scanners_to_check.push(0);

    for i in 1..scanners.len() {
        unidentified_scanners.insert(i);
    }

    while !unidentified_scanners.is_empty() {
        if let Some(base) = scanners_to_check.pop() {
            println!("checking for overlaps with scanner {}", base);
            for i in unidentified_scanners.iter() {
                if let Some(identified_scanner) =
                    scanners[base].identify_overlaping_scanner(&scanners[*i])
                {
                    scanners[*i] = identified_scanner;
                    identified_scanners.insert(*i);
                    scanners_to_check.push(*i);
                    println!("identified scanner {}", i);
                }
            }
        } else {
            panic!("programming error");
        }
        unidentified_scanners = unidentified_scanners
            .difference(&identified_scanners)
            .cloned()
            .collect();

        println!("identified_scanners: {:?}", identified_scanners);
        println!("unidentified_scanners: {:?}", unidentified_scanners);
        println!("scanners to check {:?}\n", scanners_to_check);
    }

    let mut beacon_vectors = HashSet::new();

    for scanner in &scanners {
        println!(
            "scanner {} is at position {:?}",
            scanner.number,
            scanner.position.unwrap()
        );
        for bv in &scanner.beacon_vectors {
            beacon_vectors.insert(bv);
        }
    }

    let max_distance = &scanners
        .iter()
        .combinations(2)
        .map(|pair| pair[0].manhattan_distance(&pair[1]))
        .max()
        .unwrap();

    println!("\nThe total number of beacons is: {}", beacon_vectors.len());
    println!(
        "The max manhattan distance between beacons is: {}",
        max_distance
    );
}
