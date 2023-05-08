//! Initialize the game data, by parsing the WAD contents.

use crate::utils::*;
use crate::*;
use bytes::Bytes;

pub struct DoomGameData {
    wad: WadData,
    pal: Palette,
    maps: Vec<MapData>,
    font: Font,
    gfx: Graphics,
}

impl DoomGameData {
    pub fn build(wad_data: WadData) -> Result<DoomGameData, String> {
        let wad = wad_data;
        let pal = Palette::new();
        let font = Font::new();
        let gfx = Graphics::new();

        let mut dgd = DoomGameData {
            wad,
            pal,
            maps: vec![],
            font,
            gfx,
        };

        dgd.parse_wad_lumps()?;
        dgd.validate_collected_data()?;
        Ok(dgd)
    }

    #[inline]
    pub fn palette(&self) -> &Palette {
        &self.pal
    }

    #[inline]
    pub fn map_count(&self) -> usize {
        self.maps.len()
    }

    #[inline]
    pub fn map(&self, idx: usize) -> &MapData {
        &self.maps[idx]
    }

    #[inline]
    pub fn font(&self) -> &Font {
        &self.font
    }

    #[inline]
    pub fn graphics(&self) -> &Graphics {
        &self.gfx
    }

    //-----------------

    fn parse_wad_lumps(&mut self) -> Result<(), String> {
        let mut is_flats = false;
        let mut pnames = Bytes::new();
        let mut textures = vec![];
        let mut parsing_map: Option<MapData> = None;

        // parse each lump
        for lump_idx in 0..self.wad.get_lump_count() {
            let lump = self.wad.get_lump(lump_idx)?;
            let has_bytes = lump.bytes.len() > 0;

            // parse map lumps
            if parsing_map.is_some() {
                let mut map = parsing_map.unwrap();
                let still_parsing = map.add_lump(&lump.name, lump.bytes.clone());
                if still_parsing {
                    parsing_map = Some(map);
                    continue;
                }
                // finished parsing one map
                parsing_map = None;
                if !map.is_complete() {
                    return Err(format!("Incomplete map in WAD: {}", map.name()));
                }
                self.maps.push(map);
            }
            if is_map_name(&lump.name) {
                // starting to parse new map
                parsing_map = Some(MapData::new(&lump.name));
                continue;
            }

            match lump.name.as_str() {
                "PLAYPAL" => self.pal.init_palettes(lump.bytes),
                "COLORMAP" => self.pal.init_colormaps(lump.bytes),
                "PNAMES" => pnames = lump.bytes.clone(),
                "F_START" => is_flats = true,
                "F_END" => is_flats = false,
                _ => {
                    if is_texture_name(&lump.name) {
                        textures.push(lump.bytes);
                    } else if has_bytes && is_flats {
                        self.gfx.add_flat(&lump.name, lump.bytes.clone());
                    } else if quick_check_if_lump_is_graphic(&lump.bytes) {
                        self.gfx.add_patch(&lump.name, lump.bytes.clone());
                        if is_font_name(&lump.name) {
                            self.font.add_font_lump(&lump.name, lump.bytes.clone());
                        }
                    }
                }
            }
        }

        // set up textures
        if self.pal.is_initialized() {
            self.font.compute_grayscale(&self.pal);
        }
        if pnames.is_empty() {
            return Err("PNAMES lump not found in WAD".to_string());
        }
        if textures.is_empty() {
            return Err("TEXTUREx lump(s) not found in WAD".to_string());
        }
        self.gfx.set_patch_names(&pnames)?;
        for tex_bytes in textures {
            self.gfx.add_textures(&tex_bytes)?;
        }

        Ok(())
    }

    fn validate_collected_data(&self) -> Result<(), String> {
        if !self.pal.is_initialized() {
            Err(String::from("PLAYPAL or COLORMAP lump not found in WAD"))
        } else if self.maps.len() == 0 {
            Err(String::from("Maps not found in WAD"))
        } else if !self.font.is_complete() {
            Err(String::from("Fonts not found in WAD"))
        } else {
            Ok(())
        }
    }
}

//-----------------------------
//  Internal utils

#[inline]
fn is_map_name(name: &str) -> bool {
    const E: u8 = 'E' as u8;
    const M: u8 = 'M' as u8;
    const A: u8 = 'A' as u8;
    const P: u8 = 'P' as u8;

    let b = name.as_bytes();
    if b.len() == 4 {
        b[0] == E && is_ascii_digit(b[1]) && b[2] == M && is_ascii_digit(b[3])
    } else if b.len() == 5 {
        b[0] == M && b[1] == A && b[2] == P && is_ascii_digit(b[3]) && is_ascii_digit(b[4])
    } else {
        false
    }
}

#[inline]
fn is_texture_name(name: &str) -> bool {
    name.len() == 8 && &name[0..7] == "TEXTURE" && is_ascii_digit(name.as_bytes()[7])
}

#[inline]
fn is_font_name(name: &str) -> bool {
    name.len() >= 7 && {
        let n5 = &name[0..5];
        (n5 == "STCFN") || (n5 == "FONTA")
    }
}

#[inline(always)]
fn is_ascii_digit(byte: u8) -> bool {
    byte >= ('0' as u8) && byte <= ('9' as u8)
}

// TODO improve check, to make sure nothing goes out of bounds ?!?
fn quick_check_if_lump_is_graphic(bytes: &[u8]) -> bool {
    let len = bytes.len();
    if len < 12 {
        return false;
    }

    // check that, for each column, its offset fits in the patch
    let mut max_idx = 0;
    let width = buf_to_u16(&bytes[0..=1]) as usize;
    let height = buf_to_u16(&bytes[2..=3]) as usize;
    if width == 0 || height == 0 || len < (8 + 4 * width) {
        return false;
    }
    for col in 0..width {
        let col_ofs = buf_to_u32(&bytes[8 + 4 * col..]) as usize;
        max_idx = max_idx.max(col_ofs);
        if len <= max_idx {
            return false;
        }
    }

    // check the column with the maximum offset
    loop {
        // if we went past the end of the lump bytes => NOT ok
        if max_idx >= len {
            return false;
        }
        // if we reached the end of column safely => we're ok
        if bytes[max_idx] == 0xFF {
            return true;
        }
        // skip the post
        if (max_idx + 3) >= len {
            return false;
        }
        let post_len = bytes[max_idx + 1] as usize;
        max_idx += post_len + 4;
    }
}
