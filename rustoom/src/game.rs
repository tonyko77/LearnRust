//! My wacky DOOM game engine :P

/*
>> STILL TO DO <<
    - player real movement, on the map (noclip for now)
    - paint proper SKY for map, from graphics, based on user rotation !!
    - TEST automap correctness: draw arrow for player + use yellow/choco colors correctly.
    - NEXT: limit the segs to only visible ones !!
        - see https://github.com/amroibrahim/DIYDoom/tree/master/DIYDOOM/Notes010/notes
    - player - NO LONGER move through walls
    - add Player/Actor class - see https://github.com/amroibrahim/DIYDoom/tree/master/DIYDOOM/Notes005/notes
    - doc comments !!
 */

use crate::level::ActiveLevel;
use crate::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

// key flags - for ALL keys (some only act once on press => they need 2 bits !!)
const KEY_TURN_LEFT: u32 = 1 << 0;
const KEY_TURN_RIGHT: u32 = 1 << 1;
const KEY_STRAFE_LEFT: u32 = 1 << 2;
const KEY_STRAFE_RIGHT: u32 = 1 << 3;
const KEY_MOVE_FWD: u32 = 1 << 4;
const KEY_MOVE_BACK: u32 = 1 << 5;
const KEY_AUTOMAP: u32 = 1 << 6;
const KEY_AUTOMAP_ACTED: u32 = 1 << 7;
const KEY_ZOOM_IN: u32 = 1 << 8;
const KEY_ZOOM_OUT: u32 = 1 << 9;
const KEY_USE: u32 = 1 << 10;
const KEY_SHOOT: u32 = 1 << 11;

// other gameplay flags
const FLAG_AUTOMAP: u32 = 1 << 0;

const AMAP_MOVE_SPEED: f64 = 500.0;
const AMAP_ZOOM_SPEED: f64 = 0.0625;

pub struct DoomGame {
    cfg: GameConfig,
    map_idx: usize,
    level: ActiveLevel,
    key_flags: u32,
    gameplay_flags: u32,

    // TODO remove these and make player/amap pos a "f64" vertex !!
    // also, move speed constants + calculations to ActiveLevel
    amap_x_delta: f64,
    amap_y_delta: f64,
}

impl DoomGame {
    pub fn new(cfg: GameConfig) -> Result<DoomGame, String> {
        let level = ActiveLevel::new(cfg.clone(), 0);
        let mut engine = DoomGame {
            cfg,
            map_idx: 0,
            level,
            key_flags: 0,
            gameplay_flags: FLAG_AUTOMAP,
            amap_x_delta: 0.0,
            amap_y_delta: 0.0,
        };
        engine.load_map(0);
        engine.update_state(0.0);
        Ok(engine)
    }

    pub fn load_map(&mut self, idx: usize) {
        if self.map_idx != idx && idx < self.cfg.wad().map_count() {
            self.map_idx = idx;
            self.level = ActiveLevel::new(self.cfg.clone(), idx);
        }
    }
}

