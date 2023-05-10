//! An "active" level map, where all the map data is expanded and "mutable".
//! Built from an existing MapData.

use crate::bsp::*;
use crate::utils::*;
use crate::*;
use bytes::Bytes;

pub struct LevelMap {
    name: String,
    vertexes: Bytes,
    linedefs: Bytes,
    sidedefs: Bytes,
    things: Bytes,
    bsp: BspTree,
}

impl LevelMap {
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn get_things(&self, level_filter: u8) -> Vec<Thing> {
        let cnt = self.thing_count();
        (0..cnt)
            .map(|idx| self.thing(idx))
            .filter(|th| th.is_on_skill_level(level_filter))
            .collect()
    }

    pub fn get_player(&self) -> Thing {
        let cnt = self.thing_count();
        (0..cnt)
            .map(|idx| self.thing(idx))
            .find(|th| th.type_code() == 1)
            .expect("Player not found in map's THINGS")
    }

    pub fn paint_automap(&self, painter: &mut dyn Painter, font: &Font) {
        for idx in 0..self.line_count() {
            let line = self.linedef(idx);
            let (x1, y1) = self.translate_automap_vertex(line.v1, painter);
            let (x2, y2) = self.translate_automap_vertex(line.v2, painter);

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
            painter.draw_line(x1, y1, x2, y2, color);
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
        let txt = format!("Map: {}", self.name);
        font.draw_text(3, 3, &txt, ORANGE, painter);
    }

    //---------------

    fn do_add_lump(&mut self, lump_idx: usize, bytes: Bytes) {
        self.lumps[lump_idx] = bytes;
        // if added things => also fetch 1st player's location
        if lump_idx == IDX_THINGS {
            let tcount = self.thing_count();
            for idx in 0..tcount {
                let th = self.thing(idx);
                if th.type_code() == 1 {
                    self.amap_center = th.pos();
                    break;
                }
            }
        }
    }

    fn translate_automap_vertex(&self, orig_vertex: Vertex, painter: &dyn Painter) -> (i32, i32) {
        // scale the original coordinates
        let xs = ((orig_vertex.x - self.amap_center.x) as i32) * self.amap_zoom / 100;
        let ys = ((orig_vertex.y - self.amap_center.y) as i32) * self.amap_zoom / 100;
        // translate the scaled coordinates + mirror y
        let xt = xs + (painter.get_screen_width() / 2);
        let yt = (painter.get_screen_height() / 2) - ys;
        (xt, yt)
    }

    fn paint_cross(&self, painter: &mut dyn Painter, v: Vertex, color: RGB) {
        let (x, y) = self.translate_automap_vertex(v, painter);
        painter.draw_line(x - 1, y, x + 1, y, color);
        painter.draw_line(x, y - 1, x, y + 1, color);
    }

    fn vertex(&self, idx: usize) -> Vertex {
        let i = idx << 2;
        let bytes = self.lumps[IDX_VERTEXES].as_ref();
        Vertex {
            x: buf_to_i16(&bytes[(i + 0)..(i + 2)]) as i32,
            y: buf_to_i16(&bytes[(i + 2)..(i + 4)]) as i32,
        }
    }

    #[inline]
    fn line_count(&self) -> usize {
        self.lumps[IDX_LINEDEFS].len() / 14
    }

    fn linedef(&self, idx: usize) -> LineDef {
        let i = idx * 14;
        let bytes = self.lumps[IDX_LINEDEFS].as_ref();
        let vi1 = buf_to_u16(&bytes[(i + 0)..(i + 2)]);
        let vi2 = buf_to_u16(&bytes[(i + 2)..(i + 4)]);
        LineDef {
            v1: self.vertex(vi1 as usize),
            v2: self.vertex(vi2 as usize),
            flags: buf_to_u16(&bytes[(i + 4)..(i + 6)]),
            _line_type: buf_to_u16(&bytes[(i + 6)..(i + 8)]),
            _sector_tag: buf_to_u16(&bytes[(i + 8)..(i + 10)]),
            _right_side_def: buf_to_u16(&bytes[(i + 10)..(i + 12)]),
            _left_side_def: buf_to_u16(&bytes[(i + 12)..(i + 14)]),
        }
    }

    #[inline]
    fn thing_count(&self) -> usize {
        self.things.len() / 10
    }

    fn thing(&self, idx: usize) -> Thing {
        Thing::new(&self.things[(idx * 10)..(idx * 10 + 10)])
    }
}

//--------------------
//  Internal stuff

#[derive(Debug)]
struct LineDef {
    pub v1: Vertex,
    pub v2: Vertex,
    pub flags: u16,
    pub _line_type: u16,
    pub _sector_tag: u16,
    pub _right_side_def: u16,
    pub _left_side_def: u16,
}
