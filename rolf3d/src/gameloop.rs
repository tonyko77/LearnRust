//! Main game loop

use crate::{GraphicsLoop, Painter, ScreenBuffer};
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
    scrbuf.fill_rect(0, 0, sw, sh, 0);

    // paint the palette
    let mut cidx: i32 = 0;
    for y in 0..16 {
        for x in 0..16 {
            let c = cidx as u8;
            scrbuf.fill_rect(x * 10, y * 10, 9, 9, c);
            cidx += 1;
        }
    }
}
