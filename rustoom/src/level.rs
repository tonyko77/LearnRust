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
const DEFAULT_AUTOMAP_ZOOM: f64 = 0.125;
const AUTOMAP_ZOOM_MIN: f64 = 0.04;
const AUTOMAP_ZOOM_MAX: f64 = 0.750;

const SSECTOR_FLAG: u16 = 0x8000;

pub struct ActiveLevel {
    map_data: MapData,
    player: Thing,
    amap_center: Vertex,
    amap_zoom: f64,
}

impl ActiveLevel {
    pub fn new(map_data: &MapData) -> Self {
        let player = find_player_thing(map_data);
        let amap_center = player.pos();
        Self {
            map_data: map_data.clone(),
            player,
            amap_center,
            amap_zoom: DEFAULT_AUTOMAP_ZOOM,
        }
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

    pub fn player(&self) -> &Thing {
        &self.player
    }

    pub fn paint_3d_view(&self, painter: &mut dyn Painter) {
        // TODO paint a fake sky
        let w = painter.get_screen_width();
        let h = painter.get_screen_height();
        painter.fill_rect(0, 0, w, h * 4 / 5, CYAN);
        // TODO implement this
    }

    pub fn paint_automap(&self, painter: &mut dyn Painter, font: &Font) {
        painter.fill_rect(0, 0, painter.get_screen_width(), painter.get_screen_height(), BLACK);
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
        font.draw_text(3, 3, &txt, RED, painter);

        // TODO TEMPORARY: paint the subsectors
        let segs = self.locate_player(&self.player);
        for seg in segs.iter() {
            let v1 = self.translate_automap_vertex(seg.start, painter);
            let v2 = self.translate_automap_vertex(seg.end, painter);
            painter.draw_line(v1.x, v1.y, v2.x, v2.y, GREY);
        }
    }

    pub fn update_automap(&mut self, dx: i32, dy: i32, dzoom: f64) {
        let new_center = Vertex {
            x: self.amap_center.x + dx,
            y: self.amap_center.y + dy,
        };
        self.amap_center = self.map_data.clamp_vertex(new_center);
        self.amap_zoom = f64::clamp(self.amap_zoom + dzoom, AUTOMAP_ZOOM_MIN, AUTOMAP_ZOOM_MAX);
    }

    //---------------
    // private methods
    //---------------

    fn locate_player(&self, player: &Thing) -> Vec<Seg> {
        let mut sect_collector = vec![];
        let start_idx = self.map_data.root_bsp_node_idx();
        self.render_node(player, start_idx, &mut sect_collector);
        sect_collector
    }

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

    fn translate_automap_vertex(&self, orig_vertex: Vertex, painter: &dyn Painter) -> Vertex {
        // scale the original coordinates
        let sv = (orig_vertex - self.amap_center).fscale(self.amap_zoom);
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
}

//--------------------
//  Internal stuff

fn find_player_thing(map_data: &MapData) -> Thing {
    for idx in 0..map_data.thing_count() {
        let th = map_data.thing(idx);
        if th.type_code() == 1 {
            return th;
        }
    }
    // TODO validate this upon WAD loading, so we can panic here
    panic!("No player thing found in map {}", map_data.name());
}
