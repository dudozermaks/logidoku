pub mod hidden_single;
pub mod naked_n;

use crate::{action::Action, grid::Grid};

/// Methods follow the definition from https://sudokuwiki.org, if it is avalible
/// for the given method.
pub trait MethodCreator {
    fn get_all_applications(&self, grid: &Grid) -> Vec<Action>;
}
