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

// TODO move this outside, to a different mod
struct MapData {
    _name: String,
    _width: u16,
    _height: u16,
    // TODO how to interpret the planes ???
    _plane0: Vec<u16>,
    _plane1: Vec<u16>,
    _plane2: Vec<u16>,
}

fn load_maps(ext: &str) -> Result<Vec<MapData>, String> {
    // load files
    let maphead = load_file(MAPHEAD, ext)?;
    let gamemaps = load_file(GAMEMAPS, ext)?;
    if buf_to_u16(&maphead[0..2]) != 0xABCD {
        return Err(format!("Unexpected magic number in "));
    }
    // read each map
    let mut maps = vec![];
    let mut idx = 2;
    loop {
        let mapidx = buf_to_i32(&maphead[idx..]);
        if mapidx <= 0 {
            break;
        }

        // ok to read map
        let map = load_one_map(mapidx as usize, &gamemaps)?;
        maps.push(map);
        idx += 4;
    }

    Ok(maps)
}

fn load_one_map(hdridx: usize, gamemaps: &[u8]) -> Result<MapData, String> {
    // parse map header
    if gamemaps.len() <= (hdridx + 26) {
        return Err(format!("Invalid map header index: {hdridx}"));
    }
    // offsets for each plane
    let off_planes = [
        buf_to_i32(&gamemaps[hdridx..]),
        buf_to_i32(&gamemaps[hdridx + 4..]),
        buf_to_i32(&gamemaps[hdridx + 8..]),
    ];
    // lengths of (compressed) plane chunks
    let len_planes = [
        buf_to_u16(&gamemaps[hdridx + 12..]),
        buf_to_u16(&gamemaps[hdridx + 14..]),
        buf_to_u16(&gamemaps[hdridx + 16..]),
    ];
    // map size
    let width = buf_to_u16(&gamemaps[hdridx + 18..]);
    let height = buf_to_u16(&gamemaps[hdridx + 20..]);
    // internal map name
    let name = buf_to_ascii(&gamemaps[hdridx + 22..], 16);

    // TODO implement this - use a map structure !!!
    // e.g. pub struct GameLevel { width, height, planes }
    println!("[MAP] TODO load map: {name} -> {width}x{height}");

    // parse each plane
    let mut planes = vec![vec![], vec![], vec![]];
    for i in 0..=2 {
        let ofs = off_planes[i];
        if ofs <= 0 {
            // TODO what if a plane is missing ??
            println!("[MAP] Missing plane {i} for {name}");
            continue;
        }
        let ofs = ofs as usize;
        let len = len_planes[i] as usize;
        if gamemaps.len() < (ofs + len) {
            return Err(format!("Invalid map plane {i} for {name}"));
        }
        // decompress map plane
        planes[i] = decompress_map_plane(&gamemaps[ofs..ofs + len]);
        // TODO how to interpret the map data ??
        println!(
            "[MAP] {name} => plane {i} has {} words (compressed len = {})",
            planes[i].len(),
            len
        );
    }

    // return the map data
    let plane2 = planes.pop().unwrap();
    let plane1 = planes.pop().unwrap();
    let plane0 = planes.pop().unwrap();
    Ok(MapData {
        _name: name,
        _width: width,
        _height: height,
        _plane0: plane0,
        _plane1: plane1,
        _plane2: plane2,
    })
}

/// Use RLE + Carmack decompression, to extract the map plane
fn decompress_map_plane(chunk: &[u8]) -> Vec<u16> {
    let len = chunk.len();
    let mut plane = Vec::with_capacity(1024);
    let mut idx = 0;
    // decode the words
    while (idx + 1) < len {
        let b1 = chunk[idx];
        let b2 = chunk[idx + 1];
        if (b2 == 0xA7 || b2 == 0xA8) && b1 == 0 {
            // Carmack-style escape sequence
            let b1 = chunk[idx + 2];
            let w = (b1 as u16) | ((b2 as u16) << 8);
            plane.push(w);
            idx += 3;
        } else if b2 == 0xA7 {
            // Carmack-style near pointer
            let offs = plane.len() - (chunk[idx + 2] as usize);
            idx += 3;
            for i in 0..b1 as usize {
                plane.push(plane[offs + i]);
            }
        } else if b2 == 0xA8 {
            // Carmack-style far pointer
            let offs = buf_to_u16(&chunk[idx + 2..]) as usize;
            idx += 4;
            for i in 0..b1 as usize {
                plane.push(plane[offs + i]);
            }
        } else if b1 == 0xCD && b2 == 0xAB {
            // RLE encoding
            let cnt = buf_to_u16(&chunk[idx + 2..]);
            let val = buf_to_u16(&chunk[idx + 4..]);
            idx += 6;
            for _ in 0..cnt {
                plane.push(val);
            }
        } else {
            // normal word
            let val = (b1 as u16) | ((b2 as u16) << 8);
            plane.push(val);
            idx += 2;
        }
    }
    plane
}

#[inline]
fn load_file(nameidx: usize, ext: &str) -> Result<Vec<u8>, String> {
    let fname = format!("{}.{}", FILES[nameidx], ext);
    read_file_to_bytes(&fname)
}
