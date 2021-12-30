use crate::rect::Rect;

pub struct Grid {
    pub rects: Vec<Rect>,
}

impl Grid {
    pub fn new() -> Self {
        Self { rects: Vec::new() }
    }

    pub fn add(&mut self, new_rect: Rect) {
        self.rects.retain(|rect| !new_rect.contains(&rect));

        let mut rects_to_add = vec![new_rect];

        for r in 0..self.rects.len() {
            let rect = &self.rects[r];
            let mut new_rects_to_add = vec![];
            let mut remove_from_rects_to_add = vec![];
            for i in 0..rects_to_add.len() {
                let new_rect = &rects_to_add[i];
                if rect.contains(&new_rect) {
                    remove_from_rects_to_add.push(i);
                } else if rect.intersects(&new_rect) {
                    let non_intersecting_subrects = rect.non_intersecting_subrects_of(&new_rect);
                    remove_from_rects_to_add.push(i);
                    for sub_rect in non_intersecting_subrects {
                        new_rects_to_add.push(sub_rect);
                    }
                }
            }

            remove_from_rects_to_add.sort_by(|a, b| b.cmp(a));
            for i in remove_from_rects_to_add {
                rects_to_add.remove(i);
            }
            for sub_rect in new_rects_to_add {
                rects_to_add.push(sub_rect);
            }
        }

        for rect in rects_to_add {
            self.rects.push(rect);
        }
    }

    pub fn subtract(&mut self, rect_to_subtract: Rect) {
        self.rects.retain(|rect| !rect_to_subtract.contains(&rect));

        let mut rects_to_subtract = vec![rect_to_subtract];

        let mut rects_to_remove = vec![];
        let mut subrects_to_add = vec![];

        for r in 0..self.rects.len() {
            let rect = &self.rects[r];

            let mut new_rects_to_subtract = vec![];
            let mut remove_from_rects_to_subtract = vec![];

            for i in 0..rects_to_subtract.len() {
                let rect_to_subtract = &rects_to_subtract[i];

                if rect_to_subtract.contains(&rect) {
                    rects_to_remove.push(r);
                } else if rect_to_subtract.intersects(&rect) {
                    rects_to_remove.push(r);
                    remove_from_rects_to_subtract.push(i);
                    for neg_rect in rect.clone().non_intersecting_subrects_of(&rect_to_subtract) {
                        new_rects_to_subtract.push(neg_rect);
                    }
                    for pos_rect in rect_to_subtract.clone().non_intersecting_subrects_of(&rect) {
                        subrects_to_add.push(pos_rect);
                    }
                }
            }

            remove_from_rects_to_subtract.sort_by(|a, b| b.cmp(a));
            for i in remove_from_rects_to_subtract {
                rects_to_subtract.remove(i);
            }
            for neg_rect in new_rects_to_subtract {
                rects_to_subtract.push(neg_rect);
            }
        }

        rects_to_remove.sort_by(|a, b| b.cmp(a));
        for r in rects_to_remove {
            self.rects.remove(r);
        }

        for rect in subrects_to_add {
            self.rects.push(rect);
        }
    }

