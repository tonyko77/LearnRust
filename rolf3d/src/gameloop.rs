//! Main game loop

use crate::*;
use sdl2::{event::Event, keyboard::Keycode};

pub struct GameLoop {
    scrbuf: ScreenBuffer,
    assets: GameAssets,
    tmp_idx: usize,
    tmp_x: i32,
    tmp_y: i32,
    tmp_automap: bool,
}

impl GameLoop {
    pub fn new(width: usize, height: usize, assets: GameAssets) -> Self {
        let is_sod = assets.is_sod();
        Self {
            scrbuf: ScreenBuffer::new(width, height, is_sod),
            assets,
            tmp_idx: 0,
            tmp_x: -5,
            tmp_y: -5,
            tmp_automap: false,
        }
    }
}

impl GraphicsLoop for GameLoop {
    fn handle_event(&mut self, event: &Event) -> bool {
        // check keys
        match event {
            Event::KeyDown { keycode: Some(key), .. } => match key {
                Keycode::Tab => self.tmp_automap = !self.tmp_automap,
                Keycode::Up => self.tmp_y = Ord::max(self.tmp_y - 1, -10),
                Keycode::Down => self.tmp_y = Ord::min(self.tmp_y + 1, 53),
                Keycode::Left => self.tmp_x = Ord::max(self.tmp_x - 1, -10),
                Keycode::Right => self.tmp_x = Ord::min(self.tmp_x + 1, 53),
                Keycode::PageUp => self.tmp_idx = (self.tmp_idx + 999) % 1000,
                Keycode::PageDown => self.tmp_idx = (self.tmp_idx + 1) % 1000,
                Keycode::Home => {
                    self.tmp_idx = 0;
                    self.tmp_x = -5;
                    self.tmp_y = -5;
                }
                Keycode::End => {
                    self.tmp_x = -5;
                    self.tmp_y = -5;
                }
                _ => {}
            },
            _ => {}
        }
        true
    }

    fn update_state(&mut self, _elapsed_time: f64) -> bool {
        // TODO update game state
        if self.tmp_automap {
            _temp_paint_map(self);
        } else {
            _temp_paint_gfx(self);
        }

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
    const SZ: i32 = 20;
    let sw = zelf.scrbuf.width() as i32;
    let sh = zelf.scrbuf.height() as i32;
    zelf.scrbuf.fill_rect(0, 0, sw, sh, 0);

    let mapidx = zelf.tmp_idx % zelf.assets.maps.len();
    let map = &zelf.assets.maps[mapidx];
    let mw = map.width();
    let mh = map.height();

    for y in 0..mh {
        for x in 0..mw {
            let xx = (x as i32) + zelf.tmp_x;
            let yy = (y as i32) + zelf.tmp_y;
            let widx = map.wall(xx, yy) as usize;
            let thng = map.thing(xx, yy);

            if widx > 0 {
                // draw wall rect
                let ix = (x * SZ) as i32;
                let iy = (y * SZ) as i32;
                if widx <= zelf.assets.walls.len() {
                    let wall = &zelf.assets.walls[widx - 1];
                    wall.draw_scaled(ix, iy, SZ, &mut zelf.scrbuf);
                } else {
                    let wcol = (widx & 0xFF) as u8;
                    zelf.scrbuf.fill_rect(ix, iy, SZ - 1, SZ - 1, wcol);
                }
                // draw thing
                if thng > 0 {
                    zelf.scrbuf.fill_rect(ix + 4, iy + 4, 5, 5, 0);
                    zelf.scrbuf.fill_rect(ix + 5, iy + 5, 3, 3, (thng & 0xFF) as u8);
                }
            }
        }
    }
}

// TODO temporary paint gfx
fn _temp_paint_gfx(zelf: &mut GameLoop) {
    _temp_paint_palette(&mut zelf.scrbuf);

    let x0 = (zelf.scrbuf.width() - 100) as i32;
    let y0 = (zelf.scrbuf.height() - 202) as i32;

    // paint wall
    let wallidx = zelf.tmp_idx % zelf.assets.walls.len();
    let wall = &zelf.assets.walls[wallidx];
    _temp_paint_pic(wall, x0, 10, &mut zelf.scrbuf);
    let str = format!("WALL #{wallidx}");
    zelf.assets.font1.draw_text(x0, 80, &str, 14, &mut zelf.scrbuf);

    // paint sprite
    let sprtidx = zelf.tmp_idx % zelf.assets.sprites.len();
    let sprite = &zelf.assets.sprites[sprtidx];
    _temp_paint_pic(sprite, x0, y0, &mut zelf.scrbuf);
    let str = format!("SPRT #{sprtidx}");
    zelf.assets.font1.draw_text(x0, y0 - 16, &str, 14, &mut zelf.scrbuf);

    // paint pics
    let picidx = zelf.tmp_idx % zelf.assets.pics.len();
    let pic = &zelf.assets.pics[picidx];
    _temp_paint_pic(pic, 0, y0, &mut zelf.scrbuf);
    let str = format!("PIC #{picidx}");
    zelf.assets.font1.draw_text(0, y0 - 16, &str, 14, &mut zelf.scrbuf);

    // paint fonts
    let char_idx = zelf.tmp_idx % 100;
    let ch = (char_idx + 33) as u8;
    let str = format!("{} = {}", ch as char, ch);
    zelf.assets.font1.draw_text(170, 10, &str, 11, &mut zelf.scrbuf);
    zelf.assets.font2.draw_text(170, 30, &str, 12, &mut zelf.scrbuf);
}

// TODO temporary paint a graphic
fn _temp_paint_pic(gfx: &GfxData, x0: i32, y0: i32, scrbuf: &mut ScreenBuffer) {
    const BG: u8 = 31;

    let (pw, ph) = gfx.size();
    if pw == 0 || ph == 0 {
        // empty pic !!
        scrbuf.fill_rect(x0, y0, 8, 8, BG);
    } else {
        scrbuf.fill_rect(x0, y0, pw as i32, ph as i32, BG);
        gfx.draw(x0, y0, scrbuf);
    }
}

// TODO temporary paint palette
fn _temp_paint_palette(scrbuf: &mut ScreenBuffer) {
    const SQSIZE: i32 = 8;

    let sw = scrbuf.width() as i32;
    let sh = scrbuf.height() as i32;
    scrbuf.fill_rect(0, 0, sw, sh, 0);

    // paint the palette
    let mut cidx: i32 = 0;
    for y in 0..16 {
        for x in 0..16 {
            let c = cidx as u8;
            scrbuf.fill_rect(x * SQSIZE, y * SQSIZE, SQSIZE, SQSIZE, c);
            cidx += 1;
        }
    }
}
