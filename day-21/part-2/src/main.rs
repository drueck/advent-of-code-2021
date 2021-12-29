// Advent of Code 2021: Day 21, Part 2
// https://adventofcode.com/2021/day/21
// Usage `cargo run <input-file>

// Added caching, but still pretty slow: 18+ seconds.

use cached::proc_macro::cached;
use std::{collections::HashMap, env, fs};

// the first number is the total of 3 rolls of d3
// the second number can be multiplied times the number of
// universes before the roll to get the number of universes in which this total occurred
const ROLLS_MULTIPLIERS: [(usize, usize); 7] =
    [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Player {
    number: usize,
    position: usize,
    score: usize,
}

impl Player {
    fn new(number: usize, position: usize) -> Self {
        Player {
            number,
            position,
            score: 0,
        }
    }

    fn play(&self, total_roll: usize) -> Self {
        let mut copy = self.clone();
        copy.position = wrap_to_one(self.position + total_roll, 10);
        copy.score += copy.position;
        copy
    }
}

fn wrap_to_one(n: usize, max: usize) -> usize {
    ((n - 1) % max) + 1
}

#[cached]
fn play(
    current_player: Player,
    previous_player: Player,
    universes: usize,
) -> HashMap<usize, usize> {
    let mut results: HashMap<usize, usize> = HashMap::new();

    if previous_player.score >= 21 {
        results.insert(previous_player.number, universes);
    } else {
        for (total_roll, multiplier) in ROLLS_MULTIPLIERS {
            let result = play(
                previous_player,
                current_player.play(total_roll),
                universes * multiplier,
            );

            if let Some(cp_universes) = result.get(&current_player.number) {
                *results.entry(current_player.number).or_default() += cp_universes;
            }
            if let Some(pp_universes) = result.get(&previous_player.number) {
                *results.entry(previous_player.number).or_default() += pp_universes;
            }
        }
    }

    results
}

fn main() {
    let input_file = env::args().nth(1).expect("please supply an input filename");
    let input =
        fs::read_to_string(input_file).expect("failed to read the input from the given file");

    let players: Vec<Player> = input
        .trim()
        .split("\n")
        .map(|line| {
            let parts: Vec<&str> = line.split(" starting position: ").collect();
            let number: usize = parts[0].split(" ").last().unwrap().parse().unwrap();
            let starting_position = parts[1].parse().unwrap();
            Player::new(number, starting_position)
        })
        .collect();

    let player_1 = players[0];
    let player_2 = players[1];

    let results = play(player_1, player_2, 1);
    let (player, universes) = results.iter().max_by_key(|key_value| key_value.1).unwrap();

    println!("Player {} won in {} universes", player, universes);
}
