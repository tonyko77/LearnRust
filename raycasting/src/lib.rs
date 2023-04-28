//! Main lib for the ray-casting engine/demo

mod raycaster;
mod sdl_wrapper;
mod painter;

pub use painter::*;
pub use sdl_wrapper::*;
pub use raycaster::*;

// needed because we pass Event instances to our handler
pub use sdl2::event::Event;
