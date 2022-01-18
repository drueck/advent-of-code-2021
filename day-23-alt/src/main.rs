// Advent of Code 2021: Day 23
// https://adventofcode.com/2021/day/23
// Usage `cargo run <input-file>

use day_23_alt::{Burrow, Kind, Position};
use std::collections::{BinaryHeap, HashMap};
use std::env;
use std::fs;

fn main() {
    let filename = env::args().nth(1).expect("please provide an input file");
    let input = fs::read_to_string(filename).expect("no such file");
    let burrow = Burrow::new(&input);

    let mut least_energy_used = usize::MAX;
    let mut min_energies: HashMap<Vec<(Position, Kind)>, usize> = HashMap::new();
    let mut queue: BinaryHeap<Burrow> = BinaryHeap::new();

    queue.push(burrow.clone());

    while let Some(mut burrow) = queue.pop() {
        burrow.move_into_rooms();
        if burrow.energy_used > least_energy_used {
            continue;
        }

        if burrow.organized() {
            if burrow.energy_used < least_energy_used {
                least_energy_used = burrow.energy_used;
                continue;
            }
        }

        for move_into_hallway in burrow.moves_into_hallway() {
            let mut new_burrow = burrow.clone();
            new_burrow.apply(move_into_hallway);

            let min_energy_for_burrow =
                min_energies.entry(new_burrow.state()).or_insert(usize::MAX);
            if new_burrow.energy_used >= least_energy_used
                || new_burrow.energy_used >= *min_energy_for_burrow
            {
                continue;
            }

            *min_energy_for_burrow = new_burrow.energy_used;
            queue.push(new_burrow);
        }
    }

    if least_energy_used < usize::MAX {
        println!("The least energy used was: {}", least_energy_used);
    } else {
        println!("Did not organize the burrow. :(");
    }
}
