// Advent of Code 2021: Day 20
// https://adventofcode.com/2021/day/20
// Usage `cargo run <input-file> <steps>

use std::{collections::HashSet, env, fmt, fs};

type Image = HashSet<(isize, isize)>;

struct InfiniteImage {
    top: isize,
    bottom: isize,
    left: isize,
    right: isize,
    image: Image,
    background: char,
    algorithm: Vec<char>,
}

impl InfiniteImage {
    fn new(image_vec: &Vec<Vec<char>>, algorithm: &Vec<char>) -> Self {
        let mut image: Image = HashSet::new();

        let top = 0 as isize;
        let bottom = (image_vec.len() - 1) as isize;
        let left = 0 as isize;
        let right = (image_vec[0].len() - 1) as isize;

        for row in top..=bottom {
            for col in left..=right {
                match image_vec[row as usize][col as usize] {
                    '#' => {
                        image.insert((row, col));
                    }
                    _ => (),
                }
            }
        }

        InfiniteImage {
            top,
            bottom,
            left,
            right,
            image,
            background: '.',
            algorithm: algorithm.clone(),
        }
    }

    fn algorithm_index_for(&self, (x, y): (isize, isize)) -> usize {
        let mut binary_string = String::with_capacity(9);
        for row in (x - 1)..=(x + 1) {
            for col in (y - 1)..=(y + 1) {
                // handle background case
                let digit = if (self.top..=self.bottom).contains(&row)
                    && (self.left..=self.right).contains(&col)
                {
                    match self.image.contains(&(row, col)) {
                        true => "1",
                        false => "0",
                    }
                } else {
                    match self.background {
                        '#' => "1",
                        _ => "0",
                    }
                };
                binary_string.push_str(digit);
            }
        }
        usize::from_str_radix(&binary_string, 2).expect("programming error")
    }

    fn expand(&mut self) {
        self.top -= 1;
        self.left -= 1;
        self.bottom += 1;
        self.right += 1;
    }

    fn enhance(&mut self) {
        let mut new_image: Image = HashSet::new();

        for row in (self.top - 1)..=(self.bottom + 1) {
            for col in (self.left - 1)..=(self.right + 1) {
                match self.algorithm[self.algorithm_index_for((row, col))] {
                    '#' => {
                        new_image.insert((row, col));
                    }
                    _ => (),
                }
            }
        }

        self.expand();
        self.image = new_image;
        self.background = self.algorithm[self.algorithm_index_for((self.top - 2, self.left - 2))];
    }

    fn lit_pixels(&self) -> usize {
        self.image.len()
    }
}

impl fmt::Display for InfiniteImage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n")?;
        for row in self.top..=self.bottom {
            for col in self.left..=self.right {
                let pixel = match self.image.contains(&(row, col)) {
                    true => "#",
                    false => ".",
                };
                write!(f, "{}", pixel)?;
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")
    }
}

fn main() {
    let input_file = env::args().nth(1).expect("please supply an input file");
    let steps: usize = env::args()
        .nth(2)
        .expect("please supply the number of enhance steps")
        .parse()
        .expect("steps must be a positive integer");
    let input = fs::read_to_string(input_file).expect("failed to read input from given file");
    let input_parts: Vec<&str> = input.split("\n\n").collect();

    let algorithm = input_parts[0].chars().collect();
    let image_vec: Vec<Vec<char>> = input_parts[1]
        .trim()
        .split("\n")
        .map(|line| line.chars().collect())
        .collect();

    let mut image = InfiniteImage::new(&image_vec, &algorithm);

    for _ in 0..steps {
        image.enhance();
    }

    println!("Total number of lit pixels are: {}", image.lit_pixels());
}
