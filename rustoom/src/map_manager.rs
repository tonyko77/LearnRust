//! Manages the "directory" of maps and loads each map from the WAD

use crate::map::*;
use crate::WadData;
use std::rc::Rc;

pub struct MapManager {
    wad: Rc<WadData>,
    map_indices: Vec<usize>,
}

impl MapManager {
    pub fn new(wad: &Rc<WadData>) -> Self {
        MapManager {
            wad: Rc::clone(wad),
            map_indices: vec![],
        }
    }

    pub fn add_map(&mut self, idx: usize) {
        self.map_indices.push(idx);
    }

    #[inline]
    pub fn get_map_count(&self) -> usize {
        self.map_indices.len()
    }

    pub fn get_map(&self, map_idx: usize) -> Result<LevelMap, String> {
        assert!(map_idx < self.map_indices.len());
        let lump_idx = self.map_indices[map_idx];
        let lump = self.wad.get_lump(lump_idx)?;
        let mut map = LevelMap::new(lump.name);
        // parse map lumps
        for i in (lump_idx + 1)..(lump_idx + 13) {
            let lump = self.wad.get_lump(i)?;
            let must_break = match lump.name.as_str() {
                "VERTEXES" => {
                    map.parse_vertexes(&lump.bytes);
                    false
                }
                "LINEDEFS" => {
                    map.parse_line_defs(&lump.bytes);
                    false
                }
                "THINGS" => {
                    map.parse_things(&lump.bytes);
                    false
                }
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
