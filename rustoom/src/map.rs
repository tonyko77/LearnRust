//! Parse and build maps from the WAD.

use crate::utils::*;
use crate::*;
use bytes::Bytes;

// Indexes for various MapData lumps
const IDX_THINGS: usize = 0;
const IDX_LINEDEFS: usize = 1;
const IDX_SIDEDEFS: usize = 2;
const IDX_VERTEXES: usize = 3;
const IDX_SEGS: usize = 4;
const IDX_SSECTORS: usize = 5;
const IDX_NODES: usize = 6;
const IDX_SECTORS: usize = 7;
const IDX_REJECT: usize = 8;
const IDX_BLOCKMAP: usize = 9;
const LUMP_CNT: usize = 10;

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

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub x: i32,
    pub y: i32,
}

pub struct MapData {
    name: String,
    lumps: Box<[Bytes; LUMP_CNT]>,
    amap_zoom: i32,
    amap_center: Vertex,
}

impl Clone for MapData {
    fn clone(&self) -> Self {
        let lumps: Box<[Bytes; LUMP_CNT]> = Box::new((*self.lumps).clone());
        Self {
            name: self.name.clone(),
            lumps,
            amap_zoom: self.amap_zoom,
            amap_center: self.amap_center.clone(),
        }
    }
}

impl MapData {
    pub fn new(name: &str) -> Self {
        let lumps: Box<[Bytes; LUMP_CNT]> = Box::new(Default::default());
        Self {
            name: name.to_string(),
            lumps,
            amap_zoom: DEFAULT_AUTOMAP_ZOOM,
            amap_center: Vertex { x: 0, y: 0 },
        }
    }

    pub fn add_lump(&mut self, lump: &str, bytes: &Bytes) -> bool {
        let idx = match lump {
            "VERTEXES" => IDX_VERTEXES,
            "LINEDEFS" => IDX_LINEDEFS,
            "THINGS" => IDX_THINGS,
            "SIDEDEFS" => IDX_SIDEDEFS,
            "SEGS" => IDX_SEGS,
            "SSECTORS" => IDX_SSECTORS,
            "NODES" => IDX_NODES,
            "SECTORS" => IDX_SECTORS,
            "REJECT" => IDX_REJECT,
            "BLOCKMAP" => IDX_BLOCKMAP,
            _ => usize::MAX,
        };
        // check if it was a valid lump; if not => return false, to signal the end of the map lumps
        // (all the lumps of one map are consecutive, so if we get an invalid one => we're done with this map)
        if idx < LUMP_CNT {
            self.do_add_lump(idx, bytes.clone());
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn is_complete(&self) -> bool {
        self.lumps.iter().all(|b| b.len() > 0)
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

    pub fn move_automap(&mut self, dx: i32, dy: i32) {
        self.amap_center.x += dx;
        self.amap_center.y += dy;
    }

    pub fn zoom_automap(&mut self, dzoom: i32) {
        let new_zoom = self.amap_zoom + dzoom;
        self.amap_zoom = new_zoom.max(AUTOMAP_ZOOM_MIN).min(AUTOMAP_ZOOM_MAX);
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
        self.lumps[IDX_THINGS].len() / 10
    }

    fn thing(&self, idx: usize) -> Thing {
        let bytes = &self.lumps[IDX_THINGS];
        Thing::new(&bytes[(idx * 10)..(idx * 10 + 10)])
    }
}

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
