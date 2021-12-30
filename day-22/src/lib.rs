use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rect {
    pub min_x: isize,
    pub max_x: isize,
    pub min_y: isize,
    pub max_y: isize,
}

impl Rect {
    pub fn new(x_range: (isize, isize), y_range: (isize, isize)) -> Self {
        let (min_x, max_x) = x_range;
        let (min_y, max_y) = y_range;

        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    pub fn area(&self) -> usize {
        ((self.max_x - self.min_x) * (self.max_y - self.min_y)) as usize
    }

    fn contains(&self, other: &Self) -> bool {
        other.min_x >= self.min_x
            && other.max_x <= self.max_x
            && other.min_y >= self.min_y
            && other.max_y <= self.max_y
    }

    fn does_not_intersect(&self, other: &Self) -> bool {
        self.min_y >= other.max_y    // self is above other
        || other.min_y >= self.max_y // self is below other
        || self.min_x >= other.max_x // self is right of other
        || other.min_x >= self.max_x // self is left of other
    }

    pub fn add(&self, other: &Self) -> Vec<Self> {
        if self.contains(other) {
            vec![self.clone()]
        } else if other.contains(self) {
            vec![other.clone()]
        } else if self.does_not_intersect(&other) {
            vec![self.clone(), other.clone()]
        } else {
            self.add_intersecting(&other)
        }
    }

    fn add_intersecting(&self, other: &Self) -> Vec<Self> {
        let mut other = other.clone();
        let mut results = vec![];

        results.push(self.clone());

        // slice right difference
        if other.max_x > self.max_x {
            results.push(Self::new(
                (self.max_x, other.max_x),
                (other.min_y, other.max_y),
            ));
            other.max_x = self.max_x;
        }

        // slice left difference
        if other.min_x < self.min_x {
            results.push(Self::new(
                (other.min_x, self.min_x),
                (other.min_y, other.max_y),
            ));
            other.min_x = self.min_x;
        }

        // slice top difference
        if other.max_y > self.max_y {
            results.push(Self::new(
                (other.min_x, other.max_x),
                (self.max_y, other.max_y),
            ));
            other.max_y = self.max_y;
        }

        // slice bottom difference
        if other.min_y < self.min_y {
            results.push(Self::new(
                (other.min_x, other.max_x),
                (other.min_y, self.min_y),
            ));
            other.min_y = self.min_y;
        }

        // all differences should've been sliced off if we did this correctly
        assert!(self.contains(&other));

        results
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for _ in self.min_y..self.max_y {
            for _ in self.min_x..self.max_x {
                write!(f, "X")?;
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_non_intersecting() {
        let rect_1 = Rect::new((0, 2), (0, 2));
        let rect_2 = Rect::new((3, 4), (3, 4));

        assert_eq!(rect_1.add(&rect_2), vec![rect_1, rect_2]);
    }

    #[test]
    fn add_enclosed() {
        let rect_1 = Rect::new((0, 4), (0, 4));
        let rect_2 = Rect::new((1, 3), (1, 3));

        assert_eq!(rect_1.add(&rect_2), vec![rect_1]);
    }

    #[test]
    fn add_enclosing() {
        let rect_1 = Rect::new((1, 3), (1, 3));
        let rect_2 = Rect::new((0, 4), (0, 4));

        assert_eq!(rect_1.add(&rect_2), vec![rect_2]);
    }

    #[test]
    fn add_intersecting() {
        let rect_1 = Rect::new((0, 2), (0, 2));
        let rect_2 = Rect::new((-1, 3), (1, 3));

        assert_eq!(
            rect_1.add(&rect_2),
            vec![
                rect_1,
                Rect::new((2, 3), (1, 3)),
                Rect::new((-1, 0), (1, 3)),
                Rect::new((0, 2), (2, 3))
            ]
        );
    }

    #[test]
    fn area() {
        let rect_1 = Rect::new((-5, 5), (-5, 5));

        assert_eq!(rect_1.area(), 100);
    }
}
