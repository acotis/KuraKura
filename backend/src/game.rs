
use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt::Error;
use std::ops::Not;

use crate::game::Color::*;
use crate::game::TurnPhase::*;
use crate::game::TurnError::*;
use crate::game::GameOutcome::*;
use crate::game::SpinDirection::*;

// Constants.

const WIN_LEN: usize = 5;

// Elementary types.

#[derive(Clone, Copy, PartialEq, Eq, Debug)] pub enum Color {Black, White, Empty}
#[derive(Clone, Copy, PartialEq, Eq, Debug)] pub enum SpinDirection {Right, Left}
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

    pub fn play(&mut self, player: Color, r: usize, c: usize) -> TurnResult {
        if self.outcome != None         {return Err(GameAlreadyOver);}
        if self.whose_turn != player    {return Err(NotYourTurn);}
        if self.turn_phase != Play      {return Err(PlayDuringSpinPhase);}
        if r >= self.size               {return Err(InvalidLocation);}
        if c >= self.size               {return Err(InvalidLocation);}
        if self.board[r][c] != Empty    {return Err(PieceAlreadyThere);}

        self.board[r][c] = player;
        self.turn_phase = Spin;

        // TODO: If we end up treating the play phase as a distinct action, then
        // update the game's outcome here.

        Ok(self.outcome)
    }

    pub fn spin(&mut self, player: Color, r: usize, c: usize, size: usize, dir: SpinDirection) -> TurnResult {
        if self.outcome != None         {return Err(GameAlreadyOver);}
        if self.whose_turn != player    {return Err(NotYourTurn);}
        if self.turn_phase != Spin      {return Err(SpinDuringPlayPhase);}
        if r + size - 1 >= self.size    {return Err(InvalidLocation);}
        if c + size - 1 >= self.size    {return Err(InvalidLocation);}

        let mut slice = vec![vec![Empty; size]; size];

        for r_offset in 0..size {
            for c_offset in 0..size {
                slice[r_offset][c_offset] = self.board[r+r_offset][c+c_offset];
            }
        }

        for r_offset in 0..size {
            for c_offset in 0..size {
                let slice_r = if dir == Right {size-c_offset-1} else {c_offset};
                let slice_c = if dir == Left  {size-r_offset-1} else {r_offset};
                self.board[r+r_offset][c+c_offset] = slice[slice_r][slice_c];
            }
        }

        self.whose_turn = !self.whose_turn;
        self.turn_phase = Play;

        // TODO: Update the game's outcome.

        Ok(self.outcome)
    }

    pub fn update_outcome(&mut self) {
        let mut winning_tiles: Vec<(usize, usize)> = vec![];

        for r in 0..self.size {
            for c in 0..self.size {
                for dir in vec![(0, 1), (1, 0), (1, 1)] {
                    let mut line = (0..WIN_LEN).map(|x| (r + dir.0 * x, c + dir.1 * x));

                    for color in vec![Black, White] {
                        if line.all(|(r, c)| 
                                    r < self.size &&
                                    c < self.size && 
                                    self.board[r][c] == color) {
                            winning_tiles.extend(line);
                            break;
                        }
                    }
                }
            }
        }

        // Check for wins and double wins.

        if winning_tiles.len() > 0 {
            if winning_tiles.iter().all(|&(r, c)| self.board[r][c] == Black) {self.outcome = Some(BlackWin); return;}
            if winning_tiles.iter().all(|&(r, c)| self.board[r][c] == White) {self.outcome = Some(WhiteWin); return;}
            self.outcome = Some(DoubleWin);
            return;
        }

        // Check for stalemate.

        if (0..self.size).all(|r| (0..self.size).all(|c| self.board[r][c] != Empty)) {
            self.outcome = Some(Stalemate);
        }
    }
}

impl Display for Twirl {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for r in 0..self.size {
            for c in 0..self.size {
                match self.board[r][c] {
                    Black => {write!(f, "x")?;},
                    White => {write!(f, "o")?;},
                    Empty => {write!(f, ".")?;},
                }

                if c < self.size - 1 {write!(f, " ")?;}
            }
            write!(f, "\n")?;
        }

        write!(f, "\n")?;

        match self.outcome {
            Some(BlackWin)  => {write!(f, "Black wins!")?;},
            Some(WhiteWin)  => {write!(f, "White wins!")?;},
            Some(Stalemate) => {write!(f, "Stalemate.")?;},
            Some(DoubleWin) => {write!(f, "Double win!")?;},
            None => {
                match (self.whose_turn, self.turn_phase) {
                    (Black, Play) => {write!(f, "Black's turn to play...")?;},
                    (Black, Spin) => {write!(f, "Black's turn to spin...")?;},
                    (White, Play) => {write!(f, "White's turn to play...")?;},
                    (White, Spin) => {write!(f, "White's turn to spin...")?;},
                    (Empty, _) => {panic!();},
                };
            }
        }

        Ok(())
    }
}

