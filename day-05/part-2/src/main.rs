// Advent of Code 2021: Day 5, Part 2
// https://adventofcode.com/2021/day/5
// Usage `cargo run <input-file>

use std::{collections::HashMap, env, fs::File, io::BufRead, io::BufReader};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn from_str(point: &str) -> Point {
        let coords: Vec<isize> = point
            .split(",")
            .map(|n| n.parse::<isize>().expect("not an int!"))
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
        if self.is_diagonal() {
            let mut x = self.start.x;
            let mut y = self.start.y;
            let x_step: isize = if self.end.x > self.start.x { 1 } else { -1 };
            let y_step: isize = if self.end.y > self.start.y { 1 } else { -1 };

            loop {
                points.push(Point { x, y });
                if x == self.end.x {
                    break;
                }
                x += x_step;
                y += y_step;
            }
        } else {
            let mut x_endpoints = [self.start.x, self.end.x];
            let mut y_endpoints = [self.start.y, self.end.y];
            x_endpoints.sort();
            y_endpoints.sort();

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
    let lines: Vec<Line> = buf
        .lines()
        .map(|l| l.expect("could not parse line"))
        .map(|s| Line::from_str(&s))
        .collect();

    let mut vent_map: HashMap<Point, isize> = HashMap::new();

    for line in lines {
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
