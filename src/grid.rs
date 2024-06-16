use std::{str::FromStr, array};

use crate::cell::Cell;

/// Grid represents 9 by 9 matrix of [Cells]
#[derive(Debug, PartialEq)]
pub struct Grid {
    matrix: [Cell; 81],
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

        Ok(Self { matrix })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_string_size_is_too_small() {
        let grid = Grid::from_str(
            "40100305000060508489540013603006040590005030005000128024050000700900050050009200",
        );
        assert_eq!(grid, Err(ParseGridError::InvalidSize));
    }

    #[test]
    fn init_string_size_is_too_big() {
        let grid = Grid::from_str(
            "4010030500006050848954001360300604059000503000500012802405000070090005005000920001234",
        );
        assert_eq!(grid, Err(ParseGridError::InvalidSize));
    }

    #[test]
    fn init_string_chars_are_invalid() {
        let grid = Grid::from_str(
            "4 1  3 5    6 5 848954  136 3  6 4 59   5 3   5   128 24 5    7  9   5  5   92   ",
        );
        assert_eq!(grid, Err(ParseGridError::InvalidCharacter(1)));
    }

    #[test]
    fn init_string_is_ok() {
        let grid = Grid::from_str(
            "401003050000605084895400136030060405900050300050001280240500007009000500500092000",
        );
        assert!(grid.is_ok());
    }
}
