// Advent of Code 2021: Day 23
// https://adventofcode.com/2021/day/23
// Usage `cargo run <input-file>

// test burrow
// #############
// #...........#
// ###B#C#B#D###
//   #A#D#C#A#
//   #########

// 0,0 to 10,2
// cannot move left or right if y is not 2
// cannot stop at (2,2), (4,2), (6,2), or (8,2)
// if you're in a home square:
// try moving to 0,2
// try moving to 1,2
// try moving to 3,2
// try moving to 5,2
// try moving to 7,2
// try moving to 9,2
// try moving to 10,2
// if you can make it there, add that state and cost
// if you can't try the next possibility
//
// if you're in a hallway square
// try moving to your home column depending on your type
// A => (3,0) or (3,1)
// B => (5,0) or (4,1)
// C => (7,0) or (7,1)
// D => (9,0) or (9,1)

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

        for possible_move in &burrow.possible_moves() {
            let new_burrow = burrow.apply(&possible_move);
            priority_queue.push(new_burrow);
        }
    }

    println!("The minimum energy used was {}", min_energy);
}
