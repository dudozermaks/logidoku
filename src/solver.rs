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
    methods: Vec<(Box<dyn Method>, bool)>,
}

impl Solver {
    /// Returns solver with all methods in the sudokuwiki.org order
    pub fn all_methods() -> Self {
        Solver {
            methods: vec![
                // Original sudokuwiki.org order
                (Box::new(Naked::Single), true),
                //
                (Box::new(Hidden::Single), true),
                //
                (Box::new(Naked::Pair), true),
                (Box::new(Naked::Triple), true),
                //
                (Box::new(Hidden::Pair), true),
                (Box::new(Hidden::Triple), true),
                //
                (Box::new(Naked::Quad), true),
                (Box::new(Hidden::Quad), true),
                //
                (Box::new(Pointing::Pair), true),
                (Box::new(Pointing::Triple), true),
                //
                (Box::new(BoxLineReduction::Pair), true),
                (Box::new(BoxLineReduction::Triple), true),
                //
                (Box::new(Fishes::XWing), true),
                //
                (Box::new(SimpleColoring {}), true),
            ],
        }
    }

    /// Goes through all enabled methods (in order).
    /// If `stop_after_first` is true: returns Vec of helpful actions from every method.
    /// Else: returns helpful applications from the first applicable method.
    pub fn take_step(&self, grid: &Grid, stop_after_first: bool) -> Vec<Action> {
        let mut applications = vec![];
        for (method, enabled) in &self.methods {
            if !enabled {
                continue;
            }

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

    /// Returns methods and bool, indicating whether given method is enabled.
    pub fn methods(&self) -> &Vec<(Box<dyn Method>, bool)> {
        &self.methods
    }

    /// Enables or disables method at given position.
    pub fn set_state(&mut self, index: usize, state: bool) {
        self.methods[index].1 = state;
    }

    /// Toggles the method at given position.
    pub fn toggle(&mut self, index: usize) {
        let current_state = self.methods[index].1;

        self.set_state(index, !current_state);
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
