// Advent of Code 2021: Day 16, Part 1
// https://adventofcode.com/2021/day/16
// Usage `cargo run <input-file>

// Leaving this rough (unrefactored) since I'll have to make a lot of changes for part 2!

use std::{collections::HashMap, env, fs};

const VERSION_LEN: usize = 3;
const TYPE_LEN: usize = 3;
const LITERAL_CHUNK_LEN: usize = 5;

#[derive(Debug)]
struct Node {
    op: PacketType,
    operands: Vec<Node>,
}

#[derive(Debug)]
enum PacketType {
    Literal(usize),
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl PacketType {
    fn from_str(binary: &str) -> Self {
        match binary {
            "100" => Self::Literal(0),
            "000" => Self::Sum,
            "001" => Self::Product,
            "010" => Self::Minimum,
            "011" => Self::Maximum,
            "101" => Self::GreaterThan,
            "110" => Self::LessThan,
            "111" => Self::EqualTo,
            _ => panic!("Unknown packet type!"),
        }
    }
}

enum SubPacketLengthType {
    Bits(usize),
    SubPackets(usize),
}

impl SubPacketLengthType {
    const BITS_FOR_LENGTH: usize = 15;
    const BITS_FOR_SUBPACKETS: usize = 11;

    fn from_str(binary: &str) -> Self {
        match binary {
            "0" => Self::Bits(Self::BITS_FOR_LENGTH),
            _ => Self::SubPackets(Self::BITS_FOR_SUBPACKETS),
        }
    }
}

struct Transmission {
    binary_string: String,
    pointer: usize,
}

impl Transmission {
    fn from_hex(hex_string: &str) -> Self {
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

        println!("binary string: {}", binary_string);

        Self {
            binary_string,
            pointer: 0,
        }
    }

    fn read_next_packet(&mut self) -> Node {
        let packet_type = self.next_packet_type();
        match packet_type {
            PacketType::Literal(_) => Node {
                op: PacketType::Literal(self.read_literal()),
                operands: vec![],
            },
            _ => Node {
                op: packet_type,
                operands: self.read_subpackets(),
            },
        }
    }

    fn read_literal(&mut self) -> usize {
        let mut val = String::new();
        while &self.binary_string[self.pointer..self.pointer + 1] == "1" {
            self.add_literal_chunk(&mut val);
        }
        self.add_literal_chunk(&mut val);
        println!("got binary string val for literal: {}", val);
        let int_val = usize::from_str_radix(&val, 2).unwrap();
        println!("got literal val: {}", int_val);
        int_val
    }

    fn read_subpackets(&mut self) -> Vec<Node> {
        let length_type =
            SubPacketLengthType::from_str(&self.binary_string[self.pointer..self.pointer + 1]);
        self.pointer += 1;
        let mut subpackets: Vec<Node> = vec![];
        match length_type {
            SubPacketLengthType::Bits(bits_for_length) => {
                let subpackets_length_binary =
                    &self.binary_string[self.pointer..self.pointer + bits_for_length];
                let subpackets_length = usize::from_str_radix(subpackets_length_binary, 2).unwrap();

                self.pointer += bits_for_length;
                let end_value = self.pointer + subpackets_length;
                while self.pointer < end_value {
                    subpackets.push(self.read_next_packet());
                }
            }
            SubPacketLengthType::SubPackets(bits_for_subpacket_count) => {
                let num_subpackets_binary =
                    &self.binary_string[self.pointer..self.pointer + bits_for_subpacket_count];
                let num_subpackets = usize::from_str_radix(num_subpackets_binary, 2).unwrap();

                self.pointer += bits_for_subpacket_count;

                println!("we're going for {} subpackets", num_subpackets);

                while subpackets.len() < num_subpackets {
                    println!("adding a subpacket");
                    subpackets.push(self.read_next_packet());
                }
            }
        }

        subpackets
    }

    fn add_literal_chunk(&mut self, val: &mut String) {
        val.push_str(&self.binary_string[self.pointer + 1..self.pointer + LITERAL_CHUNK_LEN]);
        self.pointer += LITERAL_CHUNK_LEN;
    }

    fn next_packet_type(&mut self) -> PacketType {
        self.pointer += VERSION_LEN; // skip version
        let packet_type =
            PacketType::from_str(&self.binary_string[self.pointer..self.pointer + TYPE_LEN]);
        self.pointer += TYPE_LEN;
        println!("found next packet type: {:?}", packet_type);
        packet_type
    }
}

fn main() {
    let input_file = env::args().nth(1).expect("please specify an input file");
    let hex_string = fs::read_to_string(input_file).expect("no such file");

    let mut transmission = Transmission::from_hex(&hex_string);
    let tree = transmission.read_next_packet();

    println!("tree: {:?}", tree);
}
