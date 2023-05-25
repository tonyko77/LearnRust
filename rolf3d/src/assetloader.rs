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

/// Holds all the assets loaded from the game files.
pub struct GameAssets {
    pub game_type: &'static str,
    pub maps: Vec<MapData>,
    pub walls: Vec<GfxData>,
    pub sprites: Vec<GfxData>,
    pub pics: Vec<GfxData>,
}

impl GameAssets {
    pub fn load() -> Result<Self, String> {
        // use a single, large buffer for all file loads
        // (just me, complicating things ...)
        let mut mutbuf = vec![0_u8; 2 * 1024 * 1024];

        // load all asset files
        let game_type = detect_game_type()?;
        let maps = load_maps(game_type, &mut mutbuf)?;
        let (walls, sprites) = load_vswap(game_type, &mut mutbuf)?;
        let pics = load_pics(game_type, &mut mutbuf)?;

        // build the asset holder
        Ok(Self {
            game_type,
            maps,
            walls,
            sprites,
            pics,
        })
    }
}

//----------------------
//  Internal stuff
//----------------------

/// All the supported asset file extensions.
const EXTENSIONS: &[&'static str] = &["WL6", "WL1", "SOD", "SDM"];

/// All the supported asset file names.
const FILES: &[&'static str] = &["MAPHEAD", "GAMEMAPS", "VGADICT", "VGAHEAD", "VGAGRAPH", "VSWAP"];
const MAPHEAD: usize = 0;
const GAMEMAPS: usize = 1;
const VGADICT: usize = 2;
const VGAHEAD: usize = 3;
const VGAGRAPH: usize = 4;
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
// Page loader (VSWAP)
//----------------------

fn load_vswap(ext: &str, mutbuf: &mut [u8]) -> Result<(Vec<GfxData>, Vec<GfxData>), String> {
    let vs_len = load_file(VSWAP, ext, mutbuf)?;
    let vswap = &mutbuf[0..vs_len];
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
    let mut cnt = 0;
    for i in 0..idx_sprite_start {
        let ofs = vec_offsets[i];
        let len = vec_lengths[i];
        if ofs > 0 && len > 0 {
            // sanity check - all walls (flats) should be 64x64
            assert_eq!(64 * 64, len);
            // read the wall - it is stored as columns
            let pixels = vswap[ofs..ofs + len].iter().cloned().collect();
            vec_walls.push(GfxData::new_wall(pixels));
            cnt += 1;
        } else {
            vec_walls.push(GfxData::new_wall(vec![]));
        }
    }
    println!("[ROLF3D] Loaded {cnt}/{} wall flats", vec_walls.len());

    // next come the sprites
    cnt = 0;
    let mut vec_sprites = Vec::with_capacity(idx_sound_start - idx_sprite_start);
    for i in idx_sprite_start..idx_sound_start {
        let ofs = vec_offsets[i];
        let len = vec_lengths[i];
        if ofs > 0 && len > 0 {
            let pixels = parse_sprite(&vswap[ofs..ofs + len]);
            vec_sprites.push(GfxData::new_sprite(pixels));
            cnt += 1;
        } else {
            vec_sprites.push(GfxData::new_sprite(vec![]));
        }
    }
    println!("[ROLF3D] Loaded {cnt}/{} sprites", vec_sprites.len());

    Ok((vec_walls, vec_sprites))
}

fn parse_sprite(compressed: &[u8]) -> Vec<u8> {
    let mut pixels = vec![0xFF; 64 * 64];
    // the first 2 words = the left and right extents of the sprite
    let left_extent = buf_to_u16(&compressed[0..2]) as usize;
    let right_extent = buf_to_u16(&compressed[2..4]) as usize;

    // the next N words are the offsets for each column,
    // then come the textels packed together (one byte each),
    // and then come the "commands" for each column
    // (one word each, zero-terminated for each sub-column)

    // moving index into the offsets to the command area for each column
    let mut ofsidx = 4;
    // moving index into the texel area
    let mut texidx = 4 + 2 * (right_extent - left_extent + 1);

    // compute texels for each column
    // -> see https://devinsmith.net/backups/bruce/wolf3d.html
    for x in left_extent..=right_extent {
        // offset to the column start, into the destination vector
        let destidx = x * 64;
        // read the offset into the command area for this column
        let mut column_ofs = buf_to_u16(&compressed[ofsidx..]) as usize;
        ofsidx += 2;
        // keep reading commands for the column
        // each command is 3 words: end_y * 2, ignored, start_y * 2
        loop {
            let end_y = (buf_to_u16(&compressed[column_ofs..]) / 2) as usize;
            if end_y == 0 {
                break;
            }
            let start_y = (buf_to_u16(&compressed[column_ofs + 4..]) / 2) as usize;
            column_ofs += 6;
            for y in start_y..end_y {
                let tex = compressed[texidx];
                assert_ne!(0xFF, tex);
                pixels[destidx + y] = tex;
                texidx += 1;
            }
        }
    }

    pixels
}

//---------------------------------
// Map loader - MAPHEAD, GAMEMAPS
//---------------------------------

fn load_maps(ext: &str, mutbuf: &mut [u8]) -> Result<Vec<MapData>, String> {
    // load files
    let mh_len = load_file(MAPHEAD, ext, mutbuf)?;
    let gm_len = load_file(GAMEMAPS, ext, &mut mutbuf[mh_len..])?;
    let maphead = &mutbuf[0..mh_len];
    let gamemaps = &mutbuf[mh_len..mh_len + gm_len];
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

//--------------------------------------------
// Pic loader - VGADICT, VGAHEAD, VGAGRAPH
//--------------------------------------------

fn load_pics(ext: &str, mutbuf: &mut [u8]) -> Result<Vec<GfxData>, String> {
    // load the 3 files ...
    let len1 = load_file(VGADICT, ext, mutbuf)?;
    assert_eq!(1024, len1);
    let len2 = load_file(VGAHEAD, ext, &mut mutbuf[len1..])?;
    let len3 = load_file(VGAGRAPH, ext, &mut mutbuf[len1 + len2..])?;
    // ... then split into buffers ...
    let vgadict = &mutbuf[0..len1];
    let vgahead = &mutbuf[len1..len1 + len2];
    let vgagraph = &mutbuf[len1 + len2..len1 + len2 + len3];
    // ... and prepare the results vector
    let cnt_chunks = (len2 / 3) - 1;
    assert_eq!(cnt_chunks * 3 + 3, len2);

    // the VGADICT file is an array of WORD pairs - first for bit=0, second for bit=1:
    //     struct huffnode { unsigned bit0, bit1; }  --> 0-255 is a character, > is a pointer to a node
    //     (bitx values > 266 => are indexes within this table, just subtract 256 from them)
    // we just collect them as a list of words, and each pair of words corresponds to (bit0, bit1)
    let mut huffnodes = Vec::with_capacity(len1 / 2);
    for i in 0..len1 / 2 {
        huffnodes.push(buf_to_u16(&vgadict[2 * i..]));
    }
    assert_eq!(512, huffnodes.len());

    // the VGAHEAD file is an array of 3-byte, little endian offsets into VGAGRAPH
    // the first is 0, the last one is the offset of the end of the VGAGRAPH file
    // we just collect them as a list of offsets
    let mut offsets = Vec::with_capacity(1 + cnt_chunks);
    for i in 0..=cnt_chunks {
        let b1 = vgahead[3 * i] as usize;
        let b2 = vgahead[3 * i + 1] as usize;
        let b3 = vgahead[3 * i + 2] as usize;
        let ofs = b1 | (b2 << 8) | (b3 << 16);
        offsets.push(ofs);
    }

    // finally, VGAGRAPH contains the Huffman-encoded data for each pic
    // BUT: pictures start at index 3, and their indexes are GAME SPECIFIC
    // (see GFXV_xxx.H/EQU files in the original sources)

    // Chunk #0 (STRUCTPIC) contains the pic dimensions
    // It contains NUMPIC entries; for each entry, there are 2 words: width, height
    let o1 = offsets[0];
    let o2 = offsets[1];
    let bytes = &vgagraph[o1..o2];
    let decoded = huff_decode_chunk(bytes, &huffnodes);
    let cnt_words = decoded.len() / 2;
    let mut pic_sizes = Vec::with_capacity(cnt_words);
    for i in 0..cnt_words {
        let w = buf_to_u16(&decoded[2 * i..]);
        pic_sizes.push(w);
    }

    // Next, try to decode each pic
    let cnt_pics = cnt_words / 2;
    let mut pics = Vec::with_capacity(cnt_pics);
    let mut cnt = 0;
    for i in 0..cnt_pics {
        let width = pic_sizes[2 * i];
        let height = pic_sizes[2 * i + 1];
        let o1 = offsets[i + 1];
        let o2 = offsets[i + 2];
        let bytes = &vgagraph[o1..o2];
        let pixels = huff_decode_chunk(bytes, &huffnodes);

        // TODO - NOT WORKING - size mismatches in the PICs !?!?!
        // ALSO: pics are "mangled :(("
        // => HOW TO DECODE A PIC ?????
        let expected_len = (width as usize) * (height as usize);
        if expected_len == pixels.len() {
            let pic = GfxData::new_pic(width, height, pixels);
            pics.push(pic);
            cnt += 1;
        } else {
            println!(
                "[WARN] PIC size mismatch: w={width}, h={height} => exp.len={expected_len}, but actual len={}",
                pixels.len()
            );
        }
    }
    println!("[ROLF3D] Loaded {cnt}/{cnt_pics} pics");

    Ok(pics)
}

/// Huffman decoding for pictures
/// -> see [CAL_SetupGrFile](https://github.com/id-Software/wolf3d/blob/master/WOLFSRC/ID_CA.C#L872)
/// and [CAL_HuffExpand](https://github.com/id-Software/wolf3d/blob/master/WOLFSRC/ID_CA.C#L409)
fn huff_decode_chunk(bytes: &[u8], huff_dict: &[u16]) -> Vec<u8> {
    // read the decoded size (4 bytes)
    let decoded_size = buf_to_u32(bytes) as usize;
    let mut decoded = Vec::with_capacity(decoded_size);

    // ok to decode data
    // TODO (!!) there is a "screen hack" in the original source
    let mut src_idx = 4;
    // root node of Huffman tree is always at index 254 (and 255 is unused)
    let mut huff_idx = 254;
    let mut current_byte = bytes[src_idx];
    let mut bit_mask = 1;
    while decoded.len() < decoded_size && src_idx < bytes.len() {
        // check current bit
        let is_zero_bit = (current_byte & bit_mask) == 0;
        let huff_target = if is_zero_bit {
            huff_dict[2 * huff_idx]
        } else {
            huff_dict[2 * huff_idx + 1]
        };
        // advance to next bit / byte
        if bit_mask == 0x80 {
            bit_mask = 1;
            src_idx += 1;
            current_byte = bytes[src_idx];
        } else {
            bit_mask = bit_mask << 1;
        }
        // act based on huff data
        if huff_target < 256 {
            // it's a byte value
            decoded.push((huff_target & 0xFF) as u8);
            huff_idx = 254;
        } else {
            // it's an index to another node
            huff_idx = (huff_target & 0xFF) as usize;
        }
    }

    assert_eq!(decoded_size, decoded.len());
    decoded
}

//--------------
//  Misc ...
//--------------

#[inline]
fn load_file(nameidx: usize, ext: &str, outbuf: &mut [u8]) -> Result<usize, String> {
    let fname = format!("{}.{}", FILES[nameidx], ext);
    read_file_to_bytes(&fname, outbuf)
}
