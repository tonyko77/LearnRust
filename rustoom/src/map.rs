//! Loads and manages a map from a wad

// TODO temporary !!!
#![allow(dead_code)]
#![allow(unused_variables)]

use crate::utils::*;

#[derive(Debug)]
pub struct Vertex {
    x: i16,
    y: i16,
}

#[derive(Debug)]
pub struct LineDef {
    v1_idx: u16,
    v2_idx: u16,
    flags: u16,
    line_type: u16,
    sector_tag: u16,
    right_side_def: u16,
    left_side_def: u16,
}

pub struct LevelMap {
    pub name: String,
    vertexes: Vec<Vertex>,
    line_defs: Vec<LineDef>,
    // TODO things: Vec<Thing>,
    // TODO side_defs: Vec<SideDef>,
    // TODO segs: Vec<Segment>,
    // TODO s_sectors: Vec<GLNode>,
    // TODO nodes: Vec<Node>,
    // TODO sectors: Vec<Sector>,
    // TODO reject: Vec<Reject>,
    // TODO blockmap: Vec<BlockMap>,
}

impl LevelMap {
    pub fn new(name: &str) -> Self {
        LevelMap {
            name: String::from(name),
            line_defs: vec![],
            vertexes: vec![],
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
}
