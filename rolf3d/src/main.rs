//! ROLF3D - a Rust implementation of the WOLF3D raycasting engine :)
//! Main starting point.

// This magic line prevents the opening of a terminal when launching a release build
//#![cfg_attr(not(any(test, debug_assertions)), windows_subsystem = "windows")]

/*
    TODO - implementation steps for ROLF3D:
    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    * (IN PROGRESS) load maps and sketch them
        - just colored rectangles (for now)

    * load graphics assets and paint them
    * add display text (of any color)

    * Automap:
        - display walls/doors etc using actual graphics
        - display things using actual graphics
    * Movement on the Automap:
        - turn and strife
        - mouse horizontal turn
        - mouse buttons
        - collision detection (with walls)

    * 3D View / Raycaster
        - walls and doors
        - sprites (actors, decorations)
        - movement through the 3D world

    * Gameplay
        - key handling (e.g. Tab = Automap)
        - open doors !!
        - shoot enemies
    * Enemy AI
    * (Almost) Full Game:
        - NO sound :/
        - Menu system (may be simplified)

DONE:
    * basic painting via ScreenBuffer
    * load palette from GAMEPAL.OBJ + hardcode it + display it
*/

use rolf3d::*;

const SCR_WIDTH: i32 = 320;
const SCR_HEIGHT: i32 = 240;
const PIXEL_SIZE: i32 = 2;
const SLEEP_KIND: SleepKind = SleepKind::YIELD;

fn main() {
    // load and prepare game assets
    let _assets = GameAssets::load().expect("ERROR in ROLF3D: failed to load game assets");

    // main game loop
    let sdl_config = SdlConfiguration::new("ROLF3D", SCR_WIDTH, SCR_HEIGHT, PIXEL_SIZE, SLEEP_KIND);
    let mut gameloop = GameLoop::new(SCR_WIDTH as usize, SCR_HEIGHT as usize);
    let result = run_game_loop(&sdl_config, &mut gameloop);

    match result {
        Ok(_) => println!("ROLF3D finished OK :)"),
        Err(msg) => println!("ERROR in ROLF3D: {msg}"),
    }
}
