use rand::Rng;
use std::fmt;
use std::io;

//===========================================================================================================
// You are playing hide and seek with your family in a forest. But you and your family have super powers.
// So it's not normal hide and seek, its hide and seek with explosions! It's your turn to hide,
// and your little brother's turn to see. In order to seek you he shoots random explosions at you until
// he finds you and hits you 3 times. Your brother goes and stands on a cliff overlooking the forest
// to get a better view and the games begin!
//
// Problem
//    Your brother can see the entire forest, a 5 x 5 grid, and knows that you are hiding somewhere in there.
//    He decides to fire off explosions at random spots in the forest. Maybe he'll hit you, maybe he won't.
//    Either way, he has a limited amount of energy can only fire off 36 explosions before his turn is over.
//    But, if your brother hits you 3 times then he will win and get to choose what to eat for dinner.
// You must simulate this duel, and print the result of who wins at the end.
//
// Other Rules
//  * You will hide in the same spot until he is hit. If you are hit you will jump to a new random spot in the forest.
//  * Every turn your brother must choose a new random spot to launch an attack on.
//  * Your sister is also hiding in a random spot in the forest. If your brother hits her,
//    your brother becomes frozen and loses three explosions. Your sister will never move.
//  * You and your sister cannot be in the same spot.
//  * At the beginning of each round output a visual of the forest.
//    "_" marks a tree 'Y' marks yourself, 'S' marks your sister.
//  * After each round (your brother's explosion) output whether he hit, who he it or if he missed,
//    as well as the location of the attack.
//  * Also after each round output your remaining stamina, and your brother's remaining explosions.
//  * Once either you are out of stamina or your brother is out of explosions the contest is over.
//    Print the winner followed by 'Time for dinner!'
//
// Challenges:
//  * Use Clap to generate a CLI for this simulation that allows the user to change the initial parameters
//    of the simulation, such as grid size, your brothers's stamina, how many sisters you have, etc.
//  * Wanna work ahead of where we are in the book? Create a Forest object to represent
//    the game's current state and implement the Display trait.
//  * Add some unit tests! I get that unit testing is kind of odd in programs that use RNG but if you design
//    your functions cleverly you can get around that. Note that you can test private functions in rust.
//===========================================================================================================

// FIXED:
//  * I didn't think of using a "sparse" representation of the forest => REFACTOR
//  * Create a Forest object to represent the game's current state and implement the Display trait.

// TODO:
//  * Add doc comments + QUIZ: generate cargo docs and view their website
//  * Use Clap to generate a CLI for this simulation ....


const GRID_SIZE_X:usize = 5;
const GRID_SIZE_Y:usize = 5;
const START_HEALTH:i32 = 3;
const START_EXPLOSIONS:i32 = 36;
const EXPLOSIONS_LOST_ON_FREEZE:i32 = 3;


/// Game status.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum GameStatus {
    Running,
    YouWon,
    BrotherWon,
}


/// Simple structure, for coordinates
#[derive(Clone, PartialEq, Eq, Debug)]
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


/// The Representation of the game grid (i.e. the forest)
struct Forest {
    player_pos: Coord,
    sister_pos: Coord,
}

impl Forest {
    fn new(player_pos: Coord, sister_pos: Coord) -> Self {
        Forest {
            player_pos: player_pos,
            sister_pos: sister_pos,
        }
    }

    fn is_player_at(&self, pos: &Coord) -> bool {
        *pos == self.player_pos
    }

    fn is_sister_at(&self, pos: &Coord) -> bool {
        *pos == self.sister_pos
    }

    fn is_tree_at(&self, pos: &Coord) -> bool {
        (*pos != self.player_pos) &&
        (*pos != self.sister_pos)
    }
}

