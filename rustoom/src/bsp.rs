//! BSP nodes and BSP tree traversal, for each map

use crate::map::MapData;
use crate::utils::*;
use crate::*;

const SSECTOR_FLAG: u16 = 0x8000;

#[derive(Clone)]
pub struct BspTree(MapData);

impl BspTree {
    #[inline]
    pub fn new(map_data: &MapData) -> Self {
        Self(map_data.clone())
    }

    pub fn locate_player(&self, player: &Vertex) -> Vec<SubSector> {
        let mut sect_collector = vec![];
        let start_idx = (self.get_node_count() - 1) as u16;
        self.render_node(player, &mut sect_collector, start_idx);
        sect_collector
    }

    fn render_node(&self, player: &Vertex, sect_collector: &mut Vec<SubSector>, node_idx: u16) {
        if (node_idx & SSECTOR_FLAG) == 0 {
            // NOT a leaf
            let node = self.get_node(node_idx as usize);
            let is_on_left = node.is_point_on_left(player);
            if is_on_left {
                // traverse LEFT first
                self.render_node(player, sect_collector, node.left_child);
                self.render_node(player, sect_collector, node.right_child);
            } else {
                // traverse RIGHT first
                self.render_node(player, sect_collector, node.right_child);
                self.render_node(player, sect_collector, node.left_child);
            }
        } else {
            // it's a LEAF => render sector
            let sect_idx = (node_idx & !SSECTOR_FLAG) as usize;
            let sect = self.sub_sector(sect_idx);
            sect_collector.push(sect);
        }
    }

    //----------------------------

    pub fn get_node_count(&self) -> usize {
        self.0.nodes().len() / 28
    }

    pub fn get_node(&self, idx: usize) -> BspNode {
        let ofs = idx * 28;
        assert!(ofs + 28 <= self.0.nodes().len());
        let buf = &self.0.nodes()[ofs..(ofs + 28)];
        let vect = buf_to_i16_vect(&buf[0..24]);
        BspNode {
            vect_orig: Vertex {
                x: vect[0] as i32,
                y: vect[1] as i32,
            },
            vect_dir: Vertex {
                x: vect[2] as i32,
                y: vect[3] as i32,
            },
            right_box_bl: Vertex {
                // TODO is this needed ??
                x: Ord::min(vect[6], vect[7]) as i32,
                y: Ord::min(vect[4], vect[5]) as i32,
            },
            right_box_tr: Vertex {
                // TODO is this needed ??
                x: Ord::max(vect[6], vect[7]) as i32,
                y: Ord::max(vect[4], vect[5]) as i32,
            },
            left_box_bl: Vertex {
                // TODO is this needed ??
                x: Ord::min(vect[10], vect[11]) as i32,
                y: Ord::min(vect[8], vect[9]) as i32,
            },
            left_box_tr: Vertex {
                // TODO is this needed ??
                x: Ord::max(vect[10], vect[11]) as i32,
                y: Ord::max(vect[8], vect[9]) as i32,
            },
            right_child: buf_to_u16(&buf[24..26]),
            left_child: buf_to_u16(&buf[26..28]),
        }
    }

    fn sub_sector(&self, sub_sect_idx: usize) -> SubSector {
        // from SSECTORS, extract the seg count and first seg index
        let ss_ofs = sub_sect_idx << 2;
        assert!(ss_ofs + 4 <= self.0.ssectors().len());
        let bytes = &self.0.ssectors()[ss_ofs..];
        let seg_count = buf_to_u16(&bytes[0..2]) as usize;
        let first_seg_idx = buf_to_u16(&bytes[2..4]) as usize;
        // from SEGS, extract each segment
        assert!((first_seg_idx + seg_count) * 12 <= self.0.segs().len());
        let segs = (0..seg_count).map(|idx| self.seg(first_seg_idx + idx)).collect();
        SubSector(segs)
    }

    fn seg(&self, seg_idx: usize) -> Seg {
        let ofs = seg_idx * 12;
        let buf = &self.0.segs()[ofs..(ofs + 12)];
        Seg {
            start: self.0.vertex(buf_to_i16(&buf[0..2]) as usize),
            end: self.0.vertex(buf_to_i16(&buf[2..4]) as usize),
            _angle: buf_to_i16(&buf[4..6]),
            _linedef_idx: buf_to_u16(&buf[6..8]),
            _direction_same: 0 == buf_to_u16(&buf[8..10]),
            _offset: buf_to_i16(&buf[10..12]),
        }
    }
}

//----------------------------

// TODO temp pub
pub struct BspNode {
    pub vect_orig: Vertex,
    pub vect_dir: Vertex,
    pub right_box_tr: Vertex,
    pub right_box_bl: Vertex,
    pub left_box_tr: Vertex,
    pub left_box_bl: Vertex,
    pub right_child: u16,
    pub left_child: u16,
}

// TODO if most data is not needed => just remove this struct,
// put the code in a function and directly access the data from the lump
impl BspNode {
    #[inline]
    fn is_point_on_left(&self, point: &Vertex) -> bool {
        let pvect = point.sub(&self.vect_orig);
        let cross_product_dir = pvect.x * self.vect_dir.y - pvect.y * self.vect_dir.x;
        cross_product_dir <= 0
    }
}

// TODO temp pub
pub struct SubSector(pub Vec<Seg>);

// TODO temp pub
pub struct Seg {
    pub start: Vertex,
    pub end: Vertex,
    _angle: i16,
    _linedef_idx: u16,
    _direction_same: bool,
    _offset: i16,
}
