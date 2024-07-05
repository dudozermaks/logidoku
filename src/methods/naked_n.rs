use std::{collections::HashSet, fmt::Display};

use crate::{action::Action, cell::Cell, figure::Figure};

use super::Method;

#[derive(Clone)]
pub enum Naked {
    Single,
    Pair,
    Triple,
    Quad
}

impl Naked {
    fn single_applications(&self, grid: &crate::grid::Grid) -> Vec<Action> {
        let mut res = vec![];

        for i in Figure::all_cells() {
            if let Cell::Pencilmarks(pencilmarks) = &grid[i] {
                if pencilmarks.len() == 1 {
                    res.push(Action::PlaceNumber(i, pencilmarks[0]))
                }
            }
        }

        res
    }
    fn multiple_applications(&self, grid: &crate::grid::Grid, dimension: usize) -> Vec<Action> {
        let mut res = vec![];

        for f in Figure::all_figures() {
            let mut candidates = vec![];
            let mut lead_sets = HashSet::new();

            for i in f.clone() {
                if let Cell::Pencilmarks(pencilmarks) = &grid[i] {
                    if (2..=dimension).contains(&pencilmarks.len()) {
                        candidates.push(i);
                    }

                    if pencilmarks.len() == dimension {
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

                if possible_positions.len() == dimension {
                    res.push(Action::RemovePencilmarks(
                        f.clone() - possible_positions.clone().into(),
                        lead_set.to_vec(),
                    ));
                }
            }
        }
        res
    }
}

impl Display for Naked{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Naked {}",
            match self {
                Naked::Single => "Single",
                Naked::Pair => "Pair",
                Naked::Triple => "Triple",
                Naked::Quad => "Quad",
            }
        )
    }
}

impl Method for Naked {
    fn get_all_applications(&self, grid: &crate::grid::Grid) -> Vec<Action> {
        let dimension = match self {
            Naked::Single => 1,
            Naked::Pair => 2,
            Naked::Triple => 3,
            Naked::Quad => 4,
        };

        if dimension == 1 {
            self.single_applications(grid)
        } else {
            self.multiple_applications(grid, dimension)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::methods::test_method;

    use super::*;

    #[test]
    fn get_and_apply_single() {
        test_method(
            "401003050000605084895407136030060405900050300050001200240500007009000500500092000",
            Naked::Single,
            vec![Action::PlaceNumber(22, 2)],
        );

        test_method(
            "401003050000605084895427136030060405900050300050001200240500007009000500500092000",
            Naked::Single,
            vec![Action::PlaceNumber(4, 8), Action::PlaceNumber(13, 1)],
        );
    }

    #[test]
    fn get_and_apply_pairs() {
        test_method(
            "400000938032094100095300240370609004529001673604703090957008300003900400240030709",
            Naked::Pair,
            vec![
                Action::RemovePencilmarks(vec![0, 3, 4, 5, 6, 7, 8].into(), vec![1, 6]),
                Action::RemovePencilmarks(vec![0, 9, 10, 11, 18, 19, 20].into(), vec![1, 6]),
                Action::RemovePencilmarks(vec![18, 19, 20, 21, 22, 24, 25].into(), vec![6, 7]),
                Action::RemovePencilmarks(vec![27, 28, 36, 37, 38, 45, 47].into(), vec![1, 8]),
                Action::RemovePencilmarks(vec![36, 37, 38, 41, 42, 43, 44].into(), vec![4, 8]),
                Action::RemovePencilmarks(vec![30, 31, 32, 41, 48, 49, 50].into(), vec![4, 8]),
                Action::RemovePencilmarks(vec![34, 35, 42, 43, 44, 52, 53].into(), vec![5, 8]),
                Action::RemovePencilmarks(vec![6, 15, 24, 42, 60, 69, 78].into(), vec![5, 8]),
            ],
        );
    }

    #[test]
    fn get_and_apply_triples() {
        test_method(
            "294513006600842319300697254000056000040080060000470000730164005900735001400928637",
            Naked::Triple,
            vec![
                Action::RemovePencilmarks(vec![0, 9, 18, 54, 63, 72].into(), vec![1, 5, 8]),
                Action::RemovePencilmarks(vec![28, 29, 37, 38, 46, 47].into(), vec![1, 5, 8]),
                Action::RemovePencilmarks(vec![33, 34, 42, 43, 51, 52].into(), vec![2, 3, 8]),
                Action::RemovePencilmarks(vec![54, 55, 57, 58, 59, 62].into(), vec![2, 8, 9]),
                Action::RemovePencilmarks(vec![54, 55, 63, 72, 73, 74].into(), vec![2, 6, 8]),
                Action::RemovePencilmarks(vec![8, 17, 26, 62, 71, 80].into(), vec![2, 3, 8]),
            ],
        );
    }

    #[test]
    fn get_and_apply_quads() {
        test_method(
            "000030086000020040090078520371856294900142375400397618200703859039205467700904132",
            Naked::Quad,
            vec![
                Action::RemovePencilmarks(vec![27, 36, 45, 54, 72].into(), vec![1, 5, 6, 8]),
                Action::RemovePencilmarks(vec![1, 2, 11, 19, 20].into(), vec![1, 5, 6, 8]),
            ],
        );
    }
}
