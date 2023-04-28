//! Simple ray-casting engine demo, using SDL2
//! Inspired by [this YouTube clip](https://www.youtube.com/watch?v=gYRrGTC7GtA)

use raycasting::{self, RayCasterBuilder, SdlConfiguration, RayCastingDemo};

const MAP_WIDTH: u32 = 10;
const MAP_HEIGHT: u32 = 10;
const MAP: &'static str = concat!(
    "AAAAAAAAAA",
    "A........A",
    "A....AAA.A",
    "A..AAA.A.A",
    "A..A...AAA",
    "A........A",
    "A.AA.....A",
    "A.A..@.A.A",
    "A........A",
    "AAAAAAAAAA",
);

const SCR_WIDTH: u32 = 400;
const SCR_HEIGHT: u32 = 200;
const PIXEL_SIZE: u32 = 3;

const SHOULD_SLEEP: bool = true;


fn main() {
    let mut builder = RayCasterBuilder::new();
    builder
        .scr_size(SCR_WIDTH, SCR_HEIGHT)
        .map_size(MAP_WIDTH, MAP_HEIGHT)
        .map_from_str(MAP);
    let _raycaster = builder.build(); // TODO use this !!!

    let sdl_config = SdlConfiguration::new(
        "Ray Caster Demo",
        SCR_WIDTH,
        SCR_HEIGHT,
        PIXEL_SIZE,
        SHOULD_SLEEP);

    // main game loop
    let mut demo = RayCastingDemo::new(SCR_WIDTH, SCR_HEIGHT);
    let ok = raycasting::run_sdl_loop(&sdl_config, &mut demo);
    if let Err(msg) = ok {
        println!("ERROR: {msg}");
    }
    else {
        println!("Raycaster demo finished OK :)");
    }
}
