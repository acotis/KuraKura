
use std::ops::Not;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;

use crate::types::Player::*;
use crate::types::Orientation::*;

// Elementary types for the game Kura Kura. All types are simple enums except TurnResult.

#[derive(Clone, Copy, PartialEq, Eq, Debug)] pub enum Orientation {Up, Right, Down, Left}
#[derive(Clone, Copy, PartialEq, Eq, Debug)] pub enum SpinDirection {CW, CCW}
#[derive(Clone, Copy, PartialEq, Eq, Debug)] pub enum Player {Black, White}
#[derive(Clone, Copy, PartialEq, Eq, Debug)] pub enum TurnPhase {Play, Spin}
#[derive(Clone, Copy, PartialEq, Eq, Debug)] pub enum GameOutcome {
    BlackWin,
    WhiteWin,
    Stalemate,
    DoubleWin,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)] pub enum TurnError {
    GameAlreadyOver,
    NotYourTurn,
    PlayDuringSpinPhase,
    SpinDuringPlayPhase,
    InvalidLocation,
    PieceAlreadyThere,
}

pub type TurnResult = Result<Option<GameOutcome>, TurnError>;

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

