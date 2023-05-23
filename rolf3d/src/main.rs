//! ROLF3D - a Rust implementation of the WOLF3D raycasting engine :)
//! Main starting point.

// This magic line prevents the opening of a terminal when launching a release build
#![cfg_attr(not(any(test, debug_assertions)), windows_subsystem = "windows")]

use rolf3d::*;

const SCR_WIDTH: i32 = 480;
const SCR_HEIGHT: i32 = 360;
const PIXEL_SIZE: i32 = 2;
const SLEEP_KIND: SleepKind = SleepKind::YIELD;

fn main() -> Result<(), String> {
    println!("Hello, ROLF3D !");

    // // build the game engine
    // let wad_data = WadData::load(WAD_PATH, true)?;
    // let cfg = GameConfig::new(wad_data, SCR_WIDTH, SCR_HEIGHT);
    // let mut doom_game = DoomGame::new(cfg)?;

    // // main game loop
    // let sdl_config = SdlConfiguration::new("RusTooM", SCR_WIDTH, SCR_HEIGHT, PIXEL_SIZE, SLEEP_KIND);
    // run_sdl_loop(&sdl_config, &mut doom_game)?;

    println!("ROLF3D finished OK :)");
    Ok(())
}
