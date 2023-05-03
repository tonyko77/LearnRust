//! Main binary for TetRusT - Tetris-like game.

// This magic line prevents the opening of a terminal when launching a release build
#![cfg_attr(not(any(test, debug_assertions)), windows_subsystem = "windows")]

use tetrust::*;

const SCR_WIDTH: i32 = 400;
const SCR_HEIGHT: i32 = 300;
const PIX_SIZE: i32 = 3;
const SLEEP_KIND: SleepKind = SleepKind::YIELD;

fn main() {
    let mut example = ExampleProgram {};

    // TEST: print ALL tetriminoes
    for i in 0..7 {
        print_tetrimino(i);
    }

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

//----------------------------
// TEST: print tetriminoes
fn print_tetrimino(idx: usize) {
    let mut tetr = Tetrimino::from_index(idx);
    let mut rots: Vec<String> = vec![];

    println!(
        "--> Tetrimino {} (color = {})",
        tetr.name(),
        tetr.color_idx()
    );

    // build strings for each rotation
    for _ in 0..4 {
        let mut bytes: [u8; 16] = ['.' as u8; 16];
        // build a string with the tetrimino data
        for i in 0..4 {
            let x = tetr.x(i);
            let y = tetr.y(i);
            let idx = y * 4 + x + 4;
            assert!(idx >= 0 && idx < 16, "Invalid index: {idx}");
            bytes[idx as usize] = '#' as u8;
        }
        // save the string
        let s = std::str::from_utf8(&bytes).unwrap();
        rots.push(s.to_string());
        // go to the next rotation
        tetr.rotate_cw();
    }

    // print tetriminoes
    assert_eq!(4, rots.len());
    for i in 0..=3 {
        for j in 0..=3 {
            let s = rots[j].as_str();
            let ss = &s[(i * 4)..=(i * 4 + 3)];
            print!("   {ss}");
        }
        println!("");
    }
}

//----------------------------
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
