use crate::{action::Action, figure::Figure};

use super::MethodCreator;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Candidate {
    pencilmark: u8,
    positions: (usize, usize),
}

pub struct XWingCreator {}

impl XWingCreator {
    fn get_all(&self, grid: &crate::grid::Grid, in_row: bool) -> Vec<crate::action::Action> {
        let mut res = vec![];

        let mut candidates = vec![];

        for i in 0..9 {
            let figure = if in_row {
                Figure::row(i)
            } else {
                Figure::col(i)
            };

            candidates.extend(grid.pencilmarks_info(figure).iter().filter_map(
                |(pencilmark, positions)| {
                    if positions.len() != 2 {
                        return None;
                    } else {
                        Some(Candidate {
                            pencilmark: *pencilmark,
                            positions: (positions[0], positions[1]),
                        })
                    }
                },
            ));
        }

        for (to_skip, candidate1) in candidates.iter().enumerate() {
            for candidate2 in candidates.iter().skip(to_skip + 1) {
                if candidate1.pencilmark != candidate2.pencilmark {
                    continue;
                }

                let figure1 = Figure::from(vec![candidate1.positions.0, candidate2.positions.0]);
                let figure2 = Figure::from(vec![candidate1.positions.1, candidate2.positions.1]);

                let on_same_figure = if in_row {
                    figure1.is_on_the_same_col().is_some() && figure2.is_on_the_same_col().is_some()
                } else {
                    figure1.is_on_the_same_row().is_some() && figure2.is_on_the_same_row().is_some()
                };

                if on_same_figure {
                    let (same1, same2) = if in_row {
                        (
                            Figure::col(figure1.is_on_the_same_col().unwrap()),
                            Figure::col(figure2.is_on_the_same_col().unwrap()),
                        )
                    } else {
                        (
                            Figure::row(figure1.is_on_the_same_row().unwrap()),
                            Figure::row(figure2.is_on_the_same_row().unwrap()),
                        )
                    };

                    res.push(Action::RemovePencilmarks(
                        same1 + same2 - figure1 - figure2,
                        vec![candidate1.pencilmark],
                    ));
                }
            }
        }

        res
    }
}

impl MethodCreator for XWingCreator {
    fn get_all_applications(&self, grid: &crate::grid::Grid) -> Vec<crate::action::Action> {
        let mut res = self.get_all(grid, true);

        res.append(&mut self.get_all(grid, false));

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::methods::test_method;

    use super::*;

    #[test]
    fn x_wing() {
        test_method(
            "100000569492056108056109240009640801064010000218035604040500016905061402621000005",
            XWingCreator {},
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
            XWingCreator {},
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
}
