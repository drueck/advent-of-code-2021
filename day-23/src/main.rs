// Advent of Code 2021: Day 23
// https://adventofcode.com/2021/day/23
// Usage `cargo run <input-file>

use day_23::Burrow;
use std::collections::{BinaryHeap, HashMap};
use std::{env, fs};

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = fs::read_to_string(filename).unwrap();
    let initial_burrow = Burrow::new(&input);

    let mut min_energies: HashMap<Vec<((usize, usize), char)>, usize> = HashMap::new();
    let mut priority_queue: BinaryHeap<Burrow> = BinaryHeap::new();
    let mut min_energy = usize::MAX;

    println!("{}", initial_burrow);

    priority_queue.push(initial_burrow);

    while let Some(burrow) = priority_queue.pop() {
        if burrow.energy_used > min_energy {
            break;
        }

        let amphipod_locations: Vec<((usize, usize), char)> =
            burrow.map.clone().into_iter().collect();

        let min_energy_for_burrow = min_energies.entry(amphipod_locations).or_insert(usize::MAX);

        if burrow.energy_used >= *min_energy_for_burrow {
            continue;
        }

        *min_energy_for_burrow = burrow.energy_used;

        if burrow.organized() && burrow.energy_used < min_energy {
            min_energy = burrow.energy_used;
            continue;
        }

        for next_move in &burrow.next_moves() {
            let new_burrow = burrow.apply(&next_move);
            priority_queue.push(new_burrow);
        }
    }

    println!("The minimum energy used was {}", min_energy);
}
