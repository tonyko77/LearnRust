//! Initialize the game data, by parsing the WAD contents.

/*
TODO:
    - implement THIS initializer
    - refactor/finish GFX handler + RENAME it !!!
    - improve the for-testing game loop:
        - swap between: automap, display graphics / flats / sprites
    - doc comments !!
 */

use crate::{Font, Graphics, MapManager, Palette, Textures, WadData};
use std::rc::Rc;

pub struct DoomGameData {
    wad: Rc<WadData>,
    pal: Palette,
    maps: MapManager,
    font: Font,
    gfx: Graphics, // TODO rename - to Graphics? GfxHolder??
    textures: Textures,
}

impl DoomGameData {
    pub fn build(wad_data: WadData) -> Result<DoomGameData, String> {
        let wad = Rc::from(wad_data);
        let pal = Palette::new();
        let maps = MapManager::new(&wad);
        let font = Font::new();
        let gfx = Graphics::new(&wad);
        let textures = Textures::new();

        let mut dgd = DoomGameData {
            wad,
            pal,
            maps,
            font,
            gfx,
            textures,
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
    pub fn maps(&self) -> &MapManager {
        &self.maps
    }

    #[inline]
    pub fn font(&self) -> &Font {
        &self.font
    }

    #[inline]
    pub fn graphics(&self) -> &Graphics {
        &self.gfx
    }

    #[inline]
    pub fn textures(&self) -> &Textures {
        &self.textures
    }

    //-----------------

    fn parse_wad_lumps(&mut self) -> Result<(), String> {
        let mut is_flats = false;

        for lump_idx in 0..self.wad.get_lump_count() {
            let l = self.wad.get_lump(lump_idx)?;
            let has_bytes = l.bytes.len() > 0;

            if !is_valid_lump_name(l.name) {
                return Err(format!("Invalid lump name at index {lump_idx}"));
            }

            match l.name {
                "PLAYPAL" => self.pal.init_palettes(l.bytes),
                "COLORMAP" => self.pal.init_colormaps(l.bytes),
                "PNAMES" => self.textures.parse_patch_names(l.bytes)?,
                "F_START" => is_flats = true,
                "F_END" => is_flats = false,
                _ => {}
            }

            if is_texture_name(l.name) {
                self.textures.parse_textures(l.bytes)?;
            } else if is_map_name(l.name) {
                self.maps.add_map(lump_idx);
            } else if has_bytes && is_flats {
                self.gfx.add_flat(l.name, lump_idx);
            } else if quick_check_if_lump_is_graphic(l.bytes) {
                self.gfx.add_patch(l.name, lump_idx);
                if is_font_name(l.name) {
                    self.font.add_font_lump(l.name, l.bytes, &self.pal)?;
                }
            }
        }
        Ok(())
    }

    fn validate_collected_data(&self) -> Result<(), String> {
        if !self.pal.is_palette_initialized() {
            Err(String::from("Pallete lump not found in WAD"))
        } else if !self.pal.is_colormap_initialized() {
            Err(String::from("Colormap lump not found in WAD"))
        } else if self.maps.get_map_count() == 0 {
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
fn is_valid_lump_name(name: &str) -> bool {
    name.bytes().all(|b| b > 32 && b < 127)
}

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

fn quick_check_if_lump_is_graphic(bytes: &[u8]) -> bool {
    // TODO implement this !!!
    true
}
