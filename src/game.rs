
use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt::Error;

use crate::game::Color::*;

// Elementary types.

#[derive(Clone, Copy)] struct Tile {x: usize, y: usize}
#[derive(Clone, Copy)] enum Color {Black, White, Empty}
#[derive(Clone, Copy)] enum TurnPhase {Play, Spin}
#[derive(Clone, Copy)] enum GameOutcome {BlackWin, WhiteWin, Stalemate, DoubleWin}
#[derive(Clone, Copy)] enum Error {
    GameAlreadyOver,
    NotYourTurn,
    PlayDuringSpinPhase,
    SpinDuringPlayPhase,
    InvalidLocation,
    InvalidSpinTarget,
    StoneAlreadyThere,
}

type ActionResult = Result<Option<GameOutcome>, String>;

// Game type.

pub struct Twirl {
    board:      Vec<Vec<Color>>,
    outcome:    Option<GameOutcome>, // None until the game is over
    whose_turn: Color,
    turn_phase: TurnPhase,
}

impl Twirl {
    pub fn new(size: usize) -> Self {
        Twirl {
            board:      vec![vec![Empty; size]; size],
            outcome:    None,
            whose_turn: Black,
            turn_phase: Play,
        }
    }

    pub fn play(self, player: Color, play: Tile) -> ActionResult {
        // Game cannot already be over.
        // Must be your turn.
        // Must be in the "play" phase of your turn.
        // Play parameters must be valid:
        //     Play tile is on the board.
        // Current status of play tile must be empty.

        if self.outcome != None             {return Err(GameAlreadyOver);}
        if self.whose_turn != player        {return Err(NotYourTurn);}
        if self.turn_phase != Play          {return Err(PlayDuringSpinPhase);}
        if !validate_tile(play)             {return Err(InvalidLocation);}
        if board[play.y][play.x] != Empty   {return Err(StoneAlreadyThere);}



        if !validate_tile(ul)               {return Fail;}
        if !validate_tile(br)               {return Fail;}
        if br.x < ul.x                      {return Fail;}
        if br.y < ul.y                      {return Fail;}
        if bx.x - ul.x != br.y - ul.y       {return Fail;}
    }

    pub fn spin(self, player: Color, ul: Tile, br: Tile) -> ActionResult
        // Game cannot already be over.
        // Must be your turn.
        // Must be in the "spin" phase of your turn.
        // Spin parameters must be valid:
        //     UL tile is on the board.
        //     BR tile is on the board.
        //     BR tile is lower and righter than UL tile (or on top of it).
        //     UL and BR together cut a square region of the board (not just rectangular).



    fn validate_tile(self, tile: Tile) -> bool {
        if tile.x >= self.board.len() {return false;}
        if tile.y >= self.board.len() {return false;}
        return true;
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

