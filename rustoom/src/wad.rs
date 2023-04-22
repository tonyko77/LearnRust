// WAD loader and parser

use crate::lump;
use std::collections::HashMap;
use std::{io, fs, str};
use std::io::prelude::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WadKind {
    IWAD,
    PWAD,
}

//------------------------------------------

pub struct WadData {
    lump_count: usize,
    dir_offset: usize,
    wad_bytes: Vec<u8>,
    // TODO ?? lump_dict: HashMap<&'a str, lump::LumpData<'a>>,
    // TODO to be continued ...
}


impl WadData {
    // see https://github.com/amroibrahim/DIYDoom/tree/master/DIYDOOM/Notes001/notes
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
        let wad_kind_str = str::from_utf8(&wad_bytes[0..4]);
        let wad_kind_str = match wad_kind_str {
            Ok(kind) => String::from(kind),
            Err(_) => String::from("cannot parse"),
        };
        // hdr[4..8] = number of lumps / directory entries
        let lump_count = buf_to_u32(&wad_bytes[4..8]) as usize;
        // hdr[8..12] = offset of directory entries
        let dir_offset = buf_to_u32(&wad_bytes[8..12]) as usize;

        // verify the wad kind
        let expected_kind_str = match expected_kind {
            WadKind::IWAD => "IWAD",
            WadKind::PWAD => "PWAD",
        };
        if expected_kind_str.ne(wad_kind_str.as_str()) {
            return Err(format!("Invalid WAD type: {wad_kind_str}"));
        }

        // TODO - TEMP logging
        println!("[DBG] Loaded {wad_kind_str} file: {wad_path}");
        println!("[DBG]  => Lump Count: {lump_count}");
        println!("[DBG]  => Dir Offset: 0x{:08X}", dir_offset);
        println!("[DBG]  => WAD size: {}", wad_bytes.len());

        // ok to build the WAD data
        let wad_data = WadData { lump_count, dir_offset, wad_bytes };
        Ok(wad_data)
    }

    // TODO to be continued ...
}


//-----------------------------------------------
// TODO move these to some common utils !!!

#[inline]
fn buf_to_u16(buf: &[u8]) -> u16 {
    assert!(buf.len() >= 2);

    (buf[0] as u16) |
    ((buf[1] as u16) <<  8)
}

#[inline]
fn buf_to_u32(buf: &[u8]) -> u32 {
    assert!(buf.len() >= 4);

    (buf[0] as u32) |
    ((buf[1] as u32) <<  8) |
    ((buf[2] as u32) << 16) |
    ((buf[3] as u32) << 24)
}
