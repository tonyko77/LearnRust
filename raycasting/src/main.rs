//! Simple ray-casting engine demo, using SDL2
//! Inspired by [this YouTube clip](https://www.youtube.com/watch?v=gYRrGTC7GtA)

// This magic line prevents the opening of a terminal when launching the app
//#![windows_subsystem = "windows"]

use raycasting::*;

const MAP_WIDTH: u32 = 10;
const MAP_HEIGHT: u32 = 10;
const MAP: &'static str = concat!(
    "CCCCCCCCCC",
    "C........C",
    "C....FEF.C",
    "C..FEF.F.C",
    "C..E...EFC",
    "C........C",
    "C.GB.....A",
    "C.B..@.D.C",
    "C........C",
    "CCCCCCCCCC",
);

const SCR_WIDTH: u32 = 1024;
const SCR_HEIGHT: u32 = 512;
const PIXEL_SIZE: u32 = 1;

const SHOULD_SLEEP: SleepMethod = SleepMethod::YIELD;

fn main() {
    let mut builder = RayCasterBuilder::new();
    builder
        .scr_size(SCR_WIDTH, SCR_HEIGHT)
        .map_size(MAP_WIDTH, MAP_HEIGHT)
        .map_from_str(MAP);

    let mut raycaster = builder.build();

    let sdl_config = SdlConfiguration::new(
        "Ray Caster Demo",
        SCR_WIDTH,
        SCR_HEIGHT,
        PIXEL_SIZE,
        SHOULD_SLEEP,
    );

    // main game loop
    let res = raycasting::run_sdl_loop(&sdl_config, &mut raycaster);
    if let Err(msg) = res {
        println!("ERROR: {msg}");
    } else {
        println!("Raycaster demo finished OK :)");
    }
}
