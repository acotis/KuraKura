
use std::ops::Not;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;
use serde::{Serialize, Deserialize};

use crate::game::types::Player::*;
use crate::game::types::Orientation::*;

// Elementary types for the game Kura Kura.

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Orientation {Up, Right, Down, Left}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum SpinDirection {CW, CCW}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Player {Black, White}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum GameOutcome {
    BlackWin,
    WhiteWin,
    Stalemate,
    DoubleWin,

    // Todo: distinguish between a regular win and a "Kura Kura"
    // (when you win because your opponent spun your line into
    // existence).
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum TurnError {
    GameAlreadyOver,
    NotYourTurn,
    InvalidLocation,
    PieceAlreadyThere,
    YoureNotPlaying,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Turn {
    pub play_row:       usize,
    pub play_col:       usize,
    pub spin_ul_row:    usize,
    pub spin_ul_col:    usize,
    pub spin_size:      usize,
    pub spin_dir:       SpinDirection,
}

pub type TurnResult = Result<Option<GameOutcome>, TurnError>;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum PlayerRole {
    BlackPlayer,
    WhitePlayer,
    Spectator,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Parameters {
    pub host_plays_black: bool,
    pub grid_size: usize,
    pub win_length: usize,
}

// Implementations of elementary methods.

impl Not for Player {
    type Output = Player;
    fn not(self) -> Self {
        match self {
            Black => White,
            White => Black,
        }
    }
}

impl Orientation {
    pub fn spun(self) -> Self {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

impl Display for Orientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Up    => {write!(f, "↑")?;}
            Right => {write!(f, "→")?;}
            Down  => {write!(f, "↓")?;}
            Left  => {write!(f, "←")?;}
        };

        Ok(())
    }
}

