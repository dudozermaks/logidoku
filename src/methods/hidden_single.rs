use crate::{action::Action, figure::Figure};

use super::MethodCreator;

pub struct HiddenSingleCreator {}

impl MethodCreator for HiddenSingleCreator {
    fn get_all_applications(&self, grid: &crate::grid::Grid) -> Vec<Action> {
        let mut res = vec![];

        for f in Figure::all_figures() {
            grid.pencilmarks_info(f)
                .iter()
                .filter_map(|(pencilmark, positions)| {
                    if positions.len() == 1 {
                        Some(Action::PlaceNumber(positions[0], *pencilmark))
                    } else {
                        None
                    }
                })
                .for_each(|hs| res.push(hs));
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{
        action::Action,
        cell::Cell,
        grid::Grid,
        methods::{hidden_single::HiddenSingleCreator, MethodCreator},
    };

    #[test]
    fn get_and_apply() {
        let mut grid = Grid::from_str(
            "000004028406000005100030600000301000087000140000709000002010003900000507670400000",
        )
        .unwrap();

        let mut candidates = HiddenSingleCreator {}.get_all_applications(&grid);
        assert_eq!(
            candidates.sort(),
            vec![
                Action::PlaceNumber(0, 7),
                Action::PlaceNumber(3, 1),
                Action::PlaceNumber(16, 1),
                Action::PlaceNumber(20, 8,),
                Action::PlaceNumber(26, 4),
                Action::PlaceNumber(36, 3),
                Action::PlaceNumber(44, 9),
                Action::PlaceNumber(54, 8),
                Action::PlaceNumber(59, 7),
                Action::PlaceNumber(60, 4),
                Action::PlaceNumber(80, 1),
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
