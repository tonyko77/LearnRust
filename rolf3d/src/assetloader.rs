//! Wolf3d/SOD asset loader
//! Handles maps and graphics (no sound), Huffman, de-Carmackization, RLEW etc.
//!
//! ## Some useful links:
//! * [GameMaps Format](https://moddingwiki.shikadi.net/wiki/GameMaps_Format)
//! * [Carmack Compression](https://moddingwiki.shikadi.net/wiki/Carmack_compression)
//! * [Huffman Compression](https://moddingwiki.shikadi.net/wiki/Huffman_Compression)
//! * [id Software RLEW compression](https://moddingwiki.shikadi.net/wiki/Id_Software_RLEW_compression)
//! * [WOLF3D orig sources - CAL_CarmackExpand](https://github.com/id-Software/wolf3d/blob/master/WOLFSRC/ID_CA.C#L609)

use crate::utils::*;

// TODO move this outside, to a different mod ?!
// TODO remove pub from struct fields, where not needed !!!
pub struct MapData {
    pub name: String,
    pub width: u16,
    pub height: u16,

    // TODO how to interpret the map data?
    //  -> looks like each map has 64*64 = 4096 words, between 0x00 and 0xFF
    //      => it's JUST a 2D array :))
    //  -> plane #1 seems to contain WALLS, plane #2 seems to contain THINGS
    //  -> plane #3 seems to ALWAYS have 0-s => NOT USED ?!?, check SOD, WL6 etc
    pub walls: Vec<u16>,
    pub things: Vec<u16>,
}

pub struct GameAssets {
    pub game_type: &'static str,
    pub maps: Vec<MapData>,
    // TODO - maps, textures, sprites, graphics ...
}

impl GameAssets {
    pub fn load() -> Result<Self, String> {
        let game_type = detect_game_type()?;
        let maps = load_maps(game_type)?;
        Ok(Self { game_type, maps })
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

fn load_maps(ext: &str) -> Result<Vec<MapData>, String> {
    // load files
    let maphead = load_file(MAPHEAD, ext)?;
    let gamemaps = load_file(GAMEMAPS, ext)?;
    let rlew_tag = buf_to_u16(&maphead[0..2]);
    // read each map
    let mut maps = vec![];
    let mut idx = 2;
    loop {
        let mapidx = buf_to_i32(&maphead[idx..]);
        if mapidx <= 0 {
            break;
        }

        // ok to read map
        let map = load_one_map(mapidx as usize, &gamemaps, rlew_tag)?;
        maps.push(map);
        idx += 4;
    }

    Ok(maps)
}

fn load_one_map(hdridx: usize, gamemaps: &[u8], rlew_tag: u16) -> Result<MapData, String> {
    // parse map header
    if gamemaps.len() <= (hdridx + 26) {
        return Err(format!("Invalid map header index: {hdridx}"));
    }
    // offsets and compressed lengths for each plane
    let ofs_plane_1 = buf_to_i32(&gamemaps[hdridx..]);
    let ofs_plane_2 = buf_to_i32(&gamemaps[hdridx + 4..]);
    // ignore plane 3, it is always ZERO
    let len_plane_1 = buf_to_u16(&gamemaps[hdridx + 12..]) as usize;
    let len_plane_2 = buf_to_u16(&gamemaps[hdridx + 14..]) as usize;

    // map size and name
    let width = buf_to_u16(&gamemaps[hdridx + 18..]);
    let height = buf_to_u16(&gamemaps[hdridx + 20..]);
    let name = buf_to_ascii(&gamemaps[hdridx + 22..], 16);

    // parse each plane
    let walls = decompress_map_plane(ofs_plane_1, len_plane_1, gamemaps, rlew_tag);
    let things = decompress_map_plane(ofs_plane_2, len_plane_2, gamemaps, rlew_tag);

    // return the map data
    Ok(MapData {
        name,
        width,
        height,
        walls,
        things,
    })
}

/// Use Carmack and RLEW decompression, to extract the map plane
fn decompress_map_plane(ofs: i32, len: usize, gamemaps: &[u8], rlew_tag: u16) -> Vec<u16> {
    if ofs <= 0 {
        // TODO what if a plane is missing ??
        panic!("Missing plane");
    }

    // decompress map plane
    let ofs = ofs as usize;
    let chunk = &gamemaps[ofs..ofs + len];

    // first de-Carmack ...
    // first word of the Carmack-ed chunk is the decompressed length, in bytes
    let intermed_word_cnt = (buf_to_u16(chunk) / 2) as usize;
    let mut intermediate = Vec::with_capacity(intermed_word_cnt);
    let mut idx = 2;
    while intermediate.len() < intermed_word_cnt {
        let b1 = chunk[idx];
        let b2 = chunk[idx + 1];
        if (b2 == 0xA7 || b2 == 0xA8) && b1 == 0 {
            // Carmack-style escape sequence
            let b1 = chunk[idx + 2];
            let w = (b1 as u16) | ((b2 as u16) << 8);
            intermediate.push(w);
            idx += 3;
        } else if b2 == 0xA7 {
            // Carmack-style near pointer
            let offs = intermediate.len() - (chunk[idx + 2] as usize);
            idx += 3;
            for i in 0..b1 as usize {
                intermediate.push(intermediate[offs + i]);
            }
        } else if b2 == 0xA8 {
            // Carmack-style far pointer
            let offs = buf_to_u16(&chunk[idx + 2..]) as usize;
            idx += 4;
            for i in 0..b1 as usize {
                intermediate.push(intermediate[offs + i]);
            }
        } else {
            // normal word
            let val = (b1 as u16) | ((b2 as u16) << 8);
            intermediate.push(val);
            idx += 2;
        }
    }

    // ... and then decompress RLEW
    // first word of the RLEW-ed data is the decompressed length, in bytes
    let decoded_word_cnt = (intermediate[0] / 2) as usize;
    let mut plane = Vec::with_capacity(decoded_word_cnt);
    let mut idx = 1;
    while plane.len() < decoded_word_cnt {
        let next = intermediate[idx];
        if next == rlew_tag {
            // RLEW sequence
            let cnt = intermediate[idx + 1];
            let val = intermediate[idx + 2];
            idx += 3;
            for _ in 0..cnt {
                plane.push(val);
            }
        } else {
            // normal word
            idx += 1;
            plane.push(next);
        }
    }

    plane
}

#[inline]
fn load_file(nameidx: usize, ext: &str) -> Result<Vec<u8>, String> {
    let fname = format!("{}.{}", FILES[nameidx], ext);
    read_file_to_bytes(&fname)
}
