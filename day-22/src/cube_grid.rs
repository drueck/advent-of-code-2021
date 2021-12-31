use crate::cube::Cube;

pub struct CubeGrid {
    pub cubes: Vec<Cube>,
}

impl CubeGrid {
    pub fn new() -> Self {
        Self { cubes: Vec::new() }
    }

    pub fn add(&mut self, new_cube: Cube) {
        self.cubes.retain(|cube| !new_cube.contains(&cube));

        let mut cubes_to_add = vec![new_cube];

        for r in 0..self.cubes.len() {
            let cube = &self.cubes[r];
            let mut new_cubes_to_add = vec![];
            let mut remove_from_cubes_to_add = vec![];
            for i in 0..cubes_to_add.len() {
                let new_cube = &cubes_to_add[i];
                if cube.contains(&new_cube) {
                    remove_from_cubes_to_add.push(i);
                } else if cube.intersects(&new_cube) {
                    let non_intersecting_subcubes = cube.non_intersecting_subcubes_of(&new_cube);
                    remove_from_cubes_to_add.push(i);
                    for sub_cube in non_intersecting_subcubes {
                        new_cubes_to_add.push(sub_cube);
                    }
                }
            }

            remove_from_cubes_to_add.sort_by(|a, b| b.cmp(a));
            for i in remove_from_cubes_to_add {
                cubes_to_add.remove(i);
            }
            for sub_cube in new_cubes_to_add {
                cubes_to_add.push(sub_cube);
            }
        }

        for cube in cubes_to_add {
            self.cubes.push(cube);
        }
    }

    pub fn subtract(&mut self, cube_to_subtract: Cube) {
        self.cubes.retain(|cube| !cube_to_subtract.contains(&cube));

        let mut cubes_to_subtract = vec![cube_to_subtract];

        let mut cubes_to_remove = vec![];
        let mut subcubes_to_add = vec![];

        for r in 0..self.cubes.len() {
            let cube = &self.cubes[r];

            let mut new_cubes_to_subtract = vec![];
            let mut remove_from_cubes_to_subtract = vec![];

            for i in 0..cubes_to_subtract.len() {
                let cube_to_subtract = &cubes_to_subtract[i];

                if cube_to_subtract.contains(&cube) {
                    // this same cube is being considered more than once and added to the cubes to
                    // remove list twice. this should be a set, and should be cubes, not indices
                    // same with all the
                    cubes_to_remove.push(r);
                } else if cube_to_subtract.intersects(&cube) {
                    cubes_to_remove.push(r);
                    remove_from_cubes_to_subtract.push(i);
                    for neg_cube in cube.clone().non_intersecting_subcubes_of(&cube_to_subtract) {
                        new_cubes_to_subtract.push(neg_cube);
                    }
                    for pos_cube in cube_to_subtract.clone().non_intersecting_subcubes_of(&cube) {
                        subcubes_to_add.push(pos_cube);
                    }
                }
            }

            remove_from_cubes_to_subtract.sort_by(|a, b| b.cmp(a));
            for i in remove_from_cubes_to_subtract {
                cubes_to_subtract.remove(i);
            }
            for neg_cube in new_cubes_to_subtract {
                cubes_to_subtract.push(neg_cube);
            }
        }

        cubes_to_remove.sort_by(|a, b| b.cmp(a));
        for r in cubes_to_remove {
            self.cubes.remove(r);
        }

        for cube in subcubes_to_add {
            self.cubes.push(cube);
        }
    }

    pub fn volume(&self) -> usize {
        self.cubes.iter().map(|cube| cube.volume()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_test() {
        let mut grid = CubeGrid::new();

        let cube_1 = Cube::new((10, 13), (10, 13), (10, 13));
        let cube_2 = Cube::new((11, 14), (11, 14), (11, 14));
        let cube_3 = Cube::new((9, 12), (9, 12), (9, 12));
        let cube_4 = Cube::new((10, 11), (10, 11), (10, 11));

        grid.add(cube_1);
        assert_eq!(grid.volume(), 27);

        grid.add(cube_2);
        assert_eq!(grid.volume(), 27 + 19);

        grid.subtract(cube_3);
        assert_eq!(grid.volume(), 27 + 19 - 8);

        grid.add(cube_4);
        assert_eq!(grid.volume(), 27 + 19 - 8 + 1);
    }

    #[test]
    fn medium_test() {
        let mut grid = CubeGrid::new();

        grid.add(Cube::new((-20, 27), (-36, 18), (-47, 8)));
        grid.add(Cube::new((-20, 34), (-21, 24), (-26, 29)));
        grid.add(Cube::new((-22, 29), (-29, 24), (-38, 17)));
        grid.add(Cube::new((-46, 8), (-6, 47), (-50, 0)));
        grid.add(Cube::new((-49, 2), (-3, 47), (-24, 29)));
        grid.add(Cube::new((2, 48), (-22, 23), (-23, 28)));
        grid.add(Cube::new((-27, 24), (-28, 27), (-21, 30)));
        grid.add(Cube::new((-39, 6), (-6, 48), (-3, 45)));
        grid.add(Cube::new((-30, 22), (-8, 44), (-13, 35)));
        grid.add(Cube::new((-22, 27), (-27, 21), (-29, 20)));
        grid.subtract(Cube::new((-48, -31), (26, 42), (-47, -36)));
        grid.add(Cube::new((-12, 36), (6, 51), (-50, -1)));
        grid.subtract(Cube::new((-48, -31), (-32, -15), (-15, -4)));
        grid.add(Cube::new((-18, 27), (-33, 16), (-7, 47)));
        grid.subtract(Cube::new((-40, -21), (-38, -27), (23, 42)));
        grid.add(Cube::new((-16, 36), (-41, 11), (-47, 7)));
        grid.subtract(Cube::new((-32, -22), (11, 31), (-14, 4)));
        grid.add(Cube::new((-49, -4), (-3, 46), (-29, 19)));
        grid.subtract(Cube::new((18, 31), (-20, -7), (-3, 14)));
        grid.add(Cube::new((-41, 10), (-7, 44), (-33, 16)));

        assert_eq!(grid.volume(), 590784);
    }
}
