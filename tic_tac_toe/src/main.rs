// Tic Tac Toe implementation, usign a dumb AI

use rand::Rng;
use std::{fmt, io};

// TODO:
//  * doc comments
//  * UTs
//  * make the grid size an adjustable constant, instead of being hardcoded to 3
//  * auto-compute status after each move, instead of checking the whole board
//  * COOL IDEA: also implement a smarter AI, then use a common trait to switch between Dumb and Smart AI


//----------------------------------------------
// Enum for a cell type - `X`, `O` or empty
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Cross,
    Naught,
}


impl Cell {
    #[inline]
    pub fn to_win_type(&self) -> Option<GameStatus> {
        match self {
            Cell::Empty  => None,
            Cell::Cross  => Some(GameStatus::CrossWon),
            Cell::Naught => Some(GameStatus::NaughtWon),
        }
    }

    #[inline]
    pub fn opposite(&self) -> Option<Cell> {
        match self {
            Cell::Empty  => None,
            Cell::Cross  => Some(Cell::Naught),
            Cell::Naught => Some(Cell::Cross),
        }
    }
}


impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Cross  => { write!(f, "`X`")?; },
            Cell::Naught => { write!(f, "`O`")?; },
            Cell::Empty  => { write!(f, "`empty cell`")?; },
        };
        Ok(())
    }
}


//--------------------------------------
// Enum for the game status
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    Running,
    Draw,
    CrossWon,
    NaughtWon,
}

impl GameStatus {
    #[inline]
    pub fn win_to_cell(&self) -> Option<Cell> {
        match self {
            GameStatus::CrossWon  => Some(Cell::Cross),
            GameStatus::NaughtWon => Some(Cell::Naught),
            _ => None,
        }
    }
}


//--------------------------------------
// BOARD
#[derive(Clone)]
pub struct Board {
    grid: [Cell; 9],
    moves: Vec<usize>,
}


impl Board {
    pub fn new() -> Self {
        Board {
            grid: [Cell::Empty; 9],
            moves: vec![],
        }
    }

    #[inline]
    pub fn cell(&self, idx: usize) -> Cell {
        self.grid[idx]
    }

    pub fn put(&mut self, idx: usize, item: Cell) -> bool {
        if self.grid[idx] == Cell::Empty {
            self.grid[idx] = item;
            self.moves.push(idx);
            true
        }
        else {
            false
        }
    }

