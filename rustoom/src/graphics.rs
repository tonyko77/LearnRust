//! Handler for WAD graphics + flats

// TODO finish implementation for building TEXTURES !!!
#![allow(dead_code)]

use crate::utils::*;
use crate::*;
use bytes::Bytes;
use std::collections::HashMap;

pub struct Graphics {
    patches: HashMap<u64, Bytes>,
    flats: HashMap<u64, Bytes>,
    patch_map: Vec<u64>,
    textures: HashMap<u64, Texture>,
    // TODO delete these later
    dbg_patch_keys: Vec<u64>,
    dbg_flat_keys: Vec<u64>,
    dbg_tex_keys: Vec<u64>,
}

impl Graphics {
    pub fn new() -> Self {
        Graphics {
            patches: HashMap::new(),
            flats: HashMap::new(),
            patch_map: vec![],
            textures: HashMap::new(),
            dbg_patch_keys: vec![],
            dbg_flat_keys: vec![],
            dbg_tex_keys: vec![],
        }
    }

    pub fn dbg_patch_keys(&self) -> &Vec<u64> {
        &self.dbg_patch_keys
    }

    pub fn dbg_flat_keys(&self) -> &Vec<u64> {
        &self.dbg_flat_keys
    }

    pub fn dbg_texture_keys(&self) -> &Vec<u64> {
        &self.dbg_tex_keys
    }

    pub fn add_patch(&mut self, name: &str, lump: Bytes) {
        let key = hash_lump_name(name.as_bytes());
        self.patches.insert(key, lump);
        self.dbg_patch_keys.push(key); // TODO TEMP !!!
    }

    pub fn add_flat(&mut self, name: &str, lump: Bytes) {
        let key = hash_lump_name(name.as_bytes());
        self.flats.insert(key, lump);
        self.dbg_flat_keys.push(key); // TODO TEMP !!!
    }

    pub fn set_patch_names(&mut self, patches: &[u8]) -> Result<(), String> {
        if patches.len() <= 4 {
            return Err(String::from("PNAMES lump size too small"));
        }
        let cnt = buf_to_u32(patches) as usize;
        if patches.len() < (4 + cnt * 8) {
            return Err(String::from("PNAMES lump size too small"));
        }
        // parse patch names
        self.patch_map = Vec::with_capacity(cnt);
        for i in 0..cnt {
            let idx = 4 + i * 8;
            let key = hash_lump_name(&patches[idx..idx + 8]);
            self.patch_map.push(key);
        }
        Ok(())
    }

    pub fn add_textures(&mut self, bytes: &[u8]) -> Result<(), String> {
        if self.patch_map.len() == 0 {
            return Err("TEXTUREx cannot be parsed without PNAMES".to_string());
        }

        let len = bytes.len();
        if len <= 8 {
            return Err(format!("TEXTUREx lump size too small: {len}"));
        }
        let cnt = buf_to_u32(bytes) as usize;
        if len <= 4 + 4 * cnt {
            return Err(format!("TEXTUREx lump size too small: {len}"));
        }

        for t in 0..cnt {
            let offs = buf_to_u32(&bytes[4 + 4 * t..]) as usize;
            if len <= (offs + 28) {
                return Err(format!("TEXTUREx entry #{t} out of bounds: len={len} < ofs={offs}"));
            }
            // parse one texture
            let key = hash_lump_name(&bytes[offs..offs + 8]);
            let width = buf_to_u16(&bytes[offs + 12..]) as i32;
            let height = buf_to_u16(&bytes[offs + 14..]) as i32;
            let patch_count = buf_to_u16(&bytes[offs + 20..]) as usize;
            if len < (offs + 22 + 10 * patch_count) {
                return Err(format!("TEXTUREx entry #{t} out of bounds: len={len} < ofs={offs}"));
            }
            // parse patches
            let mut patches = Vec::with_capacity(patch_count);
            for p in 0..patch_count {
                let p_ofs = offs + 22 + 10 * p;
                let origin_x = buf_to_i16(&bytes[p_ofs..]) as i32;
                let origin_y = buf_to_i16(&bytes[p_ofs + 2..]) as i32;
                let patch_idx = buf_to_u16(&bytes[p_ofs + 4..]) as usize;
                if patch_idx >= self.patch_map.len() {
                    return Err(format!(
                        "TEXTUREx entry #{t} contains an invalid patch index: {patch_idx} < ofs={offs}"
                    ));
                }
                let patch_key = self.patch_map[patch_idx];
                patches.push(TexturePatch {
                    origin_x,
                    origin_y,
                    patch_key,
                });
            }
            // store texture
            self.textures.insert(key, Texture { width, height, patches });
            self.dbg_tex_keys.push(key); // TODO TEMP !!!
        }
        Ok(())
    }

    pub fn get_patch(&self, key: u64) -> Option<PixMap> {
        if let Some(bytes) = self.patches.get(&key) {
            return match PixMap::from_patch(bytes) {
                Ok(pixmap) => Some(pixmap),
                Err(err) => {
                    let name = lump_name_from_hash(key);
                    println!("[ERROR] Failed to load patch {name}: {err}");
                    None
                }
            };
        }
        None
    }

    pub fn get_flat(&self, key: u64) -> Option<PixMap> {
        if let Some(bytes) = self.flats.get(&key) {
            return Some(PixMap::from_flat(&bytes));
        }
        None
    }

    pub fn get_texture(&self, key: u64) -> Option<PixMap> {
        if let Some(tex) = self.textures.get(&key) {
            Some(self.build_texture_pixmap(tex))
        } else {
            None
        }
    }

    //-----------------------

    fn build_texture_pixmap(&self, _tex: &Texture) -> PixMap {
        // TODO implement this !!!
        PixMap::new_empty()
    }
}

//----------------------------

struct TexturePatch {
    origin_x: i32,
    origin_y: i32,
    patch_key: u64,
}

struct Texture {
    width: i32,
    height: i32,
    patches: Vec<TexturePatch>,
}
