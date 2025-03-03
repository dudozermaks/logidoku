use std::{
    collections::{BTreeSet, HashSet},
    fmt::Display,
};

use itertools::Itertools;

use crate::{action::Action, cell::Cell, figure::Figure, grid::Grid};

use super::Method;

#[derive(Clone, Ord, Eq, PartialEq, PartialOrd, Hash, Debug)]
struct PosInfo {
    row: u8,
    col: u8,
    sqr: u8,
    neighbours: Figure,
}

impl PosInfo {
    fn have_common_figures(&self, other: &Self) -> bool {
        self.row == other.row || self.col == other.col || self.sqr == other.sqr
    }
}

impl From<usize> for PosInfo {
    fn from(val: usize) -> Self {
        PosInfo {
            row: Figure::row_of(val),
            col: Figure::col_of(val),
            sqr: Figure::sqr_of(val),
            neighbours: Figure::neighbours(val),
        }
    }
}

#[allow(clippy::derived_hash_with_manual_eq)]
#[derive(Eq, PartialOrd, Ord, Clone, Hash, Debug)]
struct ChainLink {
    number: u8,
    pos: usize,
    is_colored: bool,
    info: Option<PosInfo>,
}

impl ChainLink {
    fn get_next(&self, grid: &Grid) -> Vec<Self> {
        let mut res = vec![];

        let col = Figure::col_of(self.pos);
        let row = Figure::row_of(self.pos);
        let sqr = Figure::sqr_of(self.pos);

        let figures = vec![Figure::col(col), Figure::row(row), Figure::sqr(sqr)];

        for figure in figures {
            let info = grid.pencilmarks_info(figure);

            if let Some(number_positions) = info.get(&self.number) {
                if number_positions.len() == 2 {
                    let new_link_position = number_positions
                        .iter()
                        .find(|&&pos| pos != self.pos)
                        .unwrap();

                    let new_link = ChainLink {
                        number: self.number,
                        pos: *new_link_position,
                        is_colored: !self.is_colored,
                        info: None,
                    };

                    res.push(new_link);
                }
            }
        }

        res
    }

    fn get_chain(&self, grid: &Grid) -> Vec<Self> {
        let mut res: HashSet<ChainLink> = HashSet::from_iter(self.get_next(grid));
        res.insert(self.clone());

        let mut stack = res.clone();

        while !stack.is_empty() {
            let mut new_stack = HashSet::new();

            for chain_link in stack {
                let next_links = chain_link
                    .get_next(grid)
                    .into_iter()
                    // before extending new stack, removing every duplicate, already inserted in res,
                    .filter(|link| !res.contains(link));

                new_stack.extend(next_links);
            }

            stack = new_stack;
            res.extend(stack.clone());
        }

        res.into_iter().collect()
    }

    fn generate_info(&mut self) {
        if self.info.is_none() {
            self.info = Some(self.pos.into());
        }
    }

    fn rule2(&self, other: &ChainLink, chain: &Vec<ChainLink>) -> Option<Action> {
        if self.is_colored != other.is_colored {
            return None;
        }

        let info1 = self.info.clone().unwrap();
        let info2 = other.info.clone().unwrap();

        if !info1.have_common_figures(&info2) {
            return None;
        }

        let mut positions_to_remove_from = vec![];

        for link in chain {
            if link.is_colored == self.is_colored {
                positions_to_remove_from.push(link.pos);
            }
        }

        Some(Action::RemovePencilmarks {
            figure: positions_to_remove_from.into(),
            pencilmarks: vec![self.number],
        })
    }

    fn rule4(&self, other: &ChainLink) -> Option<Action> {
        if self.is_colored == other.is_colored {
            return None;
        }

        let info1 = self.info.clone().unwrap();
        let info2 = other.info.clone().unwrap();

        let intersection = info1.neighbours.intersection(info2.neighbours);

        if intersection.is_empty() {
            return None;
        }

        Some(Action::RemovePencilmarks {
            // I can't think of an edge case, where you need to substract not only this 2
            // positions, but the full chain's positions. Maybe I'm wrong here.
            figure: intersection - vec![self.pos, other.pos].into(),
            pencilmarks: vec![self.number],
        })
    }
}

impl PartialEq for ChainLink {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number && self.pos == other.pos
    }
}

#[derive(Clone, Debug)]
pub struct SimpleColoring;

impl SimpleColoring {
    fn get_chains_for_number(&self, grid: &Grid, number: u8) -> Vec<Vec<ChainLink>> {
        let mut res = vec![];

        for pos in Figure::all_cells() {
            if let Cell::Pencilmarks(pencilmarks) = &grid[pos] {
                if pencilmarks.contains(&number)
                    // the pos was not in any chain before
                    && res.iter().all(|chain: &Vec<ChainLink>| {
                        chain.iter().all(|chain_link| chain_link.pos != pos)
                    })
                {
                    res.push(
                        ChainLink {
                            number,
                            pos,
                            is_colored: false,
                            info: None,
                        }
                        .get_chain(grid),
                    );
                }
            }
        }

        res
    }
}

impl Display for SimpleColoring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Simple Coloring")
    }
}

