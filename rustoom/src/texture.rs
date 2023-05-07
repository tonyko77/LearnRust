//! Manager for textures

use crate::utils::*;
use std::collections::HashMap;

pub struct Textures {
    patch_map: Vec<u64>,
    text_map: HashMap<u32, i32>,
}

impl Textures {
    pub fn new() -> Self {
        Textures {
            patch_map: vec![],
            text_map: HashMap::new(),
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
        // TODO implement this !!!
        Ok(())
    }

    pub fn is_initialized(&self) -> bool {
        // TODO implement this !!!
        true
    }
}
