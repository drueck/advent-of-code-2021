// Advent of Code 2021: Day 16, Part 1
// https://adventofcode.com/2021/day/16
// Usage `cargo run <input-file>

// Leaving this rough (unrefactored) since I'll have to make a lot of changes for part 2!

use std::{collections::HashMap, env, fs};

const VERSION_LEN: usize = 3;
const TYPE_LEN: usize = 3;
const LITERAL_PACKET_TYPE: &str = "100";
const BITS_LENGTH_TYPE: &str = "0";
const LITERAL_CHUNK_LEN: usize = 5;
const BITS_FOR_LENGTH: usize = 15;
const BITS_FOR_SUBPACKETS: usize = 11;

#[derive(Debug)]
enum PacketType {
    Literal,
    Operator,
}

impl PacketType {
    fn from_str(binary: &str) -> PacketType {
        match binary {
            LITERAL_PACKET_TYPE => PacketType::Literal,
            _ => PacketType::Operator,
        }
    }
}

enum LengthType {
    Bits(usize),
    Subpackets(usize),
}

impl LengthType {
    fn from_str(binary: &str) -> LengthType {
        match binary {
            BITS_LENGTH_TYPE => LengthType::Bits(BITS_FOR_LENGTH),
            _ => LengthType::Subpackets(BITS_FOR_SUBPACKETS),
        }
    }
}

fn main() {
    let input_file = env::args().nth(1).expect("please specify an input file");
    let hex_string = fs::read_to_string(input_file).expect("no such file");
    let mut binary_string = String::with_capacity(hex_string.len() * 4);

    let hex_to_binary: HashMap<char, &str> = HashMap::from([
        ('0', "0000"),
        ('1', "0001"),
        ('2', "0010"),
        ('3', "0011"),
        ('4', "0100"),
        ('5', "0101"),
        ('6', "0110"),
        ('7', "0111"),
        ('8', "1000"),
        ('9', "1001"),
        ('A', "1010"),
        ('B', "1011"),
        ('C', "1100"),
        ('D', "1101"),
        ('E', "1110"),
        ('F', "1111"),
    ]);

    for char in hex_string.trim().chars() {
        binary_string.push_str(hex_to_binary.get(&char).expect("not a hex digit"));
    }

    let mut pointer: usize = 0;
    let mut version_numbers: Vec<usize> = vec![];

    while pointer + VERSION_LEN + TYPE_LEN < binary_string.len() {
        let packet_version =
            usize::from_str_radix(&binary_string[pointer..pointer + VERSION_LEN], 2).unwrap();
        pointer += VERSION_LEN;
        version_numbers.push(packet_version);

        let packet_type = PacketType::from_str(&binary_string[pointer..pointer + TYPE_LEN]);
        pointer += TYPE_LEN;

        match packet_type {
            PacketType::Literal => {
                while &binary_string[pointer..pointer + 1] == "1" {
                    pointer += LITERAL_CHUNK_LEN;
                }
                pointer += LITERAL_CHUNK_LEN;
            }
            PacketType::Operator => {
                let length_type = LengthType::from_str(&binary_string[pointer..pointer + 1]);
                pointer += 1;
                match length_type {
                    LengthType::Bits(bits_for_length) => {
                        pointer += bits_for_length;
                    }
                    LengthType::Subpackets(bits_for_subpackets) => {
                        pointer += bits_for_subpackets;
                    }
                }
            }
        };
    }

    println!("version sum: {}", version_numbers.iter().sum::<usize>());
}
