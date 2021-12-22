// Advent of Code 2021: Day 18, Part 2
// https://adventofcode.com/2021/day/18
// Usage `cargo run <input-file>

use std::{env, fmt, fs::File, io::BufRead, io::BufReader};

enum Token {
    Number(u32),
    OpenBracket,
    CloseBracket,
    Comma,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(n) => f.write_str(&n.to_string()),
            Token::OpenBracket => f.write_str("["),
            Token::CloseBracket => f.write_str("]"),
            Token::Comma => f.write_str(","),
        }
    }
}

struct SnailfishNumber {
    number: Vec<Token>,
    pointer: usize,
}

impl fmt::Debug for SnailfishNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut number_string = String::with_capacity(self.number.len());
        for token in &self.number {
            number_string.push_str(&format!("{:?}", token))
        }

        f.debug_struct("SnailfishNumber")
            .field("number", &number_string)
            .field("pointer", &self.pointer)
            .finish()
    }
}

impl SnailfishNumber {
    fn new(s: &str) -> SnailfishNumber {
        let number = s
            .chars()
            .map(|c| match c {
                '[' => Token::OpenBracket,
                ']' => Token::CloseBracket,
                ',' => Token::Comma,
                n => Token::Number(n.to_digit(10).expect("not a number")),
            })
            .collect();

        SnailfishNumber { number, pointer: 0 }
    }

    fn magnitude(&mut self) -> u32 {
        while self.reduce_magnitudes() {}
        match self.number[0] {
            Token::Number(magnitude) => magnitude,
            _ => {
                panic!("failed to get a single magnitude");
            }
        }
    }

    fn reduce_magnitudes(&mut self) -> bool {
        if self.number.len() == 1 {
            return false;
        }

        self.pointer = 0;
        let mut reductions = 0;
        while self.pointer < self.number.len() {
            match self.number[self.pointer] {
                Token::Number(left) => {
                    if self.pointer + 2 >= self.number.len() {
                        break;
                    }
                    if let Token::Number(right) = self.number[self.pointer + 2] {
                        reductions += 1;
                        self.number.splice(
                            (self.pointer - 1)..(self.pointer + 4),
                            vec![Token::Number(left * 3 + right * 2)],
                        );
                        self.pointer += 1;
                    } else {
                        self.pointer += 1;
                    }
                }
                _ => {
                    self.pointer += 1; // to the next token
                }
            }
        }
        return reductions > 0;
    }

    fn add(&mut self, other: &mut SnailfishNumber) {
        self.number.insert(0, Token::OpenBracket);
        self.number.push(Token::Comma);
        self.number.append(&mut other.number);
        self.number.push(Token::CloseBracket);
        self.reduce();
    }

    fn reduce(&mut self) {
        loop {
            if self.explode() {
                continue;
            }
            if self.split() {
                continue;
            }
            break;
        }
    }

    fn explode(&mut self) -> bool {
        self.pointer = 0;
        let mut depth = 0;
        while self.pointer < self.number.len() {
            match self.number[self.pointer] {
                Token::OpenBracket => {
                    depth += 1;
                }
                Token::CloseBracket => {
                    depth -= 1;
                }
                Token::Comma => (),
                Token::Number(left_of_pair) => {
                    if depth == 5 {
                        self.explode_left(&left_of_pair);
                        if let Token::Number(right_of_pair) = self.number[self.pointer + 2] {
                            self.explode_right(&right_of_pair);
                        } else {
                            panic!("programming error in explode");
                        }
                        self.number.splice(
                            (self.pointer - 1)..(self.pointer + 4),
                            vec![Token::Number(0)],
                        );
                        return true;
                    }
                }
            }
            self.pointer += 1;
        }
        false
    }

    fn explode_left(&mut self, number: &u32) {
        if let Some(left_index) = self.previous_number_index() {
            if let Token::Number(left_number) = self.number[left_index] {
                self.number[left_index] = Token::Number(left_number + number);
            } else {
                panic!("programming error in explode_left")
            }
        }
    }

    fn explode_right(&mut self, number: &u32) {
        if let Some(right_index) = self.next_number_index() {
            if let Token::Number(right_number) = self.number[right_index] {
                self.number[right_index] = Token::Number(right_number + number);
            } else {
                panic!("programming error in explode_right")
            }
        }
    }

    fn previous_number_index(&self) -> Option<usize> {
        let mut i = self.pointer - 1;
        loop {
            if let Token::Number(_) = self.number[i] {
                return Some(i);
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
        None
    }

    fn next_number_index(&self) -> Option<usize> {
        let mut i = self.pointer + 4; // after tuple closing bracket
        loop {
            if let Token::Number(_) = self.number[i] {
                return Some(i);
            }
            if i == self.number.len() - 1 {
                break;
            }
            i += 1;
        }
        None
    }

    fn split(&mut self) -> bool {
        self.pointer = 0;
        while self.pointer < self.number.len() {
            match self.number[self.pointer] {
                Token::Number(number) if number > 9 => {
                    let middle = number as f32 / 2 as f32;
                    let left = Token::Number(middle.floor() as u32);
                    let right = Token::Number(middle.ceil() as u32);
                    let replacement = vec![
                        Token::OpenBracket,
                        left,
                        Token::Comma,
                        right,
                        Token::CloseBracket,
                    ];

                    self.number
                        .splice(self.pointer..self.pointer + 1, replacement);
                    return true;
                }
                _ => {
                    self.pointer += 1;
                }
            }
        }
        false
    }
}

fn main() {
    let input_file = env::args().nth(1).expect("please supply an input file");
    let file = File::open(input_file).expect("no such file");
    let reader = BufReader::new(file);

    let snailfish_numbers: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let mut largest_magnitude = 0;

    for a in 0..snailfish_numbers.len() {
        for b in 0..snailfish_numbers.len() {
            if a == b {
                continue;
            }
            let mut number_a = SnailfishNumber::new(&snailfish_numbers[a]);
            let mut number_b = SnailfishNumber::new(&snailfish_numbers[b]);

            number_a.add(&mut number_b);
            let magnitude = number_a.magnitude();
            if magnitude > largest_magnitude {
                largest_magnitude = magnitude;
            }
        }
    }

    println!("The largest magnitude is: {}", largest_magnitude);
}
