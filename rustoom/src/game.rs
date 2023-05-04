// Main DOOM game

use crate::map::*;
use crate::wad::*;
use crate::{GraphicsLoop, Painter, RGB};
use sdl2::event::Event;

pub struct DoomGame {
    wad_data: Box<WadData>,
    scr_width: i32,
    scr_height: i32,
    _map_idx: usize,
    map: Box<LevelMap>,
}

impl DoomGame {
    pub fn new(wad_data: WadData, scr_width: i32, scr_height: i32) -> Result<DoomGame, String> {
        let first_map = wad_data.get_map(0)?;
        Ok(DoomGame {
            wad_data: Box::from(wad_data),
            scr_width,
            scr_height,
            _map_idx: 0,
            map: Box::from(first_map),
        })
    }

    #[inline]
    pub fn get_map_count(&self) -> usize {
        self.wad_data.get_map_count()
    }

    pub fn load_map(&mut self, idx: usize) -> Result<(), String> {
        self.map = Box::from(self.wad_data.get_map(idx)?);
        Ok(())
    }

    // TODO to be continued ...
}

impl GraphicsLoop for DoomGame {
    fn handle_event(&mut self, _event: &Event) -> bool {
        true
    }

    fn update_state(&mut self, _elapsed_time: f64) -> bool {
        true
    }

    fn paint(&self, painter: &mut dyn Painter) {
        // TEMPORARY - draw random pixels
        for y in 0..self.scr_height {
            for x in 0..self.scr_width {
                let r: u8 = fastrand::u8(0..=255);
                let g: u8 = fastrand::u8(0..=255);
                let b: u8 = fastrand::u8(0..=255);
                painter.draw_pixel(x, y, RGB::from(r, g, b));
            }
        }
    }
}
