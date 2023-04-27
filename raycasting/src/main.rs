//! Simple ray-casting engine demo, using SDL2
//! Inspired by [this YouTube clip](https://www.youtube.com/watch?v=gYRrGTC7GtA)

use raycasting::{self, RayCasterBuilder, SdlConfiguration, GraphicsLoop, RGB, Event, Painter};

const MAP_WIDTH: u32 = 8;
const MAP_HEIGHT: u32 = 8;
const MAP: &'static str = concat!(
    "########",
    "#...#..#",
    "#.###..#",
    "#.#....#",
    "#......#",
    "#....#.#",
    "#..@...#",
    "########",
);

//------------------------------------

fn main() {
    // build the ray caster
    let mut builder = RayCasterBuilder::new();
    builder.width(MAP_WIDTH).height(MAP_HEIGHT).map_from_str(MAP);
    let _raycaster = builder.build();

    // main game loop
    let sdl_config = SdlConfiguration::new("Ray Caster Demo", 320, 240, 3);
    let mut demo = Demo {};
    let ok = raycasting::run_sdl_loop(&sdl_config, &mut demo);
    if let Err(msg) = ok {
        println!("ERROR: {msg}");
    }
    else {
        println!("Raycaster demo finished OK :)");
    }
}

//------------------------------------

struct Demo {
}


impl GraphicsLoop for Demo {
    fn handle_event(&mut self, _delta_time: f64, _event: &Event) -> bool {
        true
    }

    fn run(&mut self, _delta_time: f64, cfg: &SdlConfiguration, painter: &mut dyn Painter) -> bool {
        for y in 0 .. cfg.height() {
            for x in 0 .. cfg.width() {
                let r = 128_u8;
                let g = (x & 0xFF) as u8;
                let b = (y & 0xFF) as u8;
                painter.draw_pixel(x, y, RGB::from(r, g, b));
            }
        }
        true
    }
}