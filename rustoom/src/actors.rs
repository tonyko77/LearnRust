//! Actors = player and monsters
//! See [thing tipes at Doom Wiki](https://doomwiki.org/wiki/Thing_types)

use crate::{utils::*, Vertex};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ThingType {
    Player(u8),
    Monster(u8),
    Weapon(u8),
    Ammo(u8, u8),
    ArtifactItem,
    Collectible,
    Key,
    Obstacle,
    Decoration,
    Other(u16),
    // TODO .....
    Unknown,
}

pub struct Thing {
    x_pos: i16,
    y_pos: i16,
    pub angle: i16,
    pub type_code: u16,
    pub flags: u16,
    _typ: ThingType,
    _radius: u8,
    _height: u8,
    _sprite: [u8; 4],
}

impl Thing {
    pub fn new(lump_data: &[u8]) -> Self {
        assert!(lump_data.len() >= 10);
        let thing = Self {
            x_pos: buf_to_i16(&lump_data[0..2]),
            y_pos: buf_to_i16(&lump_data[2..4]),
            angle: buf_to_i16(&lump_data[4..6]),
            type_code: buf_to_u16(&lump_data[6..8]),
            flags: buf_to_u16(&lump_data[8..10]),
            _typ: ThingType::Unknown,
            _radius: 0,
            _height: 0,
            _sprite: [0; 4],
        };
        // TODO (later) fill in other values, based on type code and flags
        thing
    }

    #[inline]
    pub fn pos(&self) -> Vertex {
        Vertex {
            x: self.x_pos as i32,
            y: self.y_pos as i32,
        }
    }
}
