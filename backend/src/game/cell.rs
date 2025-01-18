
use crate::game::types::Orientation;
use crate::game::types::Player::{self, *};

// Cell type for game Kura Kura. Each cell contains:
//
//     1. A stone (optional).
//         - Has an ID number. Lowest ID is 1.
//         - Has an orientation: up, right, down, or left.
//         - Is part of a win, or not.
//

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)] pub struct Cell {
    pub stone: Option<(usize, Orientation, bool)>,
}

impl Cell {
    fn spun(&self) -> Self {
        Cell {
            stone: match self.stone {
                None => None,
                Some((num, or, win)) => Some((num, or.spun(), win)),
            },
        }
    }

    pub fn who(&self) -> Option<Player> {
        match self.stone {
            None => None,
            Some((num, _, _)) => if num % 2 == 1 {Some(Black)} else {Some(White)},
        }
    }
}

pub fn spin_cell_grid(grid: Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    let size = grid.len();
    let mut ret = vec![];

    for r in 0..size {
        ret.push(vec![]);

        for c in 0..size {
            ret[r].push(grid[size-c-1][r].spun());
        }
    }

    ret
}

