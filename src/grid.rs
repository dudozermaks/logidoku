use std::{array, collections::HashMap, ops::Index, str::FromStr};

use crate::{cell::Cell, figure::Figure};

/// Grid represents 9 by 9 matrix of [Cells]
#[derive(Debug, PartialEq, Clone)]
pub struct Grid {
    matrix: [Cell; 81],
}

impl Grid {
    fn updtae_cell_neighbours(&mut self, i: usize) {
        let center_cell = self.matrix[i].clone();

        if let Cell::Number(n) = center_cell {
            for neighbour_pos in Figure::neighbours(i) {
                let neighbour = &mut self.matrix[neighbour_pos];
                match neighbour {
                    Cell::Number(_) => (),
                    Cell::Pencilmarks(p) => p.retain(|&x| x != n),
                };
            }
        }
    }

    pub fn set_number(&mut self, position: usize, number: u8) {
        self.matrix[position] = Cell::Number(number);
        self.updtae_cell_neighbours(position);
    }

    pub fn set_pencilmarks(&mut self, position: usize, pencilmarks: Vec<u8>) {
        self.matrix[position] = Cell::Pencilmarks(pencilmarks);
    }

    /// Returns map: number to cell in which it occurs.
    pub fn pencilmarks_info(&self, figure: Figure) -> HashMap<u8, Vec<usize>> {
        let mut res: HashMap<u8, Vec<usize>> = HashMap::new();

        for i in figure {
            if let Cell::Pencilmarks(pencilmarks) = &self[i] {
                for pencilmark in pencilmarks {
                    if let Some(info) = res.get_mut(pencilmark) {
                        info.push(i);
                    } else {
                        res.insert(*pencilmark, vec![i]);
                    }
                }
            }
        }

        res
    }

    /// Returns `true` if every cell in grid is `Number`
    pub fn is_solved(&self) -> bool {
        !self.matrix.iter().any(|cell| cell.is_pencilmarks())
    }
}

impl FromStr for Grid {
    type Err = ParseGridError;

    /// For now, only supports one-line Sudoku.
    /// Empty cell - 0. Everything else returns InvalidCharacter error.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 9 * 9 {
            return Err(ParseGridError::InvalidSize(s.len()));
        }

        let mut matrix = array::from_fn(|_| Cell::all_pencilmarks());

        for (i, c) in s.chars().enumerate() {
            let digit = c.to_digit(10);
            if let Some(digit) = digit {
                if digit != 0 {
                    matrix[i] = Cell::Number(digit as u8);
                }
            } else {
                return Err(ParseGridError::InvalidCharacter(i));
            }
        }

        let mut grid = Self { matrix };

        for cell in Figure::all_cells() {
            grid.updtae_cell_neighbours(cell);
        }

        Ok(grid)
    }
}

impl Index<usize> for Grid {
    type Output = Cell;

    fn index(&self, index: usize) -> &Self::Output {
        &self.matrix[index]
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseGridError {
    InvalidSize(usize),
    InvalidCharacter(usize),
}

impl std::fmt::Display for ParseGridError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ParseGridError::InvalidSize(size) => write!(f, "Invalid Sudoku grid size: {}", size),
            ParseGridError::InvalidCharacter(pos) => {
                write!(f, "Invalid character at position: {}", pos)
            }
        }
    }
}

