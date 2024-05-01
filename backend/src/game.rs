
use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt::Error;
use std::ops::Not;

use crate::game::Color::*;
use crate::game::TurnPhase::*;
use crate::game::TurnError::*;

// Elementary types.

#[derive(Clone, Copy, PartialEq, Eq, Debug)] pub enum Color {Black, White, Empty}
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
    StoneAlreadyThere,
}

pub type TurnResult = Result<Option<GameOutcome>, TurnError>;

impl Not for Color {
    type Output = Color;
    fn not(self) -> Self {
        match self {
            Black => White,
            White => Black,
            Empty => panic!(),
        }
    }
}

// Game type.

pub struct Twirl {
    size:       usize,
    board:      Vec<Vec<Color>>,
    outcome:    Option<GameOutcome>, // None until the game is over
    whose_turn: Color,
    turn_phase: TurnPhase,
}

impl Twirl {
    pub fn new(size: usize) -> Self {
        Twirl {
            size:       size,
            board:      vec![vec![Empty; size]; size],
            outcome:    None,
            whose_turn: Black,
            turn_phase: Play,
        }
    }

    pub fn play(&mut self, player: Color, x: usize, y: usize) -> TurnResult {
        if self.outcome != None         {return Err(GameAlreadyOver);}
        if self.whose_turn != player    {return Err(NotYourTurn);}
        if self.turn_phase != Play      {return Err(PlayDuringSpinPhase);}
        if x >= self.size               {return Err(InvalidLocation);}
        if y >= self.size               {return Err(InvalidLocation);}
        if self.board[y][x] != Empty    {return Err(StoneAlreadyThere);}

        self.board[y][x] = player;
        self.turn_phase = Spin;

        // TODO: If we end up treating the play phase as a distinct action, then
        // update the game's outcome here.

        Ok(self.outcome)
    }

    pub fn spin(&mut self, player: Color, x: usize, y: usize, size: usize) -> TurnResult {
        if self.outcome != None         {return Err(GameAlreadyOver);}
        if self.whose_turn != player    {return Err(NotYourTurn);}
        if self.turn_phase != Spin      {return Err(SpinDuringPlayPhase);}
        if x + size >= self.size        {return Err(InvalidLocation);}
        if y + size >= self.size        {return Err(InvalidLocation);}

        // TODO: Perform the spin.

        self.whose_turn = !self.whose_turn;
        self.turn_phase = Play;

        // TODO: Update the game's outcome.

        Ok(self.outcome)
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

        match (self.whose_turn, self.turn_phase) {
            (Black, Play) => {write!(f, "Black's turn to play")?;},
            (Black, Spin) => {write!(f, "Black's turn to spin")?;},
            (White, Play) => {write!(f, "White's turn to play")?;},
            (White, Spin) => {write!(f, "White's turn to spin")?;},
            (Empty, _) => {panic!();},
        };
        
        Ok(())
    }
}

