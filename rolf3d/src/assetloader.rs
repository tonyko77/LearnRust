//! Wolf3d/SOD asset loader
//! Handles maps and graphics (no sound), Huffman, de-Carmackization, RLEW etc.
//!
//! ## Some useful links:
//! * WIKI-s:
//!     * [Cool Wiki @ devinsmith.net](https://devinsmith.net/backups/bruce/wolf3d.html)
//!     * [GameMaps Format](https://moddingwiki.shikadi.net/wiki/GameMaps_Format)
//!     * [Carmack Compression](https://moddingwiki.shikadi.net/wiki/Carmack_compression)
//!     * [Huffman Compression](https://moddingwiki.shikadi.net/wiki/Huffman_Compression)
//!     * [id Software RLEW compression](https://moddingwiki.shikadi.net/wiki/Id_Software_RLEW_compression)
//! * [WOLF3D original sources on GitHub](https://github.com/id-Software/wolf3d)
//!     * [Maps - CAL_CarmackExpand](https://github.com/id-Software/wolf3d/blob/master/WOLFSRC/ID_CA.C#L609)
//!     * [VSWAP - PML_OpenPageFile](https://github.com/id-Software/wolf3d/blob/master/WOLFSRC/ID_PM.C#L500)

/*
Findings about file formats:
~~~~~~~~~~~~~~~~~~~~~~~~~~~

VSWAP -> see https://github.com/id-Software/wolf3d/blob/master/WOLFSRC/ID_PM.C#L500
    -> ALSO SEE!!! https://devinsmith.net/backups/bruce/wolf3d.html

    OFFS    SIZE                    Contents
    ------------------------------------------------------------------------------
    0       u16                     ChunksInFile = number of chunks in the file
    2       u16                     PMSpriteStart = !?
    4       u16                     PMSoundStart = !?
    6       ChunksInFile * u32      Offsets for each chunk (may be 0)
    ...     ChunksInFile * u16      Lengths for each chunk (may be 0)
    ...     ...                     Actual chunks - seem to be UNCOMPRESSED !?
*/

use crate::assets::*;
use crate::utils::*;

pub struct GameAssets {
    pub game_type: &'static str,
    maps: Vec<MapData>,
    walls: Vec<GfxData>,
    sprites: Vec<GfxData>,
    pics: Vec<GfxData>,
}

impl GameAssets {
    pub fn load() -> Result<Self, String> {
        let game_type = detect_game_type()?;
        let maps = load_maps(game_type)?;
        let (walls, sprites) = load_vswap(game_type)?;
        let pics = vec![]; // TODO load pics
        Ok(Self {
            game_type,
            maps,
            walls,
            sprites,
            pics,
        })
    }

    #[inline]
    pub fn map_count(&self) -> usize {
        self.maps.len()
    }

    #[inline]
    pub fn map(&self, idx: usize) -> &MapData {
        &self.maps[idx]
    }
}

//----------------------
//  Internal stuff

