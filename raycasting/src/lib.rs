//! Main lib for the ray-casting engine/demo

mod raycaster;
mod sdl_wrapper;
mod painter;

pub use painter::*;
pub use sdl_wrapper::*;
pub use raycaster::*;

// needed because we pass Event instances to our handler
pub use sdl2::event::Event;
pub use sdl2::keyboard::Keycode;


pub const      BLACK: RGB = RGB { r:   0, g:   0, b:   0 };
pub const  DARK_GREY: RGB = RGB { r:  64, g:  64, b:  64 };
pub const       GREY: RGB = RGB { r: 128, g: 128, b: 128 };
pub const LIGHT_GREY: RGB = RGB { r: 192, g: 192, b: 192 };
pub const      WHITE: RGB = RGB { r: 255, g: 255, b: 255 };

pub const     DARK_RED: RGB = RGB { r: 64, g: 0, b: 0 };
pub const   DARK_GREEN: RGB = RGB { r: 0, g: 64, b: 0 };
pub const    DARK_BLUE: RGB = RGB { r: 0, g: 0, b: 64 };
pub const    DARK_CYAN: RGB = RGB { r: 0, g: 64, b: 64 };
pub const DARK_MAGENTA: RGB = RGB { r: 64, g: 0, b: 64 };
pub const  DARK_YELLOW: RGB = RGB { r: 64, g: 64, b: 0 };
pub const   DARK_BROWN: RGB = RGB { r: 64, g: 32, b: 0 };

pub const     RED: RGB = RGB { r: 160, g: 0, b: 0 };
pub const   GREEN: RGB = RGB { r: 0, g: 160, b: 0 };
pub const    BLUE: RGB = RGB { r: 0, g: 0, b: 160 };
pub const    CYAN: RGB = RGB { r: 0, g: 160, b: 160 };
pub const MAGENTA: RGB = RGB { r: 160, g: 0, b: 160 };
pub const  YELLOW: RGB = RGB { r: 160, g: 160, b: 0 };
pub const   BROWN: RGB = RGB { r: 160, g: 80, b: 0 };
pub const  ORANGE: RGB = RGB { r: 255, g: 128, b: 0 };

pub const     LIGHT_RED: RGB = RGB { r: 255, g: 0, b: 0 };
pub const   LIGHT_GREEN: RGB = RGB { r: 0, g: 255, b: 0 };
pub const    LIGHT_BLUE: RGB = RGB { r: 0, g: 0, b: 255 };
pub const    LIGHT_CYAN: RGB = RGB { r: 0, g: 255, b: 255 };
pub const LIGHT_MAGENTA: RGB = RGB { r: 255, g: 0, b: 255 };
pub const  LIGHT_YELLOW: RGB = RGB { r: 255, g: 255, b: 0 };
