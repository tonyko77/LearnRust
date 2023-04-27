//! Main lib for the ray-casting engine/demo

pub mod sdl_wrapper;
pub mod raycaster;

pub use sdl_wrapper::{RGB, Painter, SdlConfiguration, GraphicsLoop, run_sdl_loop};

pub use raycaster::RayCaster;
pub use raycaster::RayCasterBuilder;

pub use sdl2::event::Event;
