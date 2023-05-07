//! Manages the "directory" of maps and loads each map from the WAD

use crate::map::*;
use crate::WadData;
use bytes::Bytes;

pub struct MapManager {
    maps: Vec<RawMapData>,
}

impl MapManager {
    pub fn new() -> Self {
        MapManager { maps: vec![] }
    }

    pub fn add_map(&mut self, name: String, lump_idx: usize, wad: &WadData) -> Result<(), String> {
        let mut raw = RawMapData::new(name.clone());
        for i in (lump_idx + 1)..=(lump_idx + LUMP_CNT) {
            let lump = wad.get_lump(i)?;
            let ok_to_continue = raw.add_lump(&lump.name, lump.bytes);
            if !ok_to_continue {
                break;
            }
        }
        if raw.is_complete() {
            self.maps.push(raw);
            Ok(())
        } else {
            Err(format!("Incomplete map data for {name}"))
        }
    }

    #[inline]
    pub fn get_map_count(&self) -> usize {
        self.maps.len()
    }

    pub fn get_map(&self, map_idx: usize) -> Result<LevelMap, String> {
        if map_idx < self.maps.len() {
            let raw = &self.maps[map_idx];
            Ok(raw.to_map())
        } else {
            Err(format!("Invalid map index: {} >= {}", map_idx, self.maps.len()))
        }
    }
}

//---------------------

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

struct RawMapData {
    name: String,
    lumps: Vec<Bytes>,
}

impl RawMapData {
    fn new(name: String) -> RawMapData {
        RawMapData {
            name,
            lumps: vec![Bytes::new(); LUMP_CNT],
        }
    }

    fn add_lump(&mut self, lump: &str, bytes: Bytes) -> bool {
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
        if idx < LUMP_CNT {
            self.lumps[idx] = bytes;
            true
        } else {
            false
        }
    }

    fn is_complete(&self) -> bool {
        self.lumps.iter().all(|b| b.len() > 0)
    }

    fn to_map(&self) -> LevelMap {
        let mut map = LevelMap::new(self.name.clone());
        // TODO (!) so NOT parse some/all of the lumps (e.g. VERTEXES)
        // those should be ease to parse as-is
        // In fact, maybe RawMapData and LevelMap should be combined into one class !?!
        map.parse_vertexes(&self.lumps[IDX_VERTEXES]);
        map.parse_line_defs(&self.lumps[IDX_LINEDEFS]);
        map.parse_things(&self.lumps[IDX_THINGS]);
        // TODO set other lump types ...
        map
    }
}
