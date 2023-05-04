// Main DOOM game

use sdl2::event::Event;
use crate::{wad, Painter, GraphicsLoop, RGB};

pub struct DoomGame {
    wad_data: wad::WadData,
    scr_width: i32,
    scr_height: i32,
    // TODO to be continued ...
}

impl DoomGame {
    pub fn new(wad_data: wad::WadData, scr_width: i32, scr_height: i32) -> DoomGame {
        DoomGame { wad_data, scr_width, scr_height }
    }

    // TODO to be continued ...
}


impl GraphicsLoop for DoomGame {
    fn handle_event(&mut self, event: &Event) -> bool {
       true 
    }

    fn update_state(&mut self, elapsed_time: f64) -> bool {
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