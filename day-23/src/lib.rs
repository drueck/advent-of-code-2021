use std::collections::HashMap;
use std::fmt;

// only includes possible places to land
const HALLWAY: [(usize, usize); 7] = [(0, 2), (1, 2), (3, 2), (5, 2), (7, 2), (9, 2), (10, 2)];

// test burrow
// #############
// #...........#
// ###B#C#B#D###
//   #A#D#C#A#
//   #########

// A,A,B,B,C,C,D,D
const ROOMS: [(usize, usize); 8] = [
    (2, 0),
    (2, 1),
    (4, 0),
    (4, 1),
    (6, 0),
    (6, 1),
    (8, 0),
    (8, 1),
];

fn room_for(kind: char) -> &'static [(usize, usize)] {
    match kind {
        'A' => &ROOMS[0..2],
        'B' => &ROOMS[2..4],
        'C' => &ROOMS[4..6],
        'D' => &ROOMS[6..8],
        _ => panic!("no such species!"),
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
        if to.0 < from.0 {
            for x in to.0..=(from.0 - 1) {
                spaces.push((x, 2));
            }
        } else {
            for x in (from.0 + 1)..=to.0 {
                spaces.push((x, 2));
            }
        }
        for y in (from.1 - 1)..=to.1 {
            spaces.push((to.0, y));
        }
    } else {
        // we might be moving to the hallway or to another room...
        // but regardless we'll move up first then left or right
        // first move to the hallway
        for y in (from.1 + 1)..=2 {
            spaces.push((from.0, y));
        }
        // now move left or right toward the destination
        if to.0 < from.0 {
            for x in to.0..=(from.0 - 1) {
                spaces.push((x, 2));
            }
        } else {
            for x in (from.0 + 1)..=to.0 {
                spaces.push((x, 2));
            }
        }
        // move down if we're moving to a room
        if to.1 < 2 {
            for y in to.1..=1 {
                spaces.push((to.0, y));
            }
        }
    }

    println!("spaces between {:?} and {:?}", from, to);
    for space in &spaces {
        println!("{:?}", space);
    }

    spaces
}

#[derive(Debug, PartialEq, Eq)]
pub struct Move {
    kind: char,
    from: (usize, usize),
    to: (usize, usize),
    cost: usize,
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

pub struct Burrow {
    pub map: HashMap<(usize, usize), char>,
    energy_used: usize,
}

impl Burrow {
    pub fn new(input: &str) -> Self {
        let mut map = HashMap::new();

        let lines: Vec<Vec<char>> = input
            .lines()
            .skip(2)
            .map(|line| {
                line.chars()
                    .filter(|&char| char != '#' && char != ' ')
                    .collect()
            })
            .collect();

        map.insert((2, 1), lines[0][0]);
        map.insert((4, 1), lines[0][1]);
        map.insert((6, 1), lines[0][2]);
        map.insert((8, 1), lines[0][3]);
        map.insert((2, 0), lines[1][0]);
        map.insert((4, 0), lines[1][1]);
        map.insert((6, 0), lines[1][2]);
        map.insert((8, 0), lines[1][3]);

        Self {
            map,
            energy_used: 0,
        }
    }

    pub fn possible_moves_from(&self, x: usize, y: usize) -> Vec<Move> {
        let mut moves = vec![];

        // TODO probably add a special case for blocked bottom amphipod
        // because we can just check one space and not have to do all that
        // stuff

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

    // if the move is possible (nothing is blocking) then return Some(move with cost)
    // otherwise return None
    fn try_move(&self, from: (usize, usize), to: (usize, usize)) -> Option<Move> {
        let kind = *self.map.get(&from).unwrap();
        let spaces = spaces_between(from, to);
        if ROOMS.contains(&from) {
            if HALLWAY.contains(&to) {
                match spaces.iter().any(|space| self.map.contains_key(space)) {
                    false => Some(Move {
                        from,
                        to,
                        kind,
                        cost: spaces.len() * move_cost_for(kind),
                    }),
                    true => None,
                }
            } else if room_for(kind).contains(&to) {
                match spaces.iter().any(|space| self.map.contains_key(space)) {
                    false => Some(Move {
                        from,
                        to,
                        kind,
                        cost: spaces.len() * move_cost_for(kind),
                    }),
                    true => None,
                }
            } else {
                None
            }
        } else if HALLWAY.contains(&to) {
            if room_for(kind).contains(&to) {
                match spaces.iter().any(|space| self.map.contains_key(space)) {
                    false => Some(Move {
                        from,
                        to,
                        kind,
                        cost: spaces.len() * move_cost_for(kind),
                    }),
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
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    fn test_burrow() -> Burrow {
        let input = fs::read_to_string("test-input.txt").unwrap();
        Burrow::new(&input)
    }

    #[test]
    fn new_burrow() {
        let input = fs::read_to_string("test-input.txt").unwrap();
        let burrow = Burrow::new(&input);

        let mut expected_map = HashMap::new();
        expected_map.insert((2, 1), 'B');
        expected_map.insert((4, 1), 'C');
        expected_map.insert((6, 1), 'B');
        expected_map.insert((8, 1), 'D');
        expected_map.insert((2, 0), 'A');
        expected_map.insert((4, 0), 'D');
        expected_map.insert((6, 0), 'C');
        expected_map.insert((8, 0), 'A');

        assert_eq!(burrow.map, expected_map);
    }

    #[test]
    fn amphipods_trapped_below_others_cant_move() {
        let burrow = test_burrow();

        assert_eq!(burrow.possible_moves_from(2, 0), vec![]);
        assert_eq!(burrow.possible_moves_from(4, 0), vec![]);
        assert_eq!(burrow.possible_moves_from(6, 0), vec![]);
        assert_eq!(burrow.possible_moves_from(8, 0), vec![]);
    }

    // test burrow
    // #############
    // #...........#
    // ###B#C#B#D###
    //   #A#D#C#A#
    //   #########

    #[test]
    fn unblocked_top_row_amphibods_possible_moves() {
        let burrow = test_burrow();
        assert_eq!(
            burrow.possible_moves_from(2, 1),
            vec![
                Move {
                    kind: 'B',
                    from: (2, 1),
                    to: (0, 2),
                    cost: move_cost_for('B') * 3,
                },
                Move {
                    kind: 'B',
                    from: (2, 1),
                    to: (1, 2),
                    cost: move_cost_for('B') * 2,
                },
                Move {
                    kind: 'B',
                    from: (2, 1),
                    to: (3, 2),
                    cost: move_cost_for('B') * 2,
                },
                Move {
                    kind: 'B',
                    from: (2, 1),
                    to: (5, 2),
                    cost: move_cost_for('B') * 4,
                },
                Move {
                    kind: 'B',
                    from: (2, 1),
                    to: (7, 2),
                    cost: move_cost_for('B') * 6,
                },
                Move {
                    kind: 'B',
                    from: (2, 1),
                    to: (9, 2),
                    cost: move_cost_for('B') * 8,
                },
                Move {
                    kind: 'B',
                    from: (2, 1),
                    to: (10, 2),
                    cost: move_cost_for('B') * 9,
                },
            ]
        );
    }
}