/// All the supported asset file extensions.
const EXTENSIONS: &[&'static str] = &["WL6", "SOD", "WL3", "WL1", "SDM"];

/// All the supported asset file names.
const FILES: &[&'static str] = &["MAPHEAD", "GAMEMAPS", "VGADICT", "VGAHEAD", "VGAGRAPH", "VSWAP"];
const MAPHEAD: usize = 0;
const GAMEMAPS: usize = 1;
// const VGADICT: usize = 2;
// const VGAHEAD: usize = 3;
// const VGAGRAPH: usize = 4;
const VSWAP: usize = 5;

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

//----------------------
// Page (VSWAP) loader

fn load_vswap(ext: &str) -> Result<(Vec<GfxData>, Vec<GfxData>), String> {
    let vswap = load_file(VSWAP, ext)?;
    // read the 3 counters
    let cnt_chunks_in_file = buf_to_u16(&vswap[0..2]) as usize;
    let idx_sprite_start = buf_to_u16(&vswap[2..4]) as usize;
    let idx_sound_start = buf_to_u16(&vswap[4..6]) as usize;
    // read the offsets to each chunk
    let mut idx = 6;
    let mut vec_offsets = Vec::with_capacity(cnt_chunks_in_file);
    for _ in 0..cnt_chunks_in_file {
        let ofs = buf_to_u32(&vswap[idx..]) as usize;
        vec_offsets.push(ofs);
        idx += 4;
    }
    // read the lengths for each chunk
    let mut vec_lengths = Vec::with_capacity(cnt_chunks_in_file);
    for _ in 0..cnt_chunks_in_file {
        let len = buf_to_u16(&vswap[idx..]) as usize;
        vec_lengths.push(len);
        idx += 2;
    }
    // first come the wall chunks
    let mut vec_walls = Vec::with_capacity(idx_sprite_start);
    for i in 0..idx_sprite_start {
        let ofs = vec_offsets[i];
        let len = vec_lengths[i];
        if ofs > 0 && len > 0 {
            // sanity check - all walls (flats) should be 64x64
            assert_eq!(64 * 64, len);
            // read the wall - it is stored as columns
            let pixels = vswap[ofs..ofs + len].iter().cloned().collect();
            vec_walls.push(GfxData::new_wall(pixels));
        } else {
            println!("[DBG] empty WALL at idx {i}");
            vec_walls.push(GfxData::new_wall(vec![]));
        }
    }
    // next come the sprites
    let mut vec_sprites = Vec::with_capacity(idx_sound_start - idx_sprite_start);
    for i in idx_sprite_start..idx_sound_start {
        let ofs = vec_offsets[i];
        let len = vec_lengths[i];
        if ofs > 0 && len > 0 {
            let pixels = parse_sprite(&vswap[ofs..ofs + len]);
            vec_sprites.push(GfxData::new_sprite(pixels));
        } else {
            println!("[DBG] empty SPRITE at idx {i}");
            vec_sprites.push(GfxData::new_sprite(vec![]));
        }
    }

    Ok((vec_walls, vec_sprites))
}

// TODO represent sprites more sparsely (do NOT fill with 0xFF pixels, up to 64x64)
fn parse_sprite(compressed: &[u8]) -> Vec<u8> {
    let mut pixels = vec![0xFF; 64 * 64];
    // the first 2 words = the left and right extents of the sprite
    let left_extent = buf_to_u16(&compressed[0..2]) as usize;
    let right_extent = buf_to_u16(&compressed[2..4]) as usize;
    // the next N words are the offsets for each column
    for x in left_extent..=right_extent {
        let ofsidx = 4 + 2 * (x - left_extent);
        let column_ofs = buf_to_u16(&compressed[ofsidx..]) as usize;
        let destidx = x * 64;
        // TODO - complicated layout of sprite columns !!!
        // -> see https://devinsmith.net/backups/bruce/wolf3d.html
    }

    pixels
}

//-------------
// map loader

fn load_maps(ext: &str) -> Result<Vec<MapData>, String> {
    // load files
    let maphead = load_file(MAPHEAD, ext)?;
    let gamemaps = load_file(GAMEMAPS, ext)?;
    let rlew_tag = buf_to_u16(&maphead[0..2]);
    // read each map
    let mut maps = vec![];
    let mut idx = 2;
    while (idx + 3) < maphead.len() {
        let mapidx = buf_to_i32(&maphead[idx..]);
        if mapidx <= 0 {
            break;
        }

        // ok to read map
        let map = load_one_map(mapidx as usize, &gamemaps, rlew_tag)?;
        maps.push(map);
        idx += 4;
    }

    println!("[ROLF3D] Loaded {} maps of type {}", maps.len(), ext);
    Ok(maps)
}

fn load_one_map(hdridx: usize, gamemaps: &[u8], rlew_tag: u16) -> Result<MapData, String> {
    // parse map header
    if gamemaps.len() <= (hdridx + 26) {
        return Err(format!("Invalid map header index: {hdridx}"));
    }

    // offsets and compressed lengths for each of the 3 planes
    // * ignore plane 3, it is always ZERO
    // * also ignore compressed lengths (encoded data also contains len of decoded data)
    let ofs_plane_1 = buf_to_i32(&gamemaps[hdridx..]);
    let ofs_plane_2 = buf_to_i32(&gamemaps[hdridx + 4..]);

    // map size and name
    let width = buf_to_u16(&gamemaps[hdridx + 18..]);
    let height = buf_to_u16(&gamemaps[hdridx + 20..]);
    let name = buf_to_ascii(&gamemaps[hdridx + 22..], 16);

    // parse each plane
    if ofs_plane_1 <= 0 || ofs_plane_2 <= 0 {
        return Err(format!("Missing plane in GAMEMAPS for {name} @ 0x{hdridx:04X}"));
    }
    let ofs1 = ofs_plane_1 as usize;
    let ofs2 = ofs_plane_2 as usize;
    let walls = decompress_map_plane(&gamemaps[ofs1..], rlew_tag);
    let things = decompress_map_plane(&gamemaps[ofs2..], rlew_tag);

    // return the map data
    Ok(MapData::new(name, width, height, walls, things))
}

/// Use Carmack and RLEW decompression, to extract the map plane
fn decompress_map_plane(chunk: &[u8], rlew_tag: u16) -> Vec<u16> {
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
