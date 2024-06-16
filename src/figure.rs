use std::collections::BTreeSet;

/// Figure can be row, column, square, or some set of positions.
#[derive(Debug, PartialEq)]
pub struct Figure {
    // Choose BTreeSet here because iterating over its elements is deterministic
    positions: BTreeSet<usize>,
}

#[derive(Debug, PartialEq, Eq)]
struct FigureNumberOutOfBoundError;

impl std::fmt::Display for FigureNumberOutOfBoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Figure number is out of bounds (0.=9)")
    }
}

impl std::error::Error for FigureNumberOutOfBoundError {}

impl Figure {
    fn row_checked(n: u8) -> Result<Figure, FigureNumberOutOfBoundError> {
        if n > 8 {
            return Err(FigureNumberOutOfBoundError);
        }

        Ok(Figure {
            positions: BTreeSet::from_iter((0..9_usize).map(|i| i + n as usize * 9)),
        })
    }
    fn col_checked(n: u8) -> Result<Figure, FigureNumberOutOfBoundError> {
        if n > 8 {
            return Err(FigureNumberOutOfBoundError);
        }

        Ok(Figure {
            positions: BTreeSet::from_iter((0..9_usize).map(|i| i * 9 + n as usize)),
        })
    }
    fn sqr_checked(n: u8) -> Result<Figure, FigureNumberOutOfBoundError> {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row() {
        let second_row = Figure::row_checked(1).unwrap();
        let ninth_row = Figure::row_checked(8).unwrap();

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
        let second_col = Figure::col_checked(1).unwrap();
        let ninth_col = Figure::col_checked(8).unwrap();

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
        let second_sqr = Figure::sqr_checked(1).unwrap();
        let ninth_sqr = Figure::sqr_checked(8).unwrap();

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
    fn checked_figures() {
        assert!(Figure::row_checked(9).is_err());
        assert!(Figure::col_checked(9).is_err());
        assert!(Figure::sqr_checked(9).is_err());
    }
}
