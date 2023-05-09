// Main DOOM game

/*
TODO:
    - add Player/Actor class - see https://github.com/amroibrahim/DIYDoom/tree/master/DIYDOOM/Notes005/notes
    - add BSP code - see https://github.com/amroibrahim/DIYDoom/tree/master/DIYDOOM/Notes007/notes
    - doc comments !!
 */

use crate::map::*;
use crate::utils::lump_name_from_hash;
use crate::wad::*;
use crate::*;
use crate::{GraphicsLoop, Painter};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct DoomGame {
    wad_data: WadData,
    map_idx: usize,
    map: MapData,
    _x_mode: i32,
    _x_idx: usize,
    _sprite_key: u64,
    _sprite_gfx: PixMap,
    _tex_gfx: Texture,
    _pink_outline: bool,
}

impl DoomGame {
    pub fn new(wad_data: WadData) -> Result<DoomGame, String> {
        let mut engine = DoomGame {
            wad_data,
            map_idx: 9999,
            map: MapData::new(""),
            _x_mode: 0,
            _x_idx: 0,
            _sprite_key: 0,
            _sprite_gfx: PixMap::new_empty(),
            _tex_gfx: Texture::new(0, 0, 0),
            _pink_outline: false,
        };
        engine.update_state(0.0);
        Ok(engine)
    }

    pub fn load_map(&mut self, idx: usize) {
        if self.map_idx != idx && idx < self.wad_data.map_count() {
            self.map_idx = idx;
            self.map = self.wad_data.map(idx).clone();
        }
    }

    //----------------

    fn paint_graphics(&self, painter: &mut dyn Painter, hdr: &str, text_color: RGB) {
        const CC: i32 = 140;
        // draw center lines
        let sw = painter.get_screen_width();
        let sh = painter.get_screen_height();
        painter.draw_horiz_line(0, sw, CC, DARK_GREY);
        painter.draw_vert_line(CC, 0, sh, DARK_GREY);

        // draw sprite
        self._sprite_gfx.paint(CC, CC, painter, self.wad_data.palette());
        // draw rectangle around the sprite
        let (w, h) = (self._sprite_gfx.width() as i32, self._sprite_gfx.height() as i32);
        if self._pink_outline {
            let xo = CC - 1 + self._sprite_gfx.x_offset();
            let yo = CC - 1 + self._sprite_gfx.y_offset();
            painter.draw_rect(xo, yo, w + 2, h + 2, PINK);
        }

        // draw sprite name
        let name = lump_name_from_hash(self._sprite_key);
        let text = format!("{hdr}: {name} --> {w} x {h}");
        self.wad_data.font().draw_text(3, 3, &text, text_color, painter);
    }

    fn paint_texture(&self, painter: &mut dyn Painter, hdr: &str, text_color: RGB) {
        const CC: i32 = 60;
        // draw center lines
        let sw = painter.get_screen_width();
        let sh = painter.get_screen_height();
        painter.draw_horiz_line(0, sw, CC, DARK_GREY);
        painter.draw_vert_line(CC, 0, sh, DARK_GREY);

        // draw texture
        let (w, h) = (self._tex_gfx.width() as i32, self._tex_gfx.height() as i32);
        painter.fill_rect(CC, CC, w, h, PINK);
        self._tex_gfx.paint(CC, CC, painter, self.wad_data.palette());
        // draw rectangle around the texture
        if self._pink_outline {
            painter.draw_rect(CC - 1, CC - 1, w + 2, h + 2, PINK);
        }

        // draw texture name
        let name = lump_name_from_hash(self._sprite_key);
        let text = format!("{hdr}: {name} --> {w} x {h}");
        self.wad_data.font().draw_text(3, 3, &text, text_color, painter);
    }

    fn handle_key_down(&mut self, key: &Keycode) {
        match key {
            Keycode::KpPlus => {
                self.map.zoom_automap(1);
            }
            Keycode::KpMinus => {
                self.map.zoom_automap(-1);
            }
            Keycode::Left => {
                self.map.move_automap(-50, 0);
            }
            Keycode::Right => {
                self.map.move_automap(50, 0);
            }
            Keycode::Up => {
                self.map.move_automap(0, 50);
            }
            Keycode::Down => {
                self.map.move_automap(0, -50);
            }
            Keycode::PageUp => {
                if self._x_idx > 0 {
                    self._x_idx -= 1;
                }
            }
            Keycode::PageDown => {
                if self._x_idx < usize::MAX {
                    self._x_idx += 1;
                }
            }
            Keycode::Home => {
                self._x_idx = 0;
            }
            Keycode::End => {
                self._x_mode = (self._x_mode + 1) & 0x03;
                self._x_idx = 0;
            }
            Keycode::Insert => {
                self._pink_outline = !self._pink_outline;
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
            Event::KeyDown { keycode: Some(key), .. } => {
                self.handle_key_down(key);
            }
            Event::KeyUp { keycode: Some(key), .. } => {
                self.handle_key_up(key);
            }
            _ => {}
        }
        true
    }

    fn update_state(&mut self, _elapsed_time: f64) -> bool {
        // update map
        match self._x_mode {
            0 => {
                let midx = self._x_idx % self.wad_data.map_count();
                self.load_map(midx);
            }
            1 => {
                let keys = self.wad_data.graphics().dbg_patch_keys();
                let kidx = self._x_idx % keys.len();
                let k = keys[kidx];
                self._sprite_gfx = self
                    .wad_data
                    .graphics()
                    .get_patch(k)
                    .expect(format!("texture not found: {kidx} >= {}", keys.len()).as_str());
                self._sprite_key = k;
            }
            2 => {
                let keys = self.wad_data.graphics().dbg_flat_keys();
                let kidx = self._x_idx % keys.len();
                let k = keys[kidx];
                self._sprite_gfx = self
                    .wad_data
                    .graphics()
                    .get_flat(k)
                    .expect(format!("texture not found: {kidx} >= {}", keys.len()).as_str());
                self._sprite_key = k;
            }
            3 => {
                let keys = self.wad_data.graphics().dbg_texture_keys();
                let kidx = self._x_idx % keys.len();
                let k = keys[kidx];
                self._tex_gfx = self
                    .wad_data
                    .graphics()
                    .get_texture(k)
                    .expect(format!("texture not found: {kidx} >= {}", keys.len()).as_str());
                self._sprite_key = k;
            }
            _ => {
                self._x_mode = 0;
            }
        }

        // update sprite
        true
    }

    fn paint(&self, painter: &mut dyn Painter) {
        painter.fill_rect(0, 0, painter.get_screen_width(), painter.get_screen_height(), BLACK);
        match self._x_mode {
            1 => self.paint_graphics(painter, "PATCH", YELLOW),
            2 => self.paint_graphics(painter, "FLAT", CYAN),
            3 => self.paint_texture(painter, "TEXTURE", WHITE),
            _ => self.map.paint_automap(painter, self.wad_data.font()),
        }
    }
}
