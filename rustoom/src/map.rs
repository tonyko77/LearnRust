//! Store maps from the WAD, just as collections of lump bytes.
//! Used as immutable storage, from which to build each level map when it becomes active.

use crate::{utils::buf_to_i16, Vertex};
use bytes::Bytes;

// Indexes for various MapData lumps
const IDX_THINGS: usize = 0;
const IDX_LINEDEFS: usize = 1;
const IDX_SIDEDEFS: usize = 2;
const IDX_VERTEXES: usize = 3;
const IDX_SEGS: usize = 4;
const IDX_SSECTORS: usize = 5;
const IDX_NODES: usize = 6;
const IDX_SECTORS: usize = 7;
const IDX_REJECT: usize = 8;
const IDX_BLOCKMAP: usize = 9;
const LUMP_CNT: usize = 10;

pub struct MapData {
    name: String,
    lumps: Box<[Bytes; LUMP_CNT]>,
}

impl Clone for MapData {
    fn clone(&self) -> Self {
        let lumps: Box<[Bytes; LUMP_CNT]> = Box::new((*self.lumps).clone());
        Self {
            name: self.name.clone(),
            lumps,
        }
    }
}

impl MapData {
    pub fn new(name: &str) -> Self {
        let lumps: Box<[Bytes; LUMP_CNT]> = Box::new(Default::default());
        Self {
            name: name.to_string(),
            lumps,
        }
    }

    #[inline]
    pub fn is_complete(&self) -> bool {
        self.lumps.iter().all(|b| b.len() > 0)
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn vertex_count(&self) -> usize {
        self.lumps[IDX_VERTEXES].len() >> 2
    }

    #[inline]
    pub fn vertex(&self, idx: usize) -> Vertex {
        let i = idx << 2;
        let bytes = &self.lumps[IDX_VERTEXES];
        Vertex {
            x: buf_to_i16(&bytes[(i + 0)..(i + 2)]) as i32,
            y: buf_to_i16(&bytes[(i + 2)..(i + 4)]) as i32,
        }
    }

    #[inline]
    pub fn linedefs(&self) -> &Bytes {
        &self.lumps[IDX_LINEDEFS]
    }

    #[inline]
    pub fn things(&self) -> &Bytes {
        &self.lumps[IDX_THINGS]
    }

    #[inline]
    pub fn sidedefs(&self) -> &Bytes {
        &self.lumps[IDX_SIDEDEFS]
    }

    #[inline]
    pub fn segs(&self) -> &Bytes {
        &self.lumps[IDX_SEGS]
    }

    #[inline]
    pub fn ssectors(&self) -> &Bytes {
        &self.lumps[IDX_SSECTORS]
    }

    #[inline]
    pub fn nodes(&self) -> &Bytes {
        &self.lumps[IDX_NODES]
    }

    #[inline]
    pub fn sectors(&self) -> &Bytes {
        &self.lumps[IDX_SECTORS]
    }

    #[inline]
    pub fn reject(&self) -> &Bytes {
        &self.lumps[IDX_REJECT]
    }

    #[inline]
    pub fn blockmap(&self) -> &Bytes {
        &self.lumps[IDX_BLOCKMAP]
    }

    pub fn add_lump(&mut self, lump: &str, bytes: &Bytes) -> bool {
        let idx = match lump {
            "VERTEXES" => IDX_VERTEXES,
            "LINEDEFS" => IDX_LINEDEFS,
            "THINGS" => IDX_THINGS,
            "SIDEDEFS" => IDX_SIDEDEFS,
            "SEGS" => IDX_SEGS,
            "SSECTORS" => IDX_SSECTORS,
            "NODES" => IDX_NODES,
            "SECTORS" => IDX_SECTORS,
            "REJECT" => IDX_REJECT,
            "BLOCKMAP" => IDX_BLOCKMAP,
            _ => usize::MAX,
        };
        // check if it was a valid lump; if not => return false, to signal the end of the map lumps
        // (all the lumps of one map are consecutive, so if we get an invalid one => we're done with this map)
        if idx < LUMP_CNT {
            self.lumps[idx] = bytes.clone();
            true
        } else {
            false
        }
    }
}