impl fmt::Display for Forest {
    // Write into the supplied output stream: `f`.
    // Returns `fmt::Result` which indicates whether the operation succeeded or failed.
    // Note that `write!` uses syntax which is very similar to `println!`.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..GRID_SIZE_Y {
            for x in 0..GRID_SIZE_X {
                let pos = Coord {x, y};
                let chr =
                    if pos == self.player_pos { 'P' }
                    else if pos == self.sister_pos { 'S' }
                    else { '_' };
                write!(f, "{chr}")?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}


/// The Representation of the hide and seek game
struct GameState {
    forest: Forest,
    your_health: i32,
    bro_expl_cnt: i32,
    blast_history: Vec<Coord>,  // remember bombed positions (until bro gets a hit => we move => clear history)
    bombed_sis_pos: bool,       // remember if bro bombed sis (she doesn't move => he doesn't bomb her twice)
}

impl GameState {
    // QUIZ: How can you generate cargo docs and view their website
    /// Initializes the GameState
    pub fn new() -> Self {
        let player_pos = Coord::random();
        let mut sister_pos = Coord::random();
        while sister_pos == player_pos {
            sister_pos = Coord::random();
        }
        Self::new_customized(player_pos, sister_pos)
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
        print!("{}", self.forest)
    }

    pub fn perform_step(&mut self) {
        let bomb_pos = self.pick_position_to_blast();
        let new_player_pos = self.random_pos_in_forest();
        self.perform_step_customized(bomb_pos, new_player_pos);
    }

    fn new_customized(player_pos: Coord, sister_pos: Coord) -> Self {
        GameState {
            forest: Forest::new(player_pos, sister_pos),
            your_health: START_HEALTH,
            bro_expl_cnt: START_EXPLOSIONS,
            blast_history: vec![],
            bombed_sis_pos: false,
        }
    }

    fn perform_step_customized(&mut self, bomb_pos: Coord, new_player_pos: Coord) {
        println!("Your brother is attacking ({}, {})!", bomb_pos.x, bomb_pos.y);
        self.bro_expl_cnt -= 1;
        if self.forest.is_player_at(&bomb_pos) {
            self.brother_hit_player(new_player_pos);
        }
        else if self.forest.is_sister_at(&bomb_pos) {
            self.brother_hit_sister();
        }
        else {
            self.brother_hit_nothing(bomb_pos);
        }
        println!("Your health: {}", self.your_health);
        println!("Brother's blasts left: {}", self.bro_expl_cnt);
    }

    fn brother_hit_player(&mut self, new_player_pos: Coord) {
        println!("Your brother hit you!");
        self.your_health -= 1;
        // move away
        self.forest.player_pos = new_player_pos;
        // reset the blast history
        self.blast_history.clear();
    }

    fn brother_hit_sister(&mut self) {
        println!("Your brother hit your sister and was frozen!");
        self.bro_expl_cnt -= EXPLOSIONS_LOST_ON_FREEZE;
        self.bombed_sis_pos = true;
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
            game.forest.is_tree_at(coord)
        )
    }

    fn pick_position_to_blast(&self) -> Coord {
        self.random_pos_conditioned(|game, coord|
            game.good_position_to_blast(coord)
        )
    }

    fn good_position_to_blast(&self, pos: &Coord) -> bool {
        if self.bombed_sis_pos && *pos == self.forest.sister_pos {
            return false;
        }
        !self.blast_history.contains(pos)
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
        let player_pos = Coord { x: 0, y: 0 };
        let sister_pos = Coord { x: 0, y: 1 };
        let game = GameState::new_customized(player_pos.clone(), sister_pos.clone());

        assert_eq!(player_pos, game.forest.player_pos);
        assert_eq!(sister_pos, game.forest.sister_pos);
    }

    /// Tests that the initial game state is as expected
    #[test]
    fn test_valid_initial_state() {
        let game = GameState::new();

        assert_eq!(START_HEALTH, game.your_health);
        assert_eq!(START_EXPLOSIONS, game.bro_expl_cnt);
        assert!(game.blast_history.is_empty());
        assert_eq!(false, game.bombed_sis_pos);
        assert_eq!(GameStatus::Running, game.status());
    }

