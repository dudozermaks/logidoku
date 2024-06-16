#[derive(Debug, PartialEq, Clone)]
pub enum Cell {
    Number(u8),
    Pencilmarks(Vec<u8>),
}

impl Cell {
    pub fn all_pencilmarks() -> Cell {
        Cell::Pencilmarks(Vec::from([1, 2, 3, 4, 5, 6, 7, 8, 9]))
    }
}
