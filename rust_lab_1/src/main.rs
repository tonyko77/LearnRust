use rand::Rng;
use std::fmt;
use std::io;


const GRID_SIZE_X:usize = 5;
const GRID_SIZE_Y:usize = 5;
const START_HEALTH:i32 = 3;
const START_EXPLOSIONS:i32 = 36;
const EXPLOSIONS_LOST_ON_FREEZE:i32 = 3;


/// What can occupy a space in the grid.
#[derive(Clone, Copy, PartialEq, Eq)]
enum GridOccupant {
    Tree,
    Player,
    Sister,
    // TODO Bombed ??
}

impl fmt::Display for GridOccupant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write into the supplied output stream: `f`.
        // Returns `fmt::Result` which indicates whether the operation succeeded or failed.
        // Note that `write!` uses syntax which is very similar to `println!`.
        let c = match self {
            GridOccupant::Tree   => '_',
            GridOccupant::Player => 'P',
            GridOccupant::Sister => 'S',
            //GridOccupant::Bombed => '*',
        };
        write!(f, "{}", c)
    }    
}


/// Simple structure, for coordinates
#[derive(Clone, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        Coord {
            x: rng.gen_range(0..GRID_SIZE_X),
            y: rng.gen_range(0..GRID_SIZE_Y),
        }
    }

    #[inline]
    fn to_index(&self) -> usize {
        self.y * GRID_SIZE_X + self.x        
    }
}


/// The Representation of the hide and seek game
struct GameState {
    // TODO: make this 2d
    map: Vec<GridOccupant>,
    your_health: i32,
    bro_expl_cnt: i32,
    blast_history: Vec<Coord>,
    bombed_sis_pos: Option<Coord>,
}

impl GameState {
    // QUIZ: How can you generate cargo docs and view their website
    /// Initializes the GameState
    pub fn new() -> Self {
        let mut game = GameState {
            map: vec![GridOccupant::Tree; GRID_SIZE_X * GRID_SIZE_Y],
            your_health: START_HEALTH,
            bro_expl_cnt: START_EXPLOSIONS,
            blast_history: vec![],
            bombed_sis_pos: Option::None,
        };
        // place yourself
        let pos = game.random_pos_in_forest();
        game.map[pos.to_index()] = GridOccupant::Player;
        // place sister
        let pos = game.random_pos_in_forest();
        game.map[pos.to_index()] = GridOccupant::Sister;
        // done
        game
    }

    pub fn still_running(&self) -> bool {
        (self.bro_expl_cnt > 0) && (self.your_health > 0)
    }

    pub fn you_won(&self) -> bool {
        (self.bro_expl_cnt <= 0) && (self.your_health > 0)
    }

    pub fn print_map(&self) {
        for y in 0..GRID_SIZE_Y {
            for x in 0..GRID_SIZE_X {
                let pos = Coord {x, y};
                print!("{}", self.map[pos.to_index()]);
            }
            println!("");
        }
    }

    // TODO improvements:
    // - remember bombed positions (until bro gets a hit => we move => bombed positions are reset)
    // - remember if bro bombed sis (so he doesn't bomb her twice)
    pub fn perform_step(&mut self) {
        let pos = self.pick_position_to_bomb();
        println!("Your brother is attacking ({}, {})!", pos.x, pos.y);
        self.bro_expl_cnt -= 1;
        match self.map[pos.to_index()] {
            GridOccupant::Player => {
                println!("Your brother hit you!");
                self.your_health -= 1;
                // move away
                let new_pos = self.random_pos_in_forest();
                self.map[pos.to_index()] = GridOccupant::Tree;
                self.map[new_pos.to_index()] = GridOccupant::Player;
                // reset the blast history
                self.blast_history.clear();
                if let Some(sis_pos) = self.bombed_sis_pos.as_ref() {
                    self.blast_history.push(sis_pos.clone());
                }
            },
            GridOccupant::Sister => {
                println!("Your brother hit your sister and was frozen!");
                self.bro_expl_cnt -= EXPLOSIONS_LOST_ON_FREEZE;
                self.bombed_sis_pos = Some(pos.clone());
                self.blast_history.push(pos);
            },
            GridOccupant::Tree => {
                println!("Your brother missed completely!");
                self.blast_history.push(pos);
            },
        }
        println!("Your health: {}", self.your_health);
        println!("Brother's blasts left: {}", self.bro_expl_cnt);
    }

    fn random_pos_conditioned(&self, cond: fn(GridOccupant) -> bool) -> Coord {
        let mut coord = Coord::random();
        //while !cond(self.map[coord.to_index()]) {
        while !cond(self.map[coord.to_index()]) {
            coord = Coord::random();
        }
        coord
    }

    fn random_pos_in_forest(&self) -> Coord {
        self.random_pos_conditioned(|go| go == GridOccupant::Tree)
    }

    fn pick_position_to_bomb(&self) -> Coord {
        let mut coord = Coord::random();
        while self.blast_history.contains(&coord) {
            coord = Coord::random();
        }
        coord
    }
}



//-----------------------
//   MAIN
//-----------------------
fn main() {
    let mut buffer = String::with_capacity(256);
    let mut game = GameState::new();
    while game.still_running() {
        game.print_map();
        game.perform_step();
        // read line
        println!("<-- Press ENTER --> ");
        let iores = io::stdin().read_line(&mut buffer);
        iores.unwrap();
        buffer.clear();
    }

    // game over
    if game.you_won() {
        println!("Congratulations, you WON :)");
    }
    else {
        println!("Too bad, your brother WON :(");
    }
    println!("Time for dinner!");
}


//=============================================================
//  TESTS
//=============================================================

/* TODO - Read about Rust UTs, then IMPLEMENT the tests !!!
#[cfg(test)]
mod test {
    use super::*;
    /// Tests that a grid is valid. IE it contains trees, you, and your sister.
    #[test]
    fn test_valid_grid() {}
}
*/
