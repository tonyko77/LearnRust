//! An "active" level map, where all the map data is expanded and "mutable".
//! Built from an existing MapData.

use crate::font::Font;
use crate::map::*;
use crate::map_items::{Seg, Vertex};
use crate::things::Thing;
use crate::*;

// LineDef flags
const LINE_BLOCKS: u16 = 0x0001;
//const LINE_BLOCKS_MONSTERS: u16 = 0x0002;
const LINE_TWO_SIDED: u16 = 0x0004;
//const LINE_UPPER_UNPEGGED: u16 = 0x0008;
//const LINE_LOWER_UNPEGGED: u16 = 0x0010;
const LINE_SECRET: u16 = 0x0020;
//const LINE_BLOCKS_SND: u16 = 0x0040;
const LINE_NEVER_ON_AMAP: u16 = 0x0080;
const LINE_ALWAYS_ON_AMAP: u16 = 0x0100;

// Automap zoom limits
const DEFAULT_AUTOMAP_ZOOM: i32 = 12;
const AUTOMAP_ZOOM_MIN: i32 = 5;
const AUTOMAP_ZOOM_MAX: i32 = 60;

const SSECTOR_FLAG: u16 = 0x8000;

pub struct ActiveLevel {
    map_data: MapData,
    player: Thing,
    amap_center: Vertex,
    amap_bl: Vertex,
    amap_tr: Vertex,
    amap_zoom: i32,
}

impl ActiveLevel {
    pub fn new(map_data: &MapData) -> Self {
        let mut map = Self {
            map_data: map_data.clone(),
            player: Default::default(),
            amap_center: Vertex { x: 0, y: 0 },
            amap_bl: Vertex { x: 0, y: 0 },
            amap_tr: Vertex { x: 0, y: 0 },
            amap_zoom: DEFAULT_AUTOMAP_ZOOM,
        };
        // compute bounds
        map.compute_automap_bounds();
        // fetch player
        if let Some(th) = map.find_thing_by_type(1) {
            map.amap_center = th.pos();
            map.player = th;
        } else {
            // TODO improve error handling ?!
            panic!("No player found in map's THINGS");
        }
        // TODO map.build_bsp();
        map
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.map_data.name()
    }

    pub fn get_things(&self, level_filter: u8) -> Vec<Thing> {
        (0..self.map_data.thing_count())
            .map(|idx| self.map_data.thing(idx))
            .filter(|th| th.is_on_skill_level(level_filter))
            .collect()
    }

    pub fn get_player(&self) -> Thing {
        let cnt = self.map_data.thing_count();
        (0..cnt)
            .map(|idx| self.map_data.thing(idx))
            .find(|th| th.type_code() == 1)
            .expect("Player not found in map's THINGS")
    }

    pub fn paint_automap(&self, painter: &mut dyn Painter, font: &Font) {
        for idx in 0..self.map_data.linedef_count() {
            let line = self.map_data.linedef(idx);
            let v1 = self.translate_automap_vertex(line.v1, painter);
            let v2 = self.translate_automap_vertex(line.v2, painter);

            // select color based on line type
            let f = line.flags;
            let color = if f & LINE_SECRET != 0 {
                CYAN
            } else if f & LINE_BLOCKS != 0 {
                RED
            } else if f & LINE_TWO_SIDED != 0 {
                // TODO: yellow for ceiling diff, choco for floor diff !!
                CHOCO
            } else if f & LINE_ALWAYS_ON_AMAP != 0 {
                WHITE
            } else if f & LINE_NEVER_ON_AMAP != 0 {
                DARK_GREY
            } else {
                MAGENTA
            };
            painter.draw_line(v1.x, v1.y, v2.x, v2.y, color);
        }
        // draw the things
        for thing in self.get_things(0) {
            let color = match thing.type_code() {
                1 => WHITE,
                2 => ORANGE,
                3 => BLUE,
                4 => GREEN,
                _ => DARK_GREY,
            };
            self.paint_cross(painter, thing.pos(), color);
        }

        // draw map name
        let txt = format!("Map: {}", self.name());
        font.draw_text(3, 3, &txt, ORANGE, painter);
    }

    pub fn move_automap(&mut self, dx: i32, dy: i32) {
        self.amap_center.x = Ord::clamp(self.amap_center.x + dx, self.amap_bl.x, self.amap_tr.x);
        self.amap_center.y = Ord::clamp(self.amap_center.y + dy, self.amap_bl.y, self.amap_tr.y);
    }

    pub fn zoom_automap(&mut self, dzoom: i32) {
        self.amap_zoom = Ord::clamp(self.amap_zoom + dzoom, AUTOMAP_ZOOM_MIN, AUTOMAP_ZOOM_MAX);
    }

    pub fn locate_player(&self, player: &Thing) -> Vec<Seg> {
        let mut sect_collector = vec![];
        let start_idx = self.map_data.root_bsp_node_idx();
        self.render_node(player, start_idx, &mut sect_collector);
        sect_collector
    }

    //----------------------------

    fn render_node(&self, player: &Thing, node_idx: u16, seg_collector: &mut Vec<Seg>) {
        if (node_idx & SSECTOR_FLAG) == 0 {
            // NOT a leaf
            let node = self.map_data.bsp_node(node_idx as usize);
            let is_on_left = node.is_point_on_left(player.pos());
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
        let segs = self.map_data.sub_sector(idx);
        for s in segs {
            // TODO render each segment
            seg_collector.push(s);
        }
    }

    //---------------
    // private methods
    //---------------

    // TODO temp pub !!
    pub fn translate_automap_vertex(&self, orig_vertex: Vertex, painter: &dyn Painter) -> Vertex {
        // scale the original coordinates
        let sv = (orig_vertex - self.amap_center).scale(self.amap_zoom, 100);
        // translate the scaled coordinates + mirror y
        Vertex {
            x: sv.x + (painter.get_screen_width() / 2),
            y: (painter.get_screen_height() / 2) - sv.y,
        }
    }

    fn paint_cross(&self, painter: &mut dyn Painter, v: Vertex, color: RGB) {
        let v = self.translate_automap_vertex(v, painter);
        painter.draw_line(v.x - 1, v.y, v.x + 1, v.y, color);
        painter.draw_line(v.x, v.y - 1, v.x, v.y + 1, color);
    }

    fn compute_automap_bounds(&mut self) {
        self.amap_bl = self.map_data.vertex(0);
        self.amap_tr = self.map_data.vertex(0);
        for idx in 1..self.map_data.vertex_count() {
            let v = self.map_data.vertex(idx);
            self.amap_bl.x = Ord::min(self.amap_bl.x, v.x);
            self.amap_bl.y = Ord::min(self.amap_bl.y, v.y);
            self.amap_tr.x = Ord::max(self.amap_tr.x, v.x);
            self.amap_tr.y = Ord::max(self.amap_tr.y, v.y);
        }
    }

    fn find_thing_by_type(&self, thing_type: u16) -> Option<Thing> {
        for idx in 0..self.map_data.thing_count() {
            let th = self.map_data.thing(idx);
            if th.type_code() == thing_type {
                return Some(th);
            }
        }
        None
    }
}

//--------------------
//  Internal stuff
