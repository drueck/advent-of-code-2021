use std::cmp::Ordering;
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

const HALLWAY_ROW: usize = 1;
const HALLWAY_POSITIONS: [(usize, usize); 7] = [
    (1, HALLWAY_ROW),
    (2, HALLWAY_ROW),
    (4, HALLWAY_ROW),
    (6, HALLWAY_ROW),
    (8, HALLWAY_ROW),
    (10, HALLWAY_ROW),
    (11, HALLWAY_ROW),
];

const A_ROOM_COL: usize = 3;
const B_ROOM_COL: usize = 5;
const C_ROOM_COL: usize = 7;
const D_ROOM_COL: usize = 9;

const KINDS: [char; 4] = ['A', 'B', 'C', 'D'];

fn energy_cost_for_kind(kind: &Kind) -> usize {
    match kind {
        &'A' => 1,
        &'B' => 10,
        &'C' => 100,
        &'D' => 1000,
        _ => panic!("invalid kind"),
    }
}

pub type Position = (usize, usize);
pub type Kind = char;

#[derive(PartialEq, Eq, Debug)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub kind: Kind,
    pub cost: usize,
}

impl Move {
    pub fn new(from: Position, to: Position, kind: Kind, cost: usize) -> Move {
        Move {
            from,
            to,
            kind,
            cost,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
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

    pub fn state(&self) -> Vec<(Position, Kind)> {
        self.map.clone().into_iter().collect()
    }

    fn room_for(&self, kind: &Kind) -> Vec<Position> {
        let x = match kind {
            &'A' => A_ROOM_COL,
            &'B' => B_ROOM_COL,
            &'C' => C_ROOM_COL,
            &'D' => D_ROOM_COL,
            _ => panic!("Invalid amphipod species"),
        };

        (2..(self.height - 1)).map(move |y| (x, y)).collect()
    }

    pub fn organized(&self) -> bool {
        KINDS.iter().all(|kind| {
            self.room_for(&kind)
                .iter()
                .all(|position| match self.map.get(&position) {
                    Some(kind_in_space) => kind_in_space == kind,
                    None => false,
                })
        })
    }

    fn open_spaces_in_rooms(&self) -> Vec<(Position, Kind)> {
        let mut open_spaces = vec![];

        for kind in KINDS {
            for position in self.room_for(&kind).into_iter().rev() {
                match self.map.get(&position) {
                    Some(kind_in_position) => {
                        if kind_in_position != &kind {
                            break;
                        }
                    }
                    None => {
                        open_spaces.push((position, kind));
                        break;
                    }
                }
            }
        }

        open_spaces
    }

    fn positions_between(&self, from: &Position, to: &Position) -> Vec<Position> {
        let mut spaces = vec![];

        // move from a room to the hallway if needed
        if from.1 > HALLWAY_ROW {
            for y in HALLWAY_ROW..=(from.1 - 1) {
                spaces.push((from.0, y));
            }
        }

        // move left or right in the hallway
        if to.0 < from.0 {
            for x in to.0..=(from.0 - 1) {
                spaces.push((x, HALLWAY_ROW));
            }
        } else {
            for x in (from.0 + 1)..=to.0 {
                spaces.push((x, HALLWAY_ROW));
            }
        }

        // move from the hallway into a room if needed
        if to.1 > HALLWAY_ROW {
            for y in (HALLWAY_ROW + 1)..=to.1 {
                spaces.push((to.0, y));
            }
        }

        spaces
    }

    fn try_move(&self, from: Position, to: Position) -> Option<Move> {
        let kind = self
            .map
            .get(&from)
            .expect("no amphipod was found at the from location");
        let positions = self.positions_between(&from, &to);

        if positions
            .iter()
            .all(|position| self.map.get(position) == None)
        {
            return Some(Move::new(
                from,
                to,
                *kind,
                energy_cost_for_kind(&kind) * positions.len(),
            ));
        }

        None
    }

    fn away_from_home_amphipod_positions(&self, kind: &Kind) -> Vec<Position> {
        let room_for_kind = self.room_for(kind);

        self.map
            .iter()
            .filter(|(_, kind_in_space)| *kind_in_space == kind)
            .filter(|(position, _)| !room_for_kind.contains(&position))
            .map(|(position, _)| position)
            .cloned()
            .collect()
    }

    fn next_move_into_room(&self) -> Option<Move> {
        for (to, kind) in self.open_spaces_in_rooms() {
            for from in self.away_from_home_amphipod_positions(&kind) {
                if let Some(move_to_home) = self.try_move(from, to) {
                    return Some(move_to_home);
                }
            }
        }

        None
    }

    pub fn apply(&mut self, possible_move: Move) {
        assert!(self.map.get(&possible_move.from) == Some(&possible_move.kind));
        assert!(self.map.get(&possible_move.to) == None);

        self.map.remove(&possible_move.from);
        self.map.insert(possible_move.to, possible_move.kind);
        self.energy_used += possible_move.cost;
    }

    // move all amphipods that can move into their home rooms into them
    // until all possible moves into home rooms are exhausted
    // the energy used should be updated accordingly
    pub fn move_into_rooms(&mut self) {
        while let Some(possible_move) = self.next_move_into_room() {
            self.apply(possible_move)
        }
    }

    fn moveable_amphipods_in_rooms_positions(&self) -> Vec<Position> {
        let mut positions = vec![];

        for kind in KINDS {
            let mut non_native_species_present = false;
            let mut possible_position: Option<Position> = None;
            for position in self.room_for(&kind).iter().rev() {
                match self.map.get(position) {
                    Some(kind_in_position) if kind_in_position != &kind => {
                        non_native_species_present = true;
                        possible_position = Some(position.clone())
                    }
                    Some(_) => {
                        if non_native_species_present {
                            possible_position = Some(position.clone())
                        }
                    }
                    None => {
                        break;
                    }
                }
            }
            if let Some(position) = possible_position {
                positions.push(position);
            }
        }

        positions
    }

    pub fn moves_into_hallway(&self) -> Vec<Move> {
        let mut moves = vec![];

        for from in self.moveable_amphipods_in_rooms_positions() {
            for to in HALLWAY_POSITIONS {
                if let Some(valid_move) = self.try_move(from, to) {
                    moves.push(valid_move);
                }
            }
        }

        moves
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

impl Ord for Burrow {
    fn cmp(&self, other: &Self) -> Ordering {
        other.energy_used.cmp(&self.energy_used)
    }
}

impl PartialOrd for Burrow {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_next_move_into_room_small_burrow() {
        let input = "
        #############
        #B....A...D.#
        ###.#C#.#.###
          #A#B#D#C#
          #########
    ";

        let burrow = Burrow::new(input);

        assert_eq!(
            burrow.next_move_into_room(),
            Some(Move::new((6, 1), (3, 2), 'A', 4))
        )
    }

    #[test]
    fn test_apply_move() {
        let input = "
        #############
        #B....A...D.#
        ###.#C#.#.###
          #A#B#D#C#
          #########
    ";

        let mut burrow = Burrow::new(input);
        let move_a = burrow.try_move((6, 1), (3, 2)).unwrap();
        let move_a_cost = move_a.cost;
        burrow.apply(move_a);

        assert_eq!(burrow.energy_used, move_a_cost);
        assert_eq!(burrow.map.get(&(6, 1)), None);
        assert_eq!(burrow.map.get(&(3, 2)), Some(&'A'));
    }

    #[test]
    fn test_move_into_rooms_small_burrow() {
        let input = "
        #############
        #B....A...DC#
        ###.#C#.#.###
          #A#B#D#.#
          #########
    ";
        let mut burrow = Burrow::new(input);

        burrow.move_into_rooms();

        // A:  4 *    1 =    4
        // B:  5 *   10 =   50
        // C: 10 *  100 = 1000
        // D:  8 * 1000 = 8000

        assert!(burrow.organized());
        assert_eq!(burrow.energy_used, 4 + 50 + 1000 + 8000);
    }

    #[test]
    fn test_moveable_amphipods_in_rooms_positions() {
        let input = "
        #############
        #B....B.....#
        ###.#.#A#D###
          #A#C#D#C#
          #########
    ";
        let burrow = Burrow::new(input);

        assert_eq!(
            burrow.moveable_amphipods_in_rooms_positions(),
            vec![(5, 3), (7, 2), (9, 2)]
        );
    }

    #[test]
    fn test_moves_into_hallway() {
        let input = "
        #############
        #...........#
        ###B#C#A#D###
          #A#B#D#C#
          #########
    ";
        let burrow = Burrow::new(input);

        assert_eq!(burrow.moves_into_hallway().len(), 28);
    }
}
