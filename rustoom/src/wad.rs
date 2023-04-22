// WAD loader and parser

use crate::lump;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WadKind {
    IWAD,
    PWAD,
}


pub struct WadData<'a> {
    kind: WadKind,
    lump_data: Vec<u8>,
    lump_dict: HashMap<&'a str, lump::LumpData<'a>>,
    // TODO to be continued ...
}


impl<'a> WadData<'a> {
    // see https://github.com/amroibrahim/DIYDoom/tree/master/DIYDOOM/Notes001/notes
    pub fn load(wad_path: &str, expected_kind: WadKind) -> Result<WadData<'a>, String> {
        let mut wad_data = WadData {
            kind: expected_kind,
            lump_data: vec![],
            lump_dict: HashMap::new(),
        };

        // TODO load WAD bytes, parse header/check kind, parse directory + each lump ...

        Ok(wad_data)
    }

    // TODO to be continued ...
}
