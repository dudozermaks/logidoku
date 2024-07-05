pub mod box_line_reduction;
pub mod hidden_n;
pub mod naked_n;
pub mod pointing_ns;
pub mod fishes;
pub mod simple_coloring;

use std::{str::FromStr, fmt::Display};

use crate::{action::Action, grid::Grid};

/// Methods follow the definition from https://sudokuwiki.org, if it is avalible
/// for the given method.
pub trait Method: Display {
    fn get_all_applications(&self, grid: &Grid) -> Vec<Action>;
    fn get_all_helpful_applications(&self, grid: &Grid) -> Vec<Action> {
        self.get_all_applications(grid)
            .iter()
            .filter(|method| method.is_helpful(grid))
            .cloned()
            .collect()
    }
}

// This is used for internal testing of the methods.
// Rust does not mark functions used in tests as used,
// so disable the warning
#[allow(dead_code)]
fn test_method<T: Method>(grid: &str, creator: T, mut valid_candidates: Vec<Action>) {
    let grid = Grid::from_str(grid).unwrap();

    let mut candidates = creator.get_all_applications(&grid);

    candidates.sort();
    valid_candidates.sort();

    assert_eq!(candidates, valid_candidates);
}
