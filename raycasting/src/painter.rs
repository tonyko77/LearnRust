//! Painter module

#[derive(Clone, Copy)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    #[inline]
    pub fn from(r: u8, g: u8, b: u8) -> Self {
        RGB { r, g, b }
    }
}


/// Painter interface, to be passed to client code so it can perform painting.
/// *This is not meant to be implemented by client code.*
pub trait Painter {
    fn draw_pixel(&mut self, x: i32, y: i32, color: RGB);


    fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: RGB) {
        if w > 0 && h > 0 {
            let x2 = x + w - 1;
            let y2 = y + h - 1;
            for xx in x..=x2 {
                self.draw_pixel(xx, y, color);
                self.draw_pixel(xx, y2, color);
            }
            for yy in (y+1)..y2 {
                self.draw_pixel(x, yy, color);
                self.draw_pixel(x2, yy, color);
            }
        }
    }

    fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: RGB) {
        if w > 0 && h > 0 {
            for yy in y .. (y+h) {
                for xx in x .. (x+w) {
                    self.draw_pixel(xx, yy, color);
                }
            }
        }
    }


    // very basic, using floats
    // TODO improve this (better algo)
    fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: RGB) {
        if x1 == x2 {
            self.draw_vert_line(x1, y1, y2, color);
            return;
        }
        if y1 == y2 {
            self.draw_horiz_line(x1, x2, y1, color);
            return;
        }

        let dist_x = if x1 < x2 { x2 - x1 } else { x1 - x2 };
        let dist_y = if y1 < y2 { y2 - y1 } else { y1 - y2 };
        let dist_max = if dist_x > dist_y { dist_x } else { dist_y };
        let dx = ((x2 as f64) - (x1 as f64)) / (dist_max as f64);
        let dy = ((y2 as f64) - (y1 as f64)) / (dist_max as f64);
        let mut x = (x1 as f64) + 0.5;
        let mut y = (y1 as f64) + 0.5;
        self.draw_pixel(x1, y1, color);
        for _ in 0..dist_max {
            x += dx;
            y += dy;
            self.draw_pixel(x as i32, y as i32, color);
        }
    }

    fn draw_horiz_line(&mut self, x1: i32, x2: i32, y: i32, color: RGB) {
        if x1 == x2 {
            self.draw_pixel(x1, y, color);
            return;
        }
        let (xmin, xmax) = if x1 < x2 { (x1, x2) } else { (x2, x1) } ;
        for x in xmin ..= xmax {
            self.draw_pixel(x, y, color);
        }
    }

    fn draw_vert_line(&mut self, x: i32, y1: i32, y2: i32, color: RGB) {
        if y1 == y2 {
            self.draw_pixel(x, y1, color);
            return;
        }
        let (ymin, ymax) = if y1 < y2 { (y1, y2) } else { (y2, y1) } ;
        for y in ymin ..= ymax {
            self.draw_pixel(x, y, color);
        }
    }


    // very basic, using floats
    // TODO improve this (better algo)
    fn draw_circle(&mut self, x: i32, y: i32, r: i32, color: RGB) {
        let mut r2 = r * r;
        let mut sub = 1;
        let imax = ((r as f64) / 1.4142135 + 0.5) as i32;

        for px in 0 ..= imax {
            let py = ((r2 as f64).sqrt() + 0.5) as i32;
            r2 -= sub;
            sub += 2;

            self.draw_pixel(x + px, y + py, color);
            self.draw_pixel(x + px, y - py, color);
            self.draw_pixel(x - px, y + py, color);
            self.draw_pixel(x - px, y - py, color);

            self.draw_pixel(x + py, y + px, color);
            self.draw_pixel(x + py, y - px, color);
            self.draw_pixel(x - py, y + px, color);
            self.draw_pixel(x - py, y - px, color);
        }
    }

    fn fill_circle(&mut self, x: i32, y: i32, r: i32, color: RGB) {
        todo!();
    }

}
