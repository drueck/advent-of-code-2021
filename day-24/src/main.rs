use day_24::ALU;
use std::env;
use std::fs;

fn main() {
    let filename = env::args().nth(1).expect("please supply an input filename");
    let program = fs::read_to_string(filename).expect("failed to read program from input file");

    let mut alu = ALU::new();

    let model: Vec<isize> = "13579246899999"
        .chars()
        .map(|c| c.to_digit(10).unwrap() as isize)
        .collect();

    match alu.execute(&program, &model) {
        Ok(result) => println!(
            "The model number was {}",
            if result == 0 { "valid" } else { "invalid" }
        ),
        Err(reason) => println!("Program failed: {}", reason),
    }
}
