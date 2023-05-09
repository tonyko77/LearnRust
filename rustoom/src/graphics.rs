//! Handler for WAD graphics + flats

use crate::utils::*;
use crate::*;
use bytes::Bytes;
use std::collections::HashMap;

pub struct Graphics {
    patches: HashMap<u64, Bytes>,
    flats: HashMap<u64, Bytes>,
    pnames: Bytes,
    textures: HashMap<u64, Bytes>,
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
            pnames: Bytes::new(),
            textures: HashMap::new(),
            // TODO delete these later
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

    pub fn add_patch(&mut self, name: &str, lump: &Bytes) {
        let key = hash_lump_name(name.as_bytes());
        self.patches.insert(key, lump.clone());
        self.dbg_patch_keys.push(key); // TODO TEMP !!!
    }

    pub fn add_flat(&mut self, name: &str, lump: &Bytes) {
        let key = hash_lump_name(name.as_bytes());
        self.flats.insert(key, lump.clone());
        self.dbg_flat_keys.push(key); // TODO TEMP !!!
    }

    pub fn set_patch_names(&mut self, patches: &Bytes) -> Result<(), String> {
        if patches.len() <= 4 {
            return Err(String::from("PNAMES lump size too small"));
        }
        let cnt = buf_to_u32(&patches[0..4]) as usize;
        if patches.len() < (4 + cnt * 8) {
            return Err(String::from("PNAMES lump size too small"));
        }
        // OK
        self.pnames = patches.clone();
        Ok(())
    }

    pub fn add_textures(&mut self, bytes: &Bytes) -> Result<(), String> {
        let len = bytes.len();
        if len <= 8 {
            return Err(format!("TEXTUREx lump size too small: {len}"));
        }
        // number of textures
        let cnt = buf_to_u32(bytes) as usize;
        if len <= 4 + 4 * cnt {
            return Err(format!("TEXTUREx lump size too small: {len}"));
        }
        // extract bytes for each texture
        for t in 0..cnt {
            let offs = buf_to_u32(&bytes[4 + 4 * t..]) as usize;
            if len <= (offs + 28) {
                return Err(format!("TEXTUREx entry #{t} out of bounds: len={len} < ofs={offs}"));
            }
            let key = hash_lump_name(&bytes[offs..offs + 8]);
            let patch_count = buf_to_u16(&bytes[offs + 20..]) as usize;
            let tex_len = 22 + 10 * patch_count;
            if len < (offs + tex_len) {
                return Err(format!("TEXTUREx entry #{t} out of bounds: len={len} < ofs={offs}"));
            }
            let tex_bytes = bytes.slice(offs..offs + tex_len);
            self.textures.insert(key, tex_bytes);
            self.dbg_tex_keys.push(key); // TODO TEMP !!!
        }

        Ok(())
    }

    pub fn get_patch(&self, key: u64) -> Option<PixMap> {
        self.patches.get(&key).map(|bytes| PixMap::from_patch(&bytes))
    }

    pub fn get_flat(&self, key: u64) -> Option<PixMap> {
        self.flats.get(&key).map(|bytes| PixMap::from_flat(&bytes))
    }

    // TODO maybe I can improve this mess ??
    pub fn get_texture(&self, key: u64) -> Option<Texture> {
        // get texture
        let tex_bytes = self.textures.get(&key)?;
        let width = buf_to_u16(&tex_bytes[12..14]);
        let height = buf_to_u16(&tex_bytes[14..16]);
        let patch_cnt = buf_to_u16(&tex_bytes[20..22]) as usize;
        let mut texture = Texture {
            width,
            height,
            patches: Vec::with_capacity(patch_cnt),
        };
        // get all patches for this texture
        for idx in 0..patch_cnt {
            let pofs = 22 + 10 * idx;
            let origin_x = buf_to_i16(&tex_bytes[(pofs + 0)..(pofs + 2)]) as i32;
            let origin_y = buf_to_i16(&tex_bytes[(pofs + 2)..(pofs + 4)]) as i32;
            let patch_idx = buf_to_u16(&tex_bytes[(pofs + 4)..(pofs + 6)]) as usize;
            let patch_key = hash_lump_name(&self.pnames[(patch_idx * 8 + 4)..(patch_idx * 8 + 12)]);
            let pixmap = self.get_patch(patch_key)?;
            texture.patches.push(TexturePatch {
                origin_x,
                origin_y,
                pixmap,
            });
        }
        Some(texture)
    }
}

//----------------------------

// TODO move this to separate source file? Or maybe pixmap.rs ??
pub struct Texture {
    width: u16,
    height: u16,
    patches: Vec<TexturePatch>,
}

impl Texture {
    pub fn new() -> Texture {
        Texture {
            width: 0,
            height: 0,
            patches: Vec::new(),
        }
    }

    #[inline]
    pub fn width(&self) -> u16 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u16 {
        self.height
    }

    // TODO this is BROKEN - WHY ????
    pub fn paint(&self, x: i32, y: i32, painter: &mut dyn Painter, mapper: &dyn ColorMapper) {
        if self.width > 0 && self.height > 0 {
            for patch in &self.patches {
                patch.pixmap.paint(x + patch.origin_x, y - patch.origin_y, painter, mapper);
            }
        }
    }
}

struct TexturePatch {
    origin_x: i32,
    origin_y: i32,
    pixmap: PixMap,
}
