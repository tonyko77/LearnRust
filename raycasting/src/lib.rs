//! Main lib for the ray-casting engine/demo

pub mod raycaster;
pub mod sdl_wrapper;

pub use sdl_wrapper::{run_sdl_loop, GraphicsLoop, Painter, SdlConfiguration, RGB};

pub use raycaster::{RayCaster, RayCasterBuilder, RayCastingDemo};

pub use sdl2::event::Event;
