//! Simple ray-casting engine demo, using SDL2
//! Inspired by [this YouTube clip](https://www.youtube.com/watch?v=gYRrGTC7GtA)

// This magic line prevents the opening of a terminal when launching a release build
#![cfg_attr(not(any(test, debug_assertions)), windows_subsystem = "windows")]

use raycasting::*;

const MAP: &[&str] = &[
    "CACACACACAC",
    "A.........A",
    "C....FEF..C",
    "A..FEF.F..A",
    "C..F...EE.C",
    "A....@..F.A",
    "C.........C",
    "A..GB.....A",
    "C..B...D..C",
    "A.........A",
    "CACACACACAC",
];

const SCR_WIDTH: i32 = 1200;
const SCR_HEIGHT: i32 = 600;
const PIXEL_SIZE: i32 = 1;

const SLEEP_KIND: SleepKind = SleepKind::YIELD;

fn main() {
    // prepare the map data
    let map_width = MAP[0].len() as i32;
    let map_height = MAP.len() as i32;
    let flatmap = MAP.concat();

    // build the ray caster "demo"
    let mut builder = RayCasterBuilder::new();
    builder
        .scr_size(SCR_WIDTH, SCR_HEIGHT)
        .map_size(map_width, map_height)
        .map_from_str(&flatmap);
    let mut raycaster = builder.build();

    // main game loop
    let sdl_config = SdlConfiguration::new("Ray Caster Demo", SCR_WIDTH, SCR_HEIGHT, PIXEL_SIZE, SLEEP_KIND);
    let res = raycasting::run_sdl_loop(&sdl_config, &mut raycaster);
    if let Err(msg) = res {
        println!("ERROR: {msg}");
    } else {
        println!("Raycaster demo finished OK :)");
    }
}
