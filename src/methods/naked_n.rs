use std::collections::HashSet;

use crate::{cell::Cell, figure::Figure};

use super::{Method, MethodCreator};

pub struct NakedNCreator {
    n: u8,
}

impl NakedNCreator {
    fn get_single_applications(&self, grid: &crate::grid::Grid) -> Vec<Naked> {
        let mut res = vec![];

        for i in Figure::all_cells() {
            if let Cell::Pencilmarks(pencilmarks) = &grid[i] {
                if pencilmarks.len() == 1 {
                    res.push(Naked::Single(i))
                }
            }
        }

        res
    }
    fn get_multiple_applications(&self, grid: &crate::grid::Grid) -> Vec<Naked> {
        let mut res = vec![];

        for f in Figure::all_figures() {
            let mut candidates = vec![];
            let mut lead_sets = HashSet::new();

            for i in f.clone() {
                if let Cell::Pencilmarks(pencilmarks) = &grid[i] {
                    if (2..=self.n as usize).contains(&pencilmarks.len()) {
                        candidates.push(i);
                    }

                    if pencilmarks.len() == self.n.into() {
                        lead_sets.insert(pencilmarks);
                    }
                }
            }

            for lead_set in lead_sets {
                let mut possible_positions = vec![];

                for candidate in candidates.clone() {
                    let pencilmarks = &grid[candidate].pencilmarks();

                    if pencilmarks.iter().all(|&x| lead_set.contains(&x)) {
                        possible_positions.push(candidate);
                    }
                }

                if possible_positions.len() == self.n.into() {
                    res.push(Naked::N(
                        f.clone() - possible_positions.clone().into(),
                        lead_set.to_vec(),
                    ));
                }
            }
        }
        res
    }
}

impl MethodCreator for NakedNCreator {
    type Method = Naked;

    fn get_all_applications(&self, grid: &crate::grid::Grid) -> Vec<Self::Method>
    where
        Self::Method: Method,
    {
        if self.n == 1 {
            self.get_single_applications(grid)
        } else {
            self.get_multiple_applications(grid)
        }
    }
}

#[derive(Debug, PartialEq, Ord, Eq, PartialOrd)]
pub enum Naked {
    Single(usize),
    N(Figure, Vec<u8>),
}

