
use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt::Error;

use crate::types::Player::{self, *};
use crate::types::Orientation::*;
use crate::types::TurnPhase::{self, *};
use crate::types::TurnError::*;
use crate::types::GameOutcome::{self, *};
use crate::types::SpinDirection::*;
use crate::types::TurnResult;
use crate::cell::Cell;
use crate::cell::spin_cell_grid;
use crate::Turn;

// Game type.

pub struct Game {
    win_len:    usize,                  // Line length needed to win.
    board:      Vec<Vec<Cell>>,         // State of the board.
    turn:       usize,                  // Number of the active turn's stone.
    turn_phase: TurnPhase,              // Phase of the active turn.

    outcome:    Option<GameOutcome>,    // [Cache] Outcome (= None until the game is over).
}

impl Game {
    pub fn new(size: usize, win_len: usize) -> Self {
        let mut board = Game {
            win_len:    win_len,
            board:      vec![],
            turn:       1,
            turn_phase: Play,
            outcome:    None,
        };

        for r in 0..size {
            board.board.push(vec![]);

            for c in 0..size {
                board.board[r].push(
                    Cell {
                        stone:      None,
                        line_up:    false,
                        line_right: false,
                        line_down:  false,
                        line_left:  false,
                    }
                );

                if r > 0        {board.board[r][c].line_up    = true;}
                if c > 0        {board.board[r][c].line_left  = true;}
                if r < size - 1 {board.board[r][c].line_down  = true;}
                if c < size - 1 {board.board[r][c].line_right = true;}
            }
        }
        
        board
    }

    pub fn turn(&mut self, turn: Turn) -> TurnResult {
        let Turn {
            player:         player,
            play_row:       pr,
            play_col:       pc,
            spin_ul_row:    su,
            spin_ul_col:    sl,
            spin_size:      sz,
            spin_dir:       sd,
        } = turn;
        
        // Validate the turn.

        if self.outcome           != None   {return Err(GameAlreadyOver);}
        if self.whose_turn()      != player {return Err(NotYourTurn);}
        if self.size() <= pr                {return Err(InvalidLocation);}
        if self.size() <= pc                {return Err(InvalidLocation);}
        if self.size() <= su + sz - 1       {return Err(InvalidLocation);}
        if self.size() <= sl + sz - 1       {return Err(InvalidLocation);}
        if self.board[pr][pc].stone != None {return Err(PieceAlreadyThere);}
        
        // Place the stone.

        self.board[pr][pc].stone = Some((self.turn, Up, false));

        // Spin the section.

        let mut slice = self.copy_slice_out(su, sl, sz);

        match sd {
            CW => {
                slice = spin_cell_grid(slice);
            },
            CCW => {
                slice = spin_cell_grid(slice);
                slice = spin_cell_grid(slice);
                slice = spin_cell_grid(slice);
            },
        };

        self.copy_slice_in(su, sl, slice);
        self.update_outcome();
        self.turn += 1;

        Ok(self.outcome)
    }

    // Check for wins.

    fn update_outcome(&mut self) {
        let mut winning_tiles: Vec<(usize, usize)> = vec![];

        for r in 0..self.size() {
            for c in 0..self.size() {
                for dir in vec![(0, 1), (1, 0), (1, 1)] {
                    let line = (0..self.win_len).map(|x| (r + dir.0 * x, c + dir.1 * x));

                    for player in vec![Black, White] {
                        if line.clone().all(|(r, c)| 
                                    r < self.size() &&
                                    c < self.size() && 
                                    self.board[r][c].who() == Some(player)) {
                            winning_tiles.extend(line);
                            break;
                        }
                    }
                }
            }
        }

        // Check for wins and double wins.

        if winning_tiles.len() > 0 {
            for &(r, c) in &winning_tiles {
                self.board[r][c].stone = match self.board[r][c].stone {
                    Some((id, or, _win)) => Some((id, or, true)),
                    _ => {panic!();},
                }
            }

            if winning_tiles.iter().all(|&(r, c)| self.board[r][c].who() == Some(Black)) {self.outcome = Some(BlackWin); return;}
            if winning_tiles.iter().all(|&(r, c)| self.board[r][c].who() == Some(White)) {self.outcome = Some(WhiteWin); return;}
            self.outcome = Some(DoubleWin);
            return;
        }

        // Check for stalemate.

        if (0..self.size()).all(|r| (0..self.size()).all(|c| self.board[r][c].stone != None)) {
            self.outcome = Some(Stalemate);
        }
    }

    // Convenience functions.

    fn whose_turn(&self) -> Player {
        if self.turn % 2 == 1 {Black} else {White}
    }

    fn size(&self) -> usize {
        self.board.len()
    }

    fn copy_slice_out(&self, u: usize, l: usize, size: usize) -> Vec<Vec<Cell>> {
        let mut ret = vec![];

        for ro in 0..size {
            ret.push(vec![]);
            for co in 0..size {
                ret[ro].push(self.board[u+ro][l+co]);
            }
        }

        ret
    }

