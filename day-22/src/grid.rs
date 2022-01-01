use crate::rect::Rect;
use std::collections::HashSet;

pub struct Grid {
    pub rects: HashSet<Rect>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            rects: HashSet::new(),
        }
    }

    pub fn add(&mut self, new_rect: Rect) {
        self.rects.retain(|rect| !new_rect.contains(&rect));

        let mut rects_to_add = HashSet::from([new_rect]);

        for rect in &self.rects {
            let mut new_rects_to_add = HashSet::new();

            rects_to_add.retain(|new_rect| {
                if rect.contains(&new_rect) {
                    false
                } else if rect.intersects(&new_rect) {
                    let non_intersecting_subrects = rect.non_intersecting_subrects_of(&new_rect);
                    for sub_rect in non_intersecting_subrects {
                        new_rects_to_add.insert(sub_rect);
                    }
                    false
                } else {
                    true
                }
            });

            for sub_rect in new_rects_to_add {
                assert!(rects_to_add.insert(sub_rect));
            }
        }

        for rect in rects_to_add {
            assert!(self.rects.insert(rect));
        }
    }

    pub fn subtract(&mut self, rect_to_subtract: Rect) {
        self.rects.retain(|rect| !rect_to_subtract.contains(&rect));

        let mut rects_to_remove = HashSet::new();
        let mut subrects_to_add = Grid::new();

        for rect in &self.rects {
            if rect_to_subtract.intersects(&rect) {
                assert!(rects_to_remove.insert(rect.clone()));
                if rect_to_subtract != *rect {
                    for pos_rect in rect_to_subtract.clone().non_intersecting_subrects_of(&rect) {
                        subrects_to_add.add(pos_rect);
                    }
                }
            }
        }

        self.rects.retain(|rect| !rects_to_remove.contains(rect));

        if subrects_to_add
            .rects
            .iter()
            .any(|&rect| rect.intersects(&rect_to_subtract))
        {
            subrects_to_add.subtract(rect_to_subtract);
        }

        for rect in subrects_to_add.rects {
            self.rects.insert(rect);
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
