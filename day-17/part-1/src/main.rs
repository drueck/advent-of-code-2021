// Advent of Code 2021: Day 17, Part 1
// https://adventofcode.com/2021/day/17
// Usage `cargo run <input-file>

use regex::Regex;
use std::{env, fs, ops::RangeInclusive};

enum LaunchResult {
    Success(isize),
    OvershotX,
    OvershotY,
}

struct Target {
    x_range: RangeInclusive<isize>,
    y_range: RangeInclusive<isize>,
}

struct Probe {
    x: isize,
    y: isize,
    x_velocity: isize,
    y_velocity: isize,
}

impl Probe {
    fn new(x_velocity: isize, y_velocity: isize) -> Self {
        Self {
            x: 0,
            y: 0,
            x_velocity,
            y_velocity,
        }
    }

    fn launch(&mut self, target: &Target) -> LaunchResult {
        let mut max_y = self.y;
        loop {
            self.step();
            if self.y > max_y {
                max_y = self.y;
            }
            if target.x_range.contains(&self.x) && target.y_range.contains(&self.y) {
                return LaunchResult::Success(max_y);
            } else if self.x > *target.x_range.end() {
                return LaunchResult::OvershotX;
            } else if self.y < *target.y_range.start() {
                return LaunchResult::OvershotY;
            }
        }
    }

    fn step(&mut self) {
        self.x += self.x_velocity;
        self.y += self.y_velocity;

        self.x_velocity += if self.x_velocity > 0 {
            -1
        } else if self.x_velocity < 0 {
            1
        } else {
            0
        };

        self.y_velocity -= 1;
    }
}

// I think the minimum viable initial x velocity is x such that the sum of the first x whole
// numbers is the lowest one in the range of the target zone. This is also the one I feel is most
// likely to produce the optimal height as it will be farthest to the left in the target zone. I'm
// not 100% confident on this, but it worked at least for the sample input.
fn minimum_x(x_start: isize, x_end: isize) -> isize {
    let mut x: isize = 0;

    while x < x_start {
        let sum_of_first_n = (x + 1) * x / 2;
        if sum_of_first_n >= x_start && sum_of_first_n <= x_end {
            return x;
        }
        x += 1;
    }
    x
}

fn main() {
    let input_file = env::args().nth(1).expect("please supply an input file");
    let input = fs::read_to_string(input_file).unwrap();

    let input_regex = Regex::new(r"x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
    let captures = input_regex.captures(&input).unwrap();

    let (x1, x2): (isize, isize) = (
        captures.get(1).unwrap().as_str().parse().unwrap(),
        captures.get(2).unwrap().as_str().parse().unwrap(),
    );
    let (y1, y2): (isize, isize) = (
        captures.get(3).unwrap().as_str().parse().unwrap(),
        captures.get(4).unwrap().as_str().parse().unwrap(),
    );

    let target = Target {
        x_range: x1..=x2,
        y_range: y1..=y2,
    };

    let mut max_y: isize = 0;
    for x_velocity in minimum_x(x1, x2)..=x2 {
        // This guess for the range of possible initial y velocities is based on some observations
        // about the positions of the steps. For any given y launched upward, it always passes
        // through the origin on its way down. If the next step is greater than the distance from
        // the origin to the bottom of the target area, then we will overshoot the target, so we
        // don't need to consider values larger than that.
        for y_velocity in 0..=(y1.abs()) {
            let mut probe = Probe::new(x_velocity, y_velocity);
            if let LaunchResult::Success(local_max_y) = probe.launch(&target) {
                if local_max_y > max_y {
                    max_y = local_max_y;
                }
            }
        }
    }

    println!(
        "The max y encountered in successful launch simulations was {}",
        max_y
    );
}