    fn copy_slice_in(&mut self, u: usize, l: usize, slice: Vec<Vec<Cell>>) {
        let size = slice.len();

        for ro in 0..size {
            for co in 0..size {
                self.board[u+ro][l+co] = slice[ro][co];
            }
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let bold        = "\x1b[1m";
        let unbold      = "\x1b[22m";
        let cyan        = "\x1b[96m";
        let cyan_bg     = "\x1b[106;30m";
        let yellow      = "\x1b[93m";
        let yellow_bg   = "\x1b[103;30m";
        let uncolor     = "\x1b[39;49m";
        let unstyle     = "\x1b[0m";

        for r in 0..self.size() {

            // Off-row above.

            for c in 0..self.size()+1 {

                // Off-column before.

                write!(f, " ")?;

                // If last off-column, break.

                if c == self.size() {break;}

                // On-column.

                let wing_up = self.board[r][c].line_up;

                match wing_up {
                    false => {write!(f, "     ")?;},
                    true  => {write!(f, "  │  ")?;},
                };
            }

            write!(f, "\n")?;

            // On-row.

            for c in 0..self.size() + 1 {

                // Off-column before.

                let left_connect  = (c > 0)             && self.board[r][c-1].line_right;
                let left_wing     = (c < self.size())   && self.board[r][c  ].line_left;
                let right_wing    = (c < self.size())   && self.board[r][c  ].line_right;

                match (left_connect, left_wing) {
                    (false, false) => {write!(f, " ")?;},
                    (false, true ) => {write!(f, "╶")?;},
                    (true , false) => {write!(f, "╴")?;},
                    (true , true ) => {write!(f, "─")?;},
                };

                // If last off-column, break.

                if c == self.size() {break;}

                // On-column.

                match left_wing {
                    false => {write!(f, " ")?;},
                    true  => {write!(f, "─")?;},
                }

                match self.board[r][c].stone {
                    Some((num, spin, win)) => {
                        match (self.board[r][c].who().unwrap(), win) {
                            (Black, false) => {write!(f, "{bold}{cyan     }{spin}{unbold}{num:02}{uncolor}")?;}
                            (Black, true ) => {write!(f, "{bold}{cyan_bg  }{spin}{unbold}{num:02}{uncolor}")?;}
                            (White, false) => {write!(f, "{bold}{yellow   }{spin}{unbold}{num:02}{uncolor}")?;}
                            (White, true ) => {write!(f, "{bold}{yellow_bg}{spin}{unbold}{num:02}{uncolor}")?;}
                        }
                    },

                    None => {
                        let up_wing   = self.board[r][c].line_up;
                        let down_wing = self.board[r][c].line_down;

                        match left_wing {
                            false => {write!(f, " ")?;},
                            true  => {write!(f, "─")?;},
                        }

                        match (up_wing, right_wing, down_wing, left_wing) {
                            (true , true , false, false) => {write!(f, "└")?;},
                            (false, true , true , false) => {write!(f, "┌")?;},
                            (false, false, true , true ) => {write!(f, "┐")?;},
                            (true , false, false, true ) => {write!(f, "┘")?;},
                            (true , true , true , false) => {write!(f, "├")?;},
                            (false, true , true , true ) => {write!(f, "┬")?;},
                            (true , false, true , true ) => {write!(f, "┤")?;},
                            (true , true , false, true ) => {write!(f, "┴")?;},
                            (true , true , true , true ) => {write!(f, "┼")?;},
                            _ => panic!("Impossible combination of lines"),
                        }

                        match right_wing {
                            false => {write!(f, " ")?;},
                            true  => {write!(f, "─")?;},
                        }
                    }
                }

                match right_wing {
                    false => {write!(f, " ")?;},
                    true  => {write!(f, "─")?;},
                }
            }

            write!(f, "\n")?;
    
            // Off-row below.

            for c in 0..self.size()+1 {

                // Off-column before.

                write!(f, " ")?;

                // If last off-column, break.

                if c == self.size() {break;}

                // On-column.

                let wing_down = self.board[r][c].line_down;

                match wing_down {
                    false => {write!(f, "     ")?;},
                    true  => {write!(f, "  │  ")?;},
                };
            }

            write!(f, "\n")?;

        }

        write!(f, "{unstyle}")?;
        write!(f, "   ")?;

        match self.outcome {
            Some(BlackWin)  => {write!(f, "Black wins!")?;},
            Some(WhiteWin)  => {write!(f, "White wins!")?;},
            Some(Stalemate) => {write!(f, "Stalemate.")?;},
            Some(DoubleWin) => {write!(f, "Double win!")?;},
            None => {
                match (self.whose_turn(), self.turn_phase) {
                    (Black, Play) => {write!(f, "Black to play...")?;},
                    (Black, Spin) => {write!(f, "Black to spin...")?;},
                    (White, Play) => {write!(f, "White to play...")?;},
                    (White, Spin) => {write!(f, "White to spin...")?;},
                };
            }
        }

        if self.outcome == None {
            write!(f, "\n   Need {} to win.", self.win_len)?;
        }

        Ok(())
    }
}

