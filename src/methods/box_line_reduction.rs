use std::{collections::BTreeSet, fmt::Display};

use crate::{action::Action, figure::Figure, grid::Grid};

use super::Method;

#[derive(Clone, Debug)]
pub enum BoxLineReduction {
    Pair,
    Triple,
}

impl BoxLineReduction {
    fn find_in_figure(&self, grid: &Grid, figure: &Figure, dimension: usize) -> BTreeSet<Action> {
        let mut res = BTreeSet::new();
        let pencilmarks_info = grid.pencilmarks_info(figure.clone());

        for (pencilmark, positions) in pencilmarks_info {
            if positions.len() == dimension {
                let pencilmarks_figure: Figure = positions.into();

                if let Some(sqr) = pencilmarks_figure.is_on_the_same_sqr() {
                    res.insert(Action::RemovePencilmarks {
                        figure: Figure::sqr(sqr) - pencilmarks_figure,
                        pencilmarks: vec![pencilmark],
                    });
                }
            }
        }

        res
    }
}

impl Display for BoxLineReduction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Box Line Reduction (for {})",
            match self {
                BoxLineReduction::Pair => "Pair",
                BoxLineReduction::Triple => "Triple",
            }
        )
    }
}

impl Method for BoxLineReduction {
    fn get_all_applications(&self, grid: &Grid) -> BTreeSet<Action> {
        let mut res = BTreeSet::new();
        let dimension = match self {
            BoxLineReduction::Pair => 2,
            BoxLineReduction::Triple => 3,
        };

        for i in 0..9 {
            res.append(&mut self.find_in_figure(grid, &Figure::row(i), dimension));
            res.append(&mut self.find_in_figure(grid, &Figure::col(i), dimension));
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::methods::test_method;

    use super::*;

    #[test]
    fn box_line_reduction_2_cells() {
        test_method(
            "016007803090800000870001260048000300650009082039000650060900020080002936924600510",
            BoxLineReduction::Pair,
            BTreeSet::from([
                Action::RemovePencilmarks {
                    figure: vec![1, 2, 10, 11, 18, 19, 20].into(),
                    pencilmarks: vec![4],
                },
                Action::RemovePencilmarks {
                    figure: vec![3, 4, 5, 12, 21, 22, 23].into(),
                    pencilmarks: vec![6],
                },
                Action::RemovePencilmarks {
                    figure: vec![3, 5, 12, 13, 14, 21, 23].into(),
                    pencilmarks: vec![9],
                },
                Action::RemovePencilmarks {
                    figure: vec![6, 7, 8, 15, 16, 24, 25].into(),
                    pencilmarks: vec![5],
                },
                Action::RemovePencilmarks {
                    figure: vec![6, 7, 8, 16, 24, 25, 26].into(),
                    pencilmarks: vec![1],
                },
                Action::RemovePencilmarks {
                    figure: vec![6, 8, 15, 17, 24, 25, 26].into(),
                    pencilmarks: vec![4],
                },
                Action::RemovePencilmarks {
                    figure: vec![30, 31, 32, 39, 40, 41, 48].into(),
                    pencilmarks: vec![8],
                },
                Action::RemovePencilmarks {
                    figure: vec![30, 31, 32, 41, 48, 49, 50].into(),
                    pencilmarks: vec![3],
                },
                Action::RemovePencilmarks {
                    figure: vec![30, 39, 40, 41, 48, 49, 50].into(),
                    pencilmarks: vec![6],
                },
                Action::RemovePencilmarks {
                    figure: vec![33, 42, 43, 44, 51, 52, 53].into(),
                    pencilmarks: vec![9],
                },
                Action::RemovePencilmarks {
                    figure: vec![57, 58, 59, 66, 67, 68, 75].into(),
                    pencilmarks: vec![3],
                },
                Action::RemovePencilmarks {
                    figure: vec![57, 58, 59, 68, 75, 76, 77].into(),
                    pencilmarks: vec![4],
                },
                Action::RemovePencilmarks {
                    figure: vec![60, 61, 69, 70, 71, 78, 79].into(),
                    pencilmarks: vec![8],
                },
            ]),
        );
    }
    #[test]
    fn box_line_reduction_3_cells() {
        test_method(
            "020943715904000600750000040500480000200000453400352000042000081005004260090208504",
            BoxLineReduction::Triple,
            BTreeSet::from([
                Action::RemovePencilmarks {
                    figure: vec![3, 4, 5, 21, 22, 23].into(),
                    pencilmarks: vec![7],
                },
                Action::RemovePencilmarks {
                    figure: vec![27, 28, 36, 37, 45, 46].into(),
                    pencilmarks: vec![9],
                },
                Action::RemovePencilmarks {
                    figure: vec![27, 29, 36, 38, 45, 47].into(),
                    pencilmarks: vec![6],
                },
                Action::RemovePencilmarks {
                    figure: vec![55, 56, 64, 65, 73, 74].into(),
                    pencilmarks: vec![3],
                },
                Action::RemovePencilmarks {
                    figure: vec![57, 59, 66, 68, 75, 77].into(),
                    pencilmarks: vec![3],
                },
                Action::RemovePencilmarks {
                    figure: vec![66, 67, 68, 75, 76, 77].into(),
                    pencilmarks: vec![7],
                },
            ]),
        );
    }
}
