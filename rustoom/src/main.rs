//======================================================================================
//  DIY Doom Engine, written in Rust :)
//
// -> see Recreating DOOM on YouTube:
//          https://www.youtube.com/playlist?list=PLi77irUVkDasNAYQPr3N8nVcJLQAlANva
// -> see DIY Doom on GitHub:
//          https://github.com/amroibrahim/DIYDoom
//======================================================================================

// TODO - TEMPORARYLY disable warnings for dead code
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod game;
mod wad;
mod lump;
mod gfx;

fn main() -> Result<(), String> {
    let wad_path = "doom.wad";
    let wad_data = wad::WadData::load(wad_path, wad::WadKind::IWAD)?;
    let _doom_game = game::DoomGame::new(wad_data);

    println!("*** Doom game loaded ok ***");
    Ok(())
}