    pub fn undo(&mut self) {
        if let Some(last_move) = self.moves.pop() {
            self.grid[last_move] = Cell::Empty;
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
        // nobody won and the board is full => it's a draw
        GameStatus::Draw
    }
}


impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..3 {
            for x in 0..3 {
                let idx: usize = 3 * y + x;
                match self.grid[idx] {
                    Cell::Cross  => { write!(f, "[X]")?; },
                    Cell::Naught => { write!(f, "[O]")?; },
                    //Cell::Empty  => { write!(f, "({idx})")?; },
                    Cell::Empty  => { write!(f, "[ ]")?; },
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
    pub fn compute_move(&self, board: &Board, my_type: Cell) -> Option<usize> {
        let other_type = my_type.opposite().expect("AI cell type cannot be empty");
        let mut board_copy = board.clone();
        let mut block_cell: Option<usize> = None;
        let mut free_cells: Vec<usize> = vec![];

        // check each empty cell
        for idx in 0..9 {
            if board_copy.cell(idx) == Cell::Empty {
                // remember that this is an empty cell
                free_cells.push(idx);
                // if we win by moving here => this is it
                if Self::move_will_win(&mut board_copy, idx, my_type) {
                    return Some(idx);
                }
                // if the other will win by moving here => we MUST move here
                if Self::move_will_win(&mut board_copy, idx, other_type) {
                    block_cell = Some(idx);
                }
            }
        }

        if block_cell.is_some() {
            // if we must block => just block
            block_cell
        }
        else if free_cells.is_empty() {
            // the board is full => it's a draw
            None
        }
        else {
            Some(Self::pick_free_cell(&mut board_copy, &free_cells))
        }
    }


    // DUMB AI: just pick a random cell
    #[allow(unused_variables)]
    fn pick_free_cell(board_copy: &mut Board, free_cells: &Vec<usize>) -> usize {
        let random_idx = rand::thread_rng().gen_range(0..free_cells.len());
        free_cells[random_idx]
    }


    fn move_will_win(board_copy: &mut Board, idx: usize, move_type: Cell) -> bool {
        if board_copy.put(idx, move_type) {
            let status_after_move = board_copy.compute_status();
            board_copy.undo();
            if move_type.to_win_type().unwrap() == status_after_move {
                return true;
            }
        }
        false
    }
}


//--------------------------------------
// GAME
pub struct Game {
    board: Board,
    status: GameStatus,
    who_moves_next: Cell,
    ai: DumbAI,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            status: GameStatus::Running,
            who_moves_next: Cell::Cross,
            ai: DumbAI { },
        }
    }

    #[inline]
    pub fn status(&self) -> GameStatus {
        self.status
    }

    #[inline]
    pub fn who_moves_next(&self) -> Cell {
        self.who_moves_next
    }

    pub fn perform_ai_move(&mut self) -> Result<usize, String> {
        if self.status == GameStatus::Running {
            let ai_move =
                self.ai.compute_move(&self.board, self.who_moves_next).unwrap();
            self.perform_move(ai_move)?;
            Ok(ai_move)
        }
        else {
            Err(String::from("AI cannot move, because the game is over"))
        }
    }

    fn perform_move(&mut self, move_idx: usize) -> Result<(), String> {
        if self.board.put(move_idx, self.who_moves_next) {
            self.status = self.board.compute_status();
            self.who_moves_next = self.who_moves_next.opposite().unwrap();
            Ok(())
        }
        else {
            let msg = format!("Cannot move {} at {}", self.who_moves_next, move_idx);
            Err(msg)
        }
    }
}


//--------------------------------------
// MAIN

fn main() {
    let mut gamenr: u64 = 0;
    loop {
        gamenr += 1;
        println!("Game #{gamenr}\n");

        // start the game
        let human = read_input("Choose X or O", vec!["x", "o"]);
        let human = if human == "x" { Cell::Cross } else { Cell::Naught };
        let mut game = Game::new();

        // run the game unti it is over
        while game.status() == GameStatus::Running {
            if human == game.who_moves_next() {
                // ask human for move
                let human_move =
                    read_number("Enter cell to move", 0..9);
                let move_ok = game.perform_move(human_move);
                if let Err(msg) = move_ok {
                    println!("ERROR: {msg}");
                }
            }
            else {
                let ai_move = game.perform_ai_move();
                match ai_move {
                    Ok(idx) => { println!("AI moved at {idx}"); },
                    Err(msg) => { println!("ERROR: {msg}"); }
                }
            };
            println!("{}", game.board);
        }

        // print the winner
        println!("\n  >>> GAME OVER - {} <<<\n", match game.status() {
            GameStatus::Running   => { panic!("Game should not be running anymore"); },
            GameStatus::Draw      => { "Draw :/" },
            GameStatus::CrossWon  => { "X won" },
            GameStatus::NaughtWon => { "O won" },
        });

        // ask if we should continue
        let keep_on =
            read_input("Play another game? (Y/N)", vec!["y", "n"]);
        if keep_on == "n" {
            return;
        }
        println!("\n\n");
    }
}

fn read_input(prompt: &str, allowed_values: Vec<&str>) -> String {
    loop {
        let line = prompt_and_read_line(prompt);
        for str in &allowed_values {
            if line.eq_ignore_ascii_case(str) {
                return line.to_ascii_lowercase();
            }
        }
    }
}

fn read_number(prompt: &str, range: std::ops::Range<usize>) -> usize {
    loop {
        let line = prompt_and_read_line(prompt);
        let num = line.parse::<usize>();
        if let Ok(val) = num {
            if range.contains(&val) {
                return val;
            }
        }
    }
}

fn prompt_and_read_line(prompt: &str) -> String {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    print!("> {prompt}: ");
    io::Write::flush(&mut stdout).expect("flush failed!");
    let mut line: String = String::new();
    stdin.read_line(&mut line).unwrap();
    line.trim().to_string()
}
