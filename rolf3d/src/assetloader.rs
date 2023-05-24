//! Wolf3d/SOD asset loader
//! Handles maps and graphics (no sound), Huffman, Carmackization etc
//!
//! ## Some useful links:
//! * [GameMaps Format](https://moddingwiki.shikadi.net/wiki/GameMaps_Format)
//! * [Carmack Compression](https://moddingwiki.shikadi.net/wiki/Carmack_compression)
//!     * [cmp-carmackize.js](https://github.com/camoto-project/gamecompjs/blob/master/formats/cmp-carmackize.js)
//! * [Huffman Compression](https://moddingwiki.shikadi.net/wiki/Huffman_Compression)
//! * [id Software RLEW compression](https://moddingwiki.shikadi.net/wiki/Id_Software_RLEW_compression)

use std::{fs::File, io::Read};

pub struct GameAssets {
    game_type: &'static str,
    // TODO - maps, textures, sprites, graphics ...
}

impl GameAssets {
    pub fn load() -> Result<Self, String> {
        let game_type = detect_game_type()?;
        let assets = Self { game_type };

        // TODO implement this ...

        // done ok
        println!("[ROLF3D] Assets loaded ok: game={}", assets.game_type);
        Ok(assets)
    }
}

//----------------------
//  Internal stuff

/// All the supported asset file names.
const FILES: &[&'static str] = &["MAPHEAD", "GAMEMAPS", "VGADICT", "VGAHEAD", "VGAGRAPH", "VSWAP"];

/// All the supported asset file extensions.
const EXTENSIONS: &[&'static str] = &["WL1", "WL3", "WL6", "SDM", "SOD"];

/// Detect the game type, by checking if all asset files for each supported extension are found.
fn detect_game_type() -> Result<&'static str, String> {
    for ext in EXTENSIONS {
        if game_files_exist(*ext).is_ok() {
            return Ok(*ext);
        }
    }
    Err(String::from("Game asset files not found"))
}

/// Check if all asset files for a given extension are found.
fn game_files_exist(ext: &str) -> std::io::Result<()> {
    let mut buf = [0; 4];
    for file in FILES {
        let fname = format!("{}.{}", *file, ext);
        let mut f = File::open(&fname)?;
        f.read_exact(&mut buf)?;
    }
    Ok(())
}
