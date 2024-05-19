
use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt::Error;
use std::ops::Not;

use crate::game::Player::*;
use crate::game::Orientation::*;
use crate::game::TurnPhase::*;
use crate::game::TurnError::*;
use crate::game::GameOutcome::*;
use crate::game::SpinDirection::*;

// Elementary types.

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

#[derive(Clone, Copy, PartialEq, Eq, Debug)] pub struct Cell {
    stone:      Option<(usize, Orientation)>,
    line_up:    bool,
    line_right: bool,
    line_down:  bool,
    line_left:  bool,
}

pub type TurnResult = Result<Option<GameOutcome>, TurnError>;

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
    fn spun(self) -> Self {
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
            Left  => {write!(f, "← ")?;}
        };

        Ok(())
    }
}

impl Cell {
    fn spun(&self) -> Self {
        Cell {
            stone: match self.stone {
                None => None,
                Some((num, or)) => Some((num, or.spun())),
            },
            line_right: self.line_up,
            line_down:  self.line_right,
            line_left:  self.line_down,
            line_up:    self.line_left,
        }
    }

    fn who(&self) -> Option<Player> {
        match self.stone {
            None => None,
            Some((num, _)) => if num % 2 == 1 {Some(Black)} else {Some(White)},
        }
    }

    fn between_char(&self, other: Self) -> char {
        match (self.line_right, other.line_left) {
            (true , true ) => '─',
            (true , false) => '╴',
            (false, true ) => '╶',
            (false, false) => ' ',
        }
    }
}

fn spin_cell_grid(grid: Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    let size = grid.len();
    let mut ret = vec![];

    for r in 0..size {
        ret.push(vec![]);

        for c in 0..size {
            ret[r].push(grid[size-c-1][r].spun());
        }
    }

    ret
}

// Game type.

pub struct Twirl {
    win_len:    usize,                  // Line length needed to win.
    board:      Vec<Vec<Cell>>,         // State of the board.
    turn:       usize,                  // Number of the active turn's stone.
    turn_phase: TurnPhase,              // Phase of the active turn.

    outcome:    Option<GameOutcome>,    // [Cache] Outcome (= None until the game is over).
}

impl Twirl {
    pub fn new(size: usize, win_len: usize) -> Self {
        let mut board = Twirl {
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

    pub fn play(&mut self, player: Player, r: usize, c: usize) -> TurnResult {
        if self.outcome           != None   {return Err(GameAlreadyOver);}
        if self.whose_turn()      != player {return Err(NotYourTurn);}
        if self.turn_phase        != Play   {return Err(PlayDuringSpinPhase);}
        if self.size() <= r                 {return Err(InvalidLocation);}
        if self.size() <= c                 {return Err(InvalidLocation);}
        if self.board[r][c].stone != None   {return Err(PieceAlreadyThere);}

        self.board[r][c].stone = Some((self.turn, Up));
        self.turn_phase = Spin;

        Ok(None)
    }

    pub fn spin(&mut self, player: Player, u: usize, l: usize, size: usize, dir: SpinDirection) -> TurnResult {
        if self.outcome           != None   {return Err(GameAlreadyOver);}
        if self.whose_turn()      != player {return Err(NotYourTurn);}
        if self.turn_phase        != Spin   {return Err(SpinDuringPlayPhase);}
        if self.size() <= u + size - 1      {return Err(InvalidLocation);}
        if self.size() <= l + size - 1      {return Err(InvalidLocation);}

        let mut slice = self.copy_slice_out(u, l, size);

        slice = spin_cell_grid(slice);
        if dir == CCW {
            slice = spin_cell_grid(slice);
            slice = spin_cell_grid(slice);
        }

        self.copy_slice_in(u, l, slice);

        self.update_outcome();

        self.turn += 1;
        self.turn_phase = Play;

        Ok(self.outcome)
    }

    // Check for wins.

    fn update_outcome(&mut self) {
        let mut winning_tiles: Vec<(usize, usize)> = vec![];

        for r in 0..self.size() {
            for c in 0..self.size() {
                for dir in vec![(0, 1), (1, 0), (1, 1)] {
                    let mut line = (0..self.win_len).map(|x| (r + dir.0 * x, c + dir.1 * x));

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

impl Display for Twirl {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let bold    = "\x1b[1m";
        let unbold  = "\x1b[22m";
        let cyan    = "\x1b[96m";
        let yellow  = "\x1b[93m";
        let uncolor = "\x1b[39m";
        let unstyle = "\x1b[0m";

        for r in 0..self.size()+1 {

            // Off-row above.

            for c in 0..self.size()+1 {

                // Off-column before.

                write!(f, "  ")?;

                // If last off-column, break.

                if c == self.size() {break;}

                // On-column.

                let wing_up = self.board[r-1][c].line_up;

                match wing_up {
                    false => {write!(f, "  │  ");},
                    true  => {write!(f, "     ");},
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
                    false => {write!(f, " ");},
                    true  => {write!(f, "─");},
                }

                match self.board[r][c].stone {
                    Some((num, spin)) => {
                        match self.board[r][c].who().unwrap() {
                            Black => {write!(f, "{bold}{cyan  }{spin}{unbold}{num:02}{uncolor}")?;}
                            White => {write!(f, "{bold}{yellow}{spin}{unbold}{num:02}{uncolor}")?;}
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

                write!(f, "  ")?;

                // If last off-column, break.

                if c == self.size() {break;}

                // On-column.

                let wing_down = self.board[r-1][c].line_down;

                match wing_down {
                    false => {write!(f, "  │  ");},
                    true  => {write!(f, "     ");},
                };
            }

            write!(f, "\n")?;

        }

        write!(f, "{unstyle}")?;
        write!(f, "  ")?;

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

        Ok(())
    }
}

