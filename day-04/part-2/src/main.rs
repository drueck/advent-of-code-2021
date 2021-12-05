// Advent of Code 2021: Day 4, Part 2
// https://adventofcode.com/2021/day/4
// Usage `cargo run <input-file>

use std::{env, fs::File, io::BufRead, io::BufReader};

type Square = (usize, bool);
type Board = Vec<Vec<Square>>;
type Boards = Vec<Board>;
type Numbers = Vec<usize>;

fn display(board: &Board) {
    for row in 0..5 {
        for col in 0..5 {
            let (val, marked) = board[row][col];
            let marked_char = if marked { 'x' } else { ' ' };
            print!("{:>2}{} ", val, marked_char);
        }
        print!("\n");
    }
}

fn mark(board: &mut Board, number: usize) {
    for row in 0..5 {
        for col in 0..5 {
            match board[row][col] {
                (n, _) if n == number => board[row][col] = (number, true),
                (_, _) => (),
            }
        }
    }
}

fn has_won(board: &Board) -> bool {
    // check for row wins
    for row in 0..5 {
        if board[row].iter().all(|(_, marked)| *marked) {
            return true;
        }
    }

    // check for col wins
    for col in 0..5 {
        let mut win = true;
        for row in 0..5 {
            if let (_, false) = board[row][col] {
                win = false
            }
        }
        if win {
            return true;
        }
    }

    false
}

fn play(numbers: Numbers, mut boards: Boards) -> Option<(Board, usize)> {
    for number in numbers {
        for board_num in 0..boards.len() {
            mark(&mut boards[board_num], number);
        }
        if boards.len() == 1 && has_won(&boards[0]) {
            return Some((boards[0].clone(), number));
        }
        boards.retain(|board| !has_won(&board))
    }

    None
}

fn score(board: &Board) -> usize {
    let mut sum_of_unmarked = 0;
    for row in 0..5 {
        for col in 0..5 {
            if let (num, false) = board[row][col] {
                sum_of_unmarked += num;
            }
        }
    }
    sum_of_unmarked
}

fn parse_input(input_file: &str) -> (Numbers, Boards) {
    let file = File::open(input_file).expect("no such file");
    let buf = BufReader::new(file);
    let mut lines_iter = buf.lines().map(|l| l.expect("could not parse line"));

    let numbers: Numbers = match lines_iter.next() {
        Some(bingo_numbers) => bingo_numbers
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect(),
        None => panic!("Unexpected end of file"),
    };

    let mut boards: Boards = vec![];
    let mut current_board: Board = Vec::with_capacity(5);

    while let Some(s) = lines_iter.next() {
        if s.is_empty() {
            if current_board.len() > 0 {
                boards.push(current_board);
                current_board = Vec::with_capacity(5);
            }
        } else {
            let row: Vec<Square> = s
                .split_whitespace()
                .map(|num_str| match num_str.parse::<usize>() {
                    Ok(num) => num,
                    Err(e) => panic!("found a non-integer: {}", e),
                })
                .map(|num| (num, false))
                .collect();
            current_board.push(row);
        }
    }
    boards.push(current_board);

    (numbers, boards)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file: &String = &args[1];

    let (numbers, boards) = parse_input(input_file);

    match play(numbers, boards) {
        Some((last_winning_board, last_number)) => {
            println!("The last winning board was\n");
            display(&last_winning_board);

            let board_score = score(&last_winning_board);
            println!("");
            println!("Board score: {}", board_score);
            println!("Last number: {}", last_number);
            println!("Final score: {}", board_score * last_number);
        }
        None => println!("Nobody won, apparently!"),
    }
}
