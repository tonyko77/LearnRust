//! The ray casting engine/demo.

use crate::{GraphicsLoop, Event, RGB, Painter};

//-------------------------------------------------------

/// `RayCaster` engine. Must be built using [`RayCasterBuilder`].
pub struct RayCaster {
    scr_width: u32,
    scr_height: u32,
    map_width: u32,
    map_height: u32,
    map: Vec<u8>, // just 1s and 0s, for now (later, we can use texture indices instead of 1)
    pos_x: f64,
    pos_y: f64,
}

impl RayCaster {
    pub fn render(&self /*, sdl wrapper OR trait draw_engine OR sth */) {
        println!("TODO render !!!");
    }

    pub fn walk(&mut self, distance: f64) {
        println!("TODO move forward/backward -> {distance}");
    }

    pub fn strafe(&mut self, distance: f64) {
        println!("TODO strafe left/right -> {distance}");
    }

    pub fn rotate(&mut self, radians: f64) {
        println!("TODO rotate left/right -> {radians}");
    }
}


//-------------------------------------------------------

/// Builder for [`RayCaster`].
pub struct RayCasterBuilder(RayCaster);

impl RayCasterBuilder {
    pub fn new() -> Self {
        RayCasterBuilder {
            0: RayCaster {
                scr_width: 0,
                scr_height: 0,
                map_width: 0,
                map_height: 0,
                map: vec![],
                pos_x: 0.0,
                pos_y: 0.0,
            }
        }
    }

    #[inline]
    pub fn map_size(&mut self, w: u32, h: u32) -> &mut Self {
        assert!(w > 0);
        assert!(h > 0);
        self.0.map_width = w;
        self.0.map_height = h;
        self
    }

    #[inline]
    pub fn scr_size(&mut self, w: u32, h: u32) -> &mut Self {
        assert!(w > 0);
        assert!(h > 0);
        self.0.scr_width = w;
        self.0.scr_height = h;
        self
    }

    pub fn map_from_str(&mut self, map_data: &str) -> &mut Self {
        assert!(self.0.map_width > 0);
        assert!(self.0.map_height > 0);

        // create an empty map
        let map_len = (self.0.map_width * self.0.map_height) as usize;
        self.0.map = vec![0; map_len];

        // fill the map based on the characters from the string
        let mut idx = 0_usize;
        for ch in map_data.chars() {
            match ch {
                'A'..='Z' => { // wall
                    self.0.map[idx] = 1 + (ch as u8) - ('A' as u8);
                    idx += 1;
                },
                '.' => { // empty space
                    self.0.map[idx] = 0;
                    idx += 1;
                },
                '@' => { // player position
                    let y = (idx as u32) / self.0.map_width;
                    let x = (idx as u32) - y * self.0.map_width;
                    self.0.pos_x = (x as f64) + 0.5;
                    self.0.pos_y = (y as f64) + 0.5;
                    idx += 1;
                },
                _ => {} // just skip all other characters
            }
            if idx >= map_len {
                break;
            }
        }

        self
    }

    pub fn build(self) -> RayCaster {
        assert!(self.0.map_width > 0);
        assert!(self.0.map_height > 0);
        let expected_len = self.0.map_width * self.0.map_height;
        assert_eq!(expected_len as usize, self.0.map.len());
        self.0
    }

}


//-------------------------------------------------------

/// Demo mini-game for ray casting.
pub struct RayCastingDemo {
    scr_width: i32,
    scr_height: i32,
}


impl RayCastingDemo {
    pub fn new(width: u32, height: u32) -> Self {
        RayCastingDemo {
            scr_width: width as i32,
            scr_height: height as i32,
        }
    }
}


impl GraphicsLoop for RayCastingDemo {
    fn handle_event(&mut self, _elapsed_time: f64, _event: &Event) -> bool {
        true
    }

    fn run(&mut self, _elapsed_time: f64, painter: &mut dyn Painter) -> bool {
        painter.draw_rect(0, 0,
            self.scr_width, self.scr_height,
            RGB::from(255, 0, 0));
/*
        let x1 = fastrand::u32(2 .. self.scr_width-2);
        let y1 = fastrand::u32(2 .. self.scr_height-2);
        let x2 = fastrand::u32(2 .. self.scr_width-2);
        let y2 = fastrand::u32(2 .. self.scr_height-2);

        let r = fastrand::u8(0..=255);
        let g = fastrand::u8(0..=255);
        let b = fastrand::u8(0..=255);
        let color = RGB::from(r, g, b);

        painter.draw_line(x1, y1, x2, y2, color);
 */
        painter.draw_circle(50, 50, 140, RGB::from(0, 255, 0));


        true
    }
}
