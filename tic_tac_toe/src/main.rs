// Tic Tac Toe implementation, usign a dumb AI

use rand::Rng;
use std::fmt;

// TODO:
//  * doc comments
//  * UTs
//  * COOL IDEA: also implement a smarter AI, then use a common trait to switch between Dumb and Smart AI


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Cross,
    Naught,
}


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    Running,
    Draw,
    CrossWon,
    NaughtWon,
}


//--------------------------------------
// BOARD
#[derive(Clone)]
pub struct Board {
    grid: [Cell; 9],
}


impl Board {
    pub fn new() -> Self {
        Board {
            grid: [Cell::Empty; 9]
        }
    }

    #[inline]
    pub fn cell(&self, idx: usize) -> Cell {
        self.grid[idx]
    }

    pub fn put(&mut self, idx: usize, item: Cell) -> bool {
        if self.grid[idx] == Cell::Empty {
            self.grid[idx] = item;
            true
        }
        else {
            false
        }
    }

    pub fn compute_status(&self) -> GameStatus {
        static ROWS: [[u8; 3]; 8] = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];
        // check for a win
        for r in ROWS {
            let c1 = self.grid[r[0] as usize];
            let c2 = self.grid[r[1] as usize];
            let c3 = self.grid[r[2] as usize];
            if c1 == c2 && c1 == c3 {
                match c1 {
                    Cell::Cross  => { return GameStatus::CrossWon; },
                    Cell::Naught => { return GameStatus::NaughtWon; },
                    Cell::Empty  => { },
                }
            }
        }
        // if at least one cell is empty, the game is still running
        for i in 0..9 {
            if self.grid[i] == Cell::Empty {
                return GameStatus::Running;
            }
        }
        GameStatus::Running
    }
}


impl fmt::Display for Board {
    // Write into the supplied output stream: `f`.
    // Returns `fmt::Result` which indicates whether the operation succeeded or failed.
    // Note that `write!` uses syntax which is very similar to `println!`.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..3 {
            for x in 0..3 {
                let idx: usize = 3 * y + x;
                match self.grid[idx] {
                    Cell::Cross  => { write!(f, "[X]")?; },
                    Cell::Naught => { write!(f, "[O]")?; },
                    Cell::Empty  => { write!(f, "({idx})")?; },
                };
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}


//--------------------------------------
// DUMB A.I.
pub struct DumbAI {

}

impl DumbAI {

}


//--------------------------------------
// GAME
pub struct Game {

}

impl Game {

}


//--------------------------------------
// MAIN

fn main() {
    println!("Hello, world!");
}
