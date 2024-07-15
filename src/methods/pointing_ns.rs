use std::{fmt::Display, collections::BTreeSet};

use crate::{action::Action, figure::Figure, grid::Grid};

use super::Method;

#[derive(Clone, Debug)]
pub enum Pointing {
    Pair,
    Triple,
}

impl Display for Pointing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Pointing {}",
            match self {
                Pointing::Pair => "Pair",
                Pointing::Triple => "Triple",
            }
        )
    }
}

impl Method for Pointing {
    fn get_all_applications(&self, grid: &Grid) -> BTreeSet<Action> {
        let mut res = BTreeSet::new();
        let dimension = match self {
            Pointing::Pair => 2,
            Pointing::Triple => 3,
        };

        for sqr_number in 0..9 {
            let sqr = Figure::sqr(sqr_number);
            let pencilmarks_info = grid.pencilmarks_info(sqr);

            for (pencilmark, positions) in pencilmarks_info {
                if positions.len() == dimension {
                    let pencilmarks_figure: Figure = positions.into();

                    let on_same_row = pencilmarks_figure.is_on_the_same_row();
                    let on_same_col = pencilmarks_figure.is_on_the_same_col();

                    let mut figures_to_add = vec![];

                    if let Some(row) = on_same_row {
                        figures_to_add.push(Figure::row(row) - pencilmarks_figure.clone())
                    }

                    if let Some(col) = on_same_col {
                        figures_to_add.push(Figure::col(col) - pencilmarks_figure);
                    }

                    for figure_to_add in figures_to_add {
                        res.insert(Action::RemovePencilmarks {
                            figure: figure_to_add,
                            pencilmarks: vec![pencilmark],
                        });
                    }
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::methods::test_method;

    use super::*;

    #[test]
    fn pointing_pairs() {
        test_method(
            "032006100410000000000901000500090004060000071300020005000508000000000519057009860",
            Pointing::Pair,
            BTreeSet::from([
                Action::RemovePencilmarks {
                    figure: vec![2, 11, 20, 38, 56, 65, 74].into(),
                    pencilmarks: vec![1],
                },
                Action::RemovePencilmarks {
                    figure: vec![3, 12, 21, 39, 57, 66, 75].into(),
                    pencilmarks: vec![1],
                },
                Action::RemovePencilmarks {
                    figure: vec![9, 10, 11, 13, 15, 16, 17].into(),
                    pencilmarks: vec![2],
                },
                Action::RemovePencilmarks {
                    figure: vec![2, 11, 20, 29, 38, 47, 74].into(),
                    pencilmarks: vec![3],
                },
                Action::RemovePencilmarks {
                    figure: vec![54, 55, 56, 57, 58, 59, 62].into(),
                    pencilmarks: vec![4],
                },
                Action::RemovePencilmarks {
                    figure: vec![2, 29, 38, 47, 56, 65, 74].into(),
                    pencilmarks: vec![5],
                },
                Action::RemovePencilmarks {
                    figure: vec![36, 37, 38, 39, 42, 43, 44].into(),
                    pencilmarks: vec![5],
                },
                Action::RemovePencilmarks {
                    figure: vec![3, 12, 21, 39, 57, 66, 75].into(),
                    pencilmarks: vec![6],
                },
                Action::RemovePencilmarks {
                    figure: vec![6, 15, 24, 42, 60, 69, 78].into(),
                    pencilmarks: vec![6],
                },
                Action::RemovePencilmarks {
                    figure: vec![1, 10, 19, 37, 55, 64, 73].into(),
                    pencilmarks: vec![7],
                },
                Action::RemovePencilmarks {
                    figure: vec![54, 55, 56, 57, 58, 59, 61].into(),
                    pencilmarks: vec![7],
                },
                Action::RemovePencilmarks {
                    figure: vec![7, 16, 25, 43, 61, 70, 79].into(),
                    pencilmarks: vec![8],
                },
            ]),
        )
    }
    #[test]
    fn pointing_triples() {
        test_method(
            "930050000200630095856002000003180570005020980080005000000800159508210004000560008",
            Pointing::Triple,
            BTreeSet::from([
                Action::RemovePencilmarks {
                    figure: vec![75, 76, 77, 78, 79, 80].into(),
                    pencilmarks: vec![1],
                },
                Action::RemovePencilmarks {
                    figure: vec![0, 1, 2, 3, 4, 5].into(),
                    pencilmarks: vec![2],
                },
                Action::RemovePencilmarks {
                    figure: vec![5, 14, 23, 32, 41, 50].into(),
                    pencilmarks: vec![3],
                },
                Action::RemovePencilmarks {
                    figure: vec![18, 19, 20, 21, 22, 23].into(),
                    pencilmarks: vec![3],
                },
                Action::RemovePencilmarks {
                    figure: vec![0, 1, 2, 3, 4, 5].into(),
                    pencilmarks: vec![6],
                },
            ]),
        )
    }
}
