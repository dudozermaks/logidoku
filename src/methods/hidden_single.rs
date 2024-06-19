use crate::figure::Figure;

use super::{Method, MethodCreator};

pub struct HiddenSingleCreator {}

impl MethodCreator for HiddenSingleCreator {
    type Method = HiddenSingle;

    fn get_all_applications(&self, grid: &crate::grid::Grid) -> Vec<Self::Method>
    where
        Self::Method: Method,
    {
        let mut res = vec![];

        for f in Figure::all_figures() {
            grid.pencilmarks_info(f)
                .iter()
                .filter_map(|(pencilmark, positions)| {
                    if positions.len() == 1 {
                        Some(HiddenSingle {
                            position: positions[0],
                            number_to_place: *pencilmark,
                        })
                    } else {
                        None
                    }
                })
                .for_each(|hs| res.push(hs));
        }

        res
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct HiddenSingle {
    position: usize,
    number_to_place: u8,
}

impl Method for HiddenSingle {
    fn apply_to_grid(&self, grid: &mut crate::grid::Grid) {
        grid.set_number(self.position, self.number_to_place);
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{
        cell::Cell,
        grid::Grid,
        methods::{
            hidden_single::{HiddenSingle, HiddenSingleCreator},
            Method, MethodCreator,
        },
    };

    #[test]
    fn get_and_apply() {
        let mut grid = Grid::from_str(
            "000004028406000005100030600000301000087000140000709000002010003900000507670400000",
        )
        .unwrap();

        let mut candidates = HiddenSingleCreator{}.get_all_applications(&grid);
        assert_eq!(
            candidates.sort(),
            vec![
                HiddenSingle {
                    position: 0,
                    number_to_place: 7
                },
                HiddenSingle {
                    position: 3,
                    number_to_place: 1
                },
                HiddenSingle {
                    position: 16,
                    number_to_place: 1
                },
                HiddenSingle {
                    position: 20,
                    number_to_place: 8,
                },
                HiddenSingle {
                    position: 26,
                    number_to_place: 4
                },
                HiddenSingle {
                    position: 36,
                    number_to_place: 3
                },
                HiddenSingle {
                    position: 44,
                    number_to_place: 9
                },
                HiddenSingle {
                    position: 54,
                    number_to_place: 8
                },
                HiddenSingle {
                    position: 59,
                    number_to_place: 7
                },
                HiddenSingle {
                    position: 60,
                    number_to_place: 4
                },
                HiddenSingle {
                    position: 80,
                    number_to_place: 1
                },
            ]
            .sort()
        );

        for candidate in candidates {
            candidate.apply_to_grid(&mut grid);
        }

        assert_eq!(grid[0], Cell::Number(7));
        assert_eq!(grid[3], Cell::Number(1));
        assert_eq!(grid[16], Cell::Number(1));
        assert_eq!(grid[20], Cell::Number(8));
        assert_eq!(grid[26], Cell::Number(4));
        assert_eq!(grid[36], Cell::Number(3));
        assert_eq!(grid[44], Cell::Number(9));
        assert_eq!(grid[54], Cell::Number(8));
        assert_eq!(grid[59], Cell::Number(7));
        assert_eq!(grid[60], Cell::Number(4));
        assert_eq!(grid[80], Cell::Number(1));
    }
}
