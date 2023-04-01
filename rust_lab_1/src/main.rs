use rand::Rng;
use std::fmt;
use std::io;


const GRID_SIZE_X:usize = 5;
const GRID_SIZE_Y:usize = 4;
const START_HEALTH:i32 = 3;
const START_EXPLOSIONS:i32 = 36;
const EXPLOSIONS_LOST_ON_FREEZE:i32 = 3;


/// Game status.
#[derive(Clone, Copy, PartialEq, Eq)]
enum GameStatus {
    Running,
    YouWon,
    BrotherWon,
}


/// What can occupy a space in the grid.
#[derive(Clone, Copy, PartialEq, Eq)]
enum GridOccupant {
    Tree,
    Player,
    Sister,
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
}


/// The Representation of the hide and seek game
struct GameState {
    map: Vec<Vec<GridOccupant>>,
    your_health: i32,
    bro_expl_cnt: i32,
    // - remember bombed positions (until bro gets a hit => we move => bombed positions are reset)
    blast_history: Vec<Coord>,
    // - remember if bro bombed sis (so he doesn't bomb her twice)
    bombed_sis_pos: Option<Coord>,
}

impl GameState {
    // QUIZ: How can you generate cargo docs and view their website
    /// Initializes the GameState
    pub fn new() -> Self {
        let mut game = GameState {
            map: vec![],
            your_health: START_HEALTH,
            bro_expl_cnt: START_EXPLOSIONS,
            blast_history: vec![],
            bombed_sis_pos: Option::None,
        };
        // init board
        (0..GRID_SIZE_Y).for_each(|_| {
            let row: Vec<GridOccupant> = vec![GridOccupant::Tree; GRID_SIZE_X];
            game.map.push(row);
        });
        // place yourself
        let pos = game.random_pos_in_forest();
        game.map[pos.y][pos.x] = GridOccupant::Player;
        // place sister
        let pos = game.random_pos_in_forest();
        game.map[pos.y][pos.x] = GridOccupant::Sister;
        // done
        game
    }

    pub fn status(&self) -> GameStatus {
        if self.your_health <= 0 {
            GameStatus::BrotherWon
        }
        else if self.bro_expl_cnt <= 0 {
            GameStatus::YouWon
        }
        else {
            GameStatus::Running
        }
    }

    pub fn print_map(&self) {
        for y in 0..GRID_SIZE_Y {
            for x in 0..GRID_SIZE_X {
                let pos = Coord {x, y};
                print!("{}", self.map[pos.y][pos.x]);
            }
            println!("");
        }
    }

    pub fn perform_step(&mut self) {
        let pos = self.pick_position_to_bomb();
        println!("Your brother is attacking ({}, {})!", pos.x, pos.y);
        self.bro_expl_cnt -= 1;
        match self.map[pos.y][pos.x] {
            GridOccupant::Player => self.brother_hit_player(pos),
            GridOccupant::Sister => self.brother_hit_sister(pos),
            GridOccupant::Tree   => self.brother_hit_nothing(pos),
        }
        println!("Your health: {}", self.your_health);
        println!("Brother's blasts left: {}", self.bro_expl_cnt);
    }

    fn brother_hit_player(&mut self, pos: Coord) {
        println!("Your brother hit you!");
        self.your_health -= 1;
        // move away
        let new_pos = self.random_pos_in_forest();
        self.map[pos.y][pos.x] = GridOccupant::Tree;
        self.map[new_pos.y][new_pos.x] = GridOccupant::Player;
        // reset the blast history
        self.blast_history.clear();
        if let Some(sis_pos) = self.bombed_sis_pos.as_ref() {
            self.blast_history.push(sis_pos.clone());
        }
    }

    fn brother_hit_sister(&mut self, pos: Coord) {
        println!("Your brother hit your sister and was frozen!");
        self.bro_expl_cnt -= EXPLOSIONS_LOST_ON_FREEZE;
        self.bombed_sis_pos = Some(pos.clone());
        self.blast_history.push(pos);
    }

    fn brother_hit_nothing(&mut self, pos: Coord) {
        println!("Your brother missed completely!");
        self.blast_history.push(pos);
    }

    fn random_pos_conditioned(&self, cond: fn(&GameState, &Coord) -> bool) -> Coord {
        let mut coord = Coord::random();
        while !cond(&self, &coord) {
            coord = Coord::random();
        }
        coord
    }

    fn random_pos_in_forest(&self) -> Coord {
        self.random_pos_conditioned(|game, coord|
            game.map[coord.y][coord.x] == GridOccupant::Tree
        )
    }

    fn pick_position_to_bomb(&self) -> Coord {
        self.random_pos_conditioned(|game, coord|
            !game.blast_history.contains(&coord)
        )
    }
}



//-----------------------
//   MAIN
//-----------------------
fn main() {
    let mut buffer = String::with_capacity(256);
    let mut game = GameState::new();
    while game.status() == GameStatus::Running {
        game.print_map();
        game.perform_step();
        // read line
        println!("<-- Press ENTER --> ");
        let iores = io::stdin().read_line(&mut buffer);
        iores.unwrap();
        buffer.clear();
    }

    // game over
    if game.status() == GameStatus::YouWon {
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
