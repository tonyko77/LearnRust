use rand::Rng;
use std::fmt;
use std::io;


const GRID_SIZE_X:usize = 5;
const GRID_SIZE_Y:usize = 4;
const START_HEALTH:i32 = 3;
const START_EXPLOSIONS:i32 = 36;
const EXPLOSIONS_LOST_ON_FREEZE:i32 = 3;


/// Game status.
#[derive(Clone, Copy, PartialEq, Debug)]
enum GameStatus {
    Running,
    YouWon,
    BrotherWon,
}


/// What can occupy a space in the grid.
#[derive(Clone, Copy, PartialEq, Debug)]
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
#[derive(Clone, PartialEq, Debug)]
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
        let your_pos = Coord::random();
        let mut sister_pos = Coord::random();
        while sister_pos == your_pos {
            sister_pos = Coord::random();
        }
        Self::new_customized(&your_pos, &sister_pos)
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
        let bomb_pos = self.pick_position_to_bomb();
        let new_player_pos = self.random_pos_in_forest();
        self.perform_step_customized(bomb_pos, new_player_pos);
    }

    fn new_customized(your_pos: &Coord, sister_pos: &Coord) -> Self {
        let mut game = GameState {
            map: vec![],
            your_health: START_HEALTH,
            bro_expl_cnt: START_EXPLOSIONS,
            blast_history: vec![],
            bombed_sis_pos: Option::None,
        };
        // init board
        (0..GRID_SIZE_Y).for_each(|_| {
            game.map.push(vec![GridOccupant::Tree; GRID_SIZE_X]);
        });
        // place yourself and your sister
        game.map[your_pos.y][your_pos.x] = GridOccupant::Player;
        game.map[sister_pos.y][sister_pos.x] = GridOccupant::Sister;
        // done
        game
    }

    fn perform_step_customized(&mut self, bomb_pos: Coord, new_player_pos: Coord) {
        println!("Your brother is attacking ({}, {})!", bomb_pos.x, bomb_pos.y);
        self.bro_expl_cnt -= 1;
        match self.map[bomb_pos.y][bomb_pos.x] {
            GridOccupant::Player => self.brother_hit_player(bomb_pos, new_player_pos),
            GridOccupant::Sister => self.brother_hit_sister(bomb_pos),
            GridOccupant::Tree   => self.brother_hit_nothing(bomb_pos),
        }
        println!("Your health: {}", self.your_health);
        println!("Brother's blasts left: {}", self.bro_expl_cnt);
    }

    fn brother_hit_player(&mut self, bomb_pos: Coord, new_player_pos: Coord) {
        println!("Your brother hit you!");
        self.your_health -= 1;
        // move away
        self.map[bomb_pos.y][bomb_pos.x] = GridOccupant::Tree;
        self.map[new_player_pos.y][new_player_pos.x] = GridOccupant::Player;
        // reset the blast history
        self.blast_history.clear();
        // if the sister was blasted before, put that in the blast history
        if let Some(sis_pos) = self.bombed_sis_pos.as_ref() {
            self.blast_history.push(sis_pos.clone());
        }
    }

    fn brother_hit_sister(&mut self, bomb_pos: Coord) {
        println!("Your brother hit your sister and was frozen!");
        self.bro_expl_cnt -= EXPLOSIONS_LOST_ON_FREEZE;
        self.bombed_sis_pos = Some(bomb_pos.clone());
        self.blast_history.push(bomb_pos);
    }

    fn brother_hit_nothing(&mut self, bomb_pos: Coord) {
        println!("Your brother missed completely!");
        self.blast_history.push(bomb_pos);
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

#[cfg(test)]
mod test {
    use super::*;

    /// Tests that a grid is valid. IE it contains trees, you, and your sister.
    #[test]
    fn test_valid_grid() {
        let your_pos = Coord { x: 0, y: 0 };
        let sister_pos = Coord { x: 0, y: 1 };
        let game = GameState::new_customized(&your_pos, &sister_pos);

        assert_eq!(GRID_SIZE_Y, game.map.len());
        assert_eq!(GridOccupant::Player, game.map[0][0]);
        assert_eq!(GridOccupant::Sister, game.map[1][0]);
        for y in 0..GRID_SIZE_Y {
            assert_eq!(GRID_SIZE_X, game.map[y].len());
            for x in 0..GRID_SIZE_X {
                if (x > 0) || (y > 1) {
                    assert_eq!(GridOccupant::Tree, game.map[y][x]);
                }
            }
        }
    }

    /// Tests that the initial game state is as expected
    #[test]
    fn test_valid_initial_state() {
        let game = GameState::new();

        assert_eq!(START_HEALTH, game.your_health);
        assert_eq!(START_EXPLOSIONS, game.bro_expl_cnt);
        assert!(game.blast_history.is_empty());
        assert!(matches!(game.bombed_sis_pos, Option::None));
        assert_eq!(GameStatus::Running, game.status());
    }

    /// Tests the state of the game after bombing an empty spot
    #[test]
    fn test_bomb_empty_spot() {
        let your_pos = Coord { x: 1, y: 0 };
        let sister_pos = Coord { x: 2, y: 1 };
        let mut game = GameState::new_customized(&your_pos, &sister_pos);
        let bomb_pos = Coord { x: 2, y: 3};

        game.perform_step_customized(bomb_pos.clone(), Coord::random());

        assert_eq!(GameStatus::Running, game.status());         // the game is still running
        assert_eq!(GridOccupant::Player, game.map[0][1]);       // player's position didn't change
        assert_eq!(GridOccupant::Sister, game.map[1][2]);       // sister's position didn't change
        assert_eq!(START_HEALTH, game.your_health);             // player's health didn't change
        assert_eq!(START_EXPLOSIONS - 1, game.bro_expl_cnt);    // bro's expl. count has decreased
        assert_eq!(1, game.blast_history.len());                // bombed position history was updated
        assert_eq!(bomb_pos, game.blast_history[0]);            // bombed position history was updated
        assert!(game.bombed_sis_pos.is_none());                 // sis's position wasn't discovered
    }

    /// Tests the state of the game after bombing the sister
    #[test]
    fn test_bomb_sister() {
        let your_pos = Coord { x: 1, y: 0 };
        let sister_pos = Coord { x: 2, y: 1 };
        let mut game = GameState::new_customized(&your_pos, &sister_pos);

        game.perform_step_customized(sister_pos.clone(), Coord::random());

        let expected_expl_after = START_EXPLOSIONS - 1 - EXPLOSIONS_LOST_ON_FREEZE;
        assert_eq!(GameStatus::Running, game.status());         // the game is still running
        assert_eq!(GridOccupant::Player, game.map[0][1]);       // player's position didn't change
        assert_eq!(GridOccupant::Sister, game.map[1][2]);       // sister's position didn't change
        assert_eq!(START_HEALTH, game.your_health);             // player's health didn't change
        assert_eq!(expected_expl_after, game.bro_expl_cnt);     // bro's expl. count has decreased
        assert_eq!(1, game.blast_history.len());                // bombed position history was updated
        assert_eq!(sister_pos, game.blast_history[0]);          // bombed position history was updated
        assert!(game.bombed_sis_pos.is_some());                 // sis's position was discovered
        assert!(game.bombed_sis_pos.unwrap() == sister_pos);    // sis's position was discovered
    }

    /// Tests the state of the game after bombing the player
    #[test]
    fn test_bomb_player() {
        let your_pos = Coord { x: 1, y: 0 };
        let sister_pos = Coord { x: 2, y: 1 };
        let new_pos = Coord { x: 3, y: 2 };
        let mut game = GameState::new_customized(&your_pos, &sister_pos);

        game.perform_step_customized(your_pos.clone(), new_pos.clone());

        assert_eq!(GameStatus::Running, game.status());         // the game is still running
        assert_eq!(GridOccupant::Tree, game.map[0][1]);         // player's position did change
        assert_eq!(GridOccupant::Player, game.map[2][3]);       // player's position did change
        assert_eq!(GridOccupant::Sister, game.map[1][2]);       // sister's position didn't change
        assert_eq!(START_HEALTH - 1, game.your_health);         // player's health did change
        assert_eq!(START_EXPLOSIONS - 1, game.bro_expl_cnt);    // bro's expl. count has decreased
        assert!(game.blast_history.is_empty());                 // bombed position history is now empty
        assert!(game.bombed_sis_pos.is_none());                 // sis's position was not discovered
    }

    /// Tests the state of the game after using the final bomb to blast the player's last life
    #[test]
    fn test_bomb_player_game_over_brother_won() {
        let your_pos = Coord { x: 1, y: 0 };
        let sister_pos = Coord { x: 2, y: 1 };
        let mut game = GameState::new_customized(&your_pos, &sister_pos);
        game.your_health = 1;
        game.bro_expl_cnt = 1;

        game.perform_step_customized(your_pos.clone(), Coord::random());

        assert_eq!(GameStatus::BrotherWon, game.status());  // the game is over, your brother won
        assert_eq!(0, game.your_health);                    // your last HP was consumed
        assert_eq!(0, game.bro_expl_cnt);                   // bro's last explosion was consumed
    }

    /// Tests the state of the game after using the final bomb, and the player is still alive
    #[test]
    fn test_bomb_player_game_over_you_won() {
        let your_pos = Coord { x: 1, y: 0 };
        let sister_pos = Coord { x: 2, y: 1 };
        let mut game = GameState::new_customized(&your_pos, &sister_pos);
        game.your_health = 2;
        game.bro_expl_cnt = 1;

        game.perform_step_customized(your_pos.clone(), Coord::random());

        assert_eq!(GameStatus::YouWon, game.status());  // the game is over, your brother won
        assert_eq!(1, game.your_health);                // your still have 1 HP
        assert_eq!(0, game.bro_expl_cnt);               // bro's last explosion was consumed
    }

    /// Tests that the blast history is updated when hitting an empty space
    #[test]
    fn test_blast_history_updated_after_empty_spot_is_hit() {
        let your_pos = Coord { x: 1, y: 0 };
        let sister_pos = Coord { x: 2, y: 1 };
        let mut game = GameState::new_customized(&your_pos, &sister_pos);
        let bomb_pos = Coord { x: 2, y: 3};
        // put some random items in history
        let hist_size_before = 3;
        for _ in 0..hist_size_before {
            game.blast_history.push(Coord::random());
        }

        game.perform_step_customized(bomb_pos.clone(), Coord::random());

        assert_eq!(1 + hist_size_before, game.blast_history.len());
        assert_eq!(bomb_pos, game.blast_history[hist_size_before]);
    }

    /// Tests that the blast history is updated when hitting the sister
    #[test]
    fn test_blast_history_updated_after_sister_is_hit() {
        let your_pos = Coord { x: 1, y: 0 };
        let sister_pos = Coord { x: 2, y: 1 };
        let mut game = GameState::new_customized(&your_pos, &sister_pos);
        // put some random items in history
        let hist_size_before = 3;
        for _ in 0..hist_size_before {
            game.blast_history.push(Coord::random());
        }

        game.perform_step_customized(sister_pos.clone(), Coord::random());

        assert_eq!(1 + hist_size_before, game.blast_history.len());
        assert_eq!(sister_pos, game.blast_history[hist_size_before]);
    }

    /// Tests that the blast history is cleared after hitting the player (and sister was not yet hit)
    #[test]
    fn test_blast_history_cleared_after_player_is_hit_and_sister_was_not_hit() {
        let your_pos = Coord { x: 1, y: 0 };
        let sister_pos = Coord { x: 2, y: 1 };
        let mut game = GameState::new_customized(&your_pos, &sister_pos);
        // put some random items in history
        let hist_size_before = 3;
        for _ in 0..hist_size_before {
            game.blast_history.push(Coord::random());
        }

        game.perform_step_customized(your_pos.clone(), Coord::random());

        assert!(game.blast_history.is_empty());
    }

    /// Tests that the sister is kept in the blast history is cleared after hitting the player (and sister was not yet hit)
    #[test]
    fn test_blast_history_after_player_is_hit_and_sister_was_hit() {
        let your_pos = Coord { x: 1, y: 0 };
        let sister_pos = Coord { x: 2, y: 1 };
        let mut game = GameState::new_customized(&your_pos, &sister_pos);
        // put some random items in history
        for _ in 0..3 {
            game.blast_history.push(Coord::random());
        }
        // hit the sister
        game.perform_step_customized(sister_pos.clone(), Coord::random());
        // put some more random items in history
        for _ in 0..3 {
            game.blast_history.push(Coord::random());
        }

        // now hit the player
        game.perform_step_customized(your_pos.clone(), Coord::random());

        assert_eq!(1, game.blast_history.len());
        assert_eq!(sister_pos, game.blast_history[0]);
    }

}
