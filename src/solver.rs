use std::any::Any;

use crate::{
    action::Action,
    grid::Grid,
    methods::{
        box_line_reduction::BoxLineReduction, fishes::Fishes, hidden_n::Hidden, naked_n::Naked,
        pointing_ns::Pointing, simple_coloring::SimpleColoring, Method,
    },
};

#[derive(Clone)]
pub struct Solver {
    methods: Vec<Box<dyn Method>>,
}

impl Solver {
    /// Returns solver with all methods in the sudokuwiki.org order
    pub fn all_methods() -> Self {
        Solver {
            methods: vec![
                // Original sudokuwiki.org order
                Box::new(Naked::Single),
                //
                Box::new(Hidden::Single),
                //
                Box::new(Naked::Pair),
                Box::new(Naked::Triple),
                //
                Box::new(Hidden::Pair),
                Box::new(Hidden::Triple),
                //
                Box::new(Naked::Quad),
                Box::new(Hidden::Quad),
                //
                Box::new(Pointing::Pair),
                Box::new(Pointing::Triple),
                //
                Box::new(BoxLineReduction::Pair),
                Box::new(BoxLineReduction::Triple),
                //
                Box::new(Fishes::XWing),
                //
                Box::new(SimpleColoring {}),
            ],
        }
    }
    pub fn take_step(&self, grid: &Grid, stop_after_first: bool) -> Vec<Action> {
        let mut applications = vec![];
        for method in &self.methods {
            let method_applications = method.get_all_helpful_applications(grid);

            if !method_applications.is_empty() {
                applications.extend(method_applications);

                if stop_after_first {
                    break;
                }
            }
        }

        applications
    }

    pub fn methods(&self) -> &Vec<Box<dyn Method>> {
        &self.methods
    }
}

impl PartialEq for Solver {
    fn eq(&self, other: &Self) -> bool {
        self.methods
            .iter()
            .zip(other.methods.iter())
            .all(|(a, b)| a.type_id() == b.type_id())
    }
}
impl Eq for Solver {}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn take_step() {
        let solver = Solver::all_methods();

        {
            let easiest_grid = Grid::from_str(
                "300967001040302080020000070070000090000873000500010003004705100905000207800621004",
            )
            .unwrap();

            let mut actions = solver.take_step(&easiest_grid, true);
            let mut predictions = vec![
                Action::PlaceNumber(2, 8),
                Action::PlaceNumber(13, 5),
                Action::PlaceNumber(66, 4),
                Action::PlaceNumber(73, 3),
            ];

            actions.sort();
            predictions.sort();

            assert_eq!(actions, predictions);
        }
        {
            let multiple_actions_grid = Grid::from_str(
                "000000700007109000680070010001090600000300020040000003008060100500000040000002005",
            )
            .unwrap();

            let mut actions = solver.take_step(&multiple_actions_grid, false);
            let mut predictions = vec![
                Action::PlaceNumber(44, 1),
                Action::RemovePencilmarks(vec![0, 1, 2, 4, 6, 7, 8].into(), vec![6]),
                Action::RemovePencilmarks(
                    vec![0, 1, 2, 4, 6, 7, 8, 12, 13, 14, 21, 22, 23].into(),
                    vec![6],
                ),
                Action::RemovePencilmarks(
                    vec![6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 24, 25, 26].into(),
                    vec![6],
                ),
                Action::RemovePencilmarks(vec![6, 7, 8, 15, 24, 25, 26].into(), vec![6]),
                Action::RemovePencilmarks(
                    vec![30, 31, 32, 39, 40, 41, 45, 46, 47, 48, 51, 52, 53].into(),
                    vec![1],
                ),
                Action::RemovePencilmarks(vec![30, 31, 32, 39, 40, 41, 48].into(), vec![1]),
            ];

            actions.sort();
            predictions.sort();

            assert_eq!(actions, predictions);
        }
        {
            let mut hard_grid = Grid::from_str(
                "000000700007109000680070010001090600000300021040000003008060100500000040000002005",
            )
            .unwrap();

            hard_grid.set_pencilmarks(7, vec![3, 5, 8, 9]);
            hard_grid.set_pencilmarks(8, vec![2, 4, 8, 9]);

            let actions = solver.take_step(&hard_grid, true);

            assert_eq!(actions, vec![]);
        }
    }
}
