//! Handler for WAD graphics + flats

use crate::utils::*;
use crate::wad::*;
use crate::*;
use std::{collections::HashMap, rc::Rc};

//-----------------
// TODO initial parsing of WAD data and building of Font, Palette, Graphics etc => move out
// (so that all elements are built during initial parse, then kept immutable)

pub struct GraphicsHandler {
    wad_data: Rc<WadData>,
    sprites: HashMap<u64, usize>,
    patches: HashMap<u64, usize>,
    flats: HashMap<u64, usize>,
    pub palette: Palette, // TODO temporary pub
    pub font: Font,       // TODO temporary pub
}

impl GraphicsHandler {
    pub fn new(wad_data: Rc<WadData>) -> Result<Self, String> {
        let mut g = GraphicsHandler {
            wad_data,
            sprites: HashMap::new(),
            patches: HashMap::new(),
            flats: HashMap::new(),
            palette: Palette::new(),
            font: Font::new(),
        };
        g.parse_and_collect_lump_data()?;
        g.validate_collected_lump_data()?;
        Ok(g)
    }

    pub fn get_sprite(&self, sprite_name: &str) -> Option<PixMap> {
        self.get_mapped_graphic(sprite_name, &self.sprites)
    }

    pub fn get_patch(&self, patch_name: &str) -> Option<PixMap> {
        self.get_mapped_graphic(patch_name, &self.patches)
    }

    pub fn get_flat(&self, flat_name: &str) -> Option<PixMap> {
        let k = hash_lump_name(flat_name);
        let lump_idx = self.flats.get(&k);
        if let Some(idx) = lump_idx {
            let lump = self.wad_data.get_lump(*idx);
            if let Ok(l) = lump {
                return Some(PixMap::from_flat(l.bytes));
            }
        }
        None
    }

    //-----------

    fn get_mapped_graphic(&self, name: &str, mapp: &HashMap<u64, usize>) -> Option<PixMap> {
        let k = hash_lump_name(name);
        let idx = mapp.get(&k);
        if let Some(i) = idx {
            let lump = self.wad_data.get_lump(*i);
            if let Ok(l) = lump {
                return self.get_lump_graphic(l.bytes);
            }
        }
        None
    }

    // interpret the graphic data from a lump, and convert it to pixmap
    fn get_lump_graphic(&self, bytes: &[u8]) -> Option<PixMap> {
        // TODO implement this
        None
    }

    fn parse_and_collect_lump_data(&mut self) -> Result<(), String> {
        let mut in_sprites = false;
        let mut in_patches = false;
        let mut in_flats = false;
        for lump_idx in 0..self.wad_data.get_lump_count() {
            let l = self.wad_data.get_lump(lump_idx)?;
            if l.name == "PLAYPAL" {
                self.palette.init_palettes(l.bytes);
            } else if l.name == "COLORMAP" {
                self.palette.init_colormaps(l.bytes);
            } else if is_texture_lump(l.name) {
                // TODO parse TEXTUREn
            } else if l.name == "PNAMES" {
                // TODO parse PNAMES (is this really needed ??)
            } else if l.name == "S_START" {
                in_sprites = true;
            } else if l.name == "S_END" {
                in_sprites = false;
            } else if l.name == "P_START" {
                in_patches = true;
            } else if l.name == "P_END" {
                in_patches = false;
            } else if l.name == "F_START" {
                in_flats = true;
            } else if l.name == "F_END" {
                in_flats = false;
            } else if in_sprites && l.bytes.len() > 0 {
                let k = hash_lump_name(l.name);
                self.sprites.insert(k, lump_idx);
            } else if in_patches && l.bytes.len() > 0 {
                let k = hash_lump_name(l.name);
                self.patches.insert(k, lump_idx);
            } else if in_flats && l.bytes.len() > 0 {
                let k = hash_lump_name(l.name);
                self.flats.insert(k, lump_idx);
            } else if l.bytes.len() > 0 {
                self.font
                    .try_add_font_lump(l.name, l.bytes, &self.palette)?;
            }
            // TODO standalone graphics ??
        }
        Ok(())
    }

    fn validate_collected_lump_data(&self) -> Result<(), String> {
        if !self.palette.is_palette_initialized() {
            Err(String::from("Pallete lump not found in WAD"))
        } else if !self.palette.is_colormap_initialized() {
            Err(String::from("Colormap lump not found in WAD"))
        } else if self.sprites.is_empty() {
            Err(String::from("Sprites not found in WAD"))
        } else if self.patches.is_empty() {
            Err(String::from("Patches not found in WAD"))
        } else if self.flats.is_empty() {
            Err(String::from("Flats not found in WAD"))
        } else if !self.font.is_complete() {
            Err(String::from("Fonts not found in WAD"))
        } else {
            Ok(())
        }
    }
}

//-----------------------
//  Internal utils

#[inline]
fn is_texture_lump(name: &str) -> bool {
    name.len() == 8 && &name[0..7] == "TEXTURE" && atoi(&name[7..]).is_some()
}
