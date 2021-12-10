// Advent of Code 2021: Day 9, Part 1
// https://adventofcode.com/2021/day/9
// Usage `cargo run <input-file>

use std::{env, fs::File, io::BufRead, io::BufReader};

fn lowest_neighbor(grid: &Vec<Vec<u32>>, row: usize, col: usize) -> u32 {
    let width = grid[0].len();
    let height = grid.len();

    let mut neighbor_coords: Vec<Vec<usize>> = vec![];

    // top
    if row > 0 {
        neighbor_coords.push(vec![row - 1, col]);
    }
    // right
    if col < width - 1 {
        neighbor_coords.push(vec![row, col + 1]);
    }
    // bottom
    if row < height - 1 {
        neighbor_coords.push(vec![row + 1, col]);
    }
    // left
    if col > 0 {
        neighbor_coords.push(vec![row, col - 1]);
    }

    let neighbors: Vec<u32> = neighbor_coords
        .iter()
        .map(|coords| grid[coords[0]][coords[1]])
        .collect();

    *neighbors.iter().min().expect("neighbors was empty!")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file: &String = &args[1];

    let file = File::open(input_file).expect("no such file");
    let buf = BufReader::new(file);
    let heights: Vec<Vec<u32>> = buf
        .lines()
        .map(|l| l.expect("could not parse line"))
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).expect("not a digit"))
                .collect()
        })
        .collect();

    let width = heights[0].len();
    let height = heights.len();

    let mut low_points: Vec<u32> = vec![];

    for row in 0..height {
        for col in 0..width {
            if heights[row][col] < lowest_neighbor(&heights, row, col) {
                low_points.push(heights[row][col]);
            }
        }
    }

    let risk_levels: Vec<u32> = low_points.iter().map(|low_point| low_point + 1).collect();

    println!("sum of risk levels: {}", risk_levels.iter().sum::<u32>());
}
