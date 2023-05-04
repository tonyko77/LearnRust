//! DIY Doom Engine, written in Rust :)
//!
//! * see [Recreating DOOM on YouTube](https://www.youtube.com/playlist?list=PLi77irUVkDasNAYQPr3N8nVcJLQAlANva)
//! * see [DIY Doom on GitHub](https://github.com/amroibrahim/DIYDoom)

// This magic line prevents the opening of a terminal when launching a release build
#![cfg_attr(not(any(test, debug_assertions)), windows_subsystem = "windows")]

/*
   TODO:
       - [DONE] load WAD
       - [next] figure out how to use SDL2 to DRAW BIG PIXELS in a FAST WAY !!!
       - [later] continue with DOOM DIY: https://github.com/amroibrahim/DIYDoom/tree/master/DIYDOOM/Notes002/notes
*/

use rustoom::*;

const SCR_WIDTH: i32 = 400;
const SCR_HEIGHT: i32 = 300;
const PIXEL_SIZE: i32 = 2;

const SLEEP_KIND: SleepKind = SleepKind::YIELD;
const WAD_PATH: &str = "DOOM1.WAD";


fn main() -> Result<(), String> {


    // load the wad
    let wad_data = WadData::load(WAD_PATH, WadKind::IWAD)?;
    println!("*** WAD loaded ok: {WAD_PATH} ***");

    // build the game engine
    let mut doom_game = DoomGame::new(wad_data, SCR_WIDTH, SCR_HEIGHT);

    // main game loop
    let sdl_config = SdlConfiguration::new("RusTooM", SCR_WIDTH, SCR_HEIGHT, PIXEL_SIZE, SLEEP_KIND);
    let res = run_sdl_loop(&sdl_config, &mut doom_game);
    if let Err(msg) = res {
        println!("ERROR: {msg}");
    } else {
        println!("RusTooM finished OK :)");
    }

    Ok(())
}
