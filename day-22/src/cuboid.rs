use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Cuboid {
    pub min_x: isize,
    pub max_x: isize,
    pub min_y: isize,
    pub max_y: isize,
    pub min_z: isize,
    pub max_z: isize,
}

impl fmt::Display for Cuboid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}), ({}, {}), ({}, {})",
            self.min_x, self.max_x, self.min_y, self.max_y, self.min_z, self.max_z
        )
    }
}

impl Cuboid {
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

    pub fn non_intersecting_subcuboids_of(&self, other: &Self) -> Vec<Self> {
        if self.contains(other) {
            vec![]
        } else if self.does_not_intersect(&other) {
            assert!(false); // for the purpose of this problem this should never occur
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn volume() {
        let cuboid_1 = Cuboid::new((-5, 5), (-5, 5), (-5, 5));

        assert_eq!(cuboid_1.volume(), 1000);
    }

    #[test]
    fn identical_cuboids_intersect() {
        let cuboid_1 = Cuboid::new((0, 1), (0, 1), (0, 1));
        let cuboid_2 = cuboid_1.clone();

        assert!(cuboid_1.intersects(&cuboid_2));
        assert!(cuboid_2.intersects(&cuboid_1));
    }

    #[test]
    fn enclosed_cuboid_intersects() {
        let cuboid_1 = Cuboid::new((0, 4), (0, 4), (0, 4));
        let cuboid_2 = Cuboid::new((1, 3), (1, 3), (1, 3));

        assert!(cuboid_1.intersects(&cuboid_2));
        assert!(cuboid_2.intersects(&cuboid_1));
    }

    #[test]
    fn intersections_in_x() {
        let cuboid_1 = Cuboid::new((0, 2), (0, 1), (0, 1));
        let cuboid_2 = Cuboid::new((1, 3), (0, 1), (0, 1));

        assert!(cuboid_1.intersects(&cuboid_2));
        assert!(cuboid_2.intersects(&cuboid_1));

        let cuboid_3 = Cuboid::new((2, 3), (0, 1), (0, 1));

        assert!(cuboid_1.does_not_intersect(&cuboid_3));
        assert!(cuboid_3.does_not_intersect(&cuboid_1));
    }

    #[test]
    fn intersections_in_y() {
        let cuboid_1 = Cuboid::new((0, 1), (0, 2), (0, 1));
        let cuboid_2 = Cuboid::new((0, 1), (1, 3), (0, 1));

        assert!(cuboid_1.intersects(&cuboid_2));
        assert!(cuboid_2.intersects(&cuboid_1));

        let cuboid_3 = Cuboid::new((0, 1), (2, 3), (0, 1));

        assert!(cuboid_1.does_not_intersect(&cuboid_3));
        assert!(cuboid_3.does_not_intersect(&cuboid_1));
    }

    #[test]
    fn intersections_in_z() {
        let cuboid_1 = Cuboid::new((0, 1), (0, 1), (0, 2));
        let cuboid_2 = Cuboid::new((0, 1), (0, 1), (1, 3));

        assert!(cuboid_1.intersects(&cuboid_2));
        assert!(cuboid_2.intersects(&cuboid_1));

        let cuboid_3 = Cuboid::new((0, 1), (2, 3), (0, 1));

        assert!(cuboid_1.does_not_intersect(&cuboid_3));
        assert!(cuboid_3.does_not_intersect(&cuboid_1));
    }

    #[test]
    fn niso_enclosed() {
        let cuboid_1 = Cuboid::new((1, 3), (1, 3), (1, 3));
        let cuboid_2 = Cuboid::new((0, 4), (0, 4), (0, 4));

        let slices = cuboid_1.non_intersecting_subcuboids_of(&cuboid_2);
        assert_eq!(slices.len(), 6);

        let mut slices_set = HashSet::new();

        for slice in &slices {
            slices_set.insert(slice.clone());
        }

        assert_eq!(slices_set.len(), 6);

        let slices_volume: usize = slices.iter().map(|slice| slice.volume()).sum();

        assert_eq!(slices_volume, cuboid_2.volume() - cuboid_1.volume());
    }
}
