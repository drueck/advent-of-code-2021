// Advent of Code 2021: Day 21
// https://adventofcode.com/2021/day/21
// Usage `cargo run <input-file>

use std::{env, fs};

#[derive(Debug, Clone)]
struct Player {
    name: String,
    position: usize,
    score: usize,
}

impl Player {
    fn play(&mut self, die: &mut DeterministicDie) {
        let moves = die.roll() + die.roll() + die.roll();
        self.position = wrap_shifted(self.position + moves, 10);
        self.score += self.position;
    }
}

struct DeterministicDie {
    last_roll: usize,
    rolls: usize,
}

impl DeterministicDie {
    fn new() -> Self {
        DeterministicDie {
            last_roll: 100,
            rolls: 0,
        }
    }

    fn roll(&mut self) -> usize {
        self.next().unwrap()
    }
}

impl Iterator for DeterministicDie {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.rolls += 1;
        self.last_roll = wrap_shifted(self.last_roll + 1, 100);

        Some(self.last_roll)
    }
}

fn wrap_shifted(n: usize, max: usize) -> usize {
    ((n - 1) % max) + 1
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
            Player {
                name: parts[0].to_string(),
                position: parts[1].parse().unwrap(),
                score: 0,
            }
        })
        .collect();

    let mut player1 = players[0].clone();
    let mut player2 = players[1].clone();

    let mut die = DeterministicDie::new();

    loop {
        player1.play(&mut die);
        if player1.score >= 1000 {
            println!("player 1 wins!");
            println!("player 2 score: {}", player2.score);
            println!("die rolls: {}", die.rolls);
            println!("result: {}", player2.score * die.rolls);
            break;
        }
        player2.play(&mut die);
        if player2.score >= 1000 {
            println!("player 2 wins!");
            println!("player 1 score: {}", player1.score);
            println!("die rolls: {}", die.rolls);
            println!("result: {}", player1.score * die.rolls);
            break;
        }
    }
}
