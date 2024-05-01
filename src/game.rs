
use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt::Error;

use crate::game::Color::*;

struct Tile {
    x: usize,
    y: usize,
}

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

    pub fn play(self, player: Color, play: Tile, ul: Tile, br: Tile) -> PlayResult {
        // Game cannot already be over.
        // Must be your turn.
        // Play parameters must be valid:
        //     Play tile is on the board.
        //     UL tile is on the board.
        //     BR tile is on the board.
        //     BR tile is lower and righter than UL tile (or on top of it).
        //     UL and BR together cut a square region of the board (not just rectangular).
        // Current status of play tile must be empty.

        if self.outcome != None             {return Fail;}
        if player != self.next_turn         {return Fail;}
        if !validate_tile(play)             {return Fail;}
        if !validate_tile(ul)               {return Fail;}
        if !validate_tile(br)               {return Fail;}
        if br.x < ul.x                      {return Fail;}
        if br.y < ul.y                      {return Fail;}
        if bx.x - ul.x != br.y - ul.y       {return Fail;}
        if board[play.y][play.x] != Empty   {return Fail;}
    }

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