    /// Tests the state of the game after bombing an empty spot
    #[test]
    fn test_bomb_empty_spot() {
        let player_pos = Coord { x: 1, y: 0 };
        let sister_pos = Coord { x: 2, y: 1 };
        let mut game = GameState::new_customized(player_pos.clone(), sister_pos.clone());
        let bomb_pos = Coord { x: 2, y: 3};

        game.perform_step_customized(bomb_pos.clone(), Coord::random());

        assert_eq!(GameStatus::Running, game.status());         // the game is still running
        assert_eq!(player_pos, game.forest.player_pos);           // player's position didn't change
        assert_eq!(sister_pos, game.forest.sister_pos);         // sister's position didn't change
        assert_eq!(START_HEALTH, game.your_health);             // player's health didn't change
        assert_eq!(START_EXPLOSIONS - 1, game.bro_expl_cnt);    // bro's expl. count has decreased
        assert_eq!(1, game.blast_history.len());                // bombed position history was updated
        assert_eq!(false, game.bombed_sis_pos);                 // sister's position wasn't discovered
    }

    /// Tests the state of the game after bombing the sister
    #[test]
    fn test_bomb_sister() {
        let player_pos = Coord { x: 1, y: 0 };
        let sister_pos = Coord { x: 2, y: 1 };
        let mut game = GameState::new_customized(player_pos.clone(), sister_pos.clone());

        game.perform_step_customized(sister_pos.clone(), Coord::random());

        let expected_expl_after = START_EXPLOSIONS - 1 - EXPLOSIONS_LOST_ON_FREEZE;
        assert_eq!(GameStatus::Running, game.status());         // the game is still running
        assert_eq!(player_pos, game.forest.player_pos);           // player's position didn't change
        assert_eq!(sister_pos, game.forest.sister_pos);         // sister's position didn't change
        assert_eq!(START_HEALTH, game.your_health);             // player's health didn't change
        assert_eq!(expected_expl_after, game.bro_expl_cnt);     // bro's expl. count has decreased
        assert_eq!(true, game.bombed_sis_pos);                  // sister's position WAS discovered
    }

    /// Tests the state of the game after bombing the player
    #[test]
    fn test_bomb_player() {
        let player_pos = Coord { x: 1, y: 0 };
        let sister_pos = Coord { x: 2, y: 1 };
        let new_pos = Coord { x: 3, y: 2 };
        let mut game = GameState::new_customized(player_pos.clone(), sister_pos.clone());

        game.perform_step_customized(player_pos.clone(), new_pos.clone());

        assert_eq!(GameStatus::Running, game.status());         // the game is still running
        assert_eq!(new_pos, game.forest.player_pos);            // player's position DID change
        assert_eq!(sister_pos, game.forest.sister_pos);         // sister's position didn't change
        assert_eq!(START_HEALTH - 1, game.your_health);         // player's health did change
        assert_eq!(START_EXPLOSIONS - 1, game.bro_expl_cnt);    // bro's expl. count has decreased
        assert_eq!(false, game.bombed_sis_pos);                 // sister's position wasn't discovered
    }

    /// Tests the state of the game after using the final bomb to blast the player's last life
    #[test]
    fn test_bomb_player_game_over_brother_won() {
        let player_pos = Coord { x: 1, y: 0 };
        let sister_pos = Coord { x: 2, y: 1 };
        let mut game = GameState::new_customized(player_pos.clone(), sister_pos.clone());
        game.your_health = 1;
        game.bro_expl_cnt = 1;

        game.perform_step_customized(player_pos.clone(), Coord::random());

        assert_eq!(GameStatus::BrotherWon, game.status());  // the game is over, your brother won
        assert_eq!(0, game.your_health);                    // your last HP was consumed
        assert_eq!(0, game.bro_expl_cnt);                   // bro's last explosion was consumed
    }

