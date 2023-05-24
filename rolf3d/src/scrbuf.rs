//! Screen bufer - collects what needs to be painted and paints it using the palette.

use crate::{Painter, RGB};

/// Screen buffer - holds one buffer of screen data and paints it on the screen.
pub struct ScreenBuffer {
    width: usize,
    height: usize,
    bytes: Vec<u8>,
}

impl ScreenBuffer {
    /// Create a new screen buffer.
    pub fn new(width: usize, height: usize) -> Self {
        let len = width * height;
        Self {
            width,
            height,
            bytes: vec![0; len],
        }
    }

    /// Screen buffer width.
    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    /// Screen buffer height.
    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    /// Put a pixel in the buffer, *with* transparency.
    #[inline]
    pub fn put_pixel(&mut self, x: i32, y: i32, c: u8) {
        if x >= 0 && y >= 0 && c != 0xFF {
            let xx = x as usize;
            let yy = y as usize;
            if xx < self.width && yy < self.height {
                let idx = yy * self.width + xx;
                self.bytes[idx] = c;
            }
        }
    }

    /// Fill a rectangle inside the buffer, *without* transparency.
    pub fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, c: u8) {
        let isw = self.width as i32;
        let ish = self.height as i32;
        if w <= 0 || h <= 0 || x >= isw || y >= ish {
            return;
        }

        // shift top-left corner inside the screen
        let xx = Ord::max(x, 0);
        let yy = Ord::max(y, 0);
        let w = w - xx + x;
        let h = h - yy + y;
        if w <= 0 || h <= 0 {
            return;
        }

        // shift bottom right corner inside the screen
        let sw = Ord::min(w, isw - xx) as usize;
        let sh = Ord::min(h, ish - yy) as usize;
        let mut idx = (yy * isw + xx) as usize;
        let step = self.width - sw;

        // ok to paint
        for _ in 0..sh {
            for _ in 0..sw {
                self.bytes[idx] = c;
                idx += 1;
            }
            idx += step;
        }
    }

    /// Paint the buffer onto the screen.
    pub fn paint(&self, painter: &mut dyn Painter) {
        let mut idx = 0;
        for y in 0..(self.height as i32) {
            for x in 0..(self.width as i32) {
                let color = palette_to_rgb(self.bytes[idx]);
                painter.draw_pixel(x, y, color);
                idx += 1;
            }
        }
    }
}

//--------------------------
//  Internal stuff

fn palette_to_rgb(c: u8) -> RGB {
    // TODO temporary !!
    RGB::from(c, c, c)
}
