use crate::cuboid::Cuboid;
use std::collections::HashSet;

pub struct CuboidGrid {
    pub cuboids: HashSet<Cuboid>,
}

impl CuboidGrid {
    pub fn new() -> Self {
        Self {
            cuboids: HashSet::new(),
        }
    }

    pub fn add(&mut self, new_cuboid: Cuboid) {
        self.cuboids.retain(|cuboid| !new_cuboid.contains(&cuboid));

        let mut cuboids_to_add = HashSet::from([new_cuboid]);

        for cuboid in &self.cuboids {
            let mut new_cuboids_to_add = HashSet::new();

            cuboids_to_add.retain(|new_cuboid| {
                if cuboid.contains(&new_cuboid) {
                    false
                } else if cuboid.intersects(&new_cuboid) {
                    let non_intersecting_subcuboids =
                        cuboid.non_intersecting_subcuboids_of(&new_cuboid);
                    for sub_cuboid in non_intersecting_subcuboids {
                        new_cuboids_to_add.insert(sub_cuboid);
                    }
                    false
                } else {
                    true
                }
            });

            for sub_cuboid in new_cuboids_to_add {
                cuboids_to_add.insert(sub_cuboid);
            }

            if cuboids_to_add.is_empty() {
                break;
            }
        }

        for cuboid in cuboids_to_add {
            self.cuboids.insert(cuboid);
        }
    }

    pub fn subtract(&mut self, cuboid_to_subtract: Cuboid) {
        self.cuboids
            .retain(|cuboid| !cuboid_to_subtract.contains(&cuboid));

        let mut cuboids_to_subtract = HashSet::from([cuboid_to_subtract]);

        let mut cuboids_to_remove = HashSet::new();
        let mut subcuboids_to_add = HashSet::new();

        for cuboid in &self.cuboids {
            let mut new_cuboids_to_subtract = HashSet::new();

            cuboids_to_subtract.retain(|cuboid_to_subtract| {
                if cuboid_to_subtract.intersects(&cuboid) {
                    cuboids_to_remove.insert(cuboid.clone());
                    if cuboid_to_subtract != cuboid {
                        for neg_cuboid in cuboid
                            .clone()
                            .non_intersecting_subcuboids_of(&cuboid_to_subtract)
                        {
                            new_cuboids_to_subtract.insert(neg_cuboid);
                        }
                        for pos_cuboid in cuboid_to_subtract
                            .clone()
                            .non_intersecting_subcuboids_of(&cuboid)
                        {
                            subcuboids_to_add.insert(pos_cuboid);
                        }
                    }
                    false
                } else {
                    true
                }
            });

            for neg_cuboid in &new_cuboids_to_subtract {
                cuboids_to_subtract.insert(*neg_cuboid);
            }

            if cuboids_to_subtract.is_empty() {
                break;
            }
        }

        self.cuboids
            .retain(|cuboid| !cuboids_to_remove.contains(cuboid));

        for cuboid in subcuboids_to_add {
            self.add(cuboid);
        }
    }

    pub fn volume(&self) -> usize {
        self.cuboids.iter().map(|cuboid| cuboid.volume()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_test() {
        let mut grid = CuboidGrid::new();

        let cuboid_1 = Cuboid::new((10, 13), (10, 13), (10, 13));
        let cuboid_2 = Cuboid::new((11, 14), (11, 14), (11, 14));
        let cuboid_3 = Cuboid::new((9, 12), (9, 12), (9, 12));
        let cuboid_4 = Cuboid::new((10, 11), (10, 11), (10, 11));

        grid.add(cuboid_1);
        assert_eq!(grid.volume(), 27);

        grid.add(cuboid_2);
        assert_eq!(grid.volume(), 27 + 19);

        grid.subtract(cuboid_3);
        assert_eq!(grid.volume(), 27 + 19 - 8);

        grid.add(cuboid_4);
        assert_eq!(grid.volume(), 27 + 19 - 8 + 1);
    }

    #[test]
    fn medium_test() {
        let mut grid = CuboidGrid::new();

        grid.add(Cuboid::new((-20, 27), (-36, 18), (-47, 8)));
        grid.add(Cuboid::new((-20, 34), (-21, 24), (-26, 29)));
        grid.add(Cuboid::new((-22, 29), (-29, 24), (-38, 17)));
        grid.add(Cuboid::new((-46, 8), (-6, 47), (-50, 0)));
        grid.add(Cuboid::new((-49, 2), (-3, 47), (-24, 29)));
        grid.add(Cuboid::new((2, 48), (-22, 23), (-23, 28)));
        grid.add(Cuboid::new((-27, 24), (-28, 27), (-21, 30)));
        grid.add(Cuboid::new((-39, 6), (-6, 48), (-3, 45)));
        grid.add(Cuboid::new((-30, 22), (-8, 44), (-13, 35)));
        grid.add(Cuboid::new((-22, 27), (-27, 21), (-29, 20)));
        grid.subtract(Cuboid::new((-48, -31), (26, 42), (-47, -36)));
        grid.add(Cuboid::new((-12, 36), (6, 51), (-50, -1)));
        grid.subtract(Cuboid::new((-48, -31), (-32, -15), (-15, -4)));
        grid.add(Cuboid::new((-18, 27), (-33, 16), (-7, 47)));
        grid.subtract(Cuboid::new((-40, -21), (-38, -27), (23, 42)));
        grid.add(Cuboid::new((-16, 36), (-41, 11), (-47, 7)));
        grid.subtract(Cuboid::new((-32, -22), (11, 31), (-14, 4)));
        grid.add(Cuboid::new((-49, -4), (-3, 46), (-29, 19)));
        grid.subtract(Cuboid::new((18, 31), (-20, -7), (-3, 14)));
        grid.add(Cuboid::new((-41, 10), (-7, 44), (-33, 16)));

        assert_eq!(grid.volume(), 590784);
    }
}
