//! Handler for WAD graphics + flats

use crate::utils::*;
use crate::wad::*;
use std::{collections::HashMap, rc::Rc};

pub struct Graphic {}

pub struct GraphicsHandler {
    wad_data: Rc<WadData>,
    sprites: HashMap<u64, usize>,
    patches: HashMap<u64, usize>,
    flats: HashMap<u64, usize>,
    font: Vec<usize>,
    pal_lump_idx: usize,
    pal_cnt: usize,
    pal_selection: usize,
    cmap_lump_idx: usize,
    cmap_cnt: usize,
    cmap_selection: usize,
}

impl GraphicsHandler {
    pub fn new(wad_data: Rc<WadData>) -> Result<Self, String> {
        let mut g = GraphicsHandler {
            wad_data,
            sprites: HashMap::new(),
            patches: HashMap::new(),
            flats: HashMap::new(),
            font: vec![],
            pal_lump_idx: 0,
            pal_cnt: 0,
            pal_selection: 0,
            cmap_lump_idx: 0,
            cmap_cnt: 0,
            cmap_selection: 0,
        };
        g.parse_and_collect_lump_data()?;
        g.validate_collected_lump_data()?;
        Ok(g)
    }

    //-----------

    fn parse_and_collect_lump_data(&mut self) -> Result<(), String> {
        let mut in_sprites = false;
        let mut in_patches = false;
        let mut in_flats = false;
        let mut fontchars = HashMap::<i32, usize>::new();

        for lump_idx in 0..self.wad_data.get_lump_count() {
            let l = self.wad_data.get_lump(lump_idx)?;
            if l.name == "PLAYPAL" {
                self.pal_lump_idx = lump_idx;
                self.pal_cnt = (l.bytes.len() >> 8) / 3;
            } else if l.name == "COLORMAP" {
                self.pal_lump_idx = lump_idx;
                self.pal_cnt = (l.bytes.len() >> 8) / 3;
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
                validate_graphics_lump(l.bytes)?;
                let k = hash_lump_name(l.name);
                self.sprites.insert(k, lump_idx);
            } else if in_patches && l.bytes.len() > 0 {
                validate_graphics_lump(l.bytes)?;
                let k = hash_lump_name(l.name);
                self.patches.insert(k, lump_idx);
            } else if in_flats && l.bytes.len() > 0 {
                let k = hash_lump_name(l.name);
                self.flats.insert(k, lump_idx);
            } else if l.bytes.len() > 0 {
                if let Some(code) = get_font_code_from_name(l.name) {
                    fontchars.insert(code, lump_idx);
                }
            }
            // TODO standalone graphics ??
        }
        // TODO parse and validate font map
        Ok(())
    }

    fn validate_collected_lump_data(&self) -> Result<(), String> {
        if self.pal_cnt == 0 {
            Err(String::from("Pallete lump not found in WAD"))
        } else if self.cmap_cnt == 0 {
            Err(String::from("Colormap lump not found in WAD"))
        } else if self.sprites.is_empty() {
            Err(String::from("Sprites not found in WAD"))
        } else if self.patches.is_empty() {
            Err(String::from("Patches not found in WAD"))
        } else if self.flats.is_empty() {
            Err(String::from("Flats not found in WAD"))
        } else if self.font.is_empty() {
            Err(String::from("Fonts not found in WAD"))
        } else {
            Ok(())
        }
    }
}

//-----------------------
//  Internal utils

fn is_texture_lump(name: &str) -> bool {
    // TODO check for TEXTURE1 .. TEXTURE9
    name == "TEXTURE1" || name == "TEXTURE2"
}

fn validate_graphics_lump(_bytes: &[u8]) -> Result<(), String> {
    // TODO implement this !!!
    Ok(())
}

fn get_font_code_from_name(_name: &str) -> Option<i32> {
    // TODO implement this !!!
    None
}
