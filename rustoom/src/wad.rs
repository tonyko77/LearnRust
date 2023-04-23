// WAD loader and parser

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


// see https://github.com/amroibrahim/DIYDoom/tree/master/DIYDOOM/Notes001/notes
pub struct WadData {
    lump_count: usize,
    dir_offset: usize,
    wad_bytes: Vec<u8>,
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

        Ok(WadData {
            lump_count,
            dir_offset,
            wad_bytes
        })
    }

    pub fn get_lump_count(&self) -> usize {
        self.lump_count
    }

    pub fn get_lump(&self, idx: usize) -> Result<LumpData, String> {
        if idx >= self.lump_count {
            Err(format!("Invalid lump index: index {idx} >= count {} ", self.lump_count))
        }
        else {
            let offs = self.dir_offset + 16 * idx;
            let lump_start = utils::buf_to_u32(&self.wad_bytes[offs .. (offs+4)]) as usize;
            let lump_size = utils::buf_to_u32(&self.wad_bytes[(offs+4) .. (offs+8)]) as usize;
            let wad_len = self.wad_bytes.len();
            let lump_end = lump_start + lump_size;
            if lump_end >= wad_len {
                Err(format!("Lump too big: offs {lump_start} + size {lump_size} >= wad len {wad_len} "))
            }
            else {
                let name_start = offs + 8;
                let mut name_end = offs + 16;
                // dismiss all null bytes at the end
                while (name_end > name_start) && (0 == self.wad_bytes[name_end - 1]) {
                    name_end -= 1;
                }
                let name_bytes = &self.wad_bytes[name_start .. name_end];
                let name_str = std::str::from_utf8(name_bytes);
                match name_str {
                    Ok(name) => Ok(LumpData {
                        name,
                        bytes: &self.wad_bytes[lump_start .. lump_end],
                    }),
                    // this should not happen anyway - lump names should always be ASCII
                    Err(_) => Err(format!("Invalid lump name at index {idx}")),
                }
            }
        }
    }

}
