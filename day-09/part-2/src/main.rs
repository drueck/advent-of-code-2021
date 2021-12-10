// Advent of Code 2021: Day 9, Part 2
// https://adventofcode.com/2021/day/9
// Usage `cargo run <input-file>

use std::{collections::HashSet, env, fs::File, io::BufRead, io::BufReader};

type Point = (usize, usize);

fn neighbor_coords(grid: &Vec<Vec<u32>>, row: usize, col: usize) -> Vec<Point> {
    let width = grid[0].len();
    let height = grid.len();

    let mut neighbor_coords: Vec<Point> = vec![];

    // top
    if row > 0 {
        neighbor_coords.push((row - 1, col));
    } // right
    if col < width - 1 {
        neighbor_coords.push((row, col + 1));
    }
    // bottom
    if row < height - 1 {
        neighbor_coords.push((row + 1, col));
    }
    // left
    if col > 0 {
        neighbor_coords.push((row, col - 1));
    }

    neighbor_coords
}

fn lowest_neighbor(grid: &Vec<Vec<u32>>, row: usize, col: usize) -> u32 {
    let neighbor_coords: Vec<Point> = neighbor_coords(&grid, row, col);

    let neighbors: Vec<u32> = neighbor_coords
        .iter()
        .map(|coords| grid[coords.0][coords.1])
        .collect();

    *neighbors.iter().min().expect("neighbors was empty!")
}

fn basin_size(grid: &Vec<Vec<u32>>, row: usize, col: usize) -> usize {
    let mut basin: HashSet<Point> = HashSet::new();
    let mut coords_checked: HashSet<Point> = HashSet::new();
    let mut coords_to_check: HashSet<Point> = HashSet::new();

    coords_to_check.insert((row, col));

    loop {
        let mut new_neighbors: HashSet<Point> = HashSet::new();
        for coords in &coords_to_check {
            coords_checked.insert(*coords);
            for neighbor in neighbor_coords(&grid, coords.0, coords.1)
                .iter()
                .filter(|coords| grid[coords.0][coords.1] != 9)
            {
                new_neighbors.insert(*neighbor);
                basin.insert(*neighbor);
            }
        }
        coords_to_check = new_neighbors
            .difference(&coords_checked)
            .map(|coords| *coords)
            .collect();
        if coords_to_check.is_empty() {
            break;
        }
    }

    basin.len()
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

    let mut low_point_coords: Vec<(usize, usize)> = vec![];

    for row in 0..height {
        for col in 0..width {
            if heights[row][col] < lowest_neighbor(&heights, row, col) {
                low_point_coords.push((row, col));
            }
        }
    }

    let mut basin_sizes: Vec<usize> = low_point_coords
        .iter()
        .map(|coords| basin_size(&heights, coords.0, coords.1))
        .collect();

    basin_sizes.sort();

    let num_basins = basin_sizes.len();
    let biggest_three_basins = &basin_sizes[(num_basins - 3)..num_basins];
    let product: usize = biggest_three_basins.iter().product();

    println!("product of three largest basin sizes: {}", product);
}
