//! Main binary for TetRusT - Tetris-like game.

// This magic line prevents the opening of a terminal when launching the app
// TODO - FIGURE THIS OUT !!!
//#![cfg_attr(dev, windows_subsystem = "windows")]

use tetrust::*;

const SCR_WIDTH: i32 = 320;
const SCR_HEIGHT: i32 = 240;
const PIX_SIZE: i32 = 3;
const SLEEP_KIND: SleepKind = SleepKind::YIELD;

fn main() {
    let mut example = ExampleProgram {};

    // main game loop
    let sdl_config = SdlConfiguration::new(
        "Ray Caster Demo",
        SCR_WIDTH,
        SCR_HEIGHT,
        PIX_SIZE,
        SLEEP_KIND,
    );
    let res = tetrust::run_sdl_loop(&sdl_config, &mut example);
    if let Err(msg) = res {
        println!("ERROR: {msg}");
    } else {
        println!("TetRusT finished OK :)");
    }
}

// TEMPORARY demo
struct ExampleProgram {}

impl GraphicsLoop for ExampleProgram {
    fn handle_event(&mut self, _event: &sdl2::event::Event) -> bool {
        true
    }

    fn update_state(&mut self, _elapsed_time: f64) -> bool {
        true
    }

    fn paint(&self, painter: &mut dyn Painter) {
        // TEMPORARY - draw random pixels
        for y in 0..SCR_HEIGHT {
            for x in 0..SCR_WIDTH {
                let r: u8 = fastrand::u8(0..=255);
                let g: u8 = fastrand::u8(0..=255);
                let b: u8 = fastrand::u8(0..=255);
                painter.draw_pixel(x, y, RGB::from(r, g, b));
            }
        }
    }
}