impl Method for Naked {
    fn apply_to_grid(&self, grid: &mut crate::grid::Grid) {
        match self {
            Naked::Single(position) => {
                let number_to_set = grid[*position].pencilmarks()[0];

                grid.set_number(*position, number_to_set);
            }
            Naked::N(figure, pencilmarks_to_remove) => {
                grid.remove_pencilmarks(figure, pencilmarks_to_remove)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::grid::Grid;

    use super::*;

    // TODO: Remove this duplication of code
    #[test]
    fn get_and_apply_single() {
        let mut grid = Grid::from_str(
            "401003050000605084895407136030060405900050300050001200240500007009000500500092000",
        )
        .unwrap();

        let candidates = NakedNCreator { n: 1 }.get_all_applications(&grid);
        assert_eq!(candidates, vec![Naked::Single(22)]);

        candidates[0].apply_to_grid(&mut grid);
        assert_eq!(grid[22], Cell::Number(2));

        let mut candidates = NakedNCreator { n: 1 }.get_all_applications(&grid);
        assert_eq!(
            candidates.sort(),
            vec![Naked::Single(4), Naked::Single(13)].sort()
        );
    }

    #[test]
    fn get_and_apply_pairs() {
        let mut grid = Grid::from_str(
            "400000938032094100095300240370609004529001673604703090957008300003900400240030709",
        )
        .unwrap();

        let candidates = NakedNCreator { n: 2 }.get_all_applications(&grid);
        assert_eq!(
            candidates,
            vec![
                Naked::N(vec![0, 3, 4, 5, 6, 7, 8].into(), vec![1, 6]),
                Naked::N(vec![0, 9, 10, 11, 18, 19, 20].into(), vec![1, 6]),
                Naked::N(vec![18, 19, 20, 21, 22, 24, 25].into(), vec![6, 7]),
                Naked::N(vec![27, 28, 36, 37, 38, 45, 47].into(), vec![1, 8]),
                Naked::N(vec![36, 37, 38, 41, 42, 43, 44].into(), vec![4, 8]),
                Naked::N(vec![30, 31, 32, 41, 48, 49, 50].into(), vec![4, 8]),
                Naked::N(vec![34, 35, 42, 43, 44, 52, 53].into(), vec![5, 8]),
                Naked::N(vec![6, 15, 24, 42, 60, 69, 78].into(), vec![5, 8])
            ]
        );

        for candidate in candidates {
            candidate.apply_to_grid(&mut grid);
        }

        assert_eq!(grid[3], Cell::Pencilmarks(vec![2, 5]));
        assert_eq!(grid[4], Cell::Pencilmarks(vec![2, 5, 7]));
        assert_eq!(grid[5], Cell::Pencilmarks(vec![2, 5, 7]));
        assert_eq!(grid[18], Cell::Pencilmarks(vec![8]));
        assert_eq!(grid[22], Cell::Pencilmarks(vec![1, 8]));
        assert_eq!(grid[31], Cell::Pencilmarks(vec![2, 5]));
        assert_eq!(grid[34], Cell::Pencilmarks(vec![1, 2]));
        assert_eq!(grid[49], Cell::Pencilmarks(vec![2, 5]));
        assert_eq!(grid[53], Cell::Pencilmarks(vec![1, 2]));
    }

    #[test]
    fn get_and_apply_triples() {
        let mut grid = Grid::from_str(
            "294513006600842319300697254000056000040080060000470000730164005900735001400928637",
        )
        .unwrap();

        let candidates = NakedNCreator { n: 3 }.get_all_applications(&grid);
        assert_eq!(
            candidates,
            vec![
                Naked::N(vec![0, 9, 18, 54, 63, 72].into(), vec![1, 5, 8]),
                Naked::N(vec![28, 29, 37, 38, 46, 47].into(), vec![1, 5, 8]),
                Naked::N(vec![33, 34, 42, 43, 51, 52].into(), vec![2, 3, 8]),
                Naked::N(vec![54, 55, 57, 58, 59, 62].into(), vec![2, 8, 9]),
                Naked::N(vec![54, 55, 63, 72, 73, 74].into(), vec![2, 6, 8]),
                Naked::N(vec![8, 17, 26, 62, 71, 80].into(), vec![2, 3, 8])
            ]
        );

        for candidate in candidates {
            candidate.apply_to_grid(&mut grid);
        }

        assert_eq!(grid[28], Cell::Pencilmarks(vec![2, 7]));
        assert_eq!(grid[29], Cell::Pencilmarks(vec![2, 3, 7, 9]));
        assert_eq!(grid[38], Cell::Pencilmarks(vec![2, 3, 7, 9]));
        assert_eq!(grid[46], Cell::Pencilmarks(vec![2, 6]));
        assert_eq!(grid[47], Cell::Pencilmarks(vec![2, 3, 6, 9]));
    }

    #[test]
    fn get_and_apply_quads() {
        let mut grid = Grid::from_str(
            "000030086000020040090078520371856294900142375400397618200703859039205467700904132",
        )
        .unwrap();

        let candidates = NakedNCreator { n: 4 }.get_all_applications(&grid);

        assert_eq!(
            candidates,
            vec![
                Naked::N(vec![27, 36, 45, 54, 72].into(), vec![1, 5, 6, 8]),
                Naked::N(vec![1, 2, 11, 19, 20].into(), vec![1, 5, 6, 8]),
            ]
        );

        for candidate in candidates {
            candidate.apply_to_grid(&mut grid);
        }

        assert_eq!(grid[1], Cell::Pencilmarks(vec![2, 4]));
        assert_eq!(grid[2], Cell::Pencilmarks(vec![2, 4, 7]));
        assert_eq!(grid[11], Cell::Pencilmarks(vec![3, 7]));
        assert_eq!(grid[20], Cell::Pencilmarks(vec![3, 4]));
    }
}
