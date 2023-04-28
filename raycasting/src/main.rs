//! Simple ray-casting engine demo, using SDL2
//! Inspired by [this YouTube clip](https://www.youtube.com/watch?v=gYRrGTC7GtA)

use raycasting::{self, RayCasterBuilder, SdlConfiguration, GraphicsLoop, RGB, Event, Painter};

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

const SCR_WIDTH: u32 = 320;
const SCR_HEIGHT: u32 = 240;
const PIXEL_SIZE: u32 = 3;

const SLEEP_MS: u32 = 0;

//------------------------------------

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
        SLEEP_MS);

    // main game loop
    let mut demo = Demo { cnt: 1, dir: 1, };
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
    cnt: i32,
    dir: i32,
}


impl GraphicsLoop for Demo {
    fn handle_event(&mut self, _elapsed_time: f64, _event: &Event) -> bool {
        true
    }

    fn run(&mut self, _elapsed_time: f64, painter: &mut dyn Painter) -> bool {
        if self.cnt > 0 && self.cnt < 128 {
            self.cnt += self.dir;
        }
        else {
            self.cnt -= self.dir;
            self.dir = -self.dir;
        }

        for y in 0 .. SCR_HEIGHT {
            for x in 0 .. SCR_WIDTH {
                // let r = (self.cnt & 0x7F) as u8; // + fastrand::u8(0..64);
                // let g = ((x * 256 / SCR_WIDTH) & 0xFF) as u8;
                // let b = ((y * 256 / SCR_HEIGHT) & 0xFF) as u8;
                let r = fastrand::u8(0..=255);
                let g = fastrand::u8(0..=255);
                let b = fastrand::u8(0..=255);
                painter.draw_pixel(x, y, RGB::from(r, g, b));
            }
        }
        true
    }
}