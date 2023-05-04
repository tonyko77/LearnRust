//! Loads and manages a map from a wad

use crate::utils::*;
use crate::wad::*;
use debug_print::*;

#[derive(Debug)]
pub struct Vertex {
    pub x: i16,
    pub y: i16,
}

#[derive(Debug)]
pub struct LineDef {
    pub v1_idx: u16,
    pub v2_idx: u16,
    pub flags: u16,
    pub action: u16,
    pub sect_tag: u16,
    pub front_side_idx: u16,
    pub back_side_idx: u16,
}

pub struct LevelMap {
    name: String,
    // TODO things: Vec<Thing>,
    line_defs: Vec<LineDef>,
    // TODO side_defs: Vec<SideDef>,
    vertexes: Vec<Vertex>,
    // TODO segs: Vec<Segment>,
    // TODO s_sectors: Vec<GLNode>,
    // TODO nodes: Vec<Node>,
    // TODO sectors: Vec<Sector>,
    // TODO reject: Vec<Reject>,
    // TODO blockmap: Vec<BlockMap>,
}

impl LevelMap {
    pub fn from_wad(wad: &WadData, idx: usize) -> Result<Self, String> {
        let lump = wad.get_lump(idx)?;
        let mut map = LevelMap {
            name: String::from(lump.name),
            line_defs: vec![],
            vertexes: vec![],
        };
        debug_println!("Parsing map: {}", map.name);

        // parse map lumps
        for i in (idx + 1)..(idx + 13) {
            let lump = wad.get_lump(i)?;
            let ok_to_continue = map.parse_lump(&lump)?;
            if !ok_to_continue {
                break;
            }
        }

        // validate and return
        map.validate_and_return()
    }

    //--------------------

    fn parse_lump(&mut self, lump: &LumpData) -> Result<bool, String> {
        match lump.name {
            "THINGS" => self.parse_things(lump),
            "LINEDEFS" => self.parse_line_defs(lump),
            "SIDEDEFS" => self.parse_side_defs(lump),
            "VERTEXES" => self.parse_vertexes(lump),
            "SEGS" => self.parse_segs(lump),
            "SSECTORS" => self.parse_s_sectors(lump),
            "NODES" => self.parse_nodes(lump),
            "SECTORS" => self.parse_sectors(lump),
            "REJECT" => self.parse_reject(lump),
            "BLOCKMAP" => self.parse_blockmap(lump),
            _ => Ok(false),
        }
    }

    fn parse_vertexes(&mut self, lump: &LumpData) -> Result<bool, String> {
        let vertex_cnt = lump.bytes.len() >> 2;
        self.vertexes = Vec::with_capacity(vertex_cnt);
        for i in 0..vertex_cnt {
            let idx = i << 2;
            let v = Vertex {
                x: buf_to_i16(&lump.bytes[idx + 0..=idx + 1]),
                y: buf_to_i16(&lump.bytes[idx + 2..=idx + 3]),
            };
            //debug_println!("  -> ({i} / {vertex_cnt}) {v:?}");
            self.vertexes.push(v);
        }
        Ok(true)
    }

    fn parse_line_defs(&mut self, lump: &LumpData) -> Result<bool, String> {
        let line_cnt = lump.bytes.len() / 14;
        self.line_defs = Vec::with_capacity(line_cnt);
        for i in 0..line_cnt {
            let idx = i * 14;
            let ld = LineDef {
                v1_idx: buf_to_u16(&lump.bytes[idx + 0..=idx + 1]),
                v2_idx: buf_to_u16(&lump.bytes[idx + 2..=idx + 3]),
                flags: buf_to_u16(&lump.bytes[idx + 4..=idx + 5]),
                action: buf_to_u16(&lump.bytes[idx + 6..=idx + 7]),
                sect_tag: buf_to_u16(&lump.bytes[idx + 8..=idx + 9]),
                front_side_idx: buf_to_u16(&lump.bytes[idx + 10..=idx + 11]),
                back_side_idx: buf_to_u16(&lump.bytes[idx + 12..=idx + 13]),
            };
            //debug_println!("  -> ({i} / {line_cnt}) {ld:?}");
            self.line_defs.push(ld);
        }
        Ok(true)
    }

    fn parse_things(&mut self, _lump: &LumpData) -> Result<bool, String> {
        // TODO implement this
        Ok(true)
    }

    fn parse_side_defs(&mut self, _lump: &LumpData) -> Result<bool, String> {
        // TODO implement this
        Ok(true)
    }

    fn parse_segs(&mut self, _lump: &LumpData) -> Result<bool, String> {
        // TODO implement this
        Ok(true)
    }

    fn parse_s_sectors(&mut self, _lump: &LumpData) -> Result<bool, String> {
        // TODO implement this
        Ok(true)
    }

    fn parse_nodes(&mut self, _lump: &LumpData) -> Result<bool, String> {
        // TODO implement this
        Ok(true)
    }

    fn parse_sectors(&mut self, _lump: &LumpData) -> Result<bool, String> {
        // TODO implement this
        Ok(true)
    }

    fn parse_reject(&mut self, _lump: &LumpData) -> Result<bool, String> {
        // TODO implement this
        Ok(true)
    }

    fn parse_blockmap(&mut self, _lump: &LumpData) -> Result<bool, String> {
        // TODO implement this
        Ok(true)
    }

    fn validate_and_return(self) -> Result<Self, String> {
        // check vertexes
        if self.vertexes.len() < 2 {
            return Err(format!("Not enough vertexes in map {}", self.name));
        }
        // TODO validate the rest

        Ok(self)
    }
}