impl GraphicsLoop for DoomGame {
    fn handle_event(&mut self, event: &Event) -> bool {
        // check keys
        match event {
            Event::KeyDown { keycode: Some(key), .. } => {
                match key {
                    Keycode::Tab => self.key_flags |= KEY_AUTOMAP,
                    Keycode::KpPlus => self.key_flags |= KEY_ZOOM_IN,
                    Keycode::KpMinus => self.key_flags |= KEY_ZOOM_OUT,
                    Keycode::Left => self.key_flags |= KEY_TURN_LEFT,
                    Keycode::Right => self.key_flags |= KEY_TURN_RIGHT,
                    Keycode::Up | Keycode::W => self.key_flags |= KEY_MOVE_FWD,
                    Keycode::Down | Keycode::S => self.key_flags |= KEY_MOVE_BACK,
                    Keycode::A => self.key_flags |= KEY_STRAFE_LEFT,
                    Keycode::D => self.key_flags |= KEY_STRAFE_RIGHT,
                    Keycode::Space | Keycode::E => self.key_flags |= KEY_USE,
                    Keycode::RCtrl | Keycode::LAlt => self.key_flags |= KEY_SHOOT,
                    Keycode::PageUp => {
                        // TODO temp
                        if self.map_idx > 0 {
                            let new_map_idx = self.map_idx - 1;
                            self.load_map(new_map_idx);
                        }
                    }
                    Keycode::PageDown => {
                        // TODO temp
                        if self.map_idx < self.cfg.wad().map_count() - 1 {
                            let new_map_idx = self.map_idx + 1;
                            self.load_map(new_map_idx);
                        }
                    }
                    _ => {}
                }
            }
            Event::KeyUp { keycode: Some(key), .. } => match key {
                Keycode::Tab => self.key_flags &= !(KEY_AUTOMAP | KEY_AUTOMAP_ACTED),
                Keycode::KpPlus => self.key_flags &= !KEY_ZOOM_IN,
                Keycode::KpMinus => self.key_flags &= !KEY_ZOOM_OUT,
                Keycode::Left => self.key_flags &= !KEY_TURN_LEFT,
                Keycode::Right => self.key_flags &= !KEY_TURN_RIGHT,
                Keycode::Up | Keycode::W => self.key_flags &= !KEY_MOVE_FWD,
                Keycode::Down | Keycode::S => self.key_flags &= !KEY_MOVE_BACK,
                Keycode::A => self.key_flags &= !KEY_STRAFE_LEFT,
                Keycode::D => self.key_flags &= !KEY_STRAFE_RIGHT,
                Keycode::Space | Keycode::E => self.key_flags &= KEY_USE,
                Keycode::RCtrl | Keycode::LAlt => self.key_flags |= KEY_SHOOT,
                _ => {}
            },
            _ => {}
        }
        true
    }

    fn update_state(&mut self, elapsed_time: f64) -> bool {
        // enable/disable automap
        if self.key_flags & (KEY_AUTOMAP | KEY_AUTOMAP_ACTED) == KEY_AUTOMAP {
            self.gameplay_flags ^= FLAG_AUTOMAP;
            self.key_flags |= KEY_AUTOMAP_ACTED;
        }

        // update movement deltas
        if self.gameplay_flags & FLAG_AUTOMAP != 0 {
            // in automap mode
            let mut zoom: f64 = 0.0;
            if self.key_flags & KEY_ZOOM_IN != 0 {
                zoom = AMAP_ZOOM_SPEED * elapsed_time;
            }
            if self.key_flags & KEY_ZOOM_OUT != 0 {
                zoom = -AMAP_ZOOM_SPEED * elapsed_time;
            }
            if self.key_flags & (KEY_TURN_LEFT | KEY_STRAFE_LEFT) != 0 {
                self.amap_x_delta -= AMAP_MOVE_SPEED * elapsed_time;
            }
            if self.key_flags & (KEY_TURN_RIGHT | KEY_STRAFE_RIGHT) != 0 {
                self.amap_x_delta += AMAP_MOVE_SPEED * elapsed_time;
            }
            if self.key_flags & KEY_MOVE_FWD != 0 {
                self.amap_y_delta += AMAP_MOVE_SPEED * elapsed_time;
            }
            if self.key_flags & KEY_MOVE_BACK != 0 {
                self.amap_y_delta -= AMAP_MOVE_SPEED * elapsed_time;
            }
            // update automap
            let x: i32 = self.amap_x_delta as i32;
            let y: i32 = self.amap_y_delta as i32;
            self.level.update_automap(x, y, zoom);
            self.amap_x_delta -= x as f64;
            self.amap_y_delta -= y as f64;
        } else {
            // in 3D view mode
            // TODO implement this ...
        }

        // update view

        true
    }

    fn paint(&self, painter: &mut dyn Painter) {
        if self.gameplay_flags & FLAG_AUTOMAP != 0 {
            self.level.paint_automap(painter);
        } else {
            self.level.paint_3d_view(painter);
        }
    }
}
