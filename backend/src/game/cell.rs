
use crate::types::Orientation;
use crate::types::Player::{self, *};

// Cell type for game Kura Kura. Each cell has two properties:
//
//     1. The background.
//         - May have a line pointing up.
//         - May have a line pointing right.
//         - May have a line pointing down.
//         - May have a line pointing left.
//     2. The stone (optional).
//         - Has an ID number. Lowest ID is 1.
//         - Has an orientation: up, right, down, or left.
//         - Is part of a win, or not.
//

#[derive(Clone, Copy, PartialEq, Eq, Debug)] pub struct Cell {
    pub stone:      Option<(usize, Orientation, bool)>,
    pub line_up:    bool,
    pub line_right: bool,
    pub line_down:  bool,
    pub line_left:  bool,
}

impl Cell {
    fn spun(&self) -> Self {
        Cell {
            stone: match self.stone {
                None => None,
                Some((num, or, win)) => Some((num, or.spun(), win)),
            },
            line_right: self.line_up,
            line_down:  self.line_right,
            line_left:  self.line_down,
            line_up:    self.line_left,
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

