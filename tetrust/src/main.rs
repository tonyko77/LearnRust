use olc_pixel_game_engine as olc;

const SCREEN_WIDTH:i32 = 320;
const SCREEN_HEIGHT:i32 = 240;
const PIXEL_SIZE:i32 = 3;


// Very simple example application that prints "Hello, World!" on screen.
struct ExampleProgram {}

impl olc::Application for ExampleProgram {
  fn on_user_create(&mut self) -> Result<(), olc::Error> {
    // Mirrors `olcPixelGameEngine::onUserCreate`. Your code goes here.
    olc::set_pixel_mode(olc::PixelMode::NORMAL);
    Ok(())
  }

  fn on_user_update(&mut self, _elapsed_time: f32) -> Result<(), olc::Error> {
    // Mirrors `olcPixelGameEngine::onUserUpdate`. Your code goes here.

    // Clears screen and sets black colour.
    //olc::clear(olc::BLACK);

    // (TONI) draws random pixels
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            let r: u8 = fastrand::u8(0..=255);
            let g: u8 = fastrand::u8(0..=255);
            let b: u8 = fastrand::u8(0..=255);
            let p = olc::Pixel { r, g, b, a: 255 };
            olc::draw(x, y, p);
        }
    }

    // Prints the string starting at the position (40, 40) and using white colour.
    //olc::draw_string(80, 80, "Hello, World!", olc::WHITE)?;

    Ok(())
  }

  fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
    // Mirrors `olcPixelGameEngine::onUserDestroy`. Your code goes here.
    Ok(())
  }
}

fn main() {
  let mut example = ExampleProgram {};
  // Launches the program in 200x100 "pixels" screen, where each "pixel" is 4x4 pixel square,
  // and starts the main game loop.
  olc::start("Hello, World!", &mut example,
    SCREEN_WIDTH, SCREEN_HEIGHT, PIXEL_SIZE, PIXEL_SIZE)
    .expect("Cannot initialize OLC");
}
