#[derive(Debug, PartialEq, Clone)]
pub enum Cell {
    Number(u8),
    Pencilmarks(Vec<u8>),
}

impl Cell {
    pub fn all_pencilmarks() -> Cell {
        Cell::Pencilmarks(Vec::from([1, 2, 3, 4, 5, 6, 7, 8, 9]))
    }

    pub fn is_number(&self) -> bool {
        match self {
            Cell::Number(_) => true,
            Cell::Pencilmarks(_) => false,
        }
    }

    pub fn is_pencilmarks(&self) -> bool {
        match self {
            Cell::Number(_) => false,
            Cell::Pencilmarks(_) => true,
        }
    }

    /// Panics if `self` is `Pencilmarks`
    pub fn number(&self) -> u8 {
        match self {
            Cell::Number(n) => *n,
            Cell::Pencilmarks(_) => {
                panic!("{}", "called `Cell::number()` on a `Pencilmarks` value")
            }
        }
    }

    /// Panics if `self` is `Number`
    pub fn pencilmarks(&self) -> Vec<u8> {
        match self {
            Cell::Number(_) => panic!("{}", "called `Cell::pencilmarks()` on a `Number` value"),
            Cell::Pencilmarks(p) => p.to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn number_unwrap() {
        Cell::Pencilmarks([1, 2, 3].to_vec()).number();
    }

    #[test]
    #[should_panic]
    fn pencilmarks_unwrap() {
        Cell::Number(1).pencilmarks();
    }

    #[test]
    fn is_pencilmarks_or_number() {
        let pencilmarks = Cell::Pencilmarks([1, 2, 3].to_vec());
        let number = Cell::Number(1);

        assert!(pencilmarks.is_pencilmarks());
        assert!(!pencilmarks.is_number());

        assert!(!number.is_pencilmarks());
        assert!(number.is_number());
    }
}
