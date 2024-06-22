use crate::{action::Action, figure::Figure};

use super::MethodCreator;

struct BoxLineReductionCreator {
    n: u8,
}

impl BoxLineReductionCreator {
    fn find_in_figure(
        &self,
        grid: &crate::grid::Grid,
        figure: &Figure,
    ) -> Vec<crate::action::Action> {
        let mut res = vec![];
        let pencilmarks_info = grid.pencilmarks_info(figure.clone());

        for (pencilmark, positions) in pencilmarks_info {
            if positions.len() == self.n.into() {
                let pencilmarks_figure: Figure = positions.into();

                if let Some(sqr) = pencilmarks_figure.is_on_the_same_sqr() {
                    res.push(Action::RemovePencilmarks(
                        Figure::sqr(sqr) - pencilmarks_figure,
                        vec![pencilmark],
                    ))
                }
            }
        }

        res
    }
}

impl MethodCreator for BoxLineReductionCreator {
    fn get_all_applications(&self, grid: &crate::grid::Grid) -> Vec<crate::action::Action> {
        let mut res = vec![];

        for i in 0..9 {
            res.append(&mut self.find_in_figure(grid, &Figure::row(i)));
            res.append(&mut self.find_in_figure(grid, &Figure::col(i)));
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
            BoxLineReductionCreator { n: 2 },
            vec![
                Action::RemovePencilmarks(vec![1, 2, 10, 11, 18, 19, 20].into(), vec![4]),
                Action::RemovePencilmarks(vec![3, 4, 5, 12, 21, 22, 23].into(), vec![6]),
                Action::RemovePencilmarks(vec![3, 5, 12, 13, 14, 21, 23].into(), vec![9]),
                Action::RemovePencilmarks(vec![6, 7, 8, 15, 16, 24, 25].into(), vec![5]),
                Action::RemovePencilmarks(vec![6, 7, 8, 16, 24, 25, 26].into(), vec![1]),
                Action::RemovePencilmarks(vec![6, 8, 15, 17, 24, 25, 26].into(), vec![4]),
                Action::RemovePencilmarks(vec![30, 31, 32, 39, 40, 41, 48].into(), vec![8]),
                Action::RemovePencilmarks(vec![30, 31, 32, 41, 48, 49, 50].into(), vec![3]),
                Action::RemovePencilmarks(vec![30, 39, 40, 41, 48, 49, 50].into(), vec![6]),
                Action::RemovePencilmarks(vec![33, 42, 43, 44, 51, 52, 53].into(), vec![9]),
                Action::RemovePencilmarks(vec![57, 58, 59, 66, 67, 68, 75].into(), vec![3]),
                Action::RemovePencilmarks(vec![57, 58, 59, 68, 75, 76, 77].into(), vec![4]),
                Action::RemovePencilmarks(vec![60, 61, 69, 70, 71, 78, 79].into(), vec![8]),
            ],
        );
    }
    #[test]
    fn box_line_reduction_3_cells() {
        test_method(
            "020943715904000600750000040500480000200000453400352000042000081005004260090208504",
            BoxLineReductionCreator { n: 3 },
            vec![
                Action::RemovePencilmarks(vec![3, 4, 5, 21, 22, 23].into(), vec![7]),
                Action::RemovePencilmarks(vec![27, 28, 36, 37, 45, 46].into(), vec![9]),
                Action::RemovePencilmarks(vec![27, 29, 36, 38, 45, 47].into(), vec![6]),
                Action::RemovePencilmarks(vec![55, 56, 64, 65, 73, 74].into(), vec![3]),
                Action::RemovePencilmarks(vec![57, 59, 66, 68, 75, 77].into(), vec![3]),
                Action::RemovePencilmarks(vec![66, 67, 68, 75, 76, 77].into(), vec![7]),
            ],
        );
    }
}
