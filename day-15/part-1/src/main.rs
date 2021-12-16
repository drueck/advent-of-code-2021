// Advent of Code 2021: Day 15, Part 1
// https://adventofcode.com/2021/day/15
// Usage `cargo run <input-file>

use std::{cmp, collections::HashSet, env, fs::File, io::BufRead, io::BufReader};

fn unvisited_neighbors(
    node: &(usize, usize),
    unvisited_set: &HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let (row, col) = *node;
    let mut possible_neighbors: HashSet<(usize, usize)> = HashSet::new();
    if row > 0 {
        possible_neighbors.insert((row - 1, col));
    }
    possible_neighbors.insert((row, col + 1));
    possible_neighbors.insert((row + 1, col));
    if col > 0 {
        possible_neighbors.insert((row, col - 1));
    }

    possible_neighbors
        .intersection(unvisited_set)
        .map(|pair| *pair)
        .collect()
}

fn main() {
    let input_file = env::args().nth(1).expect("please supply the input file");
    let file = File::open(input_file).expect("no such file");
    let reader = BufReader::new(file);

    let risk_level_grid: Vec<Vec<usize>> = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let mut unvisited_set: HashSet<(usize, usize)> = HashSet::new();
    let mut lowest_costs = risk_level_grid.clone();

    let width = risk_level_grid[0].len();
    let height = risk_level_grid.len();

    for row in 0..height {
        for col in 0..width {
            unvisited_set.insert((row, col));
            lowest_costs[row][col] = usize::MAX;
        }
    }
    lowest_costs[0][0] = 0;

    let mut current: (usize, usize) = (0, 0);

    loop {
        // update all unvisited neighbors
        for neighbor in unvisited_neighbors(&current, &unvisited_set) {
            let current_node_cost = lowest_costs[current.0][current.1];
            let existing_neighbor_cost = lowest_costs[neighbor.0][neighbor.1];
            let cost_via_current_node = current_node_cost + risk_level_grid[neighbor.0][neighbor.1];

            let best = cmp::min(existing_neighbor_cost, cost_via_current_node);

            lowest_costs[neighbor.0][neighbor.1] = best;
        }

        // mark the current node as visited
        unvisited_set.remove(&current);

        // if we've reached the target destination, break
        if current == (height - 1, width - 1) {
            break;
        }

        current = *unvisited_set
            .iter()
            .min_by_key(|(row, col)| lowest_costs[*row][*col])
            .unwrap();
    }

    println!(
        "Lowest cost to current: {}",
        lowest_costs[current.0][current.1]
    );
}
