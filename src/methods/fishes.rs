use itertools::Itertools;
use std::{collections::HashMap, fmt::Display};

use crate::{action::Action, figure::Figure};

use super::Method;

#[derive(Clone, Debug)]
struct Candidate {
    rows: Vec<u8>,
    figure: Figure,
}

impl Candidate {
    fn new(figure: Figure, rotated: bool) -> Self {
        let rows = figure
            .clone()
            .into_iter()
            .map(|pos| {
                if rotated {
                    Figure::col_of(pos)
                } else {
                    Figure::row_of(pos)
                }
            })
            .collect();

        Self { rows, figure }
    }

    fn do_candidates_match(candidates: Vec<&Self>, dimensions: usize) -> Option<Vec<u8>> {
        // Row number to row count in candidates rows
        let mut all_rows: HashMap<u8, u8> = HashMap::new();

        for candidate in candidates {
            for row in &candidate.rows {
                if let Some(this_row_count) = all_rows.get_mut(&row) {
                    *this_row_count += 1;
                } else {
                    all_rows.insert(*row, 1);
                }
            }
        }

        if all_rows.len() == dimensions && all_rows.iter().all(|(_, count)| count >= &2) {
            return Some(all_rows.into_iter().map(|(row, _)| row).collect());
        }

        None
    }
}

#[derive(Clone)]
pub enum Fishes {
    XWing,
    Swordfish,
    Jellyfish,
}

impl Fishes {
    /// If not rotated, search in columns.
    /// If rotated, change every row to column, and vise versa. Logic does not changes.
    fn get_all_in_row_or_col(&self, grid: &crate::grid::Grid, rotated: bool) -> Vec<Action> {
        let mut res = vec![];

        let dimensions = match self {
            Fishes::XWing => 2,
            Fishes::Swordfish => 3,
            Fishes::Jellyfish => 4,
        };

        let mut numbers_to_candidates: HashMap<u8, Vec<Candidate>> = HashMap::new();

        for i in 0..9 {
            let column = if rotated {
                Figure::row(i)
            } else {
                Figure::col(i)
            };

            let info = grid.pencilmarks_info(column);

            let figure_candidates = info.iter().filter_map(|(pencilmark, positions)| {
                if (2..=dimensions).contains(&positions.len()) {
                    return Some((
                        *pencilmark,
                        Candidate::new(positions.clone().into(), rotated),
                    ));
                } else {
                    return None;
                }
            });

            for (number, candidate) in figure_candidates {
                if let Some(global_candidates) = numbers_to_candidates.get_mut(&number) {
                    global_candidates.push(candidate);
                } else {
                    numbers_to_candidates.insert(number, vec![candidate]);
                }
            }
        }

        for (number, candidates) in numbers_to_candidates {
            for combination in candidates.iter().combinations(dimensions) {
                if let Some(valid_candidate_rows) =
                    Candidate::do_candidates_match(combination.clone(), dimensions)
                {
                    let figures = valid_candidate_rows.into_iter().map(|row| {
                        if rotated {
                            Figure::col(row)
                        } else {
                            Figure::row(row)
                        }
                    });

                    let mut figure = vec![].into();

                    for f in figures {
                        figure += f;
                    }
                    for candidate in combination {
                        figure -= candidate.figure.clone();
                    }

                    res.push(Action::RemovePencilmarks(figure, vec![number]));
                }
            }
        }

        res
    }
}

impl Display for Fishes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fishes::XWing => write!(f, "X-Wing"),
            Fishes::Swordfish => write!(f, "Swordfish"),
            Fishes::Jellyfish => write!(f, "Jellyfish"),
        }
    }
}

