use std::collections::HashMap;
use std::fmt;

const TOP_LEFT: char = '╔';
const TOP_RIGHT: char = '╗';
const BOTTOM_LEFT: char = '╚';
const BOTTOM_RIGHT: char = '╝';
const HORIZONTAL: char = '═';
const VERTICAL: char = '║';
const HORIZONTAL_WITH_STEM: char = '╩';

const BURROW_MAX_WIDTH: usize = 13;
const BURROW_MIN_WIDTH: usize = 9;

const A_ROOM_COL: usize = 3;
const B_ROOM_COL: usize = 5;
const C_ROOM_COL: usize = 7;
const D_ROOM_COL: usize = 9;

const KINDS: [char; 4] = ['A', 'B', 'C', 'D'];

pub type Position = (usize, usize);
pub type Kind = char;

pub struct Burrow {
    pub map: HashMap<Position, Kind>,
    pub height: usize,
    pub energy_used: usize,
}

impl Burrow {
    pub fn new(input: &str) -> Self {
        let mut map = HashMap::new();

        let lines: Vec<Vec<char>> = input
            .trim()
            .lines()
            .map(|line| {
                let mut chars: Vec<char> = line.trim().chars().collect();
                if chars.len() == BURROW_MIN_WIDTH {
                    for _ in 0..((BURROW_MAX_WIDTH - BURROW_MIN_WIDTH) / 2) {
                        chars.insert(0, ' ');
                        chars.push(' ');
                    }
                }
                chars
            })
            .collect();

        let height = lines.len();

        for row in 0..height {
            for col in 0..BURROW_MAX_WIDTH {
                let c = lines[row][col];
                if KINDS.contains(&c) {
                    map.insert((col, row), c);
                }
            }
        }

        Self {
            map,
            height,
            energy_used: 0,
        }
    }

    fn room_for(&self, kind: &Kind) -> impl Iterator<Item = Position> + '_ {
        let x = match kind {
            &'A' => A_ROOM_COL,
            &'B' => B_ROOM_COL,
            &'C' => C_ROOM_COL,
            &'D' => D_ROOM_COL,
            _ => panic!("Invalid amphipod species"),
        };

        (2..(self.height - 2)).map(move |y| (x, y))
    }

    pub fn organized(&self) -> bool {
        KINDS.iter().all(|kind| {
            self.room_for(&kind)
                .all(|position| match self.map.get(&position) {
                    Some(kind_in_space) => kind_in_space == kind,
                    None => false,
                })
        })
    }
}

