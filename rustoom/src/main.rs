//======================================================
//  DIY Doom Engine, written in Rust :)
//
// -> see Recreating DOOM on YouTube:
//          https://www.youtube.com/playlist?list=PLi77irUVkDasNAYQPr3N8nVcJLQAlANva
// -> see DIY Doom on GitHub:
//          https://github.com/amroibrahim/DIYDoom
//======================================================

/*
    TODO:
        - [DONE] load WAD
        - [next] figure out how to use SDL2 to DRAW BIG PIXELS in a FAST WAY !!!
        - [later] continue with DOOM DIY: https://github.com/amroibrahim/DIYDoom/tree/master/DIYDOOM/Notes002/notes
 */


// TODO - TEMPORARYLY disable warnings for dead code
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod game;
mod wad;
mod gfx;
mod utils;

fn main() -> Result<(), String> {
    let wad_path = "doom.wad";
    let wad_data = wad::WadData::load(wad_path, wad::WadKind::IWAD)?;
    let doom_game = game::DoomGame::new(wad_data)?;

    println!("*** Doom game loaded ok ***");
    test_sdl2()?;

    Ok(())
}


//--------------------------
// Test bench for SDL2

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::mouse::Cursor;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;

use fastrand;

use std::time::Instant;
use std::time::Duration;
use std::env;
use std::path::Path;


const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const PIX_SIZE: u32 = 3;
const SCR_WIDTH: u32 = WIDTH * PIX_SIZE;
const SCR_HEIGHT: u32 = HEIGHT * PIX_SIZE;


pub fn test_sdl2() -> Result<(), String> {
    let mut time_cnt = 0;
    let mut time_sum = 0.0;
    let mut time_min = 0.0;
    let mut time_max = 0.0;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", SCR_WIDTH, SCR_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    // create texture, to paint on
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, WIDTH, HEIGHT)
        .map_err(|e| e.to_string())?;

    //let mut surface = Surface::new(WIDTH, HEIGHT, PixelFormatEnum::RGB24)?;

    // canvas.set_draw_color(Color::RGB(255, 0, 0));
    // canvas.clear();
    // canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    let mut moment = Instant::now();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // compute time
        let next_moment = Instant::now();
        let duration = next_moment.duration_since(moment).as_secs_f64();
        moment = next_moment;
        // do some time measurements
        time_sum += duration;
        time_cnt += 1;
        if (duration > time_max) || (time_cnt <= 1) {
            time_max = duration;
        }
        if (duration < time_min) || (time_cnt <= 1) {
            time_min = duration;
        }
        if time_cnt >= 100 {
            let avg = time_sum / (time_cnt as f64);
            let fps = if avg <= 0.0 { 0.0 } else { 1.0 / avg };
            println!("[FPS] fps={fps}, avg={avg}, min={time_min}, max={time_max}");
            time_cnt = 0;
            time_sum = 0.0;
        }

        // draw stuff
        // improved this using SDL textures:
        // - see: https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/renderer-texture.rs
        // - see: https://www.reddit.com/r/cpp_questions/comments/eqwsao/sdl_rendering_way_too_slow/
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            // all painting must be done in this closure
            for y in 0..(HEIGHT as usize) {
                for x in 0..(WIDTH as usize) {
                    let r = fastrand::u8(0..=255);
                    let g = fastrand::u8(0..=255);
                    let b = fastrand::u8(0..=255);
    
                    let offset = y * pitch + x * 3;
                    buffer[offset + 0] = r;
                    buffer[offset + 1] = g;
                    buffer[offset + 2] = b;
                }
            }
        })?;

/* OLD DRAW, directly on canvas (I need to read a SDL tutorial ...)
        for y in 0 .. HEIGHT {
            for x in 0 .. WIDTH {
                // let r = rng.gen_range(0..=255);
                // let g = rng.gen_range(0..=255);
                // let b = rng.gen_range(0..=255);
                let r = fastrand::u8(0..=255);
                let g = fastrand::u8(0..=255);
                let b = fastrand::u8(0..=255);

                canvas.set_draw_color(Color::RGB(r, g, b));
                //canvas.draw_point(Point::new(x as i32, y as i32))?;
                canvas.fill_rect(Rect::new(
                    (x * PIX_SIZE) as i32,
                    (y * PIX_SIZE) as i32,
                    PIX_SIZE,
                    PIX_SIZE
                ))?;
            }
        }
*/
        canvas.copy(&texture, None, None)?;
        canvas.present();

        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 100));
        // The rest of the game loop goes here...
    }

    Ok(())
}
