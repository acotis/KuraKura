
use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt::Error;

use crate::game::Color::*;
use crate::game::TurnPhase::*;
use crate::game::TurnError::*;

// Elementary types.

#[derive(Clone, Copy)] pub struct Tile {x: usize, y: usize}
#[derive(Clone, Copy, PartialEq, Eq)] pub enum Color {Black, White, Empty}
#[derive(Clone, Copy, PartialEq, Eq)] pub enum TurnPhase {Play, Spin}
#[derive(Clone, Copy, PartialEq, Eq)] pub enum GameOutcome {
    BlackWin,
    WhiteWin,
    Stalemate,
    DoubleWin,
}
#[derive(Clone, Copy, PartialEq, Eq)] pub enum TurnError {
    GameAlreadyOver,
    NotYourTurn,
    PlayDuringSpinPhase,
    SpinDuringPlayPhase,
    InvalidLocation,
    StoneAlreadyThere,
}

pub type TurnResult = Result<Option<GameOutcome>, TurnError>;

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

    pub fn play(mut self, player: Color, play: Tile) -> TurnResult {
        if self.outcome != None                 {return Err(GameAlreadyOver);}
        if self.whose_turn != player            {return Err(NotYourTurn);}
        if self.turn_phase != Play              {return Err(PlayDuringSpinPhase);}
        if play.x >= self.size                  {return Err(InvalidLocation);}
        if play.y >= self.size                  {return Err(InvalidLocation);}
        if self.board[play.y][play.x] != Empty  {return Err(StoneAlreadyThere);}

        self.board[play.y][play.x] = player;

        // TODO: If we end up treating the play phase as a distinct action, then
        // update the game's outcome here.

        Ok(self.outcome)
    }

    pub fn spin(mut self, player: Color, ul: Tile, size: usize) -> TurnResult {
        if self.outcome != None             {return Err(GameAlreadyOver);}
        if self.whose_turn != player        {return Err(NotYourTurn);}
        if self.turn_phase != Spin          {return Err(SpinDuringPlayPhase);}
        if ul.x + size >= self.size         {return Err(InvalidLocation);}
        if ul.y + size >= self.size         {return Err(InvalidLocation);}

        // TODO: Perform the spin.
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
        
        Ok(())
    }
}

