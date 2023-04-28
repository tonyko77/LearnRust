//! SDL2 wrapper, to simplify using SDL2

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;

//use sdl2::Sdl;
//use sdl2::render::TextureCreator;
//use sdl2::video::Window;
//use sdl2::render::{Canvas, RenderTarget};
//use sdl2::render::Texture;
//use sdl2::pixels::Color;
//use sdl2::mouse::Cursor;
//use sdl2::rect::Rect;
//use sdl2::rect::Point;
//use sdl2::surface::Surface;
//use sdl2::video::WindowContext;

use std::time::Instant;



#[derive(Clone, Copy)]
pub struct RGB { r: u8, g: u8, b: u8 }

impl RGB {
    #[inline]
    pub fn from(r: u8, g: u8, b: u8) -> Self {
        RGB { r, g, b }
    }
}


pub trait Painter {
    fn draw_pixel(&mut self, x: u32, y: u32, color: RGB);

    fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: RGB) {
        if w > 0 && h > 0 {
            let x2 = x + w - 1;
            let y2 = y + h - 1;
            for xx in x..=x2 {
                self.draw_pixel(xx, y, color);
                self.draw_pixel(xx, y2, color);
            }
            for yy in (y+1)..y2 {
                self.draw_pixel(x, yy, color);
                self.draw_pixel(x2, yy, color);
            }
        }
    }

    fn fill_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: RGB) {
        if w > 0 && h > 0 {
            for yy in y .. (y+h) {
                for xx in x .. (x+w) {
                    self.draw_pixel(xx, yy, color);
                }
            }
        }
    }

}

pub struct SdlConfiguration {
    title: String,
    width: u32,
    height: u32,
    pixel_size: u32,
}

impl SdlConfiguration {
    pub fn new(title: &str, width: u32, height: u32, pixel_size: u32) -> Self {
        SdlConfiguration {
            title: String::from(title),
            width,
            height,
            pixel_size,
        }
    }
}


/// Must be implemented by whoever wants to use SDL
pub trait GraphicsLoop {
    fn handle_event(&mut self, elapsed_time: f64, event: &Event) -> bool;

    fn run(&mut self, elapsed_time: f64, painter: &mut dyn Painter) -> bool;
}



/// Main function to run the continuous SDL loop
pub fn run_sdl_loop(cfg: &SdlConfiguration, gfx_loop: &mut dyn GraphicsLoop) -> Result<(), String> {
    assert!(cfg.width > 0);
    assert!(cfg.height > 0);
    assert!(cfg.pixel_size > 0);
    let win_width = cfg.width * cfg.pixel_size;
    let win_height = cfg.height * cfg.pixel_size;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut window = video_subsystem
        .window(&cfg.title, win_width, win_height)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    // let mut canvas =
    //     window.into_canvas().build().map_err(|e| e.to_string())?;
    // canvas.set_blend_mode(sdl2::render::BlendMode::None);

    let mut srf = Surface::new(cfg.width, cfg.height, PixelFormatEnum::RGB24)?;


    // prepare for the loop
    let mut time_cnt = 0;
    let mut time_sum = 0.0_f64;
    let mut event_pump = sdl_context.event_pump()?;
    let mut moment = Instant::now();

    // main game loop
    'running: loop {
        // compute time
        let next_moment = Instant::now();
        let elapsed_time = next_moment.duration_since(moment).as_secs_f64();
        moment = next_moment;
        // compute FPS
        time_sum += elapsed_time;
        time_cnt += 1;
        if time_sum >= 1.0 {
            let avg = time_sum / (time_cnt as f64);
            let fps = if avg <= 0.0 { 999999.0 } else { 1.0 / avg };
            time_cnt = 0;
            time_sum = 0.0;
            window.set_title(format!("{} - FPS: {}", cfg.title, fps as usize).as_str()).unwrap();
        }

        // consume the event loop
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {
                    if !gfx_loop.handle_event(elapsed_time, &event) {
                        break 'running;
                    }
                }
            }
        }

        // draw stuff
        let mut ok_to_continue = true;
        srf.with_lock_mut(|buffer: &mut [u8]| {
            let pitch = (cfg.width * 3) as usize;
            let mut itp = InternalTexturePainter { buffer, pitch, cfg };
            ok_to_continue = gfx_loop.run(elapsed_time, &mut itp);
        });
        if !ok_to_continue {
            break 'running;
        }

        let mut wndsrf = window.surface(&event_pump)?;
        srf.blit_scaled(None, &mut wndsrf, None)?;
        wndsrf.finish()?;

    }

    Ok(())
}


struct InternalTexturePainter<'a> {
    buffer: &'a mut [u8],
    pitch: usize,
    cfg: &'a SdlConfiguration,
}

impl<'a> Painter for InternalTexturePainter<'a> {
    fn draw_pixel(&mut self, x: u32, y: u32, color: RGB) {
        if x < self.cfg.width && y < self.cfg.height {
            let offset = (y as usize) * self.pitch + (x as usize) * 3;
            self.buffer[offset + 0] = color.r;
            self.buffer[offset + 1] = color.g;
            self.buffer[offset + 2] = color.b;
        }
    }
}
