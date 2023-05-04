//! WAD loader and parser

use crate::map::*;
use crate::utils;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WadKind {
    IWAD,
    PWAD,
}

pub struct LumpData<'a> {
    pub name: &'a str,
    pub bytes: &'a [u8],
}

/// Stores all the bytes from a WAD file.
/// Also provides raw access to the WAD directory and lumps.
/// See [DIYDoom, Notes001](https://github.com/amroibrahim/DIYDoom/tree/master/DIYDOOM/Notes001/notes).
pub struct WadData {
    lump_count: usize,
    dir_offset: usize,
    wad_bytes: Vec<u8>,
    map_indices: Vec<usize>,
}

impl WadData {
    pub fn load(wad_path: &str, expected_kind: WadKind) -> Result<WadData, String> {
        // read WAD file bytes
        let wad_read = std::fs::read(wad_path);
        if let Err(io_err) = wad_read {
            return Err(format!("Failed to read WAD file {wad_path}: {io_err}"));
        }

        // parse the WAD header
        let wad_bytes = wad_read.unwrap();
        if wad_bytes.len() <= 12 {
            return Err(format!("WAD file {wad_path} is too small"));
        }
        // hdr[0..4] => "IWAD" or "PWAD"
        let wad_kind_str = std::str::from_utf8(&wad_bytes[0..4]);
        let wad_kind_str = match wad_kind_str {
            Ok(kind) => String::from(kind),
            Err(_) => String::from("cannot parse"),
        };
        // hdr[4..8] = number of lumps / directory entries
        let lump_count = utils::buf_to_u32(&wad_bytes[4..8]) as usize;
        // hdr[8..12] = offset of directory entries
        let dir_offset = utils::buf_to_u32(&wad_bytes[8..12]) as usize;

        // verify the wad kind
        let expected_kind_str = match expected_kind {
            WadKind::IWAD => "IWAD",
            WadKind::PWAD => "PWAD",
        };
        if expected_kind_str.ne(wad_kind_str.as_str()) {
            return Err(format!("Invalid WAD type: {wad_kind_str}"));
        }

        // build and validate the result
        let mut result = WadData {
            lump_count,
            dir_offset,
            wad_bytes,
            map_indices: Vec::with_capacity(64),
        };
        result.parse_and_validate_wad_data()?;

        Ok(result)
    }

    #[inline]
    pub fn get_lump_count(&self) -> usize {
        self.lump_count
    }

    pub fn get_lump(&self, idx: usize) -> Result<LumpData, String> {
        if idx >= self.lump_count {
            Err(format!(
                "Invalid lump index: {idx} >= count {} ",
                self.lump_count
            ))
        } else {
            let offs = self.dir_offset + 16 * idx;
            let lump_start = utils::buf_to_u32(&self.wad_bytes[offs..(offs + 4)]) as usize;
            let lump_size = utils::buf_to_u32(&self.wad_bytes[(offs + 4)..(offs + 8)]) as usize;
            let wad_len = self.wad_bytes.len();
            let lump_end = lump_start + lump_size;
            if lump_end >= wad_len {
                Err(format!(
                    "Lump too big: offs {lump_start} + size {lump_size} >= wad len {wad_len} "
                ))
            } else {
                let name_start = offs + 8;
                let mut name_end = offs + 16;
                // dismiss all null bytes at the end
                while (name_end > name_start) && (0 == self.wad_bytes[name_end - 1]) {
                    name_end -= 1;
                }
                let name_bytes = &self.wad_bytes[name_start..name_end];
                let name_str = std::str::from_utf8(name_bytes);
                match name_str {
                    Ok(name) => Ok(LumpData {
                        name,
                        bytes: &self.wad_bytes[lump_start..lump_end],
                    }),
                    // this should not happen anyway - lump names should always be ASCII
                    Err(_) => Err(format!("Invalid lump name at index {idx}")),
                }
            }
        }
    }

    #[inline]
    pub fn get_map_count(&self) -> usize {
        self.map_indices.len()
    }

    pub fn get_map(&self, idx: usize) -> Result<LevelMap, String> {
        if idx >= self.map_indices.len() {
            Err(format!(
                "Invalid map index: {idx} >= count {} ",
                self.map_indices.len()
            ))
        } else {
            let mi = self.map_indices[idx];
            self.build_map(mi)
        }
    }

    //-----------

    fn parse_and_validate_wad_data(&mut self) -> Result<(), String> {
        // check that all lumps are ok
        let lump_count = self.get_lump_count();
        if lump_count == 0 {
            return Err(String::from("WAD has no lumps"));
        }
        // check each lump
        for i in 0..lump_count {
            let lump = self.get_lump(i)?;
            // check for map start markers
            if lump.bytes.len() == 0 && is_map_name(lump.name) {
                self.map_indices.push(i);
            }
            // TODO check for other known lumps ..
        }
        // check maps
        if self.map_indices.is_empty() {
            return Err(String::from("WAD has no maps"));
        }
        for mi in 0..self.map_indices.len() {
            self.get_map(mi)?;
        }

        Ok(())
    }

    fn build_map(&self, lump_idx: usize) -> Result<LevelMap, String> {
        let lump = self.get_lump(lump_idx)?;
        let mut map = LevelMap::new(lump.name);
        // parse map lumps
        for i in (lump_idx + 1)..(lump_idx + 13) {
            let lump = self.get_lump(i)?;
            let must_break = match lump.name {
                "VERTEXES" => {
                    map.parse_vertexes(&lump.bytes);
                    false
                }
                "LINEDEFS" => {
                    map.parse_line_defs(&lump.bytes);
                    false
                }
                "THINGS" => false,   // TODO...
                "SIDEDEFS" => false, // TODO...
                "SEGS" => false,     // TODO...
                "SSECTORS" => false, // TODO...
                "NODES" => false,    // TODO...
                "SECTORS" => false,  // TODO...
                "REJECT" => false,   // TODO...
                "BLOCKMAP" => false, // TODO...
                _ => true,
            };
            if must_break {
                break;
            }
        }
        // done
        Ok(map)
    }
}

//-----------------------------
//  Internal utils

fn is_map_name(name: &str) -> bool {
    const E: u8 = 'E' as u8;
    const M: u8 = 'M' as u8;
    const A: u8 = 'A' as u8;
    const P: u8 = 'P' as u8;
    const D0: u8 = '0' as u8;
    const D9: u8 = '9' as u8;

    let b = name.as_bytes();
    if b.len() == 4 {
        (b[0] == E) && (b[1] >= D0) && (b[1] <= D9) && (b[2] == M) && (b[3] >= D0) && (b[3] <= D9)
    } else if b.len() == 5 {
        (b[0] == M)
            && (b[1] == A)
            && (b[2] == P)
            && (b[3] >= D0)
            && (b[3] <= D9)
            && (b[4] >= D0)
            && (b[4] <= D9)
    } else {
        false
    }
}
