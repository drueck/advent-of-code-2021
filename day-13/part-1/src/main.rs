// Advent of Code 2021: Day 13, Part 1
// https://adventofcode.com/2021/day/13
// Usage `cargo run <input-file>

use std::{collections::HashSet, env, fs::File, io::BufRead, io::BufReader};

fn fold_up(coordinates: &mut HashSet<(usize, usize)>, line: &usize) {
    let bottom_half_translated: Vec<(usize, usize)> = coordinates
        .iter()
        .filter(|(_, y)| y > line)
        .map(|(x, y)| (*x, *line - (*y - *line)))
        .collect();

    coordinates.retain(|(_, y)| y < line);

    for (x, y) in bottom_half_translated {
        coordinates.insert((x, y));
    }
}

fn fold_left(coordinates: &mut HashSet<(usize, usize)>, line: &usize) {
    let right_half_translated: Vec<(usize, usize)> = coordinates
        .iter()
        .filter(|(x, _)| x > line)
        .map(|(x, y)| (*line - (*x - *line), *y))
        .collect();

    coordinates.retain(|(x, _)| x < line);

    for (x, y) in right_half_translated {
        coordinates.insert((x, y));
    }
}

fn main() {
    let input_file = env::args()
        .nth(1)
        .expect("please supply an input file name");
    let file = File::open(input_file).expect("no such file");
    let reader = BufReader::new(file);

    let mut coordinates: HashSet<(usize, usize)> = HashSet::new();
    let mut folds: Vec<(char, usize)> = vec![];

    let mut lines_iter = reader.lines().map(|l| l.expect("failed to parse line"));

    while let Some(line) = lines_iter.next() {
        if line.trim().is_empty() {
            break;
        }

        let coords: Vec<usize> = line
            .split(",")
            .map(|num| num.parse::<usize>().expect("not an integer"))
            .collect();
        coordinates.insert((coords[0], coords[1]));
    }

    while let Some(line) = lines_iter.next() {
        let parts: Vec<&str> = line[11..].split("=").collect();
        folds.push((
            parts[0].chars().nth(0).expect("not a char"),
            parts[1].parse::<usize>().expect("not an int"),
        ));
    }

    for fold in &folds[0..1] {
        if fold.0 == 'y' {
            fold_up(&mut coordinates, &fold.1);
        } else {
            fold_left(&mut coordinates, &fold.1);
        }
    }

    println!("The number of dots is: {}", coordinates.len());
}
