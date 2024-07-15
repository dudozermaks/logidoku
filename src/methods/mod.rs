pub mod box_line_reduction;
pub mod fishes;
pub mod hidden_n;
pub mod naked_n;
pub mod pointing_ns;
pub mod simple_coloring;

use dyn_clone::DynClone;
use std::{
    collections::BTreeSet,
    fmt::{Debug, Display},
    str::FromStr,
};

use crate::{action::Action, grid::Grid};

/// Methods follow the definition from https://sudokuwiki.org, if it is avalible
/// for the given method.
pub trait Method: Display + DynClone + Debug {
    fn get_all_applications(&self, grid: &Grid) -> BTreeSet<Action>;
    fn get_all_helpful_applications(&self, grid: &Grid, simplify: bool) -> BTreeSet<Action> {
        let mut applications = self.get_all_applications(grid);

        if simplify {
            applications = applications
                .into_iter()
                .map(|mut method| {
                    method.simplify(grid);
                    method
                })
                .collect();
        }

        applications
            .iter()
            .filter(|method| method.is_helpful(grid))
            .cloned()
            .collect()
    }
}

// Derive Clone for Box<dyn Method>
dyn_clone::clone_trait_object!(Method);

// This is used for internal testing of the methods.
// Rust does not mark functions used in tests as used,
// so disable the warning
#[allow(dead_code)]
fn test_method<T: Method>(grid: &str, creator: T, valid_candidates: BTreeSet<Action>) {
    let grid = Grid::from_str(grid).unwrap();

    let candidates = creator.get_all_applications(&grid);

    assert_eq!(candidates, valid_candidates);
}
