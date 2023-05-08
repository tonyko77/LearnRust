// Main DOOM game

/*
TODO:
    - use Bytes in: Map/MapManager, PixMap etc:
        * in PixMap:
            - keep raw Bytes + a type flag (enum)
            - decode on paint :) (should be easy)
        * in MapManager:
            - store the Bytes for each lump !!
            - some lumps don't really require extraction => just keep them as they are
        * in textures:
            - keep PNAMES directly as bytes! => no extra mem
            - maybe even TEXTURES as bytes (+ maybe some indexes, for each texture)
    - move automap code to separate module
        - then: move all DEMO code separately (bottom part of this source file is OK)
    - add BSP code
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
    game: DoomGameData,
    map_idx: usize,
    map: MapData,
    _x_mode: i32,
    _x_idx: usize,
    _sprite_key: u64,
    _sprite_gfx: PixMap,
}

impl DoomGame {
    pub fn new(wad: WadData) -> Result<DoomGame, String> {
        let game = DoomGameData::build(wad)?;
        let mut engine = DoomGame {
            game,
            map_idx: 9999,
            map: MapData::new(""),
            _x_mode: 0,
            _x_idx: 0,
            _sprite_key: 0,
            _sprite_gfx: PixMap::new_empty(),
        };
        engine.update_state(0.0);
        Ok(engine)
    }

    pub fn load_map(&mut self, idx: usize) {
        if self.map_idx != idx && idx < self.game.map_count() {
            self.map_idx = idx;
            self.map = self.game.map(idx).clone();
        }
    }

    //----------------

    fn paint_graphics(&self, painter: &mut dyn Painter, hdr: &str, color: RGB) {
        // draw center lines
        let sw = painter.get_screen_width();
        let sh = painter.get_screen_height();
        let xc = 180;
        let yc = 180;
        painter.draw_horiz_line(0, sw, yc, DARK_GREY);
        painter.draw_vert_line(xc, 0, sh, DARK_GREY);
        // draw sprite
        self._sprite_gfx.paint(xc, yc, painter, self.game.palette());
        // draw sprite name
        let name = lump_name_from_hash(self._sprite_key);
        let w = self._sprite_gfx.width();
        let h = self._sprite_gfx.height();
        let text = format!("{hdr}: {name} --> {w} x {h}");
        self.game.font().draw_text(3, 3, &text, color, painter);
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
                let midx = self._x_idx % self.game.map_count();
                self.load_map(midx);
            }
            1 => {
                let keys = self.game.graphics().dbg_patch_keys();
                let kidx = self._x_idx % keys.len();
                let k = keys[kidx];
                self._sprite_gfx = self.game.graphics().get_patch(k).unwrap();
                self._sprite_key = k;
            }
            2 => {
                let keys = self.game.graphics().dbg_flat_keys();
                let kidx = self._x_idx % keys.len();
                let k = keys[kidx];
                self._sprite_gfx = self.game.graphics().get_flat(k).unwrap();
                self._sprite_key = k;
            }
            3 => {
                let keys = self.game.graphics().dbg_texture_keys();
                let kidx = self._x_idx % keys.len();
                let k = keys[kidx];
                self._sprite_gfx = self.game.graphics().get_texture(k).unwrap();
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
            3 => self.paint_graphics(painter, "TEXTURE", WHITE),
            _ => self.map.paint_automap(painter, self.game.font()),
        }
    }
}
