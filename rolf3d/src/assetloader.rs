//! Wolf3d/SOD asset loader
//! Handles maps and graphics (no sound), Huffman, Carmackization etc
//!
//! ## Some useful links:
//! * [GameMaps Format](https://moddingwiki.shikadi.net/wiki/GameMaps_Format)
//! * [Carmack Compression](https://moddingwiki.shikadi.net/wiki/Carmack_compression)
//!     * [cmp-carmackize.js](https://github.com/camoto-project/gamecompjs/blob/master/formats/cmp-carmackize.js)
//! * [Huffman Compression](https://moddingwiki.shikadi.net/wiki/Huffman_Compression)
//! * [id Software RLEW compression](https://moddingwiki.shikadi.net/wiki/Id_Software_RLEW_compression)

use crate::utils::*;

pub struct GameAssets {
    game_type: &'static str,
    // TODO - maps, textures, sprites, graphics ...
}

impl GameAssets {
    pub fn load() -> Result<Self, String> {
        let game_type = detect_game_type()?;
        let assets = Self { game_type };
        load_maps(game_type)?;

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
const MAPHEAD: usize = 0;
const GAMEMAPS: usize = 1;
// const VGADICT: usize = 2;
// const VGAHEAD: usize = 3;
// const VGAGRAPH: usize = 4;
// const VSWAP: usize = 5;

/// All the supported asset file extensions.
const EXTENSIONS: &[&'static str] = &["WL1", "WL3", "WL6", "SDM", "SOD"];

/// Detect the game type, by checking if all asset files for each supported extension are found.
fn detect_game_type() -> Result<&'static str, String> {
    for ext in EXTENSIONS {
        let all_game_files_exist = FILES.iter().all(|f| {
            let filename = format!("{}.{}", f, ext);
            file_exist(&filename)
        });
        if all_game_files_exist {
            return Ok(*ext);
        }
    }
    Err(String::from("Game asset files not found"))
}

//---------------------
// TODO map loader

fn load_maps(ext: &str) -> Result<(), String> {
    // load files
    let maphead = load_file(MAPHEAD, ext)?;
    let gamemaps = load_file(GAMEMAPS, ext)?;
    if buf_to_u16(&maphead[0..2]) != 0xABCD {
        return Err(format!("Unexpected magic number in "));
    }
    // read each map
    let mut idx = 2;
    loop {
        let mapidx = buf_to_i32(&maphead[idx..]);
        if mapidx <= 0 {
            break;
        }

        // ok to read map
        load_one_map(mapidx as usize, &gamemaps)?;
        idx += 4;
    }

    Ok(())
}

fn load_one_map(hdridx: usize, gamemaps: &[u8]) -> Result<(), String> {
    // parse map header
    if gamemaps.len() <= (hdridx + 26) {
        return Err(format!("Invalid map header index: {hdridx}"));
    }
    // offset for each plane
    let off_plane_0 = buf_to_i32(&gamemaps[hdridx..]);
    let off_plane_1 = buf_to_i32(&gamemaps[hdridx + 4..]);
    let off_plane_2 = buf_to_i32(&gamemaps[hdridx + 8..]);
    // lengths of (compressed) plane chunks
    let len_plane_0 = buf_to_u16(&gamemaps[hdridx + 12..]);
    let len_plane_1 = buf_to_u16(&gamemaps[hdridx + 14..]);
    let len_plane_2 = buf_to_u16(&gamemaps[hdridx + 16..]);
    // map size
    let map_width = buf_to_u16(&gamemaps[hdridx + 18..]);
    let map_height = buf_to_u16(&gamemaps[hdridx + 20..]);
    // internal map name
    let internal_name = buf_to_ascii(&gamemaps[hdridx + 22..], 16);

    // TODO implement this - use a map structure !!!
    // e.g. pub struct GameLevel { width, height, planes }
    println!("[MAP] TODO load map: {internal_name} {map_width}x{map_height}");
    println!(" -> @ ({off_plane_0}, {len_plane_0}), ({off_plane_1}, {len_plane_1}), ({off_plane_2}, {len_plane_2})");
    Ok(())
}

#[inline]
fn load_file(nameidx: usize, ext: &str) -> Result<Vec<u8>, String> {
    let fname = format!("{}.{}", FILES[nameidx], ext);
    read_file_to_bytes(&fname)
}
