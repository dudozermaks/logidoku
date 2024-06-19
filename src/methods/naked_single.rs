use crate::{cell::Cell, figure::Figure};

use super::{Method, MethodCreator};

pub struct NakedSingleCreator {

}

impl MethodCreator for NakedSingleCreator {
    type Method = NakedSingle;
    fn get_all_applications(grid: &crate::grid::Grid) -> Vec<Self::Method> where Self::Method: Method {
        let mut res = vec![];

        for i in Figure::all_cells() {
            if let Cell::Pencilmarks(pencilmarks) = &grid[i] {
                if pencilmarks.len() == 1 {
                    res.push(NakedSingle { position: i })
                }
            }
        }

        res
    }
}

#[derive(Debug, PartialEq, Ord, Eq, PartialOrd)]
pub struct NakedSingle {
    position: usize,
}

impl Method for NakedSingle {
    fn apply_to_grid(&self, grid: &mut crate::grid::Grid) {
        let number_to_set = grid[self.position].pencilmarks()[0];

        grid.set_number(self.position, number_to_set);
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

        let candidates = NakedSingleCreator::get_all_applications(&grid);
        assert_eq!(candidates, vec![NakedSingle { position: 22 }]);

        candidates[0].apply_to_grid(&mut grid);
        assert_eq!(grid[22], Cell::Number(2));

        let mut candidates = NakedSingleCreator::get_all_applications(&grid);
        assert_eq!(
            candidates.sort(),
            vec![NakedSingle { position: 4 }, NakedSingle { position: 13 }].sort()
        );
    }
}