    /// Tests the state of the game after using the final bomb, and the player is still alive
    #[test]
    fn test_bomb_player_game_over_you_won() {
        let player_pos = Coord { x: 1, y: 0 };
        let sister_pos = Coord { x: 2, y: 1 };
        let mut game = GameState::new_customized(player_pos.clone(), sister_pos.clone());
        game.your_health = 2;
        game.bro_expl_cnt = 1;

        game.perform_step_customized(player_pos.clone(), Coord::random());

        assert_eq!(GameStatus::YouWon, game.status());  // the game is over, your brother won
        assert_eq!(1, game.your_health);                // your still have 1 HP
        assert_eq!(0, game.bro_expl_cnt);               // bro's last explosion was consumed
    }

    /// Tests that the blast history is updated when hitting an empty space
    #[test]
    fn test_blast_history_updated_after_empty_spot_is_hit() {
        let player_pos = Coord { x: 1, y: 0 };
        let sister_pos = Coord { x: 2, y: 1 };
        let mut game = GameState::new_customized(player_pos.clone(), sister_pos.clone());
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

    /// Tests that the blast history is NOT updated when hitting the sister
    #[test]
    fn test_blast_history_not_updated_after_sister_is_hit() {
        let player_pos = Coord { x: 1, y: 0 };
        let sister_pos = Coord { x: 2, y: 1 };
        let mut game = GameState::new_customized(player_pos.clone(), sister_pos.clone());

        game.perform_step_customized(sister_pos.clone(), Coord::random());

        assert!(game.blast_history.is_empty());
    }

    /// Tests that the blast history is cleared after hitting the player (and sister was not yet hit)
    #[test]
    fn test_blast_history_cleared_after_player_is_hit_and_sister_was_not_hit() {
        let player_pos = Coord { x: 1, y: 0 };
        let sister_pos = Coord { x: 2, y: 1 };
        let mut game = GameState::new_customized(player_pos.clone(), sister_pos.clone());
        // put some random items in history
        let hist_size_before = 3;
        for _ in 0..hist_size_before {
            game.blast_history.push(Coord::random());
        }

        game.perform_step_customized(player_pos.clone(), Coord::random());

        assert!(game.blast_history.is_empty());
    }

    /// Tests that any position is ok to blast in the beginning
    #[test]
    fn test_any_position_ok_to_blast_initially() {
        let player_pos = Coord { x: 1, y: 0 };
        let sister_pos = Coord { x: 2, y: 1 };
        let game = GameState::new_customized(player_pos.clone(), sister_pos.clone());

        for x in 0..GRID_SIZE_X {
            for y in 0..GRID_SIZE_Y {
                let pos = Coord { x, y };
                assert_eq!(true, game.good_position_to_blast(&pos));
            }
        }
    }

    /// Tests that a position is NOT ok to blast if already blasted
    #[test]
    fn test_not_ok_to_blast_position_already_in_history() {
        let player_pos = Coord { x: 1, y: 0 };
        let sister_pos = Coord { x: 2, y: 1 };
        let mut game = GameState::new_customized(player_pos.clone(), sister_pos.clone());
        let blast_pos = Coord { x: 3, y: 3 };

        game.perform_step_customized(blast_pos.clone(), Coord::random());

        assert_eq!(false, game.good_position_to_blast(&blast_pos));
    }

    /// Tests that sister position is NOT ok to blast even after history reset
    #[test]
    fn test_not_ok_to_blast_sister_if_already_blasted() {
        let player_pos = Coord { x: 1, y: 0 };
        let sister_pos = Coord { x: 2, y: 1 };
        let mut game = GameState::new_customized(player_pos.clone(), sister_pos.clone());

        // initially ok to blast sister
        assert_eq!(true, game.good_position_to_blast(&sister_pos));

        // blast the sister => no longer ok
        game.perform_step_customized(sister_pos.clone(), Coord::random());
        assert_eq!(false, game.good_position_to_blast(&sister_pos));

        // even if we clear the blast history (by blasting the player), sister still not ok to blast
        game.perform_step_customized(player_pos.clone(), Coord::random());
        assert_eq!(false, game.good_position_to_blast(&sister_pos));
    }

}
