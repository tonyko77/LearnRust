// Main DOOM game

use crate::*;
use crate::map::*;
use crate::wad::*;
use crate::{GraphicsLoop, Painter};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const DEFAULT_AUTOMAP_ZOOM_PERCENT: i32 = 10;
const AUTOMAP_ZOOM_STEP: i32 = 1;
const AUTOMAP_ZOOM_MIN: i32 = 3;
const AUTOMAP_ZOOM_MAX: i32 = 30;

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
    automap_zoom: i32,
    automap_offs_x: i32,
    automap_offs_y: i32,
}

impl DoomGame {
    pub fn new(wad_data: WadData, scr_width: i32, scr_height: i32) -> Result<DoomGame, String> {
        let mut game = DoomGame {
            wad_data: wad_data,
            scr_width,
            scr_height,
            _map_idx: 0,
            map: LevelMap::default(),
            automap_zoom: 0,
            automap_offs_x: 0,
            automap_offs_y: 0,
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
        self.automap_zoom = DEFAULT_AUTOMAP_ZOOM_PERCENT;
        Ok(())
    }

    //----------------

    fn paint_automap(&self, painter: &mut dyn Painter) {
        // clear screen
        painter.fill_rect(0, 0, self.scr_width, self.scr_height, BLACK);
        // draw a rectangle around the automap
        let (x1, y1) = self.translate_automap_vertex(&self.map.v_min);
        let (x2, y2) = self.translate_automap_vertex(&self.map.v_max);
        painter.draw_rect(x1, y1, x2-x1+1, y2-y1+1, DARK_GREY);
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
    }

    #[inline]
    fn translate_automap_vertex(&self, orig_vertex: &Vertex) -> (i32, i32) {
        // TODO variable translation + scaling !!
        let x = ((orig_vertex.x - self.map.v_min.x) as i32) * self.automap_zoom / 100;
        let yf = ((orig_vertex.y - self.map.v_min.y) as i32) * self.automap_zoom / 100;
        let y = self.scr_height - yf - 1;
        (x + self.automap_offs_x, y + self.automap_offs_y)
    }

    fn handle_key_down(&mut self, key: &Keycode) {
        match key {
            Keycode::KpPlus => {
                if self.automap_zoom < AUTOMAP_ZOOM_MAX {
                    self.automap_zoom += AUTOMAP_ZOOM_STEP;
                }
            },
            Keycode::KpMinus => {
                if self.automap_zoom > AUTOMAP_ZOOM_MIN {
                    self.automap_zoom -= AUTOMAP_ZOOM_STEP;
                }
            },
            Keycode::Left => {
                self.automap_offs_x += 10;
            },
            Keycode::Right => {
                self.automap_offs_x -= 10;
            },
            Keycode::Up => {
                self.automap_offs_y += 10;
            },
            Keycode::Down => {
                self.automap_offs_y -= 10;
            },
            Keycode::PageUp => {
                let cnt = self.get_map_count();
                let idx = (self._map_idx + cnt - 1) % cnt;
                self.load_map(idx).unwrap();
            },
            Keycode::PageDown => {
                let cnt = self.get_map_count();
                let idx = (self._map_idx + 1) % cnt;
                self.load_map(idx).unwrap();
            },
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
            Event::KeyDown { keycode: Some(key), .. } => {
                self.handle_key_down(key);
            },
            Event::KeyUp { keycode: Some(key), .. } => {
                self.handle_key_up(key);
            },
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
