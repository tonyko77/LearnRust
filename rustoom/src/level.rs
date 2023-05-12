//! An "active" level map, where all the map data is expanded and "mutable".
//! Built from an existing MapData.

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
//const LINE_NEVER_ON_AMAP: u16 = 0x0080;
const LINE_ALWAYS_ON_AMAP: u16 = 0x0100;

// Automap zoom limits
const DEFAULT_AUTOMAP_ZOOM: f64 = 0.1875;
const AUTOMAP_ZOOM_MIN: f64 = 0.0625;
const AUTOMAP_ZOOM_MAX: f64 = 0.750;

// BSP node flag, for signaling leaf nodes, which point to sub-sectors instead of other nodes
const SSECTOR_FLAG: u16 = 0x8000;

pub struct ActiveLevel {
    cfg: GameConfig,
    map_data: MapData,
    player: Thing,
    amap_center: Vertex,
    amap_zoom: f64,
    sky: Texture,
}

impl ActiveLevel {
    pub fn new(cfg: GameConfig, map_idx: usize) -> Self {
        let map_data = cfg.wad().map(map_idx).clone();
        let player = find_player_thing(&map_data);
        let amap_center = player.pos();
        let sky = load_sky(&cfg);
        Self {
            cfg,
            map_data,
            player,
            amap_center,
            amap_zoom: DEFAULT_AUTOMAP_ZOOM,
            sky,
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
        painter.fill_rect(0, 0, painter.get_screen_width(), painter.get_screen_height(), BLACK);
        for idx in 0..self.map_data.linedef_count() {
            let line = self.map_data.linedef(idx);
            let v1 = self.translate_automap_vertex(line.v1);
            let v2 = self.translate_automap_vertex(line.v2);

            // select color based on line type
            let f = line.flags;
            let color = if f & LINE_SECRET != 0 {
                CYAN
            } else if line.special_type != 0 {
                // TODO temporary
                PINK
            } else if f & LINE_BLOCKS != 0 {
                RED
            } else if f & LINE_TWO_SIDED != 0 {
                let details = self.get_line_details(&line);
                let s1 = details.left_sector.unwrap();
                let s2 = details.right_sector.unwrap();
                if s1.floor_height != s2.floor_height {
                    // stairs => brown
                    CHOCO
                } else if s1.ceiling_height != s2.ceiling_height {
                    // ceiling diff
                    YELLOW
                } else {
                    BLACK
                }
            } else if f & LINE_ALWAYS_ON_AMAP != 0 {
                // TODO: this (and next) flags should be used for determining which lines to paint
                WHITE
            // } else if f & LINE_NEVER_ON_AMAP != 0 {
            //     DARK_GREY
            } else {
                // TODO temporary
                PINK
            };

            //if color != PINK {
            painter.draw_line(v1.x, v1.y, v2.x, v2.y, color);
            //}
        }

        // paint the player, as a white arrow
        let pos = self.player.pos();
        let ang = self.player.angle();
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

        // draw map name
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
            let (kid1, kid2) = node.child_indices_based_on_point_pos(player.pos());
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
