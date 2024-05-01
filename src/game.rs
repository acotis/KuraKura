
use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt::Error;

use crate::game::Color::*;

#[derive(Clone, Copy)]
enum Color {
    Black,
    White,
    Empty,
}

enum GameOutcome {
    BlackWin,
    WhiteWin,
    Draw,
}

enum PlayResult {
    Fail,
    Succeed(Option<GameOutcome>),
}


pub struct Twirl {
    board:      Vec<Vec<Color>>,
    next_turn:  Color,
    outcome:    Option<GameOutcome>, // None until the game is over
}

impl Twirl {
    pub fn new(size: usize) -> Self {
        Twirl {
            board:      vec![vec![Empty; size]; size],
            next_turn:  Black,
            outcome:    None,
        }
    }
}

impl Display for Twirl {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for row in &self.board {
            for space in row {
                match space {
                    Black => {write!(f, "x")?;},
                    White => {write!(f, "o")?;},
                    Empty => {write!(f, ".")?;},
                }
            }
            write!(f, "\n")?;
        }
        
        Ok(())
    }
}

