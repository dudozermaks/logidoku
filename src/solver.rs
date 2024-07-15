use std::{any::Any, collections::BTreeSet};

use crate::{
    action::Action,
    grid::Grid,
    methods::{
        box_line_reduction::BoxLineReduction, fishes::Fishes, hidden_n::Hidden, naked_n::Naked,
        pointing_ns::Pointing, simple_coloring::SimpleColoring, Method,
    },
};

#[derive(Clone, Debug)]
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
    /// Else: returns helpful and simplified applications from the first applicable method.
    pub fn take_step(&self, grid: &Grid, stop_after_first: bool) -> BTreeSet<Action> {
        let mut applications = BTreeSet::new();
        for (method, enabled) in &self.methods {
            if !enabled {
                continue;
            }

            let method_applications = method.get_all_helpful_applications(grid, true);

            if !method_applications.is_empty() {
                applications.extend(method_applications);

                if stop_after_first {
                    break;
                }
            }
        }

        applications
    }

    /// Goes through all enabled methods (in order).
    /// Applies the first applicable methods for the given grid.
    /// If method returns multiple actions: applies them in method's order.
    /// Returns [`Action`]s taken and grid (which might be unsloved).
    pub fn try_solve(&self, grid: &mut Grid) -> BTreeSet<Action> {
        let mut steps_taken = BTreeSet::new();

        while !grid.is_solved() {
            let actions = self.take_step(&grid, true);

            if actions.is_empty() {
                break;
            }

            for action in &actions {
                action.apply_to_grid(grid);
            }

            steps_taken.extend(actions);
        }

        steps_taken
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

            let actions = solver.take_step(&easiest_grid, true);
            let predictions = BTreeSet::from([
                Action::PlaceNumber {
                    position: 2,
                    number: 8,
                },
                Action::PlaceNumber {
                    position: 13,
                    number: 5,
                },
                Action::PlaceNumber {
                    position: 66,
                    number: 4,
                },
                Action::PlaceNumber {
                    position: 73,
                    number: 3,
                },
            ]);

            assert_eq!(actions, predictions);
        }
        {
            let multiple_actions_grid = Grid::from_str(
                "000000700007109000680070010001090600000300020040000003008060100500000040000002005",
            )
            .unwrap();

            let actions = solver.take_step(&multiple_actions_grid, false);
            let predictions = BTreeSet::from([
                Action::PlaceNumber {
                    position: 44,
                    number: 1,
                },
                Action::RemovePencilmarks {
                    figure: vec![7, 8].into(),
                    pencilmarks: vec![6],
                },
                Action::RemovePencilmarks {
                    figure: vec![40, 41].into(),
                    pencilmarks: vec![1],
                },
            ]);

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

            assert_eq!(actions, BTreeSet::new());
        }
    }

    #[test]
    fn try_solve() {
        let mut grid = Grid::from_str(
            "000004028406000005100030600000301000087000140000709000002010003900000507670400000",
        )
        .unwrap();
        let _actions = Solver::all_methods().try_solve(&mut grid);

        assert_eq!(
            grid,
            Grid::from_str(
                "735164928426978315198532674249381756387256149561749832852617493914823567673495281"
            )
            .unwrap()
        );
    }
}
