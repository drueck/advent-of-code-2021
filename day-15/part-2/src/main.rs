// Advent of Code 2021: Day 15, Part 2
// https://adventofcode.com/2021/day/15
// Usage `cargo run <input-file>
//
// TL;DR I kinda cheated on this one.
//
// I didn't know how to solve this problem, so I did a lot of research on various algorithms that
// are designed to solve this type of thing. I came across Dijkstra's Algorithm pretty quickly and
// used the Wikipedia entry for it as a reference to implement my solution for part 1. That article
// and section are linked here:
//
// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Algorithm
//
// However, when I tried my part 1 solution with the much larger input for part 2, I discovered
// that my implementation was not nearly efficient enough, so I went back to googling the algorithms.
// I read that one of the typical data structures used for this is a priority queue, so I looked that
// up in the Rust docs to see if it existed in the standard library. In the process of doing that I
// found an article in the docs which described exactly how to use a BinaryHeap (part of the
// standard library) as a priority queue specifically to implement Dijkstra's Algorithm. This docs
// page was essentially the hard part of the solution. So, this part 2 code is borrowed/stolen
// liberally from that docs page solution. That docs page is linked here:
//
// https://doc.rust-lang.org/std/collections/binary_heap/index.html

use std::{cmp::Ordering, collections::BinaryHeap, env, fs::File, io::BufRead, io::BufReader};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Cell {
    cost: usize,
    position: (usize, usize),
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn wrap(n: usize) -> usize {
    let digits = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    digits[(n - 1) % 9]
}

fn neighbors(position: &(usize, usize), risk_level_grid: &Vec<Vec<usize>>) -> Vec<Cell> {
    let (row, col) = *position;
    let width = risk_level_grid[0].len();
    let height = risk_level_grid.len();

    let mut possible_neighbors: Vec<Cell> = vec![];

    if row > 0 {
        possible_neighbors.push(Cell {
            position: (row - 1, col),
            cost: risk_level_grid[row - 1][col],
        });
    }
    if col + 1 < width {
        possible_neighbors.push(Cell {
            position: (row, col + 1),
            cost: risk_level_grid[row][col + 1],
        })
    }
    if row + 1 < height {
        possible_neighbors.push(Cell {
            position: (row + 1, col),
            cost: risk_level_grid[row + 1][col],
        })
    }
    if col > 0 {
        possible_neighbors.push(Cell {
            position: (row, col - 1),
            cost: risk_level_grid[row][col - 1],
        });
    }

    possible_neighbors
}

fn main() {
    let input_file = env::args().nth(1).expect("please supply the input file");
    let file = File::open(input_file).expect("no such file");
    let reader = BufReader::new(file);

    let input: Vec<Vec<usize>> = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let input_width = input[0].len();
    let input_height = input.len();

    let mut risk_level_grid: Vec<Vec<usize>> = vec![vec![0; 5 * input_width]; 5 * input_height];

    for row in 0..input_height {
        for col in 0..input_width {
            for col_multiplier in 0..5 {
                for row_multiplier in 0..5 {
                    risk_level_grid[row + (row_multiplier * input_height)]
                        [col + (col_multiplier * input_width)] =
                        wrap(input[row][col] + col_multiplier + row_multiplier);
                }
            }
        }
    }

    let mut lowest_costs = risk_level_grid.clone();

    let width = risk_level_grid[0].len();
    let height = risk_level_grid.len();

    for row in 0..height {
        for col in 0..width {
            // unvisited_set.insert((row, col));
            lowest_costs[row][col] = usize::MAX;
        }
    }
    lowest_costs[0][0] = 0;

    let mut heap = BinaryHeap::new();

    heap.push(Cell {
        cost: 0,
        position: (0, 0),
    });

    let mut final_cost: usize = 0;

    while let Some(Cell { cost, position }) = heap.pop() {
        if position == (height - 1, width - 1) {
            final_cost = cost;
            break;
        }

        if cost > lowest_costs[position.0][position.1] {
            continue;
        }

        for neighbor in neighbors(&position, &risk_level_grid) {
            let next = Cell {
                cost: cost + neighbor.cost,
                position: neighbor.position,
            };

            if next.cost < lowest_costs[next.position.0][next.position.1] {
                heap.push(next);
                lowest_costs[next.position.0][next.position.1] = next.cost;
            }
        }
    }

    println!("Lowest cost to end: {}", final_cost);
}
