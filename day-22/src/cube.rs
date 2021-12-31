use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Cube {
    pub min_x: isize,
    pub max_x: isize,
    pub min_y: isize,
    pub max_y: isize,
    pub min_z: isize,
    pub max_z: isize,
}

impl Cube {
    pub fn new(x_range: (isize, isize), y_range: (isize, isize), z_range: (isize, isize)) -> Self {
        let (min_x, max_x) = x_range;
        let (min_y, max_y) = y_range;
        let (min_z, max_z) = z_range;

        Self {
            min_x,
            max_x,
            min_y,
            max_y,
            min_z,
            max_z,
        }
    }

    pub fn volume(&self) -> usize {
        ((self.max_x - self.min_x) * (self.max_y - self.min_y) * (self.max_z - self.min_z)) as usize
    }

    pub fn contains(&self, other: &Self) -> bool {
        other.min_x >= self.min_x
            && other.max_x <= self.max_x
            && other.min_y >= self.min_y
            && other.max_y <= self.max_y
            && other.min_z >= self.min_z
            && other.max_z <= self.max_z
    }

    pub fn does_not_intersect(&self, other: &Self) -> bool {
        self.min_y >= other.max_y    // self is above other
        || other.min_y >= self.max_y // self is below other
        || self.min_x >= other.max_x // self is right of other
        || other.min_x >= self.max_x // self is left of other
        || self.min_z >= other.max_z // self is in front of other
        || other.min_z >= self.max_z // self is behind other
    }

    pub fn intersects(&self, other: &Self) -> bool {
        !self.does_not_intersect(&other)
    }

    pub fn non_intersecting_subcubes_of(&self, other: &Self) -> Vec<Self> {
        if self.contains(other) {
            vec![]
        } else if self.does_not_intersect(&other) {
            vec![other.clone()]
        } else {
            let mut other = other.clone();
            let mut results = vec![];

            // slice right difference
            if other.max_x > self.max_x {
                results.push(Self::new(
                    (self.max_x, other.max_x),
                    (other.min_y, other.max_y),
                    (other.min_z, other.max_z),
                ));
                other.max_x = self.max_x;
            }

            // slice left difference
            if other.min_x < self.min_x {
                results.push(Self::new(
                    (other.min_x, self.min_x),
                    (other.min_y, other.max_y),
                    (other.min_z, other.max_z),
                ));
                other.min_x = self.min_x;
            }

            // slice top difference
            if other.max_y > self.max_y {
                results.push(Self::new(
                    (other.min_x, other.max_x),
                    (self.max_y, other.max_y),
                    (other.min_z, other.max_z),
                ));
                other.max_y = self.max_y;
            }

            // slice bottom difference
            if other.min_y < self.min_y {
                results.push(Self::new(
                    (other.min_x, other.max_x),
                    (other.min_y, self.min_y),
                    (other.min_z, other.max_z),
                ));
                other.min_y = self.min_y;
            }

            // slice front difference
            if other.max_z > self.max_z {
                results.push(Self::new(
                    (other.min_x, other.max_x),
                    (other.min_y, other.max_y),
                    (self.max_z, other.max_z),
                ));
                other.max_z = self.max_z;
            }

            // slice back difference
            if other.min_z < self.min_z {
                results.push(Self::new(
                    (other.min_x, other.max_x),
                    (other.min_y, other.max_y),
                    (other.min_z, self.min_z),
                ));
                other.min_z = self.min_z;
            }

            // all differences should've been sliced off if we did this correctly
            assert!(self.contains(&other));

            results
        }
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for _ in self.min_y..self.max_y {
            for _ in self.min_x..self.max_x {
                write!(f, "X")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn volume() {
        let cube_1 = Cube::new((-5, 5), (-5, 5), (-5, 5));

        assert_eq!(cube_1.volume(), 1000);
    }
}
