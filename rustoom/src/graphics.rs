//! Handler for WAD graphics + flats

use crate::utils::*;
use crate::wad::*;
use crate::*;
use std::{collections::HashMap, rc::Rc};

//-----------------
// TODO initial parsing of WAD data and building of Font, Palette, Graphics etc => move out
// (so that all elements are built during initial parse, then kept immutable)

pub struct Graphics {
    wad_data: Rc<WadData>,
    patches: HashMap<u64, usize>,
    flats: HashMap<u64, usize>,
}

impl Graphics {
    pub fn new(wad: &Rc<WadData>) -> Self {
        Graphics {
            wad_data: Rc::clone(wad),
            patches: HashMap::new(),
            flats: HashMap::new(),
        }
    }

    pub fn add_patch(&mut self, name: &str, lump_idx: usize) {
        let key = hash_lump_name(name.as_bytes());
        self.patches.insert(key, lump_idx);
    }

    pub fn add_flat(&mut self, name: &str, lump_idx: usize) {
        let key = hash_lump_name(name.as_bytes());
        self.flats.insert(key, lump_idx);
    }

    pub fn get_patch(&self, key: u64) -> Option<PixMap> {
        if let Some(idx) = self.patches.get(&key) {
            let lump = self.wad_data.get_lump(*idx);
            if let Ok(l) = lump {
                return match PixMap::from_patch(l.bytes) {
                    Ok(pixmap) => Some(pixmap),
                    Err(err) => {
                        let name = lump_name_from_hash(key);
                        println!("[ERROR] Failed to load patch {name}: {err}");
                        None
                    }
                };
            }
        }
        None
    }

    pub fn get_flat(&self, key: u64) -> Option<PixMap> {
        if let Some(idx) = self.flats.get(&key) {
            let lump = self.wad_data.get_lump(*idx);
            if let Ok(l) = lump {
                return Some(PixMap::from_flat(l.bytes));
            }
        }
        None
    }
}
