// Advent of Code 2021: Day 23
// https://adventofcode.com/2021/day/23
// Usage `cargo run <input-file>

// test burrow
// #############
// #...........#
// ###B#C#B#D###
//   #A#D#C#A#
//   #########

// 0,0 to 10,2
// cannot move left or right if y is not 2
// cannot stop at (2,2), (4,2), (6,2), or (8,2)
// if you're in a home square:
// try moving to 0,2
// try moving to 1,2
// try moving to 3,2
// try moving to 5,2
// try moving to 7,2
// try moving to 9,2
// try moving to 10,2
// if you can make it there, add that state and cost
// if you can't try the next possibility
//
// if you're in a hallway square
// try moving to your home column depending on your type
// A => (3,0) or (3,1)
// B => (5,0) or (4,1)
// C => (7,0) or (7,1)
// D => (9,0) or (9,1)

fn main() {}
