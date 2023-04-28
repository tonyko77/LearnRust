//! SDL2 wrapper, to simplify using SDL2

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

use std::time::{Instant, Duration};



#[derive(Clone, Copy)]
pub struct RGB { r: u8, g: u8, b: u8 }

impl RGB {
    #[inline]
    pub fn from(r: u8, g: u8, b: u8) -> Self {
        RGB { r, g, b }
    }
}


/// Painter interface, to be passed to client code so it can perform painting.
/// *This is not meant to be implemented by client code.*
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


/// The configuration to be used for initializing SDL.
pub struct SdlConfiguration {
    title: String,
    width: u32,
    height: u32,
    pixel_size: u32,
    should_sleep: bool,
    sleep_duration: Duration,
}

impl SdlConfiguration {
    pub fn new(title: &str, width: u32, height: u32, pixel_size: u32, sleep_ms: u32) -> Self {
        SdlConfiguration {
            title: String::from(title),
            width,
            height,
            pixel_size,
            should_sleep: (sleep_ms > 0),
            sleep_duration: Duration::from_millis(sleep_ms as u64),
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

    // create window
    let win_width = cfg.width * cfg.pixel_size;
    let win_height = cfg.height * cfg.pixel_size;
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window(&cfg.title, win_width, win_height)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas =
        window.into_canvas().build().map_err(|e| e.to_string())?;

    // create texture, to paint on
    let texture_creator = canvas.texture_creator();
    let mut screen_buffer = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, cfg.width, cfg.height)
        .map_err(|e| e.to_string())?;

    let mut timer = FpsAndElapsedCounter::new();
    let mut last_fps = 42;
    let mut event_pump = sdl_context.event_pump()?;

    // Main game loop
    'running: loop {
        // compute time
        let elapsed_time = timer.update_and_get_ellapsed_time();
        if last_fps != timer.fps {
            last_fps = timer.fps;
            let title_with_fps = format!("{} - FPS: {}", cfg.title, last_fps);
            canvas.window_mut().set_title(&title_with_fps).map_err(|e| e.to_string())?;
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
        // improved this using SDL textures:
        // - see: https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/renderer-texture.rs
        // - see: https://www.reddit.com/r/cpp_questions/comments/eqwsao/sdl_rendering_way_too_slow/
        let mut ok = true;
        screen_buffer.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            // all painting must be done in this closure
            let mut painter = InternalTexturePainter { buffer, pitch, cfg };
            ok = gfx_loop.run(elapsed_time, &mut painter);
        })?;
        if !ok {
            break 'running;
        }

        // paint texture on screen
        canvas.copy(&screen_buffer, None, None)?;
        canvas.present();

        // sleep a bit, so we don't hog the CPU
        if cfg.should_sleep {
            std::thread::sleep(cfg.sleep_duration);
        }
    }

    Ok(())
}


//--------------------------------
// Internal details

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


struct FpsAndElapsedCounter {
    time_sum: f64,
    time_cnt: u32,
    fps: u32,
    last_moment: Instant,
}

impl FpsAndElapsedCounter {
    fn new() -> Self {
        FpsAndElapsedCounter { 
            time_cnt: 0,
            time_sum: 0.0,
            last_moment: Instant::now(),
            fps: 0
        }
    }

    fn update_and_get_ellapsed_time(&mut self) -> f64 {
        // compute time
        let next_moment = Instant::now();
        let elapsed_time = next_moment.duration_since(self.last_moment).as_secs_f64();
        self.last_moment = next_moment;

        // compute FPS
        self.time_sum += elapsed_time;
        self.time_cnt += 1;
        if self.time_sum >= 1.0 {
            let avg = self.time_sum / (self.time_cnt as f64);
            self.fps = if avg <= 0.0 { 999999 } else { (1.0 / avg) as u32 };
            self.time_cnt = 0;
            self.time_sum = 0.0;
        }

        elapsed_time
    }
}
