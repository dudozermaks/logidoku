use crate::{cell::Cell, figure::Figure};

use super::Method;

#[derive(Debug, PartialEq)]
pub struct NakedSingle {
    i: usize,
}

impl Method for NakedSingle {
    fn get_all_applications(grid: &crate::grid::Grid) -> Vec<Self>
    where
        Self: Sized,
    {
        let mut res = vec![];

        for i in Figure::all() {
            if let Cell::Pencilmarks(pencilmarks) = &grid[i] {
                if pencilmarks.len() == 1 {
                    res.push(NakedSingle { i })
                }
            }
        }

        res
    }

    fn apply_to_grid(&self, grid: &mut crate::grid::Grid) {
        let number_to_set = grid[self.i].pencilmarks()[0];

        grid.set_number(self.i, number_to_set);
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::grid::Grid;

    use super::*;

    #[test]
    fn get_and_apply() {
        let mut grid = Grid::from_str(
            "401003050000605084895407136030060405900050300050001200240500007009000500500092000",
        )
        .unwrap();

        let candidates = NakedSingle::get_all_applications(&grid);
        assert_eq!(candidates, vec![NakedSingle { i: 22 }]);

        candidates[0].apply_to_grid(&mut grid);
        assert_eq!(grid[22], Cell::Number(2));

        let candidates = NakedSingle::get_all_applications(&grid);
        assert_eq!(
            candidates,
            vec![NakedSingle { i: 4 }, NakedSingle { i: 13 }]
        );
    }
}
