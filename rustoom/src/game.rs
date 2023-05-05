// Main DOOM game

use crate::map::*;
use crate::wad::*;
use crate::*;
use crate::{GraphicsLoop, Painter};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const DEFAULT_AUTOMAP_ZOOM_PERCENT: i32 = 10;
const AUTOMAP_ZOOM_STEP: i32 = 1;
const AUTOMAP_ZOOM_MIN: i32 = 5;
const AUTOMAP_ZOOM_MAX: i32 = 50;
const AUTOMAP_TRANSL_MULT: i32 = 10;

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

pub struct DoomGame {
    wad_data: WadData,
    scr_width: i32,
    scr_height: i32,
    _map_idx: usize,
    map: LevelMap,
    amap_zoom: i32,
    amap_center: Vertex,
}

impl DoomGame {
    pub fn new(wad_data: WadData, scr_width: i32, scr_height: i32) -> Result<DoomGame, String> {
        let mut game = DoomGame {
            wad_data: wad_data,
            scr_width,
            scr_height,
            _map_idx: 0,
            map: LevelMap::default(),
            amap_zoom: 0,
            amap_center: Vertex { x: 0, y: 0 },
        };
        game.load_map(0)?;
        Ok(game)
    }

    #[inline]
    pub fn get_map_count(&self) -> usize {
        self.wad_data.get_map_count()
    }

    pub fn load_map(&mut self, idx: usize) -> Result<(), String> {
        self._map_idx = idx;
        self.map = self.wad_data.get_map(idx)?;
        // compute automap zoom and offsets
        self.amap_zoom = DEFAULT_AUTOMAP_ZOOM_PERCENT;
        self.amap_center = self.map.v_orig.clone();
        Ok(())
    }

    //----------------

    fn paint_automap(&self, painter: &mut dyn Painter) {
        // clear screen
        painter.fill_rect(0, 0, self.scr_width, self.scr_height, BLACK);
        // draw a rectangle around the automap
        let (x1, y1) = self.translate_automap_vertex(&self.map.v_min);
        let (x2, y2) = self.translate_automap_vertex(&self.map.v_max);
        painter.draw_rect(x1, y1, x2 - x1 + 1, y2 - y1 + 1, DARK_GREY);
        // draw the automap
        for line in self.map.line_defs.iter() {
            let v1 = self.map.get_vertex(line.v1_idx);
            let v2 = self.map.get_vertex(line.v2_idx);
            let (x1, y1) = self.translate_automap_vertex(v1);
            let (x2, y2) = self.translate_automap_vertex(v2);

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
        for thing in self.map.things.iter() {
            let color = match thing.thing_type {
                1 => YELLOW,
                2 => ORANGE,
                3 => BLUE,
                4 => GREEN,
                _ => DARK_GREY,
            };
            self.paint_cross(painter, &thing.pos, color);
        }

        // draw the player location
        self.paint_cross(painter, &self.map.v_orig, WHITE);
    }

    fn paint_cross(&self, painter: &mut dyn Painter, v: &Vertex, color: RGB) {
        let (x, y) = self.translate_automap_vertex(v);
        painter.draw_line(x - 1, y, x + 1, y, color);
        painter.draw_line(x, y - 1, x, y + 1, color);
    }

    #[inline]
    fn translate_automap_vertex(&self, orig_vertex: &Vertex) -> (i32, i32) {
        // scale the original coordinates
        let xs = ((orig_vertex.x - self.amap_center.x) as i32) * self.amap_zoom / 100;
        let ys = ((orig_vertex.y - self.amap_center.y) as i32) * self.amap_zoom / 100;
        // translate the scaled coordinates + mirror y
        let xt = xs + (self.scr_width / 2);
        let yt = (self.scr_height / 2) - ys;
        (xt, yt)
    }

    fn handle_key_down(&mut self, key: &Keycode) {
        match key {
            Keycode::KpPlus => {
                if self.amap_zoom < AUTOMAP_ZOOM_MAX {
                    self.amap_zoom += AUTOMAP_ZOOM_STEP;
                }
            }
            Keycode::KpMinus => {
                if self.amap_zoom > AUTOMAP_ZOOM_MIN {
                    self.amap_zoom -= AUTOMAP_ZOOM_STEP;
                }
            }
            Keycode::Left => {
                self.amap_center.x -= (self.amap_zoom * AUTOMAP_TRANSL_MULT).min(50) as i16;
            }
            Keycode::Right => {
                self.amap_center.x += (self.amap_zoom * AUTOMAP_TRANSL_MULT).min(50) as i16;
            }
            Keycode::Up => {
                self.amap_center.y += (self.amap_zoom * AUTOMAP_TRANSL_MULT).min(50) as i16;
            }
            Keycode::Down => {
                self.amap_center.y -= (self.amap_zoom * AUTOMAP_TRANSL_MULT).min(50) as i16;
            }
            Keycode::PageUp => {
                let cnt = self.get_map_count();
                let idx = (self._map_idx + cnt - 1) % cnt;
                self.load_map(idx).unwrap();
            }
            Keycode::PageDown => {
                let cnt = self.get_map_count();
                let idx = (self._map_idx + 1) % cnt;
                self.load_map(idx).unwrap();
            }
            _ => {}
        }
    }

    fn handle_key_up(&mut self, _key: &Keycode) {
        // TODO implement this ...
    }
}

impl GraphicsLoop for DoomGame {
    fn handle_event(&mut self, event: &Event) -> bool {
        // check keys
        match event {
            Event::KeyDown {
                keycode: Some(key), ..
            } => {
                self.handle_key_down(key);
            }
            Event::KeyUp {
                keycode: Some(key), ..
            } => {
                self.handle_key_up(key);
            }
            _ => {}
        }
        true
    }

    fn update_state(&mut self, _elapsed_time: f64) -> bool {
        true
    }

    fn paint(&self, painter: &mut dyn Painter) {
        self.paint_automap(painter);
    }
}