impl fmt::Display for Burrow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..BURROW_MAX_WIDTH {
                write!(
                    f,
                    "{}",
                    if row == 0 && col == 0 || row == 2 && col == 10 {
                        TOP_LEFT
                    } else if row == 0 && col == BURROW_MAX_WIDTH - 1 || row == 2 && col == 2 {
                        TOP_RIGHT
                    } else if row == 2 && col == 0 || row == self.height - 1 && col == 2 {
                        BOTTOM_LEFT
                    } else if row == 2 && col == BURROW_MAX_WIDTH - 1
                        || row == self.height - 1 && col == 10
                    {
                        BOTTOM_RIGHT
                    } else if [4, 6, 8].contains(&col) && row == self.height - 1 {
                        HORIZONTAL_WITH_STEM
                    } else if (row == 0 || row == self.height - 1 && (col > 2 && col < 10))
                        || (row == 2 && [1, 11].contains(&col))
                    {
                        HORIZONTAL
                    } else if (row == 1 && [0, BURROW_MAX_WIDTH - 1].contains(&col))
                        || (row > 2 && row < self.height - 1 && [2, 10].contains(&col))
                        || ([4, 6, 8].contains(&col) && row > 1 && row < self.height - 1)
                    {
                        VERTICAL
                    } else {
                        match self.map.get(&(col, row)) {
                            Some(c) => *c,
                            None => ' ',
                        }
                    }
                )?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[test]
fn test_new_small_burrow() {
    let input = "
        #############
        #B....A.C.D.#
        ###.#C#.#.###
          #A#D#.#B#
          #########
    ";

    let burrow = Burrow::new(input);

    assert_eq!(burrow.map.get(&(1, 1)), Some(&'B'));
    assert_eq!(burrow.map.get(&(6, 1)), Some(&'A'));
    assert_eq!(burrow.map.get(&(8, 1)), Some(&'C'));
    assert_eq!(burrow.map.get(&(10, 1)), Some(&'D'));
    assert_eq!(burrow.map.get(&(5, 2)), Some(&'C'));
    assert_eq!(burrow.map.get(&(3, 3)), Some(&'A'));
    assert_eq!(burrow.map.get(&(5, 3)), Some(&'D'));
    assert_eq!(burrow.map.get(&(9, 3)), Some(&'B'));

    assert_eq!(burrow.map.len(), 8);
    assert_eq!(burrow.height, 5);
    assert_eq!(burrow.energy_used, 0);
}

#[test]
fn test_new_large_burrow() {
    let input = "
        #############
        #B....A.C.D.#
        ###.#C#.#.###
          #A#D#.#B#
          #C#D#D#A#
          #B#C#A#B#
          #########
    ";

    let burrow = Burrow::new(input);

    assert_eq!(burrow.map.get(&(1, 1)), Some(&'B'));
    assert_eq!(burrow.map.get(&(6, 1)), Some(&'A'));
    assert_eq!(burrow.map.get(&(8, 1)), Some(&'C'));
    assert_eq!(burrow.map.get(&(10, 1)), Some(&'D'));
    assert_eq!(burrow.map.get(&(5, 2)), Some(&'C'));
    assert_eq!(burrow.map.get(&(3, 3)), Some(&'A'));
    assert_eq!(burrow.map.get(&(5, 3)), Some(&'D'));
    assert_eq!(burrow.map.get(&(9, 3)), Some(&'B'));
    assert_eq!(burrow.map.get(&(3, 4)), Some(&'C'));
    assert_eq!(burrow.map.get(&(3, 5)), Some(&'B'));
    assert_eq!(burrow.map.get(&(5, 4)), Some(&'D'));
    assert_eq!(burrow.map.get(&(5, 5)), Some(&'C'));
    assert_eq!(burrow.map.get(&(7, 4)), Some(&'D'));
    assert_eq!(burrow.map.get(&(7, 5)), Some(&'A'));
    assert_eq!(burrow.map.get(&(9, 4)), Some(&'A'));
    assert_eq!(burrow.map.get(&(9, 5)), Some(&'B'));

    assert_eq!(burrow.map.len(), 16);
    assert_eq!(burrow.height, 7);
    assert_eq!(burrow.energy_used, 0);
}

#[test]
fn test_organized_small_burrow() {
    let in_wrong_rooms = "
        #############
        #...........#
        ###B#C#A#D###
          #A#D#C#B#
          #########";
    let burrow = Burrow::new(&in_wrong_rooms);
    assert!(!burrow.organized());

    let rooms_not_full = "
        #############
        #...A.......#
        ###.#B#C#D###
          #A#B#C#D#
          #########";
    let burrow = Burrow::new(&rooms_not_full);
    assert!(!burrow.organized());

    let organized = "
        #############
        #...........#
        ###A#B#C#D###
          #A#B#C#D#
          #########";
    let burrow = Burrow::new(&organized);
    assert!(burrow.organized());
}

#[test]
fn test_organized_large_burrow() {
    let in_wrong_rooms = "
        #############
        #...........#
        ###A#B#C#D###
          #A#B#C#D#
          #C#D#B#A#
          #D#C#A#B#
          ######### ";
    let burrow = Burrow::new(&in_wrong_rooms);
    assert!(!burrow.organized());

    let rooms_not_full = "
        #############
        #..........D#
        ###A#B#C#.###
          #A#B#C#D#
          #A#B#C#D#
          #A#B#C#D#
          #########";
    let burrow = Burrow::new(&rooms_not_full);
    assert!(!burrow.organized());

    let organized = "
        #############
        #...........#
        ###A#B#C#D###
          #A#B#C#D#
          #A#B#C#D#
          #A#B#C#D#
          #########";
    let burrow = Burrow::new(&organized);
    assert!(burrow.organized());
}
