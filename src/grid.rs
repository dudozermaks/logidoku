use std::{array, str::FromStr};

use crate::{cell::Cell, figure::Figure};

/// Grid represents 9 by 9 matrix of [Cells]
#[derive(Debug, PartialEq)]
pub struct Grid {
    matrix: [Cell; 81],
}

impl Grid {
    pub fn updtae_cell_neighbours(&mut self, i: usize) {
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
}

impl FromStr for Grid {
    type Err = ParseGridError;

    /// For now, only supports one-line Sudoku.
    /// Empty cell - 0. Everything else returns InvalidCharacter error.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 9 * 9 {
            return Err(ParseGridError::InvalidSize);
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

        for cell in Figure::all() {
            grid.updtae_cell_neighbours(cell);
        }

        Ok(grid)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseGridError {
    InvalidSize,
    InvalidCharacter(usize),
}

impl std::fmt::Display for ParseGridError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ParseGridError::InvalidSize => write!(f, "Invalid Sudoku grid size"),
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
        assert_eq!(grid, Err(ParseGridError::InvalidSize));
    }

    #[test]
    fn init_string_size_is_too_big() {
        let grid = Grid::from_str(
            "4010030500006050848954001360300604059000503000500012002405000070090005005000920001234",
        );
        assert_eq!(grid, Err(ParseGridError::InvalidSize));
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
        );
        assert!(grid.is_ok());

        let grid = grid.unwrap();

        assert_eq!(grid.matrix[8], Cell::Pencilmarks([2, 9].to_vec()));
        assert_eq!(grid.matrix[38], Cell::Pencilmarks([2, 4, 6, 7, 8].to_vec()));
        assert_eq!(grid.matrix[80], Cell::Pencilmarks([1, 3, 8].to_vec()));

        assert_eq!(grid.matrix[0], Cell::Number(4));
        assert_eq!(grid.matrix[2], Cell::Number(1));
        assert_eq!(grid.matrix[26], Cell::Number(6));
        assert_eq!(grid.matrix[36], Cell::Number(9));
    }
}
