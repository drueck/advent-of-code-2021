// Advent of Code 2021: Day 11, Part 2
// https://adventofcode.com/2021/day/11
// Usage `cargo run <input-file>

use std::{env, fs::File, io::BufRead, io::BufReader};

type Coordinates = (usize, usize);

fn adjacent_cells(energy_levels: &Vec<Vec<u32>>, cell: Coordinates) -> Vec<Coordinates> {
    let rows = energy_levels[0].len() as isize;
    let cols = energy_levels.len() as isize;

    let row = cell.0 as isize;
    let col = cell.1 as isize;

    vec![
        (row - 1, col),     // top
        (row - 1, col + 1), // top right
        (row, col + 1),     // right
        (row + 1, col + 1), // bottom right
        (row + 1, col),     // bottom
        (row + 1, col - 1), // bottom left
        (row, col - 1),     // left
        (row - 1, col - 1), // top left
    ]
    .iter()
    .filter(|(row, col)| *row >= 0 && *row < rows && *col >= 0 && *col < cols)
    .map(|(row, col)| (*row as usize, *col as usize))
    .collect()
}

fn handle_flashes(energy_levels: &mut Vec<Vec<u32>>) -> usize {
    let rows = energy_levels[0].len();
    let cols = energy_levels.len();

    let mut flashes: usize = 0;

    for row in 0..rows {
        for col in 0..cols {
            if energy_levels[row][col] == 10 {
                flashes += 1;
                energy_levels[row][col] += 1; // set to 11 to mark that this one flashed
                let adjacent_cells = adjacent_cells(&energy_levels, (row, col));
                for (row, col) in adjacent_cells {
                    if energy_levels[row][col] < 10 {
                        energy_levels[row][col] += 1;
                    }
                }
            }
        }
    }

    flashes
}

fn main() {
    let input_file: String = env::args()
        .nth(1)
        .expect("please supply the input file name");

    let file = File::open(input_file).expect("no such file");
    let reader = BufReader::new(file);

    let mut energy_levels: Vec<Vec<u32>> = reader
        .lines()
        .map(|l| l.expect("failed to parse line"))
        .map(|s| {
            s.chars()
                .map(|char| char.to_digit(10).expect("not a digit"))
                .collect()
        })
        .collect();

    let rows = energy_levels[0].len();
    let cols = energy_levels.len();
    let total_octopi = rows * cols;
    let mut step = 0;

    loop {
        step += 1;

        let mut step_flashes: usize = 0;

        for row in 0..rows {
            for col in 0..cols {
                energy_levels[row][col] += 1;
            }
        }

        loop {
            let new_flashes = handle_flashes(&mut energy_levels);
            if new_flashes == 0 {
                break;
            }
            step_flashes += new_flashes;
        }

        if step_flashes == total_octopi {
            break;
        }

        for row in 0..rows {
            for col in 0..cols {
                if energy_levels[row][col] > 9 {
                    energy_levels[row][col] = 0;
                }
            }
        }
    }

    println!("All the octopi flashed on step {}", step);
}
