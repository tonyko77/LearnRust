// Main DOOM game

/*
TODO:
    - add Angle class - see https://github.com/amroibrahim/DIYDoom/tree/master/DIYDOOM/Notes010/notes
    - add Screen class - to keep resolution + FOV/aspect ratio calculations
    - refactor: move all rendering code into LevelMap !!
    - continuous keys (movement, rotation)
    - automap: draw arrow for player + use yellow/choco colors correctly.
    - add Player/Actor class - see https://github.com/amroibrahim/DIYDoom/tree/master/DIYDOOM/Notes005/notes
    - doc comments !!
 */

use crate::levelmap::LevelMap;
use crate::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const FLAG_AUTOMAP: u32 = 1 << 0;

pub struct DoomGame {
    wad_data: WadData,
    _screen: Screen, // TODO use this
    map_idx: usize,
    map: LevelMap,
    gameplay_flags: u32,
}

impl DoomGame {
    pub fn new(wad_data: WadData, _screen: Screen) -> Result<DoomGame, String> {
        let map = wad_data.load_map(0);
        let mut engine = DoomGame {
            wad_data,
            _screen,
            map_idx: 9999,
            map,
            gameplay_flags: 0,
        };
        engine.load_map(0);
        engine.update_state(0.0);
        Ok(engine)
    }

    pub fn load_map(&mut self, idx: usize) {
        if self.map_idx != idx && idx < self.wad_data.map_count() {
            self.map_idx = idx;
            self.map = self.wad_data.load_map(idx);
        }
    }

    //----------------

    fn paint_bsp(&self, painter: &mut dyn Painter) {
        self.map.paint_automap(painter, self.wad_data.font());

        // TODO TEMPORARY: paint the subsectors
        let player = &self.map.get_player();
        let segs = self.map.locate_player(player);
        for seg in segs.iter() {
            let v1 = self.map.translate_automap_vertex(seg.start, painter);
            let v2 = self.map.translate_automap_vertex(seg.end, painter);
            painter.draw_line(v1.x, v1.y, v2.x, v2.y, GREY);
        }
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
                // TODO temp
                if self.map_idx > 0 {
                    let new_map_idx = self.map_idx - 1;
                    self.load_map(new_map_idx);
                }
            }
            Keycode::PageDown => {
                // TODO temp
                if self.map_idx < self.wad_data.map_count() - 1 {
                    let new_map_idx = self.map_idx + 1;
                    self.load_map(new_map_idx);
                }
            }
            Keycode::Home => {
                self.load_map(0);
            }
            Keycode::Tab => {
                self.gameplay_flags = self.gameplay_flags ^ FLAG_AUTOMAP;
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
        // TODO proper implementation
        true
    }

    fn paint(&self, painter: &mut dyn Painter) {
        painter.fill_rect(0, 0, painter.get_screen_width(), painter.get_screen_height(), BLACK);
        self.paint_bsp(painter);
    }
}
