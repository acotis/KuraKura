
use serde::{Serialize, Deserialize};

use crate::game::types::Orientation;
use crate::game::types::Player::{self, *};

/// A stone on the Kura Kura board.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Stone {
    /// The number of this stone: 1 is black's first stone, 2 is white's first stone, and so on.
    pub number: usize,
    /// The orientation of this stone (the numbers spin along with the stone, for fun.)
    pub orientation: Orientation,
    /// Is this stone part of a win?
    pub winning: bool,
}

impl Stone {
    /// Create a non-winning, upright stone with the given number.
    pub fn place(number: usize) -> Self {
        Self { number, orientation: Orientation::Up, winning: false }
    }

    /// Spin the stone clockwise.
    pub fn spun(self) -> Self {
        Self { orientation: self.orientation.spun(), ..self }
    }

    /// Whose stone is this?
    pub fn who(&self) -> Player {
        if self.number % 2 == 1 {Black} else {White}
    }
}

/// A cell in the grid, which possibly contains a stone.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Serialize, Deserialize)] pub struct Cell {
    pub stone: Option<Stone>,
}

impl Cell {
    fn spun(&self) -> Self {
        Cell { stone: self.stone.map(|s| s.spun()) }
    }

    pub fn who(&self) -> Option<Player> {
        self.stone.map(|s| s.who())
    }
}

pub fn spin_cell_grid(grid: Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    let size = grid.len();
    let mut ret = Vec::with_capacity(size);

    for r in 0..size {
        ret.push(Vec::with_capacity(size));

        for c in 0..size {
            ret[r].push(grid[size-c-1][r].spun());
        }
    }

    ret
}

