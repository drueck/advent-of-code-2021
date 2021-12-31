use crate::cube::Cube;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref INSTRUCTION_REGEX: Regex =
        Regex::new(r"(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap();
}

pub enum Operation {
    On,
    Off,
}

pub struct Instruction {
    pub operation: Operation,
    pub cube: Cube,
}

impl Instruction {
    pub fn new(instruction: &str) -> Self {
        let captures = INSTRUCTION_REGEX.captures(instruction).unwrap();

        let operation = match captures.get(1).unwrap().as_str() {
            "on" => Operation::On,
            _ => Operation::Off,
        };

        let min_x: isize = captures.get(2).unwrap().as_str().parse().unwrap();
        let max_x: isize = captures.get(3).unwrap().as_str().parse().unwrap();
        let min_y: isize = captures.get(4).unwrap().as_str().parse().unwrap();
        let max_y: isize = captures.get(5).unwrap().as_str().parse().unwrap();
        let min_z: isize = captures.get(6).unwrap().as_str().parse().unwrap();
        let max_z: isize = captures.get(7).unwrap().as_str().parse().unwrap();

        let cube = Cube::new((min_x, max_x + 1), (min_y, max_y + 1), (min_z, max_z + 1));

        Instruction { operation, cube }
    }
}
