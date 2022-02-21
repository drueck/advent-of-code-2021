use std::collections::HashMap;

pub type Position = (usize, usize);

#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
    East,
    South,
}

pub struct SeaCucumberMap {
    pub map: HashMap<Position, Direction>,
    pub height: usize,
    pub width: usize,
}

impl SeaCucumberMap {
    pub fn new(input: &str) -> Self {
        let mut map = HashMap::new();
        let lines: Vec<&str> = input.trim().split("\n").map(|line| line.trim()).collect();
        let height = lines.len();
        let width = lines[0].len();

        lines.iter().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| match c {
                '>' => {
                    map.insert((x, y), Direction::East);
                }
                'v' => {
                    map.insert((x, y), Direction::South);
                }
                _ => (),
            });
        });

        Self { map, height, width }
    }

    pub fn move_east_herd(&mut self) -> usize {
        let mut moves = 0;
        let mut new_map = self.map.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(Direction::East) = self.map.get(&(x, y)) {
                    let new_x = (x + 1) % self.width;
                    if let None = self.map.get(&(new_x, y)) {
                        new_map.remove(&(x, y));
                        new_map.insert((new_x, y), Direction::East);
                        moves += 1;
                    }
                }
            }
        }
        self.map = new_map;
        moves
    }

    pub fn move_south_herd(&mut self) -> usize {
        let mut moves = 0;
        let mut new_map = self.map.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(Direction::South) = self.map.get(&(x, y)) {
                    let new_y = (y + 1) % self.height;
                    if let None = self.map.get(&(x, new_y)) {
                        new_map.remove(&(x, y));
                        new_map.insert((x, new_y), Direction::South);
                        moves += 1;
                    }
                }
            }
        }
        self.map = new_map;
        moves
    }

    pub fn move_both_herds(&mut self) -> usize {
        self.move_east_herd() + self.move_south_herd()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let input = "
            v...>>.vv>
            .vv>>.vv..
            >>.>v>...v
            >>v>>.>.v.
            v>v.vv.v..
            >.>>..v...
            .vv..>.>v.
            v.v..>>v.v
            ....v..v.>
        ";

        let map = SeaCucumberMap::new(&input);

        assert_eq!(map.height, 9);
        assert_eq!(map.width, 10);
        assert_eq!(map.map.get(&(0, 0)), Some(&Direction::South));
        assert_eq!(map.map.get(&(9, 8)), Some(&Direction::East));
        assert_eq!(map.map.get(&(0, 8)), None);
    }

    #[test]
    fn move_east_herd() {
        let input = "
            ..........
            .>v....v..
            .......>..
            ..........
        ";

        let after_move_east = "
            ..........
            .>v....v..
            ........>.
            ..........
        ";

        let mut original_map = SeaCucumberMap::new(&input);
        let expected_map = SeaCucumberMap::new(&after_move_east);

        assert_eq!(original_map.move_east_herd(), 1);
        assert_eq!(original_map.map, expected_map.map);
    }

    #[test]
    fn move_south_herd() {
        let input = "
            ..........
            .>v....v..
            .......>..
            ..........
        ";

        let after_move_south = "
            ..........
            .>.....v..
            ..v....>..
            ..........
        ";

        let mut original_map = SeaCucumberMap::new(&input);
        let expected_map = SeaCucumberMap::new(&after_move_south);

        assert_eq!(original_map.move_south_herd(), 1);
        assert_eq!(original_map.map, expected_map.map);
    }

    #[test]
    fn move_both_herds() {
        let input = "
            ..........
            .>v....v..
            .......>..
            ..........
        ";

        let after_moves = "
            ..........
            .>........
            ..v....v>.
            ..........
        ";

        let mut original_map = SeaCucumberMap::new(&input);
        let expected_map = SeaCucumberMap::new(&after_moves);

        assert_eq!(original_map.move_both_herds(), 3);
        assert_eq!(original_map.map, expected_map.map);
    }

    #[test]
    fn wraps() {
        let input = "
            .........>
            .........v
        ";

        let after_moves = "
            >........v
            ..........
        ";

        let mut original_map = SeaCucumberMap::new(&input);
        let expected_map = SeaCucumberMap::new(&after_moves);

        assert_eq!(original_map.move_both_herds(), 2);
        assert_eq!(original_map.map, expected_map.map);
    }

    #[test]
    fn move_until_they_stop() {
        let input = "
            v...>>.vv>
            .vv>>.vv..
            >>.>v>...v
            >>v>>.>.v.
            v>v.vv.v..
            >.>>..v...
            .vv..>.>v.
            v.v..>>v.v
            ....v..v.>
        ";

        let mut map = SeaCucumberMap::new(&input);

        let mut steps = 1;
        while map.move_both_herds() > 0 {
            steps += 1;
        }

        assert_eq!(steps, 58);
    }
}
