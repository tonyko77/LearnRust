//! Main lib for the RustooM Doom-like engine/demo

mod font;
mod game;
mod game_initializer;
mod graphics;
mod map;
mod painter;
mod palette;
mod pixmap;
mod sdl_wrapper;
mod utils;
mod wad;

pub use font::*;
pub use game::*;
pub use game_initializer::*;
pub use graphics::*;
pub use map::*;
pub use painter::*;
pub use palette::*;
pub use pixmap::*;
pub use sdl_wrapper::*;
pub use wad::*;

// TODO clean up unused colors (+ move them in another mod ?)
pub const BLACK: RGB = RGB { r: 0, g: 0, b: 0 };
pub const DARK_GREY: RGB = RGB { r: 64, g: 64, b: 64 };
pub const GREY: RGB = RGB { r: 128, g: 128, b: 128 };
pub const LIGHT_GREY: RGB = RGB { r: 192, g: 192, b: 192 };
pub const WHITE: RGB = RGB { r: 255, g: 255, b: 255 };

pub const RED: RGB = RGB { r: 160, g: 0, b: 0 };
pub const GREEN: RGB = RGB { r: 0, g: 160, b: 0 };
pub const BLUE: RGB = RGB { r: 0, g: 0, b: 160 };
pub const CYAN: RGB = RGB { r: 0, g: 160, b: 160 };
pub const MAGENTA: RGB = RGB { r: 160, g: 0, b: 160 };
pub const YELLOW: RGB = RGB { r: 160, g: 160, b: 0 };

pub const BROWN: RGB = RGB { r: 160, g: 80, b: 0 };
pub const CHOCO: RGB = RGB { r: 192, g: 128, b: 64 };
pub const ORANGE: RGB = RGB { r: 255, g: 128, b: 0 };
