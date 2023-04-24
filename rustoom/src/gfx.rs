// Graphics wrapper

// use olc_pixel_game_engine as olc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8
}

impl Color {
    #[inline]
    pub fn from(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }
 
    #[inline]
    pub fn set(&mut self, r: u8, g: u8, b: u8) {
        self.r = r;
        self.g = g;
        self.b = b;
    }
}


pub struct Gfx {
    width: i32,
    height: i32,
    pix_size: i32,
    palette: [Color; 256]
}


impl Gfx {
    pub fn new(width: i32, height: i32, pix_size: i32) -> Self {
        Gfx {
            width,
            height,
            pix_size,
            palette : [Color::from(0, 0, 0); 256]
        }
    }

    #[inline]
    pub fn width(&self) -> i32 { self.width }

    #[inline]
    pub fn height(&self) -> i32 { self.height }

    #[inline]
    pub fn set_palette_color(&mut self, color_idx: u8, r: u8, g: u8, b: u8) {
        self.palette[color_idx as usize].set(r, g, b);
    }

    pub fn put_pixel(&self, color_idx: u8) {
        // TODO to be continued ...
    }

    // TODO implement Gfx, initially using olc_pixel_game_engine ...
}
