//! Main game loop

use crate::{ScreenBuffer, GraphicsLoop, Painter};
use sdl2::event::Event;
// TODO use sdl2::keyboard::Keycode;

pub struct GameLoop {
    scrbuf: ScreenBuffer,
}

impl GameLoop {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            scrbuf: ScreenBuffer::new(width, height),
        }
    }

    pub fn run_loop(&mut self) {

    }
}

impl GraphicsLoop for GameLoop {
    fn handle_event(&mut self, _event: &Event) -> bool {
        // TODO check keys
        true
    }

    fn update_state(&mut self, _elapsed_time: f64) -> bool {
        // TODO update game state
        _temp_paint(&mut self.scrbuf);
        true
    }

    fn paint(&self, painter: &mut dyn Painter) {
        self.scrbuf.paint(painter);
    }
}

//----------------------
//  Internal stuff

// TODO temporary paint stuff
fn _temp_paint(scrbuf: &mut ScreenBuffer) {
    let sw = scrbuf.width() as i32;
    let sh = scrbuf.height() as i32;
    scrbuf.fill_rect(0, 0, sw, sh, 128);

    scrbuf.put_pixel(10, 10, 254);
    scrbuf.fill_rect(11, 11, 2, 2, 0);
    scrbuf.put_pixel(13, 13, 254);

    scrbuf.fill_rect(-3, -3, 6, 6, 10);
    scrbuf.fill_rect(-3, sh-3, 6, 6, 20);
    scrbuf.fill_rect(sw-3, -3, 6, 6, 30);
    scrbuf.fill_rect(sw-3, sh-3, 6, 6, 40);
}