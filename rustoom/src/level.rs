//! An "active" level map, where all the map data is expanded and "mutable".
//! Built from an existing MapData.

use std::f64::consts::PI;

use crate::angle::Angle;
use crate::map::*;
use crate::map_items::*;
use crate::pixmap::Texture;
use crate::things::Thing;
use crate::utils::hash_lump_name;
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
const DEFAULT_AUTOMAP_ZOOM: f64 = 0.1875;
const AUTOMAP_ZOOM_MIN: f64 = 0.1;
const AUTOMAP_ZOOM_MAX: f64 = 0.8;

// BSP node flag, for signaling leaf nodes, which point to sub-sectors instead of other nodes
const SSECTOR_FLAG: u16 = 0x8000;

const AMAP_MOVE_SPEED: f64 = 800.0;
const AMAP_ZOOM_SPEED: f64 = 0.0625;
const PLAYER_MOVE_SPEED: f64 = 200.0;
const PLAYER_ROT_SPEED: f64 = 1.5;

pub struct ActiveLevel {
    cfg: GameConfig,
    map_data: MapData,
    player: Thing,
    amap_center: Vertex,
    amap_zoom: f64,
    sky: Texture,
    player_x: f64,
    player_y: f64,
    amap_cx: f64,
    amap_cy: f64,
}

impl ActiveLevel {
    pub fn new(cfg: GameConfig, map_idx: usize) -> Self {
        let map_data = cfg.wad().map(map_idx).clone();
        let player = find_player_thing(&map_data);
        let pc = player.pos;
        let amap_center = player.pos;
        let sky = load_sky(&cfg);
        Self {
            cfg,
            map_data,
            player,
            amap_center,
            amap_zoom: DEFAULT_AUTOMAP_ZOOM,
            sky,
            // TODO: use a FloatVertex structure, that supports translation and rotation
            player_x: pc.x as f64,
            player_y: pc.y as f64,
            amap_cx: amap_center.x as f64,
            amap_cy: amap_center.y as f64,
        }
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.map_data.name()
    }

    // TODO is this needed ??
    pub fn _get_things(&self, level_filter: u8) -> Vec<Thing> {
        (0..self.map_data.thing_count())
            .map(|idx| self.map_data.thing(idx))
            .filter(|th| th.is_on_skill_level(level_filter))
            .collect()
    }

    pub fn paint_3d_view(&self, painter: &mut dyn Painter) {
        // TODO implement this .............
        let w = painter.get_screen_width();
        let h = painter.get_screen_height();
        painter.fill_rect(0, 0, w, h, CYAN);
        // TODO properly align the sky with the player's rotation + fill the whole horizon width
        self.sky.paint(0, 0, painter, self.cfg.palette());
    }

    pub fn paint_automap(&self, painter: &mut dyn Painter) {
        // clear the screen first
        painter.fill_rect(0, 0, painter.get_screen_width(), painter.get_screen_height(), BLACK);
        // paint the map itself
        for idx in 0..self.map_data.linedef_count() {
            let line = self.map_data.linedef(idx);
            let color = self.pick_automap_line_color(&line);
            if color != BLACK {
                self.draw_automap_line(line.v1, line.v2, color, painter);
            }
        }

        // paint the player, as a white arrow
        let pos = self.player.pos;
        let ang = self.player.angle;
        let v1 = pos.polar_translate(25.0, ang);
        let v2 = pos.polar_translate(18.0, -ang);
        let v3 = pos.polar_translate(25.0, -ang);
        self.draw_automap_line(v1, v3, WHITE, painter);
        // draw the arrow head + fins
        let a2 = ang + 2.7;
        let a3 = ang - 2.7;
        let va = v1.polar_translate(18.0, a2);
        let vb = v1.polar_translate(18.0, a3);
        self.draw_automap_line(v1, va, WHITE, painter);
        self.draw_automap_line(v1, vb, WHITE, painter);
        let a2 = ang + 2.5;
        let a3 = ang - 2.5;
        let va = v2.polar_translate(13.0, a2);
        let vb = v2.polar_translate(13.0, a3);
        self.draw_automap_line(v2, va, WHITE, painter);
        self.draw_automap_line(v2, vb, WHITE, painter);
        let va = v3.polar_translate(13.0, a2);
        let vb = v3.polar_translate(13.0, a3);
        self.draw_automap_line(v3, va, WHITE, painter);
        self.draw_automap_line(v3, vb, WHITE, painter);

        // text with the map name
        let txt = format!("Map: {}", self.name());
        self.cfg.font().draw_text(3, 3, &txt, RED, painter);

        // TODO TEMPORARY: paint the subsectors
        let segs = self.locate_player(&self.player);
        for seg in segs.iter() {
            let _v1 = self.translate_automap_vertex(seg.start);
            let _v2 = self.translate_automap_vertex(seg.end);
            // TODO fix this !!!
            //painter.draw_line(v1.x, v1.y, v2.x, v2.y, GREY);
        }
    }

    pub fn move_automap_x(&mut self, dx: f64) {
        self.amap_cx += dx * AMAP_MOVE_SPEED;
        let (cv, was_clamped) = clamp_value(self.amap_cx as i32, self.map_data.min_x(), self.map_data.max_x());
        self.amap_center.x = cv;
        if was_clamped {
            self.amap_cx = cv as f64;
        }
    }

    pub fn move_automap_y(&mut self, dy: f64) {
        self.amap_cy += dy * AMAP_MOVE_SPEED;
        let (cv, was_clamped) = clamp_value(self.amap_cy as i32, self.map_data.min_y(), self.map_data.max_y());
        self.amap_center.y = cv;
        if was_clamped {
            self.amap_cy = cv as f64;
        }
    }

    pub fn zoom_automap(&mut self, dzoom: f64) {
        let new_zoom = self.amap_zoom + dzoom * AMAP_ZOOM_SPEED;
        self.amap_zoom = f64::clamp(new_zoom, AUTOMAP_ZOOM_MIN, AUTOMAP_ZOOM_MAX);
    }

    pub fn move_player(&mut self, ellapsed_time: f64) {
        self.translate_player(ellapsed_time, self.player.angle);
    }

    pub fn strafe_player(&mut self, ellapsed_time: f64) {
        self.translate_player(ellapsed_time, self.player.angle - (PI * 0.5));
    }

    pub fn rotate_player(&mut self, ellapsed_time: f64) {
        self.player.angle = self.player.angle + ellapsed_time * PLAYER_ROT_SPEED;
    }

    fn translate_player(&mut self, ellapsed_time: f64, angle: Angle) {
        let (dx, dy) = float_polar_translate(ellapsed_time * PLAYER_MOVE_SPEED, angle);
        self.player_x += dx;
        self.player_y += dy;
        self.player.pos = Vertex {
            x: self.player_x as i32,
            y: self.player_y as i32,
        }
    }

    //---------------
    // private methods
    //---------------

    // select color based on line type
    // TODO some colors may be wrong, or temporary => CHECK against Crispy Doom
    fn pick_automap_line_color(&self, line: &LineDef) -> RGB {
        let f = line.flags;

        if f & LINE_SECRET != 0 {
            // TODO temporary - highlight secrets
            // (later, use cyan for not-yet-seen lines)
            return CYAN;
        }
        if line.special_type != 0 {
            // TODO temporary - highlight actionable lines
            return BLUE;
        }

        if f & LINE_ALWAYS_ON_AMAP != 0 {
            // TODO: this (and next) flags should be used for determining which lines to paint
            return WHITE;
        }
        if f & LINE_NEVER_ON_AMAP != 0 {
            // quick return, for lines that should NOT appear on automap
            return BLACK;
        }

        if f & LINE_TWO_SIDED != 0 {
            let details = self.get_line_details(&line);
            let s1 = details.left_sector.unwrap();
            let s2 = details.right_sector.unwrap();
            return if s1.floor_height != s2.floor_height {
                // stairs
                CHOCO
            } else if s1.ceiling_height != s2.ceiling_height {
                // ceiling diff
                YELLOW
            } else {
                // no height delta => simply don't draw
                BLACK
            };
        }

        if f & LINE_BLOCKS != 0 {
            return RED;
        }

        // TODO temporary - just highlight lines that don't match any of the above
        // (later, the default returned here should be BLACK)
        PINK
    }

