//! DIY Doom Engine, written in Rust :)
//!
//! * see [Recreating DOOM on YouTube](https://www.youtube.com/playlist?list=PLi77irUVkDasNAYQPr3N8nVcJLQAlANva)
//! * see [DIY Doom on GitHub](https://github.com/amroibrahim/DIYDoom)

// This magic line prevents the opening of a terminal when launching the app
// TODO - FIGURE THIS OUT !!!
//#![cfg_attr(dev, windows_subsystem = "windows")]

// TODO - TEMPORARYLY disable warnings for dead code
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use rustoom::*;

/*
   TODO:
       - [DONE] load WAD
       - [next] figure out how to use SDL2 to DRAW BIG PIXELS in a FAST WAY !!!
       - [later] continue with DOOM DIY: https://github.com/amroibrahim/DIYDoom/tree/master/DIYDOOM/Notes002/notes
*/

const SCR_WIDTH: u32 = 320;
const SCR_HEIGHT: u32 = 240;
const PIX_SIZE: u32 = 3;

fn main() -> Result<(), String> {
    let wad_path = "doom.wad";
    let wad_data = WadData::load(wad_path, WadKind::IWAD)?;
    let doom_game = DoomGame::new(wad_data)?;

    println!("*** Doom game loaded ok ***");
    //test_sdl2()?;

    Ok(())
}