impl std::error::Error for ParseGridError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_string_size_is_too_small() {
        let grid = Grid::from_str(
            "40100305000060508489540013603006040590005030005000120024050000700900050050009200",
        );
        assert_eq!(grid, Err(ParseGridError::InvalidSize(80)));
    }

    #[test]
    fn init_string_size_is_too_big() {
        let grid = Grid::from_str(
            "4010030500006050848954001360300604059000503000500012002405000070090005005000920001234",
        );
        assert_eq!(grid, Err(ParseGridError::InvalidSize(85)));
    }

    #[test]
    fn init_string_chars_are_invalid() {
        let grid = Grid::from_str(
            "4 1  3 5    6 5 848954  136 3  6 4 59   5 3   5   120 24 5    7  9   5  5   92   ",
        );
        assert_eq!(grid, Err(ParseGridError::InvalidCharacter(1)));
    }

    #[test]
    fn init_string_is_ok() {
        let grid = Grid::from_str(
            "401003050000605084895400136030060405900050300050001200240500007009000500500092000",
        )
        .unwrap();

        assert_eq!(grid.matrix[8], Cell::Pencilmarks([2, 9].to_vec()));
        assert_eq!(grid.matrix[38], Cell::Pencilmarks([2, 4, 6, 7, 8].to_vec()));
        assert_eq!(grid.matrix[80], Cell::Pencilmarks([1, 3, 8].to_vec()));

        assert_eq!(grid.matrix[0], Cell::Number(4));
        assert_eq!(grid.matrix[2], Cell::Number(1));
        assert_eq!(grid.matrix[26], Cell::Number(6));
        assert_eq!(grid.matrix[36], Cell::Number(9));
    }

    #[test]
    fn set_number() {
        let mut grid = Grid::from_str(
            "000004028406000005100030600000301000087000140000709000002010003900000507670400000",
        )
        .unwrap();

        grid.set_number(40, 5);

        let grid_should_be = Grid::from_str(
            //                                       * here is this 5
            "000004028406000005100030600000301000087050140000709000002010003900000507670400000",
        )
        .unwrap();

        assert_eq!(grid, grid_should_be);
    }

    #[test]
    fn set_pencilmarks() {
        let mut grid = Grid::from_str(
            "000004028406000005100030600000301000087000140000709000002010003900000507670400000",
        )
        .unwrap();

        let mut grid_should_be = grid.clone();

        grid.set_pencilmarks(40, vec![2, 5]);

        grid_should_be.matrix[40] = Cell::Pencilmarks(vec![2, 5]);

        assert_eq!(grid, grid_should_be);
    }
    #[test]
    fn update_cell_neighbours() {
        let mut grid = Grid::from_str(
            "000004028406000005100030600000301000087000140000709000002010003900000507670400000",
        )
        .unwrap();

        // Set manually, so that neighbours do not update
        grid.matrix[40] = Cell::Number(5);

        let mut grid_should_be = grid.clone();

        // Column
        grid_should_be.matrix[4] = Cell::Pencilmarks(vec![6, 7, 9]);
        grid_should_be.matrix[13] = Cell::Pencilmarks(vec![2, 7, 8, 9]);
        grid_should_be.matrix[31] = Cell::Pencilmarks(vec![2, 4, 6, 8]);
        grid_should_be.matrix[49] = Cell::Pencilmarks(vec![2, 4, 6, 8]);
        grid_should_be.matrix[67] = Cell::Pencilmarks(vec![2, 6, 8]);
        grid_should_be.matrix[76] = Cell::Pencilmarks(vec![2, 8, 9]);
        // Row
        grid_should_be.matrix[36] = Cell::Pencilmarks(vec![2, 3]);
        grid_should_be.matrix[39] = Cell::Pencilmarks(vec![2, 6]);
        grid_should_be.matrix[41] = Cell::Pencilmarks(vec![2, 6]);
        grid_should_be.matrix[44] = Cell::Pencilmarks(vec![2, 6, 9]);
        // Square (31, 39, 41, 49) - already done

        grid.updtae_cell_neighbours(40);

        assert_eq!(grid, grid_should_be);
    }

    #[test]
    fn pencilmarks_info() {
        let grid = Grid::from_str(
            "000004028406000005100030600000301000087000140000709000002010003900000507670400000",
        )
        .unwrap();

        let info = grid.pencilmarks_info(Figure::row(0));

        assert_eq!(
            info,
            HashMap::from([
                (1, vec![3]),
                (3, vec![0, 1, 2, 6]),
                (5, vec![0, 1, 2, 3, 4]),
                (6, vec![3, 4]),
                (7, vec![0, 4, 6]),
                (9, vec![1, 2, 3, 4, 6]),
            ])
        );
    }

    #[test]
    fn is_solved() {
        let unsolved_grid = Grid::from_str(
            "007010000000800500180009064600000003071080640400000005840600031005002000000030700",
        )
        .unwrap();
        let solved_grid = Grid::from_str(
            "567314928394826517182759364658247193971583642423961875849675231735192486216438759",
        )
        .unwrap();

        assert!(!unsolved_grid.is_solved());
        assert!(solved_grid.is_solved());
    }
}
