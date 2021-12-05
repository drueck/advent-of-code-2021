// Advent of Code 2021: Day 5, Part 1
// https://adventofcode.com/2021/day/5
// Usage `cargo run <input-file>

use std::{collections::HashMap, env, fs::File, io::BufRead, io::BufReader};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn from_str(point: &str) -> Point {
        let coords: Vec<usize> = point
            .split(",")
            .map(|n| n.parse::<usize>().expect("not an int!"))
            .collect();

        Point {
            x: coords[0],
            y: coords[1],
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn from_str(line: &str) -> Line {
        let points: Vec<Point> = line
            .split(" -> ")
            .map(|point| Point::from_str(point))
            .collect();

        Line {
            start: points[0],
            end: points[1],
        }
    }

    fn is_diagonal(&self) -> bool {
        self.start.x != self.end.x && self.start.y != self.end.y
    }

    fn all_points(&self) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];
        if !self.is_diagonal() {
            let mut y_endpoints = [self.start.y, self.end.y];
            let mut x_endpoints = [self.start.x, self.end.x];
            y_endpoints.sort();
            x_endpoints.sort();
            for x in x_endpoints[0]..=x_endpoints[1] {
                for y in y_endpoints[0]..=y_endpoints[1] {
                    points.push(Point { x, y })
                }
            }
        }
        points
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file: &String = &args[1];

    let file = File::open(input_file).expect("no such file");
    let buf = BufReader::new(file);
    let non_diagonal_lines: Vec<Line> = buf
        .lines()
        .map(|l| l.expect("could not parse line"))
        .map(|s| Line::from_str(&s))
        .filter(|line| !line.is_diagonal())
        .collect();

    let mut vent_map: HashMap<Point, usize> = HashMap::new();

    for line in non_diagonal_lines {
        for point in line.all_points() {
            *vent_map.entry(point).or_insert(0) += 1
        }
    }

    let danger_point_count = vent_map
        .values()
        .filter(|&lines_intersecting| *lines_intersecting > 1)
        .count();

    println!(
        "The number of points where two or more lines cross is {}",
        danger_point_count
    );
}
