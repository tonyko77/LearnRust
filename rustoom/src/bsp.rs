//! BSP nodes and BSP tree traversal, for each map

// TODO move this back into levelmap? the separation seems to be pointless

// TODO temporary !!!
#![allow(dead_code)]

use crate::map::MapData;
use crate::utils::*;
use crate::*;

const SSECTOR_FLAG: u16 = 0x8000;

// Lump item sizes
const NODE_SIZE: usize = 28;
const SSECTOR_SIZE: usize = 4;
const SEG_SIZE: usize = 12;
const SECTOR_SIZE: usize = 26;

#[derive(Clone)]
pub struct BspTree {
    map_data: MapData,
    sect_cnt: usize, // needed for blockmap
}

impl BspTree {
    #[inline]
    pub fn new(map_data: &MapData) -> Self {
        Self {
            map_data: map_data.clone(),
            sect_cnt: map_data.sectors().len() / SECTOR_SIZE,
        }
    }

    pub fn locate_player(&self, player: &Thing) -> Vec<Seg> {
        let mut sect_collector = vec![];
        let start_idx = (self.node_count() - 1) as u16;
        self.render_node(player, start_idx, &mut sect_collector);
        sect_collector
    }

    /// Use the REJECT table to check if there is line of sight between the player and the monster
    pub fn check_line_of_sight(&self, player_sect_idx: u16, monster_sect_idx: u16) -> bool {
        let pli = player_sect_idx as usize;
        let moi = monster_sect_idx as usize;
        let bit_idx = moi * self.sect_cnt + pli;
        let byte_idx = bit_idx >> 3;
        let bit_mask = 1 << (bit_idx & 0x07);
        self.map_data.blockmap()[byte_idx] & bit_mask == 0
    }

    //----------------------------

    fn render_node(&self, player: &Thing, node_idx: u16, seg_collector: &mut Vec<Seg>) {
        if (node_idx & SSECTOR_FLAG) == 0 {
            // NOT a leaf
            let node = self.node(node_idx as usize);
            let is_on_left = node.is_point_on_left(&player.pos());
            if is_on_left {
                // traverse LEFT first
                self.render_node(player, node.left_child, seg_collector);
                // TODO? if self.check_bounding_box(player, &node.right_box_bl, &node.right_box_tr) {
                self.render_node(player, node.right_child, seg_collector);
            } else {
                // traverse RIGHT first
                self.render_node(player, node.right_child, seg_collector);
                // TODO? if self.check_bounding_box(player, &node.left_box_bl, &node.left_box_tr) {
                self.render_node(player, node.left_child, seg_collector);
            }
        } else {
            // it's a LEAF => render sector
            self.render_sub_sector(node_idx, seg_collector);
        }
    }

    fn render_sub_sector(&self, sect_idx: u16, seg_collector: &mut Vec<Seg>) {
        let idx = (sect_idx & !SSECTOR_FLAG) as usize;
        // from SSECTORS, extract the seg count and first seg index
        let bytes = checked_slice(&self.map_data.ssectors(), idx, SSECTOR_SIZE);
        let seg_count = buf_to_u16(&bytes[0..2]);
        let first_seg_idx = buf_to_u16(&bytes[2..4]);
        // from SEGS, extract each segment
        for i in 0..seg_count {
            self.render_seg(first_seg_idx + i, seg_collector);
        }
    }

    fn render_seg(&self, seg_idx: u16, seg_collector: &mut Vec<Seg>) {
        let idx = seg_idx as usize;
        let bytes = checked_slice(&self.map_data.segs(), idx, SEG_SIZE);
        let seg = Seg::from(bytes, &self.map_data);
        seg_collector.push(seg);
    }

    #[inline(always)]
    fn node_count(&self) -> usize {
        self.map_data.nodes().len() / 28
    }

    #[inline(always)]
    fn node(&self, idx: usize) -> BspNode {
        let bytes = checked_slice(&self.map_data.nodes(), idx, NODE_SIZE);
        BspNode::from(bytes)
    }

    #[inline(always)]
    fn seg(&self, idx: usize) -> Seg {
        let bytes = checked_slice(&self.map_data.segs(), idx, SEG_SIZE);
        Seg::from(bytes, &self.map_data)
    }
}

//----------------------------

struct BspNode {
    vect_orig: Vertex,
    vect_dir: Vertex,
    // TODO use bounding boxes to optimize drawing (not relly needed, but niiice)
    _right_box_tr: Vertex,
    _right_box_bl: Vertex,
    _left_box_tr: Vertex,
    _left_box_bl: Vertex,
    right_child: u16,
    left_child: u16,
}

// TODO if most data is not needed => just remove this struct,
// put the code in a function and directly access the data from the lump
impl BspNode {
    fn from(bytes: &[u8]) -> Self {
        let vect = buf_to_i16_vect(&bytes[0..24]);
        Self {
            vect_orig: Vertex {
                x: vect[0] as i32,
                y: vect[1] as i32,
            },
            vect_dir: Vertex {
                x: vect[2] as i32,
                y: vect[3] as i32,
            },
            _right_box_bl: Vertex {
                // TODO figure out the order of the vertices
                x: Ord::min(vect[6], vect[7]) as i32,
                y: Ord::min(vect[4], vect[5]) as i32,
            },
            _right_box_tr: Vertex {
                // TODO figure out the order of the vertices
                x: Ord::max(vect[6], vect[7]) as i32,
                y: Ord::max(vect[4], vect[5]) as i32,
            },
            _left_box_bl: Vertex {
                // TODO figure out the order of the vertices
                x: Ord::min(vect[10], vect[11]) as i32,
                y: Ord::min(vect[8], vect[9]) as i32,
            },
            _left_box_tr: Vertex {
                // TODO figure out the order of the vertices
                x: Ord::max(vect[10], vect[11]) as i32,
                y: Ord::max(vect[8], vect[9]) as i32,
            },
            right_child: buf_to_u16(&bytes[24..26]),
            left_child: buf_to_u16(&bytes[26..28]),
        }
    }

    #[inline]
    fn is_point_on_left(&self, point: &Vertex) -> bool {
        let pvect = point.sub(&self.vect_orig);
        let cross_product_dir = pvect.x * self.vect_dir.y - pvect.y * self.vect_dir.x;
        cross_product_dir <= 0
    }
}

// TODO temp pub-s !!
pub struct Seg {
    pub start: Vertex,
    pub end: Vertex,
    angle: i16,
    linedef_idx: u16,
    direction_same: bool,
    offset: i16,
}

impl Seg {
    fn from(bytes: &[u8], map_data: &MapData) -> Self {
        let start = map_data.vertex(buf_to_u16(&bytes[0..2]) as usize);
        let end = map_data.vertex(buf_to_u16(&bytes[2..4]) as usize);
        Self {
            start,
            end,
            angle: buf_to_i16(&bytes[4..6]),
            linedef_idx: buf_to_u16(&bytes[6..8]),
            direction_same: 0 == buf_to_u16(&bytes[8..10]),
            offset: buf_to_i16(&bytes[10..12]),
        }
    }
}
