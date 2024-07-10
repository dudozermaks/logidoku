use crate::{cell::Cell, figure::Figure};

#[derive(Ord, Eq, PartialEq, PartialOrd, Debug, Clone)]
pub enum Action {
    PlaceNumber {
        position: usize,
        number: u8,
    },
    RemovePencilmarks {
        figure: Figure,
        pencilmarks: Vec<u8>,
    },
    PreservePencilmarks {
        figure: Figure,
        pencilmarks: Vec<u8>,
    },
}

impl Action {
    fn remove_or_preserve_pencilmarks(
        grid: &mut crate::grid::Grid,
        figure: &Figure,
        pencilmarks: &Vec<u8>,
        preserve: bool,
    ) {
        for i in figure.clone() {
            if let Cell::Pencilmarks(old_pencilmarks) = &grid[i] {
                let new_pencilmarks = old_pencilmarks
                    .iter()
                    .filter(|p| pencilmarks.contains(p) == preserve)
                    .cloned()
                    .collect();

                grid.set_pencilmarks(i, new_pencilmarks);
            }
        }
    }
    pub fn apply_to_grid(&self, grid: &mut crate::grid::Grid) {
        match self {
            Action::PlaceNumber { position, number } => {
                grid.set_number(*position, *number);
            }
            Action::RemovePencilmarks {
                figure,
                pencilmarks,
            } => {
                Self::remove_or_preserve_pencilmarks(grid, figure, pencilmarks, false);
            }
            Action::PreservePencilmarks {
                figure,
                pencilmarks,
            } => {
                Self::remove_or_preserve_pencilmarks(grid, figure, pencilmarks, true);
            }
        }
    }
    /// Returns true if method is doing something to grid.
    /// For example: if Action is RemovePencilmarks 1 and 2,
    /// but, there is no such pencilmarks on this row: return false.
    pub fn is_helpful(&self, grid: &crate::grid::Grid) -> bool {
        match self {
            Action::PlaceNumber { position, .. } => grid[*position].is_pencilmarks(),
            Action::RemovePencilmarks {
                figure,
                pencilmarks,
            } => {
                for i in figure.clone() {
                    if let Cell::Pencilmarks(cell_pencilmarks) = &grid[i] {
                        if pencilmarks.iter().any(|p| cell_pencilmarks.contains(p)) {
                            return true;
                        }
                    }
                }
                false
            }
            Action::PreservePencilmarks {
                figure,
                pencilmarks,
            } => {
                for i in figure.clone() {
                    if let Cell::Pencilmarks(cell_pencilmarks) = &grid[i] {
                        if !cell_pencilmarks.iter().all(|p| pencilmarks.contains(p)) {
                            return true;
                        }
                    }
                }
                false
            }
        }
    }

    // pub fn simplify(&mut self, grid: &crate::grid::Grid) {
    // match self {
    //     Action::PlaceNumber(_, _) => todo!(),
    //     Action::RemovePencilmarks(_, _) => todo!(),
    //     Action::PreservePencilmarks(_, _) => todo!(),
    // }
    // }
}

#[cfg(test)]
mod tests {
    use std::{str::FromStr, vec};

    use crate::{cell::Cell, grid::Grid};

    use super::Action;

    #[test]
    fn place_number() {
        let mut grid = Grid::from_str(
            "300967001040302080020000070070000090000873000500010003004705100905000207800621004",
        )
        .unwrap();

        let action = Action::PlaceNumber {
            position: 2,
            number: 8,
        };
        action.apply_to_grid(&mut grid);

        let grid_should_be = Grid::from_str(
            "308967001040302080020000070070000090000873000500010003004705100905000207800621004",
        )
        .unwrap();

        assert_eq!(grid, grid_should_be);
    }

    #[test]
    fn remove_pencilmarks() {
        let mut grid = Grid::from_str(
            "400000938032094100095300240370609004529001673604703090957008300003900400240030709",
        )
        .unwrap();

        let figure = vec![0, 3, 4, 5, 6, 7, 8].into();

        let action = Action::RemovePencilmarks {
            figure,
            pencilmarks: vec![1, 6],
        };
        action.apply_to_grid(&mut grid);

        assert_eq!(grid[0], Cell::Number(4));
        assert_eq!(grid[1], Cell::Pencilmarks(vec![1, 6]));
        assert_eq!(grid[2], Cell::Pencilmarks(vec![1, 6]));
        assert_eq!(grid[3], Cell::Pencilmarks(vec![2, 5]));
        assert_eq!(grid[4], Cell::Pencilmarks(vec![2, 5, 7]));
        assert_eq!(grid[5], Cell::Pencilmarks(vec![2, 5, 7]));
        assert_eq!(grid[6], Cell::Number(9));
        assert_eq!(grid[7], Cell::Number(3));
        assert_eq!(grid[8], Cell::Number(8));
    }

    #[test]
    fn preserve_pencilmarks() {
        let mut grid = Grid::from_str(
            "720408030080000047401076802810739000000851000000264080209680413340000008168943275",
        )
        .unwrap();

        let action = Action::PreservePencilmarks {
            figure: vec![29, 38].into(),
            pencilmarks: vec![2, 4],
        };

        action.apply_to_grid(&mut grid);

        assert_eq!(grid[29], Cell::Pencilmarks(vec![2, 4]));
        assert_eq!(grid[38], Cell::Pencilmarks(vec![2, 4]));
    }

    #[test]
    fn is_helpful() {
        let grid = Grid::from_str(
            "020943715904000600750000040500480000200000453400352000042000081005004260090208504",
        )
        .unwrap();

        {
            let place = Action::PlaceNumber {
                position: 1,
                number: 2,
            };
            assert!(!place.is_helpful(&grid));

            let remove = Action::RemovePencilmarks {
                figure: vec![0, 2].into(),
                pencilmarks: vec![1, 2],
            };
            assert!(!remove.is_helpful(&grid));

            let preserve = Action::PreservePencilmarks {
                figure: vec![0, 2].into(),
                pencilmarks: vec![8, 6],
            };
            assert!(!preserve.is_helpful(&grid));
        }

        {
            let place = Action::PlaceNumber {
                position: 66,
                number: 1,
            };
            assert!(place.is_helpful(&grid));

            let remove = Action::RemovePencilmarks {
                figure: vec![9, 10, 11, 18, 19, 20].into(),
                pencilmarks: vec![8, 6],
            };
            assert!(remove.is_helpful(&grid));

            let preserve = Action::PreservePencilmarks {
                figure: vec![10, 20].into(),
                pencilmarks: vec![1, 3],
            };
            assert!(preserve.is_helpful(&grid));
        }
    }
}
