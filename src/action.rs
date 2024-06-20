use crate::figure::Figure;

#[derive(Ord, Eq, PartialEq, PartialOrd, Debug)]
pub enum Action {
    /// 1. Position
    /// 2. Number
    PlaceNumber(usize, u8),
    /// 1. Figure to remove from
    /// 2. Pencilmarks to remove
    RemovePencilmarks(Figure, Vec<u8>),
}

impl Action {
    pub fn apply_to_grid(&self, grid: &mut crate::grid::Grid) {
        match self {
            Action::PlaceNumber(position, number) => {
                grid.set_number(*position, *number);
            }
            Action::RemovePencilmarks(figure, pencilmarks_to_remove) => {
                grid.remove_pencilmarks(figure, pencilmarks_to_remove)
            }
        }
    }
}
