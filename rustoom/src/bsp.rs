//! BSP nodes and BSP tree traversal, for each map

// TODO: implement BSP tree traversal
#![allow(dead_code)]

use crate::utils::*;
use crate::*;
use bytes::Bytes;

const SSECTOR_FLAG: u16 = 0x8000;

#[derive(Clone)]
pub struct BspTree {
    nodes: Bytes,
    // segs: Bytes,
    // sub_sectors: Bytes,
    // reject: Bytes,
    // blockmap: Bytes,
}

impl BspTree {
    pub fn new() -> Self {
        Self {
            nodes: Bytes::new(),
            // segs: Bytes::new(),
            // sub_sectors: Bytes::new(),
            // reject: Bytes::new(),
            // blockmap: Bytes::new(),
        }
    }

    pub fn set_nodes_lump(&mut self, nodes: &Bytes) {
        self.nodes = nodes.clone();
    }

    // !!!!! TODO NEXT: add SSECTOR support !!!!!!!!!!!!!!!!!!
    pub fn seek_player(&self, player: &Vertex) -> Vec<BspNode> {
        let mut node_collector = vec![];
        let start_idx = (self.get_node_count() - 1) as u16;
        self.render_node(player, &mut node_collector, start_idx);
        node_collector
    }

    fn render_node(&self, player: &Vertex, node_collector: &mut Vec<BspNode>, node_idx: u16) {
        if (node_idx & SSECTOR_FLAG) == 0 {
            // NOT a leaf
            let node = self.get_node(node_idx as usize);
            let is_on_left = node.is_point_on_left(player);
            if is_on_left {
                // traverse LEFT first
                self.render_node(player, node_collector, node.left_child);
                self.render_node(player, node_collector, node.right_child);
            } else {
                // traverse RIGHT first
                self.render_node(player, node_collector, node.right_child);
                self.render_node(player, node_collector, node.left_child);
            }
            // just push the node after that
            node_collector.push(node);
        } else {
            // it's a LEAF => render sector
            // TODO implement this !!!
        }
    }

    pub fn get_node_count(&self) -> usize {
        self.nodes.len() / 28
    }

    pub fn get_node(&self, idx: usize) -> BspNode {
        let ofs = idx * 28;
        assert!(ofs + 28 <= self.nodes.len());
        BspNode::from_buf(&self.nodes[ofs..(ofs + 28)])
    }
}

//----------------------------

// TODO temp pub
#[derive(Clone)]
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
    fn from_buf(buf: &[u8]) -> Self {
        let vect = buf_to_i16_vect(&buf[0..24]);
        Self {
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

    fn is_point_on_left(&self, point: &Vertex) -> bool {
        let pvect = point.sub(&self.vect_orig);
        let cross_product_dir = pvect.x * self.vect_dir.y - pvect.y * self.vect_dir.x;
        cross_product_dir <= 0
    }
}
