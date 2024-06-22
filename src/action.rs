use crate::{cell::Cell, figure::Figure};

#[derive(Ord, Eq, PartialEq, PartialOrd, Debug)]
pub enum Action {
    /// 1. Position
    /// 2. Number
    PlaceNumber(usize, u8),
    /// 1. Figure to remove from
    /// 2. Pencilmarks to remove
    RemovePencilmarks(Figure, Vec<u8>),
    /// 1. Figure, in which to preserve
    /// 2. Pencilmarks to preserve
    PreservePencilmarks(Figure, Vec<u8>),
}

// TODO: Test
// TODO: add method like: is_useful(&grid)
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
                    .into_iter()
                    .filter(|p| pencilmarks.contains(&p) == preserve)
                    .cloned()
                    .collect();

                grid.set_pencilmarks(i, new_pencilmarks);
            }
        }
    }
    pub fn apply_to_grid(&self, grid: &mut crate::grid::Grid) {
        match self {
            Action::PlaceNumber(position, number) => {
                grid.set_number(*position, *number);
            }
            Action::RemovePencilmarks(figure, pencilmarks) => {
                Self::remove_or_preserve_pencilmarks(grid, figure, pencilmarks, false);
            }
            Action::PreservePencilmarks(figure, pencilmarks) => {
                Self::remove_or_preserve_pencilmarks(grid, figure, pencilmarks, true);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{cell::Cell, grid::Grid};

    use super::Action;

    #[test]
    fn place_number() {
        let mut grid = Grid::from_str(
            "300967001040302080020000070070000090000873000500010003004705100905000207800621004",
        )
        .unwrap();

        let action = Action::PlaceNumber(2, 8);
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

        let action = Action::RemovePencilmarks(figure, vec![1, 6]);
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

        let action = Action::PreservePencilmarks(vec![29, 38].into(), vec![2, 4]);

        action.apply_to_grid(&mut grid);

        assert_eq!(grid[29], Cell::Pencilmarks(vec![2, 4]));
        assert_eq!(grid[38], Cell::Pencilmarks(vec![2, 4]));
    }
}
