use std::{collections::BTreeSet, ops::Sub};

/// Figure can be row, column, square, or some set of positions.
#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Ord)]
pub struct Figure {
    // Choose BTreeSet here because iterating over its elements is deterministic
    positions: BTreeSet<usize>,
}

impl Figure {
    pub fn row_checked(n: u8) -> Result<Figure, FigureNumberOutOfBoundError> {
        if n > 8 {
            return Err(FigureNumberOutOfBoundError);
        }

        Ok(Figure {
            positions: BTreeSet::from_iter((0..9_usize).map(|i| i + n as usize * 9)),
        })
    }
    pub fn col_checked(n: u8) -> Result<Figure, FigureNumberOutOfBoundError> {
        if n > 8 {
            return Err(FigureNumberOutOfBoundError);
        }

        Ok(Figure {
            positions: BTreeSet::from_iter((0..9_usize).map(|i| i * 9 + n as usize)),
        })
    }
    pub fn sqr_checked(n: u8) -> Result<Figure, FigureNumberOutOfBoundError> {
        if n > 8 {
            return Err(FigureNumberOutOfBoundError);
        }

        const ZEROETH_SQUARE: [u8; 9] = [0, 1, 2, 9, 10, 11, 18, 19, 20];

        Ok(Figure {
            positions: BTreeSet::from_iter(ZEROETH_SQUARE.into_iter().map(|i| {
                let a = 3 * (n % 3);
                let b = 27 * (n / 3);

                (i + a + b) as usize
            })),
        })
    }

    pub fn neighbours_checked(i: usize) -> Result<Figure, FigureNumberOutOfBoundError> {
        if i >= 9 * 9 {
            return Err(FigureNumberOutOfBoundError);
        }
        let row = i / 9;
        let col = i % 9;
        let sqr = (row / 3) * 3 + col / 3;

        Ok(Figure::row(row as u8) + Figure::col(col as u8) + Figure::sqr(sqr as u8))
    }

    /// Panics if n > 8
    pub fn row(n: u8) -> Figure {
        Figure::row_checked(n).unwrap()
    }

    /// Panics if n > 8
    pub fn col(n: u8) -> Figure {
        Figure::col_checked(n).unwrap()
    }

    /// Panics if n > 8
    pub fn sqr(n: u8) -> Figure {
        Figure::sqr_checked(n).unwrap()
    }

    /// Panics if n > 80
    pub fn neighbours(i: usize) -> Figure {
        Figure::neighbours_checked(i).unwrap()
    }

    pub fn all_cells() -> Figure {
        Figure {
            positions: BTreeSet::from_iter(0..81),
        }
    }

    pub fn all_figures() -> Vec<Figure> {
        let mut res = vec![];

        for i in 0..9 {
            res.push(Figure::col(i));
            res.push(Figure::row(i));
            res.push(Figure::sqr(i));
        }

        res
    }
}

impl std::ops::Add for Figure {
    type Output = Self;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        self.positions.append(&mut rhs.positions);

        self
    }
}

impl IntoIterator for Figure {
    type Item = usize;

    type IntoIter = std::collections::btree_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.positions.into_iter()
    }
}

impl From<Vec<usize>> for Figure {
    fn from(value: Vec<usize>) -> Self {
        Figure {
            positions: BTreeSet::from_iter(value.into_iter()),
        }
    }
}

impl Sub for Figure {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Figure {
            positions: self.positions.difference(&rhs.positions).cloned().collect(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FigureNumberOutOfBoundError;

impl std::fmt::Display for FigureNumberOutOfBoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Figure number is out of bounds (0.=9)")
    }
}

impl std::error::Error for FigureNumberOutOfBoundError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row() {
        let second_row = Figure::row(1);
        let ninth_row = Figure::row(8);

        assert_eq!(
            second_row,
            Figure {
                positions: [9, 10, 11, 12, 13, 14, 15, 16, 17].into()
            }
        );

        assert_eq!(
            ninth_row,
            Figure {
                positions: [72, 73, 74, 75, 76, 77, 78, 79, 80].into()
            }
        );
    }

    #[test]
    fn col() {
        let second_col = Figure::col(1);
        let ninth_col = Figure::col(8);

        assert_eq!(
            second_col,
            Figure {
                positions: [1, 10, 19, 28, 37, 46, 55, 64, 73].into()
            }
        );

        assert_eq!(
            ninth_col,
            Figure {
                positions: [8, 17, 26, 35, 44, 53, 62, 71, 80].into()
            }
        );
    }

    #[test]
    fn sqr() {
        let second_sqr = Figure::sqr(1);
        let ninth_sqr = Figure::sqr(8);

        assert_eq!(
            second_sqr,
            Figure {
                positions: [3, 4, 5, 12, 13, 14, 21, 22, 23].into()
            }
        );

        assert_eq!(
            ninth_sqr,
            Figure {
                positions: [60, 61, 62, 69, 70, 71, 78, 79, 80].into()
            }
        );
    }

    #[test]
    fn neighbours() {
        let n1 = Figure::neighbours(2);
        let n2 = Figure::neighbours(80);

        assert_eq!(n1, Figure::row(0) + Figure::col(2) + Figure::sqr(0));
        assert_eq!(n2, Figure::row(8) + Figure::col(8) + Figure::sqr(8));
    }

    #[test]
    fn checked_figures() {
        assert!(Figure::row_checked(9).is_err());
        assert!(Figure::col_checked(9).is_err());
        assert!(Figure::sqr_checked(9).is_err());
        assert!(Figure::neighbours_checked(81).is_err());
    }
}
