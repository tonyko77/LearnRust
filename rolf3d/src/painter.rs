//! Painter module

#[derive(Clone, Copy, PartialEq, Eq)]
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
    fn get_screen_width(&self) -> i32;

    fn get_screen_height(&self) -> i32;

    /// Draw a single pixel.
    /// This is the only abstract method. The others are based on this one.
    fn draw_pixel(&mut self, x: i32, y: i32, color: RGB);
}
