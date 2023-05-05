//! Main lib for the RustooM Doom-like engine/demo

mod game;
mod map;
mod painter;
mod sdl_wrapper;
mod utils;
mod wad;

pub use game::*;
pub use map::*;
pub use painter::*;
pub use sdl_wrapper::*;
pub use wad::*;

pub const   BLACK: RGB = RGB { r:   0, g:   0, b:   0 };
pub const    GREY: RGB = RGB { r: 128, g: 128, b: 128 };
pub const   WHITE: RGB = RGB { r: 255, g: 255, b: 255 };
pub const     RED: RGB = RGB { r: 160, g: 0, b: 0 };
pub const   GREEN: RGB = RGB { r: 0, g: 160, b: 0 };
pub const    BLUE: RGB = RGB { r: 0, g: 0, b: 160 };
pub const    CYAN: RGB = RGB { r: 0, g: 160, b: 160 };
pub const MAGENTA: RGB = RGB { r: 160, g: 0, b: 160 };
pub const  YELLOW: RGB = RGB { r: 160, g: 160, b: 0 };
pub const   BROWN: RGB = RGB { r: 160, g: 80, b: 0 };
pub const  ORANGE: RGB = RGB { r: 255, g: 128, b: 0 };
