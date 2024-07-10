use std::{
    collections::{BTreeSet, HashSet},
    fmt::Display,
};

use crate::{action::Action, figure::Figure};

use super::Method;

#[derive(Clone, Debug)]
pub enum Hidden {
    Single,
    Pair,
    Triple,
    Quad,
}

impl Hidden {
    fn single_applications(&self, grid: &crate::grid::Grid) -> Vec<Action> {
        let mut res = BTreeSet::new();

        for f in Figure::all_figures() {
            grid.pencilmarks_info(f)
                .iter()
                .filter_map(|(pencilmark, positions)| {
                    if positions.len() == 1 {
                        Some(Action::PlaceNumber {
                            position: positions[0],
                            number: *pencilmark,
                        })
                    } else {
                        None
                    }
                })
                .for_each(|hs| {
                    res.insert(hs);
                });
        }

        res.into_iter().collect()
    }
    fn multiple_applications(&self, grid: &crate::grid::Grid, dimension: usize) -> Vec<Action> {
        // BTreeSet: Candidates can repeat multiple times across the field
        let mut res = BTreeSet::new();

        for f in Figure::all_figures() {
            let mut candidates = vec![];
            let mut lead_positions = HashSet::new();

            for (pencilmark, positions) in grid.pencilmarks_info(f.clone()) {
                if (2..=dimension).contains(&positions.len()) {
                    candidates.push((pencilmark, positions.clone()));
                }
                if positions.len() == dimension {
                    lead_positions.insert(positions);
                }
            }

            for lead_position in lead_positions {
                let mut pencilmarks = vec![];

                for candidate in candidates.clone() {
                    if candidate.1.iter().all(|&x| lead_position.contains(&x)) {
                        pencilmarks.push(candidate.0);
                    }
                }

                pencilmarks.sort();

                if pencilmarks.len() == dimension {
                    res.insert(Action::PreservePencilmarks {
                        figure: lead_position.to_vec().into(),
                        pencilmarks,
                    });
                }
            }
        }

        res.into_iter().collect()
    }
}

impl Display for Hidden {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Hidden {}",
            match self {
                Hidden::Single => "Single",
                Hidden::Pair => "Pair",
                Hidden::Triple => "Triple",
                Hidden::Quad => "Quad",
            }
        )
    }
}

impl Method for Hidden {
    fn get_all_applications(&self, grid: &crate::grid::Grid) -> Vec<Action> {
        let dimension = match self {
            Hidden::Single => 1,
            Hidden::Pair => 2,
            Hidden::Triple => 3,
            Hidden::Quad => 4,
        };

        if dimension == 1 {
            self.single_applications(grid)
        } else {
            self.multiple_applications(grid, dimension)
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        action::Action,
        methods::{hidden_n::Hidden, test_method},
    };

    #[test]
    fn get_and_apply_single() {
        test_method(
            "000004028406000005100030600000301000087000140000709000002010003900000507670400000",
            Hidden::Single,
            vec![
                Action::PlaceNumber {
                    position: 0,
                    number: 7,
                },
                Action::PlaceNumber {
                    position: 3,
                    number: 1,
                },
                Action::PlaceNumber {
                    position: 16,
                    number: 1,
                },
                Action::PlaceNumber {
                    position: 20,
                    number: 8,
                },
                Action::PlaceNumber {
                    position: 26,
                    number: 4,
                },
                Action::PlaceNumber {
                    position: 36,
                    number: 3,
                },
                Action::PlaceNumber {
                    position: 44,
                    number: 9,
                },
                Action::PlaceNumber {
                    position: 54,
                    number: 8,
                },
                Action::PlaceNumber {
                    position: 59,
                    number: 7,
                },
                Action::PlaceNumber {
                    position: 60,
                    number: 4,
                },
                Action::PlaceNumber {
                    position: 80,
                    number: 1,
                },
            ],
        );
    }

    #[test]
    fn get_and_apply_pairs() {
        test_method(
            "720408030080000047401076802810739000000851000000264080209680413340000008168943275",
            Hidden::Pair,
            vec![
                Action::PreservePencilmarks {
                    figure: vec![29, 38].into(),
                    pencilmarks: vec![2, 4],
                },
                Action::PreservePencilmarks {
                    figure: vec![42, 51].into(),
                    pencilmarks: vec![3, 7],
                },
                Action::PreservePencilmarks {
                    figure: vec![55, 59].into(),
                    pencilmarks: vec![5, 7],
                },
                Action::PreservePencilmarks {
                    figure: vec![55, 65].into(),
                    pencilmarks: vec![5, 7],
                },
                Action::PreservePencilmarks {
                    figure: vec![69, 70].into(),
                    pencilmarks: vec![6, 9],
                },
            ],
        );
    }

    #[test]
    fn get_and_apply_triples() {
        test_method(
            "000001030231090000065003100678924300103050006000136700009360570006019843300000000",
            Hidden::Triple,
            vec![
                Action::PreservePencilmarks {
                    figure: vec![2, 47, 74].into(),
                    pencilmarks: vec![2, 4, 7],
                },
                Action::PreservePencilmarks {
                    figure: vec![3, 6, 8].into(),
                    pencilmarks: vec![2, 5, 6],
                },
                Action::PreservePencilmarks {
                    figure: vec![4, 22, 76].into(),
                    pencilmarks: vec![4, 7, 8],
                },
                Action::PreservePencilmarks {
                    figure: vec![37, 42, 43].into(),
                    pencilmarks: vec![2, 4, 9],
                },
                Action::PreservePencilmarks {
                    figure: vec![63, 64, 66].into(),
                    pencilmarks: vec![2, 5, 7],
                },
            ],
        );
    }
    #[test]
    fn get_and_apply_quads() {
        test_method(
            "901500046425090081860010020502000000019000460600000002196040253200060817000001694",
            Hidden::Quad,
            vec![
                Action::PreservePencilmarks {
                    figure: vec![1, 4, 5, 6].into(),
                    pencilmarks: vec![2, 3, 7, 8],
                },
                Action::PreservePencilmarks {
                    figure: vec![20, 47, 65, 74].into(),
                    pencilmarks: vec![3, 4, 7, 8],
                },
                Action::PreservePencilmarks {
                    figure: vec![28, 36, 46, 47].into(),
                    pencilmarks: vec![3, 4, 7, 8],
                },
                Action::PreservePencilmarks {
                    figure: vec![30, 32, 48, 50].into(),
                    pencilmarks: vec![1, 4, 6, 9],
                },
                Action::PreservePencilmarks {
                    figure: vec![64, 65, 66, 68].into(),
                    pencilmarks: vec![3, 4, 5, 9],
                },
                Action::PreservePencilmarks {
                    figure: vec![66, 68, 75, 76].into(),
                    pencilmarks: vec![2, 3, 5, 9],
                },
            ],
        )
    }
}
