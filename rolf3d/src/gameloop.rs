//! Main game loop.
//! Also acts as a facade, to hold and manage all game objects
//! (assets, renderers, other managers etc)

use std::collections::HashSet;

use crate::defs::*;
use crate::*;
use sdl2::{event::Event, keyboard::Keycode};

pub struct GameLoop {
    scrbuf: ScreenBuffer,
    assets: GameAssets,
    tmp_idx: usize,
    tmp_x: i32,
    tmp_y: i32,
    tmp_automap: bool,
    tmp_changed: bool,
    tmp_amap_scale: i32,
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
            tmp_changed: true,
            tmp_amap_scale: 12,
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
                Keycode::PageUp => {
                    self.tmp_idx = (self.tmp_idx + 999) % 1000;
                    self.tmp_changed = true;
                }
                Keycode::PageDown => {
                    self.tmp_idx = (self.tmp_idx + 1) % 1000;
                    self.tmp_changed = true;
                }
                Keycode::Home => {
                    self.tmp_idx = 0;
                    self.tmp_x = -5;
                    self.tmp_y = -5;
                    self.tmp_changed = true;
                }
                Keycode::End => {
                    self.tmp_x = -5;
                    self.tmp_y = -5;
                }
                Keycode::KpPlus => self.tmp_amap_scale = Ord::min(self.tmp_amap_scale + 1, 40),
                Keycode::KpMinus => self.tmp_amap_scale = Ord::max(self.tmp_amap_scale - 1, 6),
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

        if self.tmp_changed {
            _temp_map_debug_info(&self);
            self.tmp_changed = false;
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
    let scl = zelf.tmp_amap_scale;
    let sw = zelf.scrbuf.width() as i32;
    let sh = zelf.scrbuf.height() as i32;
    zelf.scrbuf.fill_rect(0, 0, sw, sh, 0);

    let mapidx = zelf.tmp_idx % zelf.assets.maps.len();
    let map = &zelf.assets.maps[mapidx];
    let mw = map.width as i32;
    let mh = map.height as i32;

    for y in 0..mh {
        for x in 0..mw {
            let xx = (x as i32) + zelf.tmp_x;
            let yy = (y as i32) + zelf.tmp_y;
            let tile = map.tile(xx, yy);
            let thng = map.thing(xx, yy);
            let ix = (x * scl) as i32;
            let iy = (y * scl) as i32;

            if tile == 0 {
                // 0 tiles are MY CREATION -> out of bounds tile :/
                zelf.scrbuf.fill_rect(ix + 1, iy + 1, scl - 1, scl - 1, 31);
                continue;
            }

            if tile >= 90 && tile <= 101 {
                // => door, vertical if even, lock = (tile - 90|91)/2
                let widx = if tile >= 100 { 24 } else { (tile + 8) as usize };

                if widx < zelf.assets.walls.len() {
                    let wall = &zelf.assets.walls[widx];
                    wall.draw_scaled(ix, iy, scl, &mut zelf.scrbuf);
                } else {
                    let wcol = (tile - 89) as u8;
                    zelf.scrbuf.fill_rect(ix, iy, scl, scl, 14);
                    zelf.scrbuf.fill_rect(ix + 1, iy + 1, scl - 2, scl - 2, wcol);
                }
            } else if tile == AMBUSHTILE {
                // ambush tile - has special meaning
                zelf.scrbuf.fill_rect(ix, iy, scl, scl, 31);
                zelf.scrbuf.fill_rect(ix + 1, iy + 1, scl - 2, scl - 2, 6);
            } else if tile < AREATILE {
                // solid wall => draw wall rect

                // WHICH WALL TEXTURE corresponds to each solid tile:
                // * wall textures are "in pairs" - alternating light and dark versions
                // => (tile * 2) selects light/dark version, then -2 makes it 0-based
                // !!ALSO!! LIGHT walls are used for N/S walls, and DARK for E/W walls
                let widx = if tile == 21 { 41 } else { ((tile - 1) * 2) as usize };

                if widx < zelf.assets.walls.len() {
                    let wall = &zelf.assets.walls[widx];
                    wall.draw_scaled(ix, iy, scl, &mut zelf.scrbuf);
                } else {
                    let wcol = (tile & 0xFF) as u8;
                    zelf.scrbuf.fill_rect(ix, iy, scl, scl, 15);
                    zelf.scrbuf.fill_rect(ix + 1, iy + 1, scl - 2, scl - 2, wcol);
                }
            } else {
                // empty area
                let cl = (tile - AREATILE) as u8;
                zelf.scrbuf.fill_rect(ix + 1, iy + 1, scl - 2, scl - 2, cl);

                // => TODO: what is the hidden meaning behind various empty area codes
                // they seem to be between 108 (AREATILE + 1) and ~143
                // OBSERVATION: all empty tiles in one room have THE SAME VALUE
                // => maybe a way to alert enemies from the same area ?!?
            }

            // draw thing
            if thng > 0 {
                zelf.scrbuf.fill_rect(ix + 2, iy + 2, 4, 4, 0);
                zelf.scrbuf.fill_rect(ix + 3, iy + 3, 2, 2, (thng & 0xFF) as u8);
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

// TODO TEMP get info about map
fn _temp_map_debug_info(zelf: &GameLoop) {
    let mapidx = zelf.tmp_idx % zelf.assets.maps.len();
    let map = &zelf.assets.maps[mapidx];

    // check for tiles >= AREATILE
    let mut minwall = 9999;
    let mut maxwall = 0;
    let mut non_wall = HashSet::new();
    for x in 0..64 {
        for y in 0..64 {
            let tile = map.tile(x, y);
            if tile < AREATILE {
                // solid wall go from 1 to 106 (AREATILE - 1)
                minwall = Ord::min(minwall, tile);
                maxwall = Ord::max(maxwall, tile);

                // check for missing textures
                let widx = (tile * 2 - 2) as usize;
                if widx >= zelf.assets.walls.len() {
                    if tile >= 90 && tile <= 101 {
                        // it's a door, it is ok
                    } else if tile == AMBUSHTILE {
                        // it's an ambush tile
                    } else {
                        println!(
                            "MISSING wall texture for tile {tile} => widx={widx} >= {}",
                            zelf.assets.walls.len()
                        );
                    }
                }
            } else {
                // seem to be between 108 (AREATILE + 1) and ~143
                // but what do they mean ????
                non_wall.insert(tile);
            }
        }
    }
}
