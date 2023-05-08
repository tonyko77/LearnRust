//! Pixel Maps (Patches, Flats, Fonts)

// TODO !!! store bytes directly instead of loading + add a type enum + decode upon paint !!!

use crate::utils::*;
use crate::*;
use bytes::Bytes;

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
    kind: PixMapKind,
    data: Bytes,
}

impl PixMap {
    pub fn new_empty() -> Self {
        Self {
            width: 0,
            height: 0,
            kind: PixMapKind::PlaceHolder,
            data: Bytes::new(),
        }
    }

    pub fn new_placeholder(width: usize, height: usize) -> Self {
        Self {
            width: width as u16,
            height: height as u16,
            kind: PixMapKind::PlaceHolder,
            data: Bytes::new(),
        }
    }

    pub fn from_flat(flat_bytes: Bytes) -> Self {
        let height = (flat_bytes.len() >> 6) as u16;
        Self {
            width: 64,
            height,
            kind: PixMapKind::Flat,
            data: flat_bytes,
        }
    }

    pub fn from_patch(patch_bytes: Bytes) -> Self {
        // // TODO validate patch data DURING WAD loading!
        // if patch_bytes.len() <= 12 {
        //     return Err(ERR_BAD_PATCH.to_string());
        // }

        let width = buf_to_u16(&patch_bytes[0..=1]);
        let height = buf_to_u16(&patch_bytes[2..=3]);
        Self {
            width,
            height,
            kind: PixMapKind::Patch,
            data: patch_bytes,
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

    pub fn paint(&self, x: i32, y: i32, painter: &mut dyn Painter, mapper: &dyn ColorMapper) {
        if self.width > 0 && self.height > 0 {
            match self.kind {
                PixMapKind::Flat => self.paint_flat(x, y, painter, mapper),
                PixMapKind::Patch => self.paint_patch(x, y, painter, mapper),
                PixMapKind::PlaceHolder => self.paint_pink(x, y, painter),
            }
        }
    }

    fn paint_pink(&self, x: i32, y: i32, painter: &mut dyn Painter) {
        for dy in 0 .. self.height as i32 {
            for dx in 0 .. self.width as i32 {
                painter.draw_pixel(x + dx, y + dy, RGB::from(255, 0, 255));
            }
        }
    }

    fn paint_flat(&self, x: i32, y: i32, painter: &mut dyn Painter, mapper: &dyn ColorMapper) {
        let mut idx = 0;
        for dy in 0 .. self.height as i32 {
            for dx in 0 .. self.width as i32 {
                let pixcode = self.data[idx];
                idx += 1;
                let color = mapper.byte2rgb(pixcode);
                painter.draw_pixel(x + dx, y + dy, color);
            }
        }
    }

    // TODO this may PANIC if the patch data is invalid and the index goes out of bounds
    // => IDEA: improve validation of gfx patch data during wad initialization
    fn paint_patch(&self, x: i32, y: i32, painter: &mut dyn Painter, mapper: &dyn ColorMapper) {
        let x0 = buf_to_i16(&self.data[4..6]) as i32;
        let y0 = buf_to_i16(&self.data[6..8]) as i32;

        let mut ofs_idx = 8;
        for dx in 0 .. self.width as i32 {
            // find the column index
            let mut col_idx = buf_to_u32(&self.data[ofs_idx .. ofs_idx + 4]) as usize;
            ofs_idx += 4;
            loop {
                let dy = self.data[col_idx] as i32;
                if dy == 0xFF {
                    break;
                }
                let len = self.data[col_idx + 1] as i32;
                for i in 0 .. len {
                    let pixcode = self.data[col_idx + 3 + (i as usize)];
                    let color = mapper.byte2rgb(pixcode);
                    painter.draw_pixel(x + dx - x0, y + dy + i - y0, color);
                }
                col_idx += 4 + (len as usize);
            }
        }
    }

}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PixMapKind {
    Patch,
    Flat,
    PlaceHolder,
}
