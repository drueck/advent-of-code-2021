use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

const KINDS: [char; 4] = ['A', 'B', 'C', 'D'];

const HALLWAY_ROW: usize = 3;
// const HALLWAY_COLS: [usize; 7] = [1, 2, 4, 6, 8, 10, 11];

// only includes possible places to land
const HALLWAY: [(usize, usize); 7] = [(1, 3), (2, 3), (4, 3), (6, 3), (8, 3), (10, 3), (11, 3)];

// const ROOM_ROWS: [usize; 2] = [1, 2];
// const ROOM_COLUMNS: [usize; 4] = [3, 5, 7, 9];

// A,A,B,B,C,C,D,D
const ROOMS: [(usize, usize); 8] = [
    (3, 1),
    (3, 2),
    (5, 1),
    (5, 2),
    (7, 1),
    (7, 2),
    (9, 1),
    (9, 2),
];

// const ROOMS_SPACES: [Space; 8] = [
//     Space { x: 3, y: 1 },
//     Space { x: 3, y: 2 },
//     Space { x: 5, y: 1 },
//     Space { x: 5, y: 2 },
//     Space { x: 7, y: 1 },
//     Space { x: 7, y: 2 },
//     Space { x: 9, y: 1 },
//     Space { x: 9, y: 2 },
// ];

// struct Space {
//     x: usize,
//     y: usize,
// }

// impl Space {
//     fn new(x: usize, y: usize) -> Space {
//         Space { x, y }
//     }
// }

fn room_for(kind: char) -> &'static [(usize, usize)] {
    match kind {
        'A' => &ROOMS[0..2],
        'B' => &ROOMS[2..4],
        'C' => &ROOMS[4..6],
        'D' => &ROOMS[6..8],
        _ => panic!("invalid amphipod species!"),
    }
}

fn move_cost_for(kind: char) -> usize {
    match kind {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => panic!("invalid amphipod species!"),
    }
}

// assumes valid potential moves
fn spaces_between(from: (usize, usize), to: (usize, usize)) -> Vec<(usize, usize)> {
    let mut spaces = vec![];
    // if we're starting in the hallway... we want to move left or right first
    if HALLWAY.contains(&from) {
        // move across the hallway to the room column
        if to.0 < from.0 {
            for x in to.0..=(from.0 - 1) {
                spaces.push((x, HALLWAY_ROW));
            }
        } else {
            for x in (from.0 + 1)..=to.0 {
                spaces.push((x, HALLWAY_ROW));
            }
        }
        // move down into the room
        for y in to.1..from.1 {
            spaces.push((to.0, y));
        }
    } else {
        // we might be moving to the hallway or to another room...
        // but regardless we'll move up first then left or right
        // first move to the hallway
        for y in (from.1 + 1)..=HALLWAY_ROW {
            spaces.push((from.0, y));
        }
        // now move left or right toward the destination
        if to.0 < from.0 {
            for x in to.0..=(from.0 - 1) {
                spaces.push((x, HALLWAY_ROW));
            }
        } else {
            for x in (from.0 + 1)..=to.0 {
                spaces.push((x, HALLWAY_ROW));
            }
        }
        // move down if we're moving to a room
        if to.1 < HALLWAY_ROW {
            for y in to.1..HALLWAY_ROW {
                spaces.push((to.0, y));
            }
        }
    }

    spaces
}

#[derive(Debug, PartialEq, Eq)]
pub struct Move {
    from: (usize, usize),
    to: (usize, usize),
    kind: char,
    cost: usize,
}

