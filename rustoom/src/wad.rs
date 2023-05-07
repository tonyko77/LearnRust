//! WAD loader and parser

use crate::utils;
use bytes::{Bytes, BytesMut};
use std::fs::*;
use std::io::Read;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WadKind {
    IWAD,
    PWAD,
}

pub struct LumpData {
    pub name: String,
    pub bytes: Bytes,
}

impl LumpData {
    fn new(name_bytes: &[u8], bytes: Bytes, idx: usize) -> Result<Self, String> {
        // dismiss all null bytes at the name's end
        let mut idx_end = 0;
        for ch in name_bytes {
            if *ch == 0 {
                break;
            } else if *ch <= 32 || *ch >= 127 {
                return Err(format!("Invalid lump name at index {idx}"));
            } else {
                idx_end += 1;
            }
        }
        // all ok
        let name = std::str::from_utf8(&name_bytes[0..idx_end]).unwrap().to_string();
        Ok(LumpData { name, bytes })
    }
}

/// Stores all the bytes from a WAD file.
/// Also provides raw access to the WAD directory and lumps.
/// See [DIYDoom, Notes001](https://github.com/amroibrahim/DIYDoom/tree/master/DIYDOOM/Notes001/notes).
pub struct WadData {
    lump_count: usize,
    dir_offset: usize,
    wad_bytes: Bytes,
}

impl WadData {
    pub fn load(wad_path: &str, expected_kind: WadKind) -> Result<WadData, String> {
        // read WAD file bytes
        let mut wad_bytes: BytesMut;
        {
            let mut file = File::open(wad_path).map_err(|e| e.to_string())?;
            let len = file.metadata().map_err(|e| e.to_string())?.len() as usize;
            println!("WAD size: {}", len);
            wad_bytes = BytesMut::zeroed(len);
            file.read_exact(&mut wad_bytes).map_err(|e| e.to_string())?;
        }
        let wad_bytes = wad_bytes.freeze();

        // parse the WAD header
        if wad_bytes.len() <= 12 {
            return Err(format!("WAD file {wad_path} is too small"));
        }
        // hdr[0..4] => "IWAD" or "PWAD"
        let wad_kind_str = std::str::from_utf8(&wad_bytes[0..4]).map_err(|_| String::from("Invalid WAD header"))?;
        // hdr[4..8] = number of lumps / directory entries
        let lump_count = utils::buf_to_u32(&wad_bytes[4..8]) as usize;
        // hdr[8..12] = offset of directory entries
        let dir_offset = utils::buf_to_u32(&wad_bytes[8..12]) as usize;

        // verify the wad kind
        let expected_kind_str = match expected_kind {
            WadKind::IWAD => "IWAD",
            WadKind::PWAD => "PWAD",
        };
        if expected_kind_str.ne(wad_kind_str) {
            return Err(format!(
                "Invalid WAD type: expected {expected_kind_str}, was {wad_kind_str}"
            ));
        }

        Ok(WadData {
            lump_count,
            dir_offset,
            wad_bytes,
        })
    }

    #[inline]
    pub fn get_lump_count(&self) -> usize {
        self.lump_count
    }

    pub fn get_lump(&self, lump_idx: usize) -> Result<LumpData, String> {
        if lump_idx >= self.lump_count {
            Err(format!("Invalid lump index: {lump_idx} >= count {} ", self.lump_count))
        } else {
            let offs = self.dir_offset + 16 * lump_idx;
            let lump_start = utils::buf_to_u32(&self.wad_bytes[offs..(offs + 4)]) as usize;
            let lump_size = utils::buf_to_u32(&self.wad_bytes[(offs + 4)..(offs + 8)]) as usize;
            let wad_len = self.wad_bytes.len();
            let lump_end = lump_start + lump_size;
            if lump_end >= wad_len {
                Err(format!(
                    "Lump too big: offs {lump_start} + size {lump_size} >= wad len {wad_len} "
                ))
            } else {
                let name_bytes = &self.wad_bytes[offs + 8..offs + 16];
                let bytes = self.wad_bytes.slice(lump_start..lump_end);
                LumpData::new(name_bytes, bytes, lump_idx)
            }
        }
    }
}