    #[inline]
    fn draw_automap_line(&self, v1: Vertex, v2: Vertex, color: RGB, painter: &mut dyn Painter) {
        let xv1 = self.translate_automap_vertex(v1);
        let xv2 = self.translate_automap_vertex(v2);
        painter.draw_line(xv1.x, xv1.y, xv2.x, xv2.y, color);
    }

    fn get_line_details(&self, linedef: &LineDef) -> LineDefDetails {
        let mut details = LineDefDetails {
            left_sidedef: None,
            left_sector: None,
            right_sidedef: None,
            right_sector: None,
        };
        if linedef.left_side_idx != 0xFFFF {
            let side = self.map_data.sidedef(linedef.left_side_idx as usize);
            let sect = self.map_data.sector(side.sector_idx as usize);
            details.left_sidedef = Some(side);
            details.left_sector = Some(sect);
        }
        if linedef.right_side_idx != 0xFFFF {
            let side = self.map_data.sidedef(linedef.right_side_idx as usize);
            let sect = self.map_data.sector(side.sector_idx as usize);
            details.right_sidedef = Some(side);
            details.right_sector = Some(sect);
        }

        // sanity checks
        // TODO: these should be tested during WAD validation
        assert!(details.left_sidedef.is_some());
        assert!(details.left_sector.is_some());
        if linedef.flags & LINE_TWO_SIDED == 0 {
            // NOT 2-sided
            assert!(details.right_sidedef.is_none());
            assert!(details.right_sector.is_none());
        } else {
            // is 2-sided
            assert!(details.right_sidedef.is_some());
            assert!(details.right_sector.is_some());
        }

        details
    }

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
            let (kid1, kid2) = node.child_indices_based_on_point_pos(player.pos);
            self.render_node(player, kid1, seg_collector);
            // TODO? if self.check_bounding_box(player, &node.2nd_kid_box_bl, &node.2nd_kid_box_bl)
            self.render_node(player, kid2, seg_collector);
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

    fn translate_automap_vertex(&self, orig_vertex: Vertex) -> Vertex {
        // scale the original coordinates
        let sv = (orig_vertex - self.amap_center).fscale(self.amap_zoom);
        // translate the scaled coordinates + mirror y
        Vertex {
            x: sv.x + (self.cfg.scr_width() / 2),
            y: (self.cfg.scr_height() / 2) - sv.y,
        }
    }
}

//--------------------
//  Internal stuff

struct LineDefDetails {
    right_sidedef: Option<SideDef>,
    right_sector: Option<Sector>,
    left_sidedef: Option<SideDef>,
    left_sector: Option<Sector>,
}

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

// TODO (later) pick the texture name based on level: https://doomwiki.org/wiki/Sky
// (DOOM1, DOOM, DOOMU) ExMy => SKYx
fn load_sky(cfg: &GameConfig) -> Texture {
    let name = "SKY1";
    let key = hash_lump_name(name.as_bytes());
    cfg.graphics().get_texture(key).unwrap()
}

/// Clamp a value, but also signal if it was clamped or not
#[inline]
fn clamp_value<T: PartialOrd>(val: T, min: T, max: T) -> (T, bool) {
    if val < min {
        (min, true)
    } else if val > max {
        (max, true)
    } else {
        (val, false)
    }
}

#[inline]
fn float_polar_translate(dist: f64, angle: Angle) -> (f64, f64) {
    let (s, c) = angle.rad().sin_cos();
    (dist * c, dist * s)
}
