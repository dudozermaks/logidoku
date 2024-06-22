use crate::{action::Action, figure::Figure};

use super::MethodCreator;

pub struct PointingNsCreator {
    n: u8,
}

impl MethodCreator for PointingNsCreator {
    fn get_all_applications(&self, grid: &crate::grid::Grid) -> Vec<crate::action::Action> {
        let mut res = vec![];

        for sqr_number in 0..9 {
            let sqr = Figure::sqr(sqr_number);
            let pencilmarks_info = grid.pencilmarks_info(sqr);

            for (pencilmark, positions) in pencilmarks_info {
                if positions.len() == self.n.into() {
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
                        res.push(Action::RemovePencilmarks(figure_to_add, vec![pencilmark]));
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
            PointingNsCreator { n: 2 },
            vec![
                Action::RemovePencilmarks(vec![2, 11, 20, 38, 56, 65, 74].into(), vec![1]),
                Action::RemovePencilmarks(vec![3, 12, 21, 39, 57, 66, 75].into(), vec![1]),
                Action::RemovePencilmarks(vec![9, 10, 11, 13, 15, 16, 17].into(), vec![2]),
                Action::RemovePencilmarks(vec![2, 11, 20, 29, 38, 47, 74].into(), vec![3]),
                Action::RemovePencilmarks(vec![54, 55, 56, 57, 58, 59, 62].into(), vec![4]),
                Action::RemovePencilmarks(vec![2, 29, 38, 47, 56, 65, 74].into(), vec![5]),
                Action::RemovePencilmarks(vec![36, 37, 38, 39, 42, 43, 44].into(), vec![5]),
                Action::RemovePencilmarks(vec![3, 12, 21, 39, 57, 66, 75].into(), vec![6]),
                Action::RemovePencilmarks(vec![6, 15, 24, 42, 60, 69, 78].into(), vec![6]),
                Action::RemovePencilmarks(vec![1, 10, 19, 37, 55, 64, 73].into(), vec![7]),
                Action::RemovePencilmarks(vec![54, 55, 56, 57, 58, 59, 61].into(), vec![7]),
                Action::RemovePencilmarks(vec![7, 16, 25, 43, 61, 70, 79].into(), vec![8]),
            ],
        )
    }
    #[test]
    fn pointing_triples() {
        test_method(
            "930050000200630095856002000003180570005020980080005000000800159508210004000560008",
            PointingNsCreator { n: 3 },
            vec![
                Action::RemovePencilmarks(vec![75, 76, 77, 78, 79, 80].into(), vec![1]),
                Action::RemovePencilmarks(vec![0, 1, 2, 3, 4, 5].into(), vec![2]),
                Action::RemovePencilmarks(vec![5, 14, 23, 32, 41, 50].into(), vec![3]),
                Action::RemovePencilmarks(vec![18, 19, 20, 21, 22, 23].into(), vec![3]),
                Action::RemovePencilmarks(vec![0, 1, 2, 3, 4, 5].into(), vec![6]),
            ],
        )
    }
}