impl Move {
    fn new(from: (usize, usize), to: (usize, usize), kind: char, cost: usize) -> Move {
        Move {
            from,
            to,
            kind,
            cost,
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Move {} from ({}, {}) to ({}, {}). Cost: {}",
            self.kind, self.from.0, self.from.1, self.to.0, self.to.1, self.cost
        )
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Burrow {
    pub map: HashMap<(usize, usize), char>,
    pub energy_used: usize,
}

impl fmt::Display for Burrow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let height = self.map.len();
        for row in 0..height {
            for col in 0..13 {
                write!(
                    f,
                    "{}",
                    match self.map.get(&(col, height - row - 1)) {
                        Some(c) => *c,
                        None => ' ',
                    }
                )?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Burrow {
    pub fn new(input: &str) -> Self {
        let mut map = HashMap::new();

        let lines: Vec<Vec<char>> = input
            .trim()
            .lines()
            .map(|line| {
                let mut chars: Vec<char> = line.trim().chars().collect();
                if chars.len() == 9 {
                    for _ in 0..2 {
                        chars.insert(0, ' ');
                        chars.push(' ');
                    }
                }
                chars
            })
            .rev()
            .collect();

        for row in 0..lines.len() {
            for col in 0..13 {
                let c = lines[row][col];
                if KINDS.contains(&c) {
                    map.insert((col, row), c);
                }
            }
        }

        Self {
            map,
            energy_used: 0,
        }
    }

    pub fn possible_moves(&self) -> Vec<Move> {
        self.map
            .iter()
            .flat_map(|((x, y), _)| self.possible_moves_from(*x, *y))
            .collect()
    }

    pub fn possible_moves_from(&self, x: usize, y: usize) -> Vec<Move> {
        let mut moves = vec![];

        // check for blocked bottom amphipod as a special case optimization
        if y == 1 && self.map.contains_key(&(x, 2)) {
            return moves;
        }

        let from = (x, y);
        let kind = self.map.get(&from).unwrap();

        // always try moving to the home room for the kind
        for to in room_for(*kind) {
            if let Some(valid_move) = self.try_move(from, *to) {
                moves.push(valid_move);
            }
        }
        if ROOMS.contains(&from) {
            for to in HALLWAY {
                if let Some(valid_move) = self.try_move(from, to) {
                    moves.push(valid_move);
                }
            }
        }

        moves
    }

    fn build_move(&self, from: (usize, usize), to: (usize, usize)) -> Move {
        let kind = *self.map.get(&from).unwrap();
        let spaces = spaces_between(from, to);
        let cost = spaces.len() * move_cost_for(kind);
        Move::new(from, to, kind, cost)
    }

    // if the move is possible (nothing is blocking) then return Some(move with cost)
    // otherwise return None
    fn try_move(&self, from: (usize, usize), to: (usize, usize)) -> Option<Move> {
        let kind = *self.map.get(&from).unwrap();
        let spaces = spaces_between(from, to);
        let possible_move = self.build_move(from, to);

        if self.kind_organized(kind) {
            return None;
        }

        if ROOMS.contains(&from) {
            // don't move out of your home room if you're on the bottom
            if room_for(kind).contains(&from) && from.1 == 1 {
                return None;
            }

            if HALLWAY.contains(&to) {
                match spaces.iter().any(|space| self.map.contains_key(space)) {
                    false => Some(possible_move),
                    true => None,
                }
            } else if room_for(kind).contains(&to) {
                // moves within rooms are not helpful and result in infinite loops
                if from.0 == to.0 {
                    return None;
                }
                // if the room has any foreign species, we cannot enter it
                for space in room_for(kind) {
                    if let Some(kind_in_space) = self.map.get(space) {
                        if *kind_in_space != kind {
                            return None;
                        }
                    }
                }

                // don't move to the top room space if the bottom space is open
                if to.1 == 2 && self.map.get(&(to.0, 1)) == None {
                    return None;
                }

                match spaces.iter().any(|space| self.map.contains_key(space)) {
                    false => Some(possible_move),
                    true => None,
                }
            } else {
                None
            }
        } else if HALLWAY.contains(&from) {
            if room_for(kind).contains(&to) {
                // if the room has any foreign species, we cannot enter it
                for space in room_for(kind) {
                    if let Some(kind_in_space) = self.map.get(space) {
                        if *kind_in_space != kind {
                            return None;
                        }
                    }
                }
                // don't move to the top room space if the bottom space is open
                if to.1 == 2 && self.map.get(&(to.0, 1)) == None {
                    return None;
                }
                match spaces.iter().any(|space| self.map.contains_key(space)) {
                    false => Some(possible_move),
                    true => None,
                }
            } else {
                // cannot move from hallway to anywhere but your home room
                None
            }
        } else {
            None
        }
    }

    pub fn apply(&self, m: &Move) -> Self {
        let mut new_burrow = self.clone();
        let kind = &new_burrow.map.remove(&m.from).unwrap();
        assert!(*kind == m.kind);
        assert!(self.map.get(&m.to) == None);

        new_burrow.map.insert(m.to, m.kind);
        new_burrow.energy_used += m.cost;
        new_burrow
    }

    pub fn kind_organized(&self, kind: char) -> bool {
        room_for(kind)
            .iter()
            .all(|space| self.map.get(space) == Some(&kind))
    }

    pub fn organized(&self) -> bool {
        KINDS.iter().all(|kind| self.kind_organized(*kind))
    }
}

impl Ord for Burrow {
    fn cmp(&self, other: &Self) -> Ordering {
        other.energy_used.cmp(&self.energy_used)
    }
}

impl PartialOrd for Burrow {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    // use std::collections::BinaryHeap;
    use std::fs;

    fn test_burrow() -> Burrow {
        let input = fs::read_to_string("test-input.txt").unwrap();
        Burrow::new(&input)
    }

    #[test]
    fn new_burrow() {
        let input = "
            #############
            #B....A.C.D.#
            ###.#C#.#.###
              #A#D#.#B#
              #########
        ";

        let burrow = Burrow::new(input);

        // hallway
        assert_eq!(burrow.map.get(&(1, 3)), Some(&'B'));
        assert_eq!(burrow.map.get(&(6, 3)), Some(&'A'));
        assert_eq!(burrow.map.get(&(8, 3)), Some(&'C'));
        assert_eq!(burrow.map.get(&(10, 3)), Some(&'D'));
        // rooms
        assert_eq!(burrow.map.get(&(3, 1)), Some(&'A'));
        assert_eq!(burrow.map.get(&(5, 2)), Some(&'C'));
        assert_eq!(burrow.map.get(&(5, 1)), Some(&'D'));
        assert_eq!(burrow.map.get(&(9, 1)), Some(&'B'));

        assert_eq!(burrow.map.len(), 8);
    }

    #[test]
    fn amphipods_trapped_below_others_cant_move() {
        let burrow = test_burrow();

        assert_eq!(burrow.possible_moves_from(3, 1), vec![]);
        assert_eq!(burrow.possible_moves_from(5, 1), vec![]);
        assert_eq!(burrow.possible_moves_from(7, 1), vec![]);
        assert_eq!(burrow.possible_moves_from(9, 1), vec![]);
    }

    #[test]
    fn unblocked_top_row_amphibods_possible_moves() {
        let burrow = test_burrow();

        assert_eq!(
            burrow.possible_moves_from(3, 2),
            vec![
                burrow.build_move((3, 2), (1, 3)),
                burrow.build_move((3, 2), (2, 3)),
                burrow.build_move((3, 2), (4, 3)),
                burrow.build_move((3, 2), (6, 3)),
                burrow.build_move((3, 2), (8, 3)),
                burrow.build_move((3, 2), (10, 3)),
                burrow.build_move((3, 2), (11, 3)),
            ]
        );
    }

    #[test]
    fn all_possible_starting_moves() {
        let burrow = test_burrow();
        let possible_moves = burrow.possible_moves();
        assert_eq!(possible_moves.len(), 28);
    }

    #[test]
    fn no_moves_for_bottom_of_own_room() {
        let input = "
            #############
            #B..........#
            ###.#C#B#D###
              #A#D#C#B#
              #########
        ";
        let burrow = Burrow::new(input);

        assert_eq!(burrow.possible_moves_from(3, 1), vec![])
    }

    #[test]
    fn no_moves_for_top_of_own_room_if_organized() {
        let input = "
            #############
            #...........#
            ###A#C#B#D###
              #A#D#C#B#
              #########
        ";
        let burrow = Burrow::new(input);

        assert_eq!(burrow.possible_moves_from(3, 2), vec![]);
    }

    #[test]
    fn one_move_from_hallway_to_bottom_of_home_room() {
        let input = "
            #############
            #..........A#
            ###.#.#.#.###
              #.#.#.#.#
              #########
        ";
        let burrow = Burrow::new(input);

        assert_eq!(
            burrow.possible_moves_from(11, 3),
            vec![burrow.build_move((11, 3), (3, 1))]
        );
    }

    #[test]
    fn move_from_hallway_to_top_of_home_room() {
        let input = "
            #############
            #.....C.B...#
            ###B#.#.#D###
              #A#D#C#A#
              #########
        ";

        let burrow = Burrow::new(input);

        assert_eq!(
            burrow.possible_moves_from(6, 3),
            vec![burrow.build_move((6, 3), (7, 2))]
        );
    }

    #[test]
    fn moves_into_hallway() {
        let input = "
            #############
            #D......B...#
            ###.#A#.#.###
              #B#C#.#.#
              #########
        ";
        let burrow = Burrow::new(input);

        assert_eq!(
            burrow.possible_moves_from(3, 1),
            vec![
                burrow.build_move((3, 1), (2, 3)),
                burrow.build_move((3, 1), (4, 3)),
                burrow.build_move((3, 1), (6, 3))
            ]
        );
    }

    #[test]
    fn apply_move() {
        let burrow = test_burrow();
        let m = burrow.build_move((3, 2), (8, 3));

        let new_burrow = burrow.apply(&m);

        assert_eq!(new_burrow.energy_used, m.cost);
        assert_eq!(new_burrow.map.contains_key(&m.from), false);
        assert_eq!(new_burrow.map.contains_key(&m.to), true);
        assert_eq!(new_burrow.map.get(&m.to), Some(&m.kind));
    }

    #[test]
    fn organized() {
        let initial_burrow = test_burrow();
        assert!(!initial_burrow.organized());

        let organized = "
            #############
            #...........#
            ###A#B#C#D###
              #A#B#C#D#
              #########
        ";
        let organized_burrow = Burrow::new(organized);
        assert!(organized_burrow.organized());
    }

    // #[test]
    // fn end_to_end_idea() {
    //     let initial_burrow = test_burrow();
    //     let mut priority_queue: BinaryHeap<Burrow> = BinaryHeap::new();
    //     let mut min_energy = usize::MAX;

    //     // push the initial state
    //     priority_queue.push(initial_burrow);

    //     while let Some(burrow) = priority_queue.pop() {
    //         if burrow.energy_used > min_energy {
    //             break;
    //         }

    //         if burrow.organized() && burrow.energy_used < min_energy {
    //             min_energy = burrow.energy_used;
    //             continue;
    //         }

    //         for possible_move in &burrow.possible_moves() {
    //             let new_burrow = burrow.apply(&possible_move);
    //             priority_queue.push(new_burrow);
    //         }
    //     }

    //     println!("The minimum energy used was {}", min_energy);

    //     assert_eq!(min_energy, 12521);
    // }
}
