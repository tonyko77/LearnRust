// Main DOOM game

/*
TODO:
    - add Player/Actor class - see https://github.com/amroibrahim/DIYDoom/tree/master/DIYDOOM/Notes005/notes
    - add BSP code - see https://github.com/amroibrahim/DIYDoom/tree/master/DIYDOOM/Notes007/notes
    - doc comments !!
 */

use crate::utils::lump_name_from_hash;
use crate::*;
use crate::{GraphicsLoop, Painter};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct DoomGame {
    wad_data: WadData,
    map_idx: usize,
    map: LevelMap,
    _x_mode: i32,
    _x_idx: usize,
    _sprite_key: u64,
    _sprite_gfx: PixMap,
    _tex_gfx: Texture,
    _pink_outline: bool,
    _bsp_node_idx: usize,
    _ss_idx: usize,
}

impl DoomGame {
    pub fn new(wad_data: WadData) -> Result<DoomGame, String> {
        let map = wad_data.load_map(0);
        let mut engine = DoomGame {
            wad_data,
            map_idx: 9999,
            map,
            _x_mode: 0,
            _x_idx: 0,
            _sprite_key: 0,
            _sprite_gfx: PixMap::new_empty(),
            _tex_gfx: Texture::new(0, 0, 0),
            _pink_outline: false,
            _bsp_node_idx: 0,
            _ss_idx: 0,
        };
        engine.load_map(0);
        engine.update_state(0.0);
        Ok(engine)
    }

    pub fn load_map(&mut self, idx: usize) {
        if self.map_idx != idx && idx < self.wad_data.map_count() {
            self.map_idx = idx;
            self.map = self.wad_data.load_map(idx);
            self._bsp_node_idx = self.map.bsp().node_count() - 1;
        }
    }

    //----------------

    fn paint_graphics(&self, painter: &mut dyn Painter, hdr: &str, text_color: RGB) {
        const CC: u16 = 140;
        let bad_width =
            ((self._sprite_gfx.width() + CC) as i32) + self._sprite_gfx.x_offset() > painter.get_screen_width();
        let bad_height =
            ((self._sprite_gfx.height() + CC) as i32) + self._sprite_gfx.y_offset() > painter.get_screen_height();
        let xyc = if bad_width || bad_height { CC / 3 } else { CC } as i32;

        // draw center lines
        let sw = painter.get_screen_width();
        let sh = painter.get_screen_height();
        painter.draw_horiz_line(0, sw, xyc, VERY_DARK_GREY);
        painter.draw_vert_line(xyc, 0, sh, VERY_DARK_GREY);

        // draw sprite
        self._sprite_gfx.paint(xyc, xyc, painter, self.wad_data.palette());
        // draw rectangle around the sprite
        let (w, h) = (self._sprite_gfx.width() as i32, self._sprite_gfx.height() as i32);
        if self._pink_outline {
            let xo = xyc - 1 + self._sprite_gfx.x_offset();
            let yo = xyc - 1 + self._sprite_gfx.y_offset();
            painter.draw_rect(xo, yo, w + 2, h + 2, PINK);
        }

        // draw sprite name
        let name = lump_name_from_hash(self._sprite_key);
        let text = format!("{hdr}: {name} --> {w} x {h}");
        self.wad_data.font().draw_text(3, 3, &text, text_color, painter);
    }

    fn paint_texture(&self, painter: &mut dyn Painter, hdr: &str, text_color: RGB) {
        const CC: i32 = 120;
        // draw center lines
        let sw = painter.get_screen_width();
        let sh = painter.get_screen_height();
        painter.draw_horiz_line(0, sw, CC, VERY_DARK_GREY);
        painter.draw_vert_line(CC, 0, sh, VERY_DARK_GREY);

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

    fn paint_bsp(&self, painter: &mut dyn Painter) {
        self.map.paint_automap(painter, self.wad_data.font());

        // paint the subsectors
        let player = &self.map.get_player();
        let ssects = self.map.bsp().locate_player(player);
        let idx = self._ss_idx % ssects.len();

        let segs = &ssects[idx];
        for seg in segs.iter() {
            let v1 = self.map.translate_automap_vertex(seg.start, painter);
            let v2 = self.map.translate_automap_vertex(seg.end, painter);
            painter.draw_line(v1.x, v1.y, v2.x, v2.y, GREY);
        }
        let text = format!("SSECT {idx} / {} -> {} segs", ssects.len(), segs.len());
        self.wad_data.font().draw_text(3, 26, &text, GREY, painter);

        // get the bsp node
        let idx = self._bsp_node_idx;
        let node = self.map.bsp().node(idx);
        // paint left rect
        self.paint_rect(painter, &node.left_box_bl, &node.left_box_tr, PINK);
        // paint right rect
        self.paint_rect(painter, &node.right_box_bl, &node.right_box_tr, GREEN);

        // paint dividing vector
        let ov = self.map.translate_automap_vertex(node.vect_orig, painter);
        let fff = node.vect_dir.add(&node.vect_orig);
        let dv = self.map.translate_automap_vertex(fff, painter);
        painter.draw_line(ov.x, ov.y, dv.x, dv.y, WHITE);
        painter.fill_circle(dv.x, dv.y, 1, WHITE);

        let text = format!("BSP node {idx} / {}", self.map.bsp().node_count());
        self.wad_data.font().draw_text(3, 15, &text, WHITE, painter);
    }

    fn paint_rect(&self, painter: &mut dyn Painter, v1: &Vertex, v2: &Vertex, color: RGB) {
        let tv1 = self.map.translate_automap_vertex(*v1, painter);
        let tv2 = self.map.translate_automap_vertex(*v2, painter);
        let x1 = Ord::min(tv1.x, tv2.x);
        let y1 = Ord::min(tv1.y, tv2.y);
        let x2 = Ord::max(tv1.x, tv2.x);
        let y2 = Ord::max(tv1.y, tv2.y);
        painter.draw_rect(x1, y1, x2 - x1 + 1, y2 - y1 + 1, color);
    }

    fn bsp_move(&mut self, go_right: bool) {
        let idx = self._bsp_node_idx;
        let node = self.map.bsp().node(idx);
        let next = if go_right { node.right_child } else { node.left_child };
        self._bsp_node_idx = if (next & 0x8000) == 0 {
            next as usize
        } else {
            self.map.bsp().node_count() - 1
        };
    }

    fn handle_key_down(&mut self, key: &Keycode) {
        match key {
            Keycode::KpPlus => {
                self.map.zoom_automap(1);
            }
            Keycode::KpMinus => {
                self.map.zoom_automap(-1);
            }
            Keycode::KpDivide => {
                self.bsp_move(false);
            }
            Keycode::KpMultiply => {
                self.bsp_move(true);
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
                self._x_idx = self._x_idx.wrapping_sub(1);
            }
            Keycode::PageDown => {
                self._x_idx = self._x_idx.wrapping_add(1);
            }
            Keycode::Home => {
                self._x_idx = 0;
            }
            Keycode::End => {
                self._x_mode += 1;
            }
            Keycode::Insert => {
                self._ss_idx = self._ss_idx.wrapping_sub(1);
            }
            Keycode::Delete => {
                self._ss_idx = self._ss_idx.wrapping_add(1);
            }
            Keycode::Backspace => {
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
            0 => self.paint_bsp(painter),
            1 => self.paint_graphics(painter, "PATCH", YELLOW),
            2 => self.paint_graphics(painter, "FLAT", CYAN),
            3 => self.paint_texture(painter, "TEXTURE", WHITE),
            _ => {}
        }
    }
}
