//! Handling and drawing fonts

use crate::pixmap::*;
use crate::utils::*;
use crate::*;

pub struct Font {
    font: Vec<PixMap>,
    height: u16,
    spc_width: u16,
}

impl Font {
    pub fn new() -> Self {
        Font {
            font: vec![PixMap::new_empty(); 64],
            height: 0,
            spc_width: 0,
        }
    }

    pub fn add_font_lump(&mut self, name: &str, bytes: &[u8], mapper: &dyn ColorMapper) -> Result<(), String> {
        if name.len() > 6 {
            // extract the code from the lump name
            let code = match &name[0..5] {
                "STCFN" => atoi(&name[5..]).unwrap_or(9999),
                "FONTA" => atoi(&name[5..]).unwrap_or(9999) + 32,
                _ => 9999,
            };
            // map the code to a index between 0..=63
            let idx = match code {
                33..=95 => code - 33,
                121 | 124 => 63,
                _ => 9999,
            } as usize;
            if idx <= 63 {
                let mut p = PixMap::from_patch(&bytes)?;
                p.convert_to_font(mapper);
                self.height = p.height().max(self.height);
                if idx == 39 {
                    self.spc_width = p.width();
                }
                self.font[idx] = p;
            }
        }
        Ok(())
    }

    pub fn is_complete(&self) -> bool {
        (0..=57).all(|i| !self.font[i].is_empty())
    }

    pub fn draw_text(&self, x: i32, y: i32, text: &str, color: RGB, painter: &mut dyn Painter) {
        let mapper = FontColorMapper(color);
        let mut dx = 0;
        for byte in text.bytes() {
            if byte <= 32 {
                dx += self.spc_width as i32;
            } else {
                let idx = match byte {
                    33..=95 => (byte - 33) as usize,
                    96 => 6,
                    97..=122 => (byte - 65) as usize,
                    123 => 27,
                    124 => 63,
                    125 => 29,
                    126 => 61,
                    _ => 0,
                };
                let char_pixmap = &self.font[idx];
                if !char_pixmap.is_empty() {
                    char_pixmap.paint(x + dx, y, painter, &mapper);
                    dx += char_pixmap.width() as i32;
                }
            }
        }
    }
}

//---------------

/// Internal color mapper, for painting fonts
struct FontColorMapper(RGB);

impl ColorMapper for FontColorMapper {
    fn byte2rgb(&self, color: u8) -> RGB {
        let r = (self.0.r as u32) * (color as u32) / 255;
        let g = (self.0.g as u32) * (color as u32) / 255;
        let b = (self.0.b as u32) * (color as u32) / 255;
        RGB::from(r as u8, g as u8, b as u8)
    }
}
