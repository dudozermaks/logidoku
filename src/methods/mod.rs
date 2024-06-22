pub mod box_line_reduction;
pub mod hidden_n;
pub mod naked_n;
pub mod pointing_ns;

use std::str::FromStr;

use crate::{action::Action, grid::Grid};

/// Methods follow the definition from https://sudokuwiki.org, if it is avalible
/// for the given method.
pub trait MethodCreator {
    fn get_all_applications(&self, grid: &Grid) -> Vec<Action>;
}

// This is used for internal testing of the methods.
// Rust does not mark functions used in tests as used,
// so disable the warning
#[allow(dead_code)]
fn test_method<T: MethodCreator>(grid: &str, creator: T, mut valid_candidates: Vec<Action>) {
    let grid = Grid::from_str(grid).unwrap();

    let mut candidates = creator.get_all_applications(&grid);

    candidates.sort();
    valid_candidates.sort();

    assert_eq!(candidates, valid_candidates);
}
