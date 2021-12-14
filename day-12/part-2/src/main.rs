// Advent of Code 2021: Day 12, Part 2
// https://adventofcode.com/2021/day/12
// Usage `cargo run <input-file>

use std::{collections::HashMap, env, fs::File, io::BufRead, io::BufReader};

#[derive(Debug)]
enum Path {
    DeadEnd,
    Possible(Vec<String>, String),
    Complete(Vec<String>),
}

fn is_small(cave: &str) -> bool {
    cave.to_lowercase() == cave
}

fn small_cave_visited_twice(path: &Path) -> bool {
    match path {
        Path::Possible(caves, _) => {
            let mut counts = HashMap::new();
            for small_cave in caves.iter().filter(|cave| is_small(cave)) {
                *counts.entry(small_cave).or_insert(0) += 1;
            }
            counts.values().max().unwrap() > &1
        }
        _ => false,
    }
}

fn is_dead_end(path: &Path, start: &str) -> bool {
    match path {
        Path::DeadEnd => true,
        Path::Complete(_) => false,
        Path::Possible(visited, current_cave) => {
            (is_small(current_cave)
                && visited.contains(current_cave)
                && small_cave_visited_twice(path))
                || (current_cave.eq(&start) && visited.contains(&start.to_string()))
        }
    }
}

fn all_paths_explored(paths: &Vec<Path>) -> bool {
    paths.iter().all(|path| match path {
        Path::Possible(_, _) => false,
        _ => true,
    })
}

fn find_paths(
    adjacent_caves: &HashMap<String, Vec<String>>,
    start: String,
    end: String,
) -> Vec<Path> {
    let mut paths: Vec<Path> = vec![Path::Possible(vec![], start.clone())];

    loop {
        let mut new_paths: Vec<Path> = vec![];
        for path in paths {
            for new_path in possible_paths(&path, &adjacent_caves, &start, &end) {
                new_paths.push(new_path);
            }
        }
        paths = new_paths;
        if all_paths_explored(&paths) {
            break;
        }
    }

    paths
}

fn possible_paths(
    path: &Path,
    adjacent_caves: &HashMap<String, Vec<String>>,
    start: &str,
    end: &str,
) -> Vec<Path> {
    match path {
        Path::Possible(visited, current_cave) => {
            let mut partial_path = visited.clone();
            if current_cave.eq(&end) {
                partial_path.push(current_cave.to_string());
                vec![Path::Complete(partial_path)]
            } else if is_dead_end(path, &start) {
                vec![Path::DeadEnd]
            } else {
                let mut options: Vec<Path> = vec![];
                for next_cave in &adjacent_caves[current_cave] {
                    let mut new_path = partial_path.clone();
                    new_path.push(current_cave.to_string());
                    options.push(Path::Possible(new_path, next_cave.clone()));
                }
                options
            }
        }
        Path::Complete(complete_path) => vec![Path::Complete(complete_path.to_vec())],
        Path::DeadEnd => vec![Path::DeadEnd],
    }
}

fn main() {
    let input_file: String = env::args()
        .nth(1)
        .expect("please supply an input file name");

    let file = File::open(input_file).expect("no such file");
    let reader = BufReader::new(file);

    let connections: Vec<Vec<String>> = reader
        .lines()
        .map(|l| l.expect("failed to parse line"))
        .map(|s| s.split("-").map(|cave| String::from(cave)).collect())
        .collect();

    let mut adjacent_caves: HashMap<String, Vec<String>> = HashMap::new();

    for i in 0..connections.len() {
        let start_cave = &connections[i][0];
        let end_cave = &connections[i][1];

        adjacent_caves
            .entry(start_cave.to_string())
            .or_insert(vec![])
            .push(end_cave.to_string());
        adjacent_caves
            .entry(end_cave.to_string())
            .or_insert(vec![])
            .push(start_cave.to_string());
    }

    let paths = find_paths(&adjacent_caves, "start".to_string(), "end".to_string());

    let complete_paths: Vec<&Path> = paths
        .iter()
        .filter(|path| match path {
            Path::Complete(_) => true,
            _ => false,
        })
        .collect();

    let num_complete_paths = complete_paths.len();

    println!("The number of paths was: {}", num_complete_paths);
}
