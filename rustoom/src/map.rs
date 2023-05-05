//! Loads and manages a map from a wad

use crate::utils::*;

#[derive(Debug)]
pub struct Vertex {
    x: i16,
    y: i16,
}

#[derive(Debug)]
pub struct LineDef {
    pub v1_idx: u16,
    pub v2_idx: u16,
    pub flags: u16,
    pub line_type: u16,
    pub sector_tag: u16,
    pub right_side_def: u16,
    pub left_side_def: u16,
}

#[derive(Debug)]
pub struct Thing {
    pub x: i16,
    pub y: i16,
    pub angle: u16,
    pub thing_type: u16,
    pub flags: u16,
}

pub struct LevelMap {
    pub name: String,
    pub vertexes: Vec<Vertex>,
    pub line_defs: Vec<LineDef>,
    pub things: Vec<Thing>,
    // TODO pub side_defs: Vec<SideDef>,
    // TODO pub segs: Vec<Segment>,
    // TODO pub s_sectors: Vec<GLNode>,
    // TODO pub nodes: Vec<Node>,
    // TODO pub sectors: Vec<Sector>,
    // TODO pub reject: Vec<Reject>,
    // TODO pub blockmap: Vec<BlockMap>,
    pub x_min: i16,
    pub x_max: i16,
    pub y_min: i16,
    pub y_max: i16,
}

impl LevelMap {
    pub fn new(name: &str) -> Self {
        LevelMap {
            name: String::from(name),
            line_defs: vec![],
            vertexes: vec![],
            things: vec![],
            x_min: 0,
            x_max: 0,
            y_min: 0,
            y_max: 0,
        }
    }

    pub fn parse_vertexes(&mut self, lump_bytes: &[u8]) {
        let vertex_cnt = lump_bytes.len() >> 2;
        self.vertexes = Vec::with_capacity(vertex_cnt);
        for i in 0..vertex_cnt {
            let idx = i << 2;
            let v = Vertex {
                x: buf_to_i16(&lump_bytes[idx + 0..=idx + 1]),
                y: buf_to_i16(&lump_bytes[idx + 2..=idx + 3]),
            };
            // also compute min/max for x & y
            if i == 0 {
                self.x_min = v.x;
                self.x_max = v.x;
                self.y_min = v.y;
                self.y_max = v.y;
            }
            else {
                self.x_min = i16::min(self.x_min, v.x);
                self.x_max = i16::max(self.x_max, v.x);
                self.y_min = i16::min(self.y_min, v.y);
                self.y_max = i16::max(self.y_max, v.y);
            }
            self.vertexes.push(v);
        }
    }

    pub fn parse_line_defs(&mut self, lump_bytes: &[u8]) {
        let line_cnt = lump_bytes.len() / 14;
        self.line_defs = Vec::with_capacity(line_cnt);
        for i in 0..line_cnt {
            let idx = i * 14;
            let ld = LineDef {
                v1_idx: buf_to_u16(&lump_bytes[idx + 0..=idx + 1]),
                v2_idx: buf_to_u16(&lump_bytes[idx + 2..=idx + 3]),
                flags: buf_to_u16(&lump_bytes[idx + 4..=idx + 5]),
                line_type: buf_to_u16(&lump_bytes[idx + 6..=idx + 7]),
                sector_tag: buf_to_u16(&lump_bytes[idx + 8..=idx + 9]),
                right_side_def: buf_to_u16(&lump_bytes[idx + 10..=idx + 11]),
                left_side_def: buf_to_u16(&lump_bytes[idx + 12..=idx + 13]),
            };
            self.line_defs.push(ld);
        }
    }

    pub fn parse_things(&mut self, lump_bytes: &[u8]) {
        let thing_cnt = lump_bytes.len() / 10;
        self.things = Vec::with_capacity(thing_cnt);
        for i in 0..thing_cnt {
            let idx = i * 10;
            let th = Thing {
                x: buf_to_i16(&lump_bytes[idx + 0..=idx + 1]),
                y: buf_to_i16(&lump_bytes[idx + 2..=idx + 3]),
                angle: buf_to_u16(&lump_bytes[idx + 4..=idx + 5]),
                thing_type: buf_to_u16(&lump_bytes[idx + 6..=idx + 7]),
                flags: buf_to_u16(&lump_bytes[idx + 8..=idx + 9]),
            };
            self.things.push(th);
        }
    }
}