impl Method for Fishes {
    fn get_all_applications(&self, grid: &crate::grid::Grid) -> Vec<Action> {
        let mut res = self.get_all_in_row_or_col(grid, false);

        res.append(&mut self.get_all_in_row_or_col(grid, true));

        res
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::grid::Grid;
    use crate::methods::test_method;
    use crate::methods::fishes::Fishes::{Jellyfish, Swordfish, XWing};

    use super::*;

    #[test]
    fn x_wing() {
        test_method(
            "100000569492056108056109240009640801064010000218035604040500016905061402621000005",
            XWing {},
            vec![
                Action::RemovePencilmarks(
                    vec![0, 1, 2, 4, 6, 7, 8, 72, 73, 74, 76, 78, 79, 80].into(),
                    vec![4],
                ),
                Action::RemovePencilmarks(
                    vec![0, 7, 9, 16, 18, 25, 45, 52, 54, 61, 63, 70, 72, 79].into(),
                    vec![5],
                ),
                Action::RemovePencilmarks(
                    vec![3, 7, 21, 25, 30, 34, 39, 43, 57, 61, 66, 70, 75, 79].into(),
                    vec![7],
                ),
                Action::RemovePencilmarks(
                    vec![12, 14, 21, 23, 30, 32, 39, 41, 48, 50, 57, 59, 66, 68].into(),
                    vec![4],
                ),
                Action::RemovePencilmarks(
                    vec![28, 29, 30, 31, 32, 33, 35, 37, 38, 39, 40, 41, 42, 44].into(),
                    vec![5],
                ),
            ],
        );

        test_method(
            "000000094760910050090002081070050010000709000080031067240100070010090045900000100",
            XWing {},
            vec![
                Action::RemovePencilmarks(
                    vec![1, 3, 4, 5, 6, 7, 8, 37, 39, 40, 41, 42, 43, 44].into(),
                    vec![1],
                ),
                Action::RemovePencilmarks(
                    vec![9, 11, 18, 20, 27, 29, 45, 47, 54, 56, 63, 65, 72, 74].into(),
                    vec![1],
                ),
                Action::RemovePencilmarks(
                    vec![36, 37, 38, 39, 41, 42, 44, 72, 73, 74, 75, 77, 78, 80].into(),
                    vec![2],
                ),
            ],
        )
    }

    #[test]
    fn swordfish() {
        test_method(
            "529410703006003002003200000052300076637050200190627530300069420200830600960742305",
            Swordfish,
            vec![Action::RemovePencilmarks(
                vec![
                    10, 11, 12, 14, 16, 17, 19, 20, 21, 23, 25, 26, 28, 29, 30, 32, 34, 35,
                ]
                .into(),
                vec![8],
            )],
        );

        test_method(
            "926000100537010420841000603259734816714060030368120040102000084485071360603000001",
            Swordfish,
            vec![Action::RemovePencilmarks(
                vec![
                    18, 19, 20, 21, 23, 24, 26, 54, 56, 57, 59, 60, 61, 62, 72, 74, 75, 77, 78, 80,
                ]
                .into(),
                vec![9],
            )],
        );

        test_method(
            "020043069003896200960025030890560013600030000030081026300010070009674302270358090",
            Swordfish,
            vec![
                Action::RemovePencilmarks(
                    vec![
                        2, 3, 5, 11, 12, 14, 20, 21, 23, 30, 47, 48, 50, 56, 65, 66, 68, 74, 75, 77,
                    ]
                    .into(),
                    vec![2],
                ),
                Action::RemovePencilmarks(
                    vec![
                        2, 6, 8, 11, 15, 17, 35, 38, 42, 44, 47, 51, 53, 56, 60, 62, 65, 69, 71,
                    ]
                    .into(),
                    vec![4],
                ),
                Action::RemovePencilmarks(
                    vec![
                        3, 5, 6, 12, 14, 15, 21, 23, 24, 30, 32, 33, 50, 60, 66, 68, 69, 75, 77, 78,
                    ]
                    .into(),
                    vec![9],
                ),
                Action::RemovePencilmarks(
                    vec![
                        10, 11, 12, 13, 14, 15, 17, 36, 37, 38, 40, 41, 42, 44, 46, 47, 49, 50, 51,
                        52, 53,
                    ]
                    .into(),
                    vec![4],
                ),
                Action::RemovePencilmarks(
                    vec![
                        27, 28, 30, 31, 33, 34, 35, 36, 37, 40, 42, 43, 44, 54, 55, 56, 58, 60, 61,
                        62,
                    ]
                    .into(),
                    vec![2],
                ),
                Action::RemovePencilmarks(
                    vec![
                        36, 37, 38, 40, 43, 44, 45, 46, 47, 49, 50, 52, 53, 54, 55, 56, 58, 60, 61,
                        62,
                    ]
                    .into(),
                    vec![9],
                ),
            ],
        );
    }

    #[test]
    fn jellyfish() {
        let mut grid = Grid::from_str(
            "050749080089003000600001390040007060000400809000000000060004010500210047010005030",
        )
        .unwrap();

        grid.set_pencilmarks(15, vec![4, 7]);
        grid.set_pencilmarks(51, vec![4, 7]);

        grid.set_pencilmarks(31, vec![3, 5, 8, 9]);
        grid.set_pencilmarks(40, vec![3, 5, 6]);
        grid.set_pencilmarks(49, vec![3, 5, 6, 8, 9]);

        grid.set_pencilmarks(54, vec![2, 7, 8, 9]);
        grid.set_pencilmarks(56, vec![2, 7, 8]);

        grid.set_pencilmarks(17, vec![1, 2, 4, 5]);

        grid.set_pencilmarks(9, vec![1, 2, 4]);

        let mut actions = Jellyfish.get_all_applications(&grid);

        actions.sort();

        let mut assertion = vec![
            Action::RemovePencilmarks(
                vec![
                    10, 11, 12, 13, 14, 16, 18, 19, 21, 22, 23, 24, 25, 45, 46, 47, 48, 49, 50, 52,
                    73, 75, 76, 77, 78, 79, 80,
                ]
                .into(),
                vec![4],
            ),
            Action::RemovePencilmarks(
                vec![
                    9, 10, 11, 12, 14, 15, 17, 18, 20, 21, 23, 24, 25, 26, 36, 38, 39, 40, 42, 44,
                    45, 47, 48, 49, 51, 53,
                ]
                .into(),
                vec![2],
            ),
            Action::RemovePencilmarks(
                vec![
                    9, 11, 15, 17, 18, 20, 24, 26, 36, 38, 42, 44, 45, 47, 51, 53, 63, 65, 69, 71,
                ]
                .into(),
                vec![2],
            ),
            Action::RemovePencilmarks(
                vec![
                    0, 2, 6, 8, 11, 18, 24, 27, 29, 33, 35, 36, 38, 42, 44, 45, 47, 54, 56, 60, 62,
                    63, 65, 69, 71, 78, 80,
                ]
                .into(),
                vec![4],
            ),
        ];

        assertion.sort();

        assert_eq!(actions, assertion);
    }
}
