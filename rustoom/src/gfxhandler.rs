//! Handler for WAD graphics + flats

use crate::painter::*;
use crate::utils::*;
use crate::wad::*;
use std::{collections::HashMap, rc::Rc};

pub struct Graphic {
    width: u16,
    height: u16,
    left_offs: i16,
    top_offs: i16,
    pixels: Vec<u8>,
}

// impl Graphic {
//     // TODO ...
// }

//-----------------

pub struct GraphicsHandler {
    wad_data: Rc<WadData>,
    sprites: HashMap<u64, usize>,
    patches: HashMap<u64, usize>,
    flats: HashMap<u64, usize>,
    font: Vec<usize>,
    default_ch_idx: usize,
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
            font: Vec::with_capacity(64),
            default_ch_idx: 0,
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

    pub fn select_palette(&mut self, palette_idx: usize) {
        assert!(palette_idx < self.pal_cnt);
        self.pal_selection = palette_idx;
    }

    pub fn select_colormap(&mut self, cmap_idx: usize) {
        assert!(cmap_idx < self.cmap_cnt);
        self.cmap_selection = cmap_idx;
    }

    pub fn color2rgb(&self, color_code: u8) -> RGB {
        // from the selected colormap, pick the pallette index
        let col_lump = self
            .wad_data
            .get_lump(self.cmap_lump_idx)
            .expect("Bad COLORMAP lump index");
        let col_byte_idx = self.cmap_selection << 8 + (color_code as usize);
        let pallete_item = col_lump.bytes[col_byte_idx] as usize;
        // from the selected palette, pick r/g/b
        let pal_lump = self
            .wad_data
            .get_lump(self.pal_lump_idx)
            .expect("Bad PLAYPAL lump index");
        let pallete_idx = self.pal_selection * 768 + pallete_item * 3;
        let r = pal_lump.bytes[pallete_idx + 0];
        let g = pal_lump.bytes[pallete_idx + 1];
        let b = pal_lump.bytes[pallete_idx + 2];
        RGB::from(r, g, b)
    }

    pub fn get_sprite(&self, sprite_name: &str) -> Graphic {
        self.get_graphic(sprite_name, &self.sprites)
    }

    pub fn get_patch(&self, patch_name: &str) -> Graphic {
        self.get_graphic(patch_name, &self.patches)
    }

    pub fn get_flat(&self, flat_name: &str) -> Graphic {
        let k = hash_lump_name(flat_name);
        let lump_idx = self.flats.get(&k);
        if let Some(idx) = lump_idx {
            let lump = self.wad_data.get_lump(*idx);
            if let Ok(l) = lump {
                return Graphic {
                    width: 64,
                    height: 64,
                    left_offs: 0,
                    top_offs: 0,
                    pixels: l.bytes.to_vec(),
                };
            }
        }
        create_pink_placeholder(64, 64)
    }

    //-----------

    fn get_graphic(&self, _sprite_name: &str, _mapp: &HashMap<u64, usize>) -> Graphic {
        // TODO implement this !!!
        create_pink_placeholder(4, 4)
    }

    fn parse_and_collect_lump_data(&mut self) -> Result<(), String> {
        let mut in_sprites = false;
        let mut in_patches = false;
        let mut in_flats = false;
        let mut fontchars = HashMap::<u32, usize>::new();

        for lump_idx in 0..self.wad_data.get_lump_count() {
            let l = self.wad_data.get_lump(lump_idx)?;
            if l.name == "PLAYPAL" {
                self.pal_lump_idx = lump_idx;
                self.pal_cnt = (l.bytes.len() >> 8) / 3;
            } else if l.name == "COLORMAP" {
                self.cmap_lump_idx = lump_idx;
                self.cmap_cnt = (l.bytes.len() >> 8) / 3;
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
                if let Some(code) = font_code_from_name(l.name) {
                    fontchars.insert(code, lump_idx);
                }
            }
            // TODO standalone graphics ??
        }
        // parse and validate font map
        for ch in 33..96 {
            match fontchars.get(&ch) {
                Some(idx) => {
                    if ch == 33 {
                        self.default_ch_idx = *idx;
                    }
                    self.font.push(*idx);
                }
                None => {
                    if ch <= 90 {
                        return Err(format!("Missing character for code {ch}"));
                    }
                    self.font.push(0);
                }
            }
        }
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
            println!(" -> Palette entries: {}", self.pal_cnt);
            println!(" -> Colormap entries: {}", self.cmap_cnt);
            println!(" -> Sprites: {}", self.sprites.len());
            println!(" -> Patches: {}", self.patches.len());
            println!(" -> Flats: {}", self.flats.len());
            println!(" -> Fonts: {}", self.font.len());
            Ok(())
        }
    }
}

//-----------------------
//  Internal utils

fn validate_graphics_lump(_bytes: &[u8]) -> Result<(), String> {
    // TODO implement this !!!
    Ok(())
}

fn font_code_from_name(name: &str) -> Option<u32> {
    if name.len() == 8 && &name[0..5] == "STCFN" {
        atoi(&name[5..8])
    } else if name.len() == 7 && &name[0..5] == "FONTA" {
        let x = atoi(&name[5..7]);
        match x {
            Some(n) => Some(n + 32),
            _ => None,
        }
    } else {
        None
    }
}

fn create_pink_placeholder(width: u16, height: u16) -> Graphic {
    let size = (width as usize) * (height as usize);
    Graphic {
        width,
        height,
        left_offs: 0,
        top_offs: 0,
        pixels: vec![251; size],
    }
}

fn is_texture_lump(name: &str) -> bool {
    name.len() == 8 && &name[0..7] == "TEXTURE" && atoi(&name[7..=7]).is_some()
}

fn atoi(s: &str) -> Option<u32> {
    let mut num = 0_u32;
    for b in s.bytes() {
        if b < ('0' as u8) || b > ('9' as u8) {
            return None;
        }
        num = num * 10 + (b as u32) - ('0' as u32);
    }
    Some(num)
}
