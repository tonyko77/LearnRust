//! Manager for textures

// TODO finish this .....
#![allow(dead_code)]

use crate::utils::*;
use std::collections::HashMap;

struct TexturePatch {
    origin_x: i16,
    origin_y: i16,
    patch_idx: u16,
}

struct Texture {
    width: i32,
    height: i32,
    patches: Vec<TexturePatch>,
}

pub struct TextureSet {
    patch_map: Vec<u64>,
    tex_map: HashMap<u64, Texture>,
}

impl TextureSet {
    pub fn new() -> Self {
        TextureSet {
            patch_map: vec![],
            tex_map: HashMap::new(),
        }
    }

    pub fn parse_patch_names(&mut self, bytes: &[u8]) -> Result<(), String> {
        if bytes.len() <= 4 {
            return Err(String::from("PNAMES lump size too small"));
        }
        let cnt = buf_to_u32(bytes) as usize;
        if bytes.len() < (4 + cnt * 8) {
            return Err(String::from("PNAMES lump size too small"));
        }
        // parse patch names
        self.patch_map = Vec::with_capacity(cnt);
        for i in 0..cnt {
            let idx = 4 + i * 8;
            let key = hash_lump_name(&bytes[idx..=idx + 7]);
            self.patch_map.push(key);
        }
        Ok(())
    }

    pub fn parse_textures(&mut self, bytes: &[u8]) -> Result<(), String> {
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
                let origin_x = buf_to_i16(&bytes[p_ofs..]);
                let origin_y = buf_to_i16(&bytes[p_ofs + 2..]);
                let patch_idx = buf_to_u16(&bytes[p_ofs + 4..]);
                patches.push(TexturePatch {
                    origin_x,
                    origin_y,
                    patch_idx,
                });
            }
            // store texture
            self.tex_map.insert(key, Texture { width, height, patches });
        }
        Ok(())
    }

    #[inline]
    pub fn is_initialized(&self) -> bool {
        self.patch_map.len() > 0 && self.tex_map.len() > 0
    }
}