impl Method for SimpleColoring {
    fn get_all_applications(&self, grid: &Grid) -> BTreeSet<Action> {
        let mut res = BTreeSet::new();

        let mut chains: Vec<Vec<ChainLink>> = vec![];

        for i in 1..=9 {
            chains.extend(self.get_chains_for_number(grid, i));
        }
        for chain in &mut chains {
            for link in chain {
                link.generate_info();
            }
        }

        for chain in chains {
            for links in chain.iter().combinations(2) {
                let link1 = links[0];
                let link2 = links[1];

                let rule2 = link1.rule2(link2, &chain);
                let rule4 = link1.rule4(link2);

                if let Some(rule2) = rule2 {
                    res.insert(rule2);
                }

                if let Some(rule4) = rule4 {
                    res.insert(rule4);
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    fn test(grid: &str, predictions: BTreeSet<Action>) {
        let grid = Grid::from_str(grid).unwrap();

        let actions = SimpleColoring {}.get_all_helpful_applications(&grid, false);

        assert_eq!(actions, predictions);
    }

    #[test]
    fn singles_chains() {
        test(
            "200041056405602010016095004350129640142060590069504001584216379920408165601950482",
            BTreeSet::from([
                Action::RemovePencilmarks {
                    figure: vec![4, 13, 22, 30, 39, 48].into(),
                    pencilmarks: vec![3],
                },
                Action::RemovePencilmarks {
                    figure: vec![13, 53].into(),
                    pencilmarks: vec![3],
                },
                Action::RemovePencilmarks {
                    figure: vec![17, 40].into(),
                    pencilmarks: vec![8],
                },
                Action::RemovePencilmarks {
                    figure: vec![26, 39].into(),
                    pencilmarks: vec![3],
                },
            ]),
        );

        test(
            "400100000002000004008090100006403800080000010007906200003070000200000605000002001",
            BTreeSet::from([
                Action::RemovePencilmarks {
                    figure: vec![4, 13, 22, 30, 32, 39, 40, 41, 48, 50, 58, 67, 76].into(),
                    pencilmarks: vec![1],
                },
                Action::RemovePencilmarks {
                    figure: vec![4, 13, 22, 31, 40, 49, 57, 58, 59, 66, 68, 75, 77].into(),
                    pencilmarks: vec![4],
                },
                Action::RemovePencilmarks {
                    figure: vec![5, 14, 23, 32, 41, 50, 57, 58, 66, 67, 75, 76, 77].into(),
                    pencilmarks: vec![1],
                },
                Action::RemovePencilmarks {
                    figure: vec![13, 22, 31, 49, 58, 67, 76].into(),
                    pencilmarks: vec![2],
                },
                Action::RemovePencilmarks {
                    figure: vec![27, 28, 29, 30, 31, 32, 33, 42, 43, 44, 51, 52, 53].into(),
                    pencilmarks: vec![7],
                },
                Action::RemovePencilmarks {
                    figure: vec![30, 31, 32, 36, 37, 38, 40, 42, 43, 44, 48, 49, 50].into(),
                    pencilmarks: vec![7],
                },
                Action::RemovePencilmarks {
                    figure: vec![30, 31, 32, 36, 37, 38, 41, 42, 43, 44, 48, 49, 50].into(),
                    pencilmarks: vec![2],
                },
            ]),
        );

        let mut grid = Grid::from_str(
            "200041056405602010016095004350129640142060590069504001584216379920408165601950482",
        )
        .unwrap();

        grid.set_pencilmarks(2, vec![3, 8]);
        grid.set_pencilmarks(3, vec![3, 8]);
        grid.set_pencilmarks(49, vec![3, 8]);
        grid.set_pencilmarks(44, vec![3, 8]);

        let actions = SimpleColoring {}.get_all_helpful_applications(&grid, false);

        let predictions = BTreeSet::from([
            Action::RemovePencilmarks {
                figure: vec![26, 39].into(),
                pencilmarks: vec![3],
            },
            Action::RemovePencilmarks {
                figure: vec![13, 53].into(),
                pencilmarks: vec![3],
            },
            Action::RemovePencilmarks {
                figure: vec![4, 13, 22, 30, 39, 48].into(),
                pencilmarks: vec![3],
            },
            Action::RemovePencilmarks {
                figure: vec![10, 76].into(),
                pencilmarks: vec![7],
            },
            Action::RemovePencilmarks {
                figure: vec![15, 49].into(),
                pencilmarks: vec![7],
            },
            Action::RemovePencilmarks {
                figure: vec![18, 48].into(),
                pencilmarks: vec![7],
            },
            Action::RemovePencilmarks {
                figure: vec![21, 36].into(),
                pencilmarks: vec![7],
            },
            Action::RemovePencilmarks {
                figure: vec![9, 10, 11, 12, 14, 15, 16].into(),
                pencilmarks: vec![7],
            },
            Action::RemovePencilmarks {
                figure: vec![9, 10, 11, 21, 22, 23].into(),
                pencilmarks: vec![7],
            },
            // Rule 2
            Action::RemovePencilmarks {
                figure: vec![17, 18, 21, 29, 41, 51, 67, 73].into(),
                pencilmarks: vec![7],
            },
            Action::RemovePencilmarks {
                figure: vec![17, 40].into(),
                pencilmarks: vec![8],
            },
        ]);

        assert_eq!(actions, predictions);
    }
}
