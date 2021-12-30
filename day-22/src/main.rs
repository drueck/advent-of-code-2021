// Advent of Code 2021: Day 22
// https://adventofcode.com/2021/day/22
// Usage `cargo run <input-file>

use day_22::Rect;
// use std::{env, fs};

fn main() {
    // let filename = env::args().nth(1).expect("please supply an input filename");

    // maintain a list of non-intersecting box-shaped regions that are on
    // this will initially contain the first box that is "on"
    //
    // grab the next instruction in the list
    // if it's on:
    // - for each box in the list, see if it intersects
    //   - if so, break the new box into just the parts that are new, and repeat for each sub-box
    //   - add whatever non-intersecting sub-boxes exist to the list and we're done
    // if it's off:
    // - for each box in the list, see if it intersects
    //  - if so, break the existing box into sub-boxes that represent what's left if anything
    //    and break the negative box into the opposite set of boxes that didn't intersect
    //  - continue for each remaining negative sub-box and each remaining positive box
    //

    let a = Rect::new((0, 2), (0, 2));
    let b = Rect::new((-1, 3), (1, 3));

    let results = a.add(&b);

    let area: usize = results.iter().map(|rect| rect.area()).sum();

    println!("the total area is: {}", area);

    for rect in results {
        println!("{}", rect);
    }
}
