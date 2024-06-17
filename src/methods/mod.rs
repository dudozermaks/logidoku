pub mod naked_single;

use crate::grid::Grid;

/// Methods follow the definition from https://sudokuwiki.org, if it is avalible
/// for the given method.
pub trait Method {
    fn get_all_applications(grid: &Grid) -> Vec<Self> where Self: Sized;
    fn apply_to_grid(&self, grid: &mut Grid);
}
