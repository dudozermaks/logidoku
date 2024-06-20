pub mod naked_n;
pub mod hidden_single;

use crate::grid::Grid;

pub trait MethodCreator {
    type Method;
    fn get_all_applications(&self, grid: &Grid) -> Vec<Self::Method> where Self::Method: Method;
}

/// Methods follow the definition from https://sudokuwiki.org, if it is avalible
/// for the given method.
pub trait Method {
    fn apply_to_grid(&self, grid: &mut Grid);
}