    pub fn area(&self) -> usize {
        self.rects.iter().map(|rect| rect.area()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_first_rect() {
        let mut grid = Grid::new();
        let rect = Rect::new((0, 2), (0, 2));
        grid.add(rect);

        assert_eq!(grid.area(), rect.area());
    }

    #[test]
    fn add_non_intersecting_rects() {
        let mut grid = Grid::new();
        let rect_1 = Rect::new((0, 2), (0, 2));
        let rect_2 = Rect::new((-7, -5), (-7, -5));

        grid.add(rect_1);
        grid.add(rect_2);

        assert_eq!(grid.area(), rect_1.area() + rect_2.area());
    }

    #[test]
    fn add_containing() {
        let mut grid = Grid::new();
        let rect_1 = Rect::new((0, 2), (0, 2));
        let rect_2 = Rect::new((-1, 3), (-1, 3));

        grid.add(rect_1);
        grid.add(rect_2);

        assert_eq!(grid.area(), rect_2.area());
        assert_eq!(grid.rects.len(), 1);
    }

    #[test]
    fn add_contained() {
        let mut grid = Grid::new();
        let rect_1 = Rect::new((-1, 3), (-1, 3));
        let rect_2 = Rect::new((0, 2), (0, 2));

        grid.add(rect_1);
        grid.add(rect_2);

        assert_eq!(grid.area(), rect_1.area());
        assert_eq!(grid.rects.len(), 1);
    }

    #[test]
    fn add_intersecting() {
        let mut grid = Grid::new();
        let rect_1 = Rect::new((0, 2), (0, 2));
        let rect_2 = Rect::new((1, 3), (1, 3));

        grid.add(rect_1);
        grid.add(rect_2);

        assert_eq!(grid.area(), 7);
    }

    #[test]
    fn add_end_to_end() {
        let mut grid = Grid::new();
        let rect_1 = Rect::new((0, 2), (0, 2));
        let rect_2 = Rect::new((-1, 3), (1, 3));
        let rect_3 = Rect::new((-1, 3), (-1, 1));

        grid.add(rect_1);
        grid.add(rect_2);
        grid.add(rect_3);

        // these result in 7 rects that form one contiguous region
        assert_eq!(grid.area(), 16);
        assert_eq!(grid.rects.len(), 7);

        let rect_4 = Rect::new((-1, 3), (-1, 3));

        // add a rect that covers the whole region
        grid.add(rect_4);

        // should be only that rect remaining
        assert_eq!(grid.area(), 16);
        assert_eq!(grid.rects.len(), 1);
    }

    #[test]
    fn subtract_from_empty_grid() {
        let mut grid = Grid::new();
        let rect_1 = Rect::new((0, 2), (0, 2));

        grid.subtract(rect_1);

        assert_eq!(grid.area(), 0);
    }

    #[test]
    fn subtract_non_intersecting() {
        let mut grid = Grid::new();
        let rect_1 = Rect::new((0, 2), (0, 2));
        let rect_2 = Rect::new((5, 6), (5, 6));

        grid.add(rect_1);
        grid.subtract(rect_2);

        assert_eq!(grid.area(), rect_1.area());
        assert_eq!(grid.rects.len(), 1);
    }

    #[test]
    fn subtract_containing() {
        let mut grid = Grid::new();
        let rect_1 = Rect::new((0, 2), (0, 2));
        let rect_2 = Rect::new((-1, 3), (-1, 3));

        grid.add(rect_1);
        grid.subtract(rect_2);

        assert_eq!(grid.area(), 0);
        assert_eq!(grid.rects.len(), 0);
    }

    #[test]
    fn subtract_contained() {
        let mut grid = Grid::new();
        let rect_1 = Rect::new((-1, 3), (-1, 3));
        let rect_2 = Rect::new((0, 2), (0, 2));

        grid.add(rect_1);
        grid.subtract(rect_2);

        assert_eq!(grid.area(), rect_1.area() - rect_2.area());
    }

    #[test]
    fn subtract_intersecting() {
        let mut grid = Grid::new();
        let rect_1 = Rect::new((0, 2), (0, 2));
        let rect_2 = Rect::new((1, 3), (-1, 1));

        grid.add(rect_1);
        grid.subtract(rect_2);

        assert_eq!(grid.area(), 3);
    }

    #[test]
    fn subtract_end_to_end() {
        let mut grid = Grid::new();
        let rect_1 = Rect::new((0, 4), (0, 4));
        let rect_2 = Rect::new((2, 3), (1, 2));
        let rect_3 = Rect::new((2, 5), (1, 5));
        let rect_4 = Rect::new((1, 6), (2, 3));
        let rect_5 = Rect::new((0, 2), (0, 1));

        grid.add(rect_1);
        grid.subtract(rect_2);
        grid.add(rect_3);
        grid.subtract(rect_4);
        grid.subtract(rect_5);

        assert_eq!(grid.area(), 16);
    }
}
