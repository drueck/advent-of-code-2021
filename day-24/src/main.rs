// Advent of Code 2021: Day 24
// https://adventofcode.com/2021/day/24
// Usage `cargo run <input-file>

// I ended up attempting to analyze the code by hand to come up with a more optimal algorithm, but
// I was still stuck, so I looked for hints on reddit and came across an explaination of how the
// program is pushing and popping base-26 numbers onto and off of a stack. Once I understood that
// was how it was working, I used my analysis of the problem thus far (a spreadsheet with each
// digit's processing code in a separate column so I could see the unique bits between each, and a
// sort of manual decompilation of the program) and calculated the relationships between the digits
// of a potential model number that would result in an empty stack at the end. I then deduced the
// minimum and maximum model numbers from those relationships and tested them with my simplifed
// version of the program, and they passed, so I tried them and they were correct.
//
// This was the description of the algorithm that I read that unlocked this manual solution. I
// tried not to read it too carefully, but I don't know if I would've figured it out without this:
//
// https://github.com/dphilipson/advent-of-code-2021/blob/master/src/days/day24.rs
//
// In my first commit I had an implementation of the ALU which just executed the program, but of
// course that is not really useful, unfortunately.
//
// Anyway, here are the rules I manually derived for my particular input
//
// D1  == D14 + 8
// D2  == D13 - 7
// D3  ==  D4 + 7
// D5  ==  D6 - 1
// D7  == D12 - 8
// D8  ==  D9 - 5
// D10 == D11

use day_24::MonadVariant;
use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input file");
    let program = fs::read_to_string(input_filename).expect("failed to read input from file");
    let monad = MonadVariant::from_program(&program);

    let maybe_max: isize = 92928914999991;
    let maybe_min: isize = 91811211611981;

    if monad.execute(&maybe_max) == 0 {
        println!("max: {}", maybe_max);
    }
    if monad.execute(&maybe_min) == 0 {
        println!("min: {}", maybe_min);
    }
}
