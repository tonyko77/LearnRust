//! Parse maps from the WAD.

use crate::utils::*;

#[derive(Debug, Clone, Default)]
pub struct Vertex {
    pub x: i16,
    pub y: i16,
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
    pub pos: Vertex,
    pub angle: u16,
    pub thing_type: u16,
    pub flags: u16,
}

#[derive(Default)]
pub struct LevelMap {
    pub name: String,
    vertexes: Vec<Vertex>,
    pub line_defs: Vec<LineDef>,
    pub things: Vec<Thing>,
    // TODO pub side_defs: Vec<SideDef>,
    // TODO pub segs: Vec<Segment>,
    // TODO pub s_sectors: Vec<GLNode>,
    // TODO pub nodes: Vec<Node>,
    // TODO pub sectors: Vec<Sector>,
    // TODO pub reject: Vec<Reject>,
    // TODO pub blockmap: Vec<BlockMap>,
    pub v_min: Vertex,
    pub v_max: Vertex,
    pub v_orig: Vertex,
}

impl LevelMap {
    pub fn new(name: &str) -> Self {
        LevelMap {
            name: String::from(name),
            line_defs: vec![],
            vertexes: vec![],
            things: vec![],
            v_min: Vertex { x: 0, y: 0 },
            v_max: Vertex { x: 0, y: 0 },
            v_orig: Vertex { x: 0, y: 0 },
        }
    }

    #[inline]
    pub fn get_vertex(&self, idx: u16) -> &Vertex {
        &self.vertexes[idx as usize]
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
                self.v_min = v.clone();
                self.v_max = v.clone();
            } else {
                self.v_min.x = i16::min(self.v_min.x, v.x);
                self.v_min.y = i16::min(self.v_min.y, v.y);
                self.v_max.x = i16::max(self.v_max.x, v.x);
                self.v_max.y = i16::max(self.v_max.y, v.y);
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
                pos: Vertex {
                    x: buf_to_i16(&lump_bytes[idx + 0..=idx + 1]),
                    y: buf_to_i16(&lump_bytes[idx + 2..=idx + 3]),
                },
                angle: buf_to_u16(&lump_bytes[idx + 4..=idx + 5]),
                thing_type: buf_to_u16(&lump_bytes[idx + 6..=idx + 7]),
                flags: buf_to_u16(&lump_bytes[idx + 8..=idx + 9]),
            };
            if th.thing_type == 1 {
                self.v_orig = th.pos.clone();
            }
            self.things.push(th);
        }
    }
}
