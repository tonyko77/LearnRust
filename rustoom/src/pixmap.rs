//! Pixel Maps (Patches, Flats, Fonts)

use crate::utils::*;
use crate::*;

const FLAT_MARKER: i16 = i16::MIN;
const FONT_MARKER: i16 = i16::MAX;
const ERR_BAD_PATCH: &str = "Invalid patch data in WAD";

// TODO: 251 for DOOM, 175 for HERETIC => should be detected from the PLAYPAL
const PINK_PIXEL: u8 = 251;

/// Trait which provides color mapping at runtime (u8 -> RGB).
pub trait ColorMapper {
    /// Map a byte value to a color.
    fn byte2rgb(&self, color: u8) -> RGB;
}

/// Pixel map structure.
/// Can be used for Patches, Flats or Fonts.
#[derive(Clone)]
pub struct PixMap {
    width: u16,
    height: u16,
    left_offs: i16,
    top_offs: i16,
    pixels: Vec<u8>,
}

impl PixMap {
    pub fn new_empty() -> Self {
        PixMap {
            width: 0,
            height: 0,
            left_offs: 0,
            top_offs: 0,
            pixels: vec![],
        }
    }

    pub fn new_placeholder(width: usize, height: usize) -> Self {
        let size = width * height;
        PixMap {
            width: width as u16,
            height: height as u16,
            left_offs: FLAT_MARKER,
            top_offs: 0,
            pixels: vec![PINK_PIXEL; size],
        }
    }

    pub fn from_flat(flat_bytes: &[u8]) -> Self {
        let h = (flat_bytes.len() >> 6) as u16;
        PixMap {
            width: 64,
            height: h,
            left_offs: FLAT_MARKER,
            top_offs: 0,
            pixels: flat_bytes.to_vec(),
        }
    }

    pub fn from_patch(patch_bytes: &[u8]) -> Result<Self, String> {
        if patch_bytes.len() <= 12 {
            return Err(ERR_BAD_PATCH.to_string());
        }
        // init pixmap
        let width = buf_to_u16(&patch_bytes[0..=1]);
        let height = buf_to_u16(&patch_bytes[2..=3]);
        let left_offs = buf_to_i16(&patch_bytes[4..=5]);
        let top_offs = buf_to_i16(&patch_bytes[6..=7]);
        let pix_bytes_cnt = (width as usize) * (height as usize);
        let transp_bytes_cnt = (pix_bytes_cnt + 7) << 3;
        let mut pix = PixMap {
            width,
            height,
            left_offs,
            top_offs,
            pixels: vec![0; pix_bytes_cnt + transp_bytes_cnt],
        };
        // parse each column
        for x in 0..(width as usize) {
            if patch_bytes.len() <= (x * 4 + 12) {
                return Err(ERR_BAD_PATCH.to_string());
            }
            // prepare an array of transparent pixels
            let mut column_pixels = vec![-1; height as usize];
            // parse the posts of the column
            let mut col_ofs = buf_to_u32(&patch_bytes[x * 4 + 8..x * 4 + 12]) as usize;
            loop {
                // check for loop end
                let y_ofs = patch_bytes.get(col_ofs).cloned().unwrap_or(0xFF) as usize;
                if y_ofs == 0xFF {
                    break;
                }
                // parse the post bytes
                let len = patch_bytes.get(col_ofs + 1).cloned().unwrap_or(0) as usize;
                for y in 0..len {
                    column_pixels[y + y_ofs] = patch_bytes
                        .get(col_ofs + 3 + y)
                        .cloned()
                        .unwrap_or(PINK_PIXEL)
                        as i32;
                }
                // advance to the next post
                col_ofs += len + 4;
            }
            // put the column pixels into the pixmap
            for y in 0..(height as usize) {
                let pix_idx = y * (width as usize) + x;
                if column_pixels[y] >= 0 {
                    // normal pixel
                    pix.pixels[pix_idx] = column_pixels[y] as u8;
                } else {
                    let transp_idx = pix_bytes_cnt + (pix_idx >> 3);
                    let transp_bit = 1_u8 << (pix_idx & 0x07);
                    pix.pixels[transp_idx] |= transp_bit;
                }
            }
        }

        Ok(pix)
    }

    pub fn convert_to_font(&mut self, mapper: &dyn ColorMapper) {
        let size = (self.width as usize) * (self.height as usize);
        if self.pixels.len() <= size
            || self.left_offs == FLAT_MARKER
            || self.left_offs == FONT_MARKER
        {
            // nothing to do
            return;
        }
        // convert pixels to gray shades + use 0 for transparency
        let mut pix = vec![0; size];
        let mut max_level = 0;
        for idx in 0..size {
            let transp_idx = size + (idx >> 3);
            let transp_bit = 1_u8 << (idx & 0x07);
            if self.pixels[transp_idx] & transp_bit == 0 {
                let code = self.pixels[idx];
                let rgb = mapper.byte2rgb(code);
                let gray =
                    ((rgb.r as u32) * 299 + (rgb.g as u32) * 587 + (rgb.b as u32) * 114) / 1000;
                if max_level < gray {
                    max_level = gray;
                }
                pix[idx] = match gray {
                    0..=1 => 1,
                    255.. => 255,
                    _ => gray as u8,
                };
            }
        }
        // adjust gray levels
        if max_level > 0 && max_level < 250 {
            for idx in 0..size {
                if pix[idx] != 0 {
                    let adjusted = (pix[idx] as u32) * 255 / max_level;
                    pix[idx] = adjusted as u8;
                }
            }
        }
        // replace pixmap
        self.left_offs = FONT_MARKER;
        self.pixels = pix;
    }

    pub fn paint(&self, x: i32, y: i32, painter: &mut dyn Painter, mapper: &dyn ColorMapper) {
        // compute origin
        let x0 = x - if self.left_offs != FLAT_MARKER && self.left_offs != FONT_MARKER {
            self.left_offs as i32
        } else {
            0
        };
        let y0 = y - (self.top_offs as i32);
        // paint each pixel
        let size = (self.width as usize) * (self.height as usize);
        let has_transparency = self.pixels.len() > size;
        let mut dx = -1;
        let mut dy = 0;
        for idx in 0..size {
            // go to next coordinates
            dx += 1;
            if dx >= (self.width as i32) {
                dx = 0;
                dy += 1;
            }
            // get pixel
            let pixcode = self.pixels[idx];
            let is_transparent = match self.left_offs {
                FONT_MARKER => pixcode == 0,
                FLAT_MARKER => false,
                _ => {
                    has_transparency && {
                        let transp_idx = size + (idx >> 3);
                        let transp_bit = 1_u8 << (idx & 0x07);
                        self.pixels[transp_idx] & transp_bit != 0
                    }
                }
            };
            // paint pixel
            if !is_transparent {
                let color = mapper.byte2rgb(pixcode);
                painter.draw_pixel(x0 + dx, y0 + dy, color);
            }
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    #[inline]
    pub fn width(&self) -> u16 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u16 {
        self.height
    }
}
