//! Main game loop

use crate::{GameAssets, GraphicsLoop, Painter, ScreenBuffer};
use sdl2::{event::Event, keyboard::Keycode};

pub struct GameLoop {
    scrbuf: ScreenBuffer,
    assets: GameAssets,
    tmp_idx: usize,
    tmp_x: i32,
    tmp_y: i32,
}

impl GameLoop {
    pub fn new(width: usize, height: usize, assets: GameAssets) -> Self {
        Self {
            scrbuf: ScreenBuffer::new(width, height),
            assets,
            tmp_idx: 0,
            tmp_x: 0,
            tmp_y: 0,
        }
    }
}

impl GraphicsLoop for GameLoop {
    fn handle_event(&mut self, event: &Event) -> bool {
        // check keys
        match event {
            Event::KeyDown { keycode: Some(key), .. } => {
                match key {
                    //Keycode::Tab => self.level.toggle_automap(),
                    Keycode::Up => self.tmp_y = Ord::max(self.tmp_y - 1, 0),
                    Keycode::Down => self.tmp_y = Ord::min(self.tmp_y + 1, 53),
                    Keycode::Left => self.tmp_x = Ord::max(self.tmp_x - 1, 0),
                    Keycode::Right => self.tmp_x = Ord::min(self.tmp_x + 1, 53),
                    Keycode::PageUp => self.tmp_idx = (self.tmp_idx + 999) % 1000,
                    Keycode::PageDown => self.tmp_idx = (self.tmp_idx + 1) % 1000,
                    Keycode::Home => {
                        self.tmp_idx = 0;
                        self.tmp_x = 0;
                        self.tmp_y = 0;
                    }
                    Keycode::End => {
                        self.tmp_x = 0;
                        self.tmp_y = 0;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        true
    }

    fn update_state(&mut self, _elapsed_time: f64) -> bool {
        // TODO update game state
        _temp_paint_map(self);

        true
    }

    fn paint(&self, painter: &mut dyn Painter) {
        self.scrbuf.paint(painter);
    }
}

//----------------------
//  Internal stuff

// TODO temporary paint map
fn _temp_paint_map(zelf: &mut GameLoop) {
    let sw = zelf.scrbuf.width() as i32;
    let sh = zelf.scrbuf.height() as i32;
    zelf.scrbuf.fill_rect(0, 0, sw, sh, 0);

    let mapidx = zelf.tmp_idx % zelf.assets.maps.len();
    let map = &zelf.assets.maps[mapidx];
    for y in 0..map.height {
        for x in 0..map.width {
            let xx = (x as i32) + zelf.tmp_x;
            let yy = (y as i32) + zelf.tmp_y;
            if xx >= 0 && yy >= 0 && xx < (map.width as i32) && yy < (map.height as i32) {
                let idx = (yy * (map.width as i32) + xx) as usize;
                let wall = if idx < map.walls.len() { map.walls[idx] } else { 0 };
                let thng = if idx < map.things.len() { map.things[idx] } else { 0 };
                assert!(wall <= 0xFF);
                assert!(thng <= 0xFF);

                let ix = (x * 9) as i32;
                let iy = (y * 9) as i32;

                // draw wall rect
                let wcol = (wall & 0xFF) as u8;
                zelf.scrbuf.fill_rect(ix, iy, 8, 8, wcol);

                // draw thing
                if thng > 0 {
                    zelf.scrbuf.fill_rect(ix + 2, iy + 2, 4, 4, 0);
                    zelf.scrbuf.fill_rect(ix + 3, iy + 3, 2, 2, (thng & 0xFF) as u8);
                }
            }
        }
    }
}

// TODO temporary paint palette
fn _temp_paint_palette(scrbuf: &mut ScreenBuffer) {
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
