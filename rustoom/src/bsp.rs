//! BSP nodes and BSP tree traversal, for each map

// TODO: implement BSP tree traversal
#![allow(dead_code)]

use crate::utils::*;
use crate::*;
use bytes::Bytes;

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

    pub fn set_nodes(&mut self, nodes: &Bytes) {
        self.nodes = nodes.clone();
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

pub struct BspNode {
    vect_orig: Vertex,
    vect_dir: Vertex,
    right_box_tl: Vertex,
    right_box_br: Vertex,
    left_box_tl: Vertex,
    left_box_br: Vertex,
    right_child: u16,
    left_child: u16,
}

impl BspNode {
    // TODO replace this parsing with getters, which access the bytes directly :)
    // (+ same idea for all other structures: just get data directly from bytes)
    // + buf_to_extended_int, which converts i16 to "extended" i32 (left-shifted by 16)
    fn from_buf(buf: &[u8]) -> Self {
        Self {
            vect_orig: Vertex {
                x: buf_to_i16(&buf[0..2]) as i32,
                y: buf_to_i16(&buf[2..4]) as i32,
            },
            vect_dir: Vertex {
                x: buf_to_i16(&buf[4..6]) as i32,
                y: buf_to_i16(&buf[6..8]) as i32,
            },
            right_box_tl: Vertex {
                x: buf_to_i16(&buf[12..14]) as i32,
                y: buf_to_i16(&buf[8..10]) as i32,
            },
            right_box_br: Vertex {
                x: buf_to_i16(&buf[14..16]) as i32,
                y: buf_to_i16(&buf[10..12]) as i32,
            },
            left_box_tl: Vertex {
                x: buf_to_i16(&buf[20..22]) as i32,
                y: buf_to_i16(&buf[16..18]) as i32,
            },
            left_box_br: Vertex {
                x: buf_to_i16(&buf[22..24]) as i32,
                y: buf_to_i16(&buf[18..20]) as i32,
            },
            right_child: buf_to_u16(&buf[24..26]),
            left_child: buf_to_u16(&buf[26..28]),
        }
    }
}
