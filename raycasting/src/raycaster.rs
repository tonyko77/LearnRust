//! The ray casting engine/demo.

use crate::*;
use sdl2::keyboard::*;


// constants for converting degrees to radians
// const FULL_CIRCLE: f64 = std::f64::consts::PI / 180.0; 
// const CIRCLE_1_8: f64 = FULL_CIRCLE / 8.0; 
// const CIRCLE_1_4: f64 = FULL_CIRCLE / 4.0; 
// const CIRCLE_3_8: f64 = CIRCLE_1_4 + CIRCLE_1_8;
// const CIRCLE_1_2: f64 = FULL_CIRCLE / 2.0; 
// const CIRCLE_5_8: f64 = CIRCLE_1_2 + CIRCLE_1_8;
// const CIRCLE_3_4: f64 = CIRCLE_1_2 + CIRCLE_1_4;
// const CIRCLE_7_8: f64 = FULL_CIRCLE - CIRCLE_1_8;

// TO BE ADJUSTED
const MINI_MAP_WIDTH_PERCENT: u32 = 40;
const WALK_SPEED: f64 = 4.0;
const STRAFE_SPEED: f64 = WALK_SPEED;
const ROTATE_SPEED: f64 = 1.0;
const MIN_DISTANCE_TO_WALL: f64 = 0.2;

const     KEY_WALK_FWD: u32 = 0x01;
const    KEY_WALK_BACK: u32 = 0x02;
const  KEY_STRAFE_LEFT: u32 = 0x04;
const KEY_STRAFE_RIGHT: u32 = 0x08;
const     KEY_ROT_LEFT: u32 = 0x10;
const    KEY_ROT_RIGHT: u32 = 0x20;


/// `RayCaster` engine. Must be built using [`RayCasterBuilder`].
pub struct RayCaster {
    scr_width: u32,
    scr_height: u32,
    map_width: u32,
    map_height: u32,
    map: Vec<u8>, // just 1s and 0s, for now (later, we can use texture indices instead of 1)
    pos_x: f64,
    pos_y: f64,
    pos_angle: f64, // angle in DEGREES
    mini_map_side: i32,
    view_x: i32,
    view_y: i32,
    view_width: i32,
    view_height: i32,
    keys: u32,
}


impl RayCaster {
    pub fn walk(&mut self, distance: f64) {
        //TODO println!("TODO move forward/backward -> {distance}");
        self.pos_y -= distance; // TODO fake !!
        //TODO also take care of keeping some distance to any wall
    }

    pub fn strafe(&mut self, distance: f64) {
        //TODO println!("TODO strafe left/right -> {distance}");
        self.pos_x -= distance; // TODO fake !!
    }

    pub fn rotate(&mut self, rotation_degrees: f64) {
        self.pos_angle = add_angles_in_degrees(self.pos_angle, rotation_degrees);
    }

    /// Cast one ray, from the player position, at a given delta angle compared to the player position.
    /// Returns: (distance to wall, wall color index, wall orientation(0=N, 1=W, 2=S, 3=E))
    pub fn cast_ray(&self, delta_angle: f64) -> (f64, i32, i32) {
        let angle = add_angles_in_degrees(self.pos_angle, delta_angle);
        let orientation = 3;

        let mut distance: f64 = 0.0;

        println!("TODO Implement ray casting !!!");

        (distance, 0, orientation)
    }


    #[inline]
    fn is_key_pressed(&self, flag: u32) -> bool {
        (self.keys & flag) != 0
    }

    #[inline]
    fn handle_key_down(&mut self, key: &Keycode) {
        match *key {
            Keycode::W => self.keys |= KEY_WALK_FWD,
            Keycode::S => self.keys |= KEY_WALK_BACK,
            Keycode::A => self.keys |= KEY_STRAFE_LEFT,
            Keycode::D => self.keys |= KEY_STRAFE_RIGHT,
            Keycode::Q => self.keys |= KEY_ROT_LEFT,
            Keycode::E => self.keys |= KEY_ROT_RIGHT,
            _ => { } 
        }
    }

    #[inline]
    fn handle_key_up(&mut self, key: &Keycode) {
        match *key {
            Keycode::W => self.keys &= !KEY_WALK_FWD,
            Keycode::S => self.keys &= !KEY_WALK_BACK,
            Keycode::A => self.keys &= !KEY_STRAFE_LEFT,
            Keycode::D => self.keys &= !KEY_STRAFE_RIGHT,
            Keycode::Q => self.keys &= !KEY_ROT_LEFT,
            Keycode::E => self.keys &= !KEY_ROT_RIGHT,
            _ => { } 
        }
    }


    fn draw_mini_map(&self, painter: &mut dyn Painter) {
        // draw mini map
        let ms = self.mini_map_side;
        for x in 0..self.map_width as i32 {
            for y in 0..self.map_height as i32 {
                painter.draw_rect(x * ms, y * ms, ms + 1, ms + 1, DARK_GREY);
                let idx = (y * (self.map_width as i32) + x) as usize;
                let cell = self.map[idx];
                // TODO use various colors for walls ...
                let color = if cell == 0 { BLACK } else { WHITE };
                painter.fill_rect(x * ms + 1, y * ms + 1, ms - 1, ms - 1, color);
            }
        }

        // draw the player on the mini map
        let px = (self.pos_x * (ms as f64)) as i32;
        let py = (self.pos_y * (ms as f64)) as i32;
        painter.fill_circle(px, py, 2, LIGHT_YELLOW);

        // draw the player's direction
        // TODO ...
    }


    fn draw_3d_view(&self, painter: &mut dyn Painter) {
        let width = self.scr_width as i32;

        // draw the view horizon
        let half_height = self.view_height / 2;
        for y in 0..=half_height {
            let shade_up = (y * 100 / half_height) as u8;
            let shade_down = 50 + (shade_up / 2);
            painter.draw_horiz_line(self.view_x, width-1,
                y + self.view_y,
                RGB::from(shade_up, 128, 128));
            painter.draw_horiz_line(self.view_x, width-1,
                self.view_height - y + self.view_y,
                RGB::from(shade_down, shade_down, shade_down));
        }

        // cast rays to draw the walls
        // TODO ...
    }
}


impl GraphicsLoop for RayCaster {
    fn handle_event(&mut self, event: &Event) -> bool {
        // check keys
        match event {
            Event::KeyDown { keycode: Some(Keycode::X), keymod: modd,  .. } => { 
                if modd.contains(Mod::LALTMOD) {
                    return false;
                }
            },

            Event::KeyDown { keycode: Some(key), .. } => {
                self.handle_key_down(key);
            },

            Event::KeyUp { keycode: Some(key), .. } => {
                self.handle_key_up(key);
            },

            _ => { }
        }

        // exit on LAlt + X
        if let Event::KeyDown { keycode: Some(Keycode::X), keymod: modd,  .. } = event { 
            if modd.contains(Mod::LALTMOD) {
                return false;
            }
        }

        true
    }


    fn update_state(&mut self, elapsed_time: f64) -> bool {
        // handle movement
        if self.is_key_pressed(KEY_WALK_FWD) {
            self.walk(elapsed_time * WALK_SPEED);
        }
        if self.is_key_pressed(KEY_WALK_BACK) {
            self.walk(-elapsed_time * WALK_SPEED);
        }

        if self.is_key_pressed(KEY_STRAFE_LEFT) {
            self.strafe(elapsed_time * STRAFE_SPEED);
        }
        if self.is_key_pressed(KEY_STRAFE_RIGHT) {
            self.strafe(-elapsed_time * STRAFE_SPEED);
        }

        if self.is_key_pressed(KEY_ROT_LEFT) {
            self.rotate(elapsed_time * ROTATE_SPEED);
        }
        if self.is_key_pressed(KEY_ROT_RIGHT) {
            self.rotate(-elapsed_time * ROTATE_SPEED);
        }

        true
    }


    fn paint(&self, painter: &mut dyn Painter) {
        painter.fill_rect(0, 0,
            self.scr_width as i32, self.scr_height as i32,
            GREY);

        self.draw_mini_map(painter);
        self.draw_3d_view(painter);
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
                pos_angle: 0.0,
                mini_map_side: 0,
                view_x: 0,
                view_y: 0,
                view_width: 0,
                view_height: 0,
                keys: 0,
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

    pub fn build(mut self) -> RayCaster {
        // validate the data
        assert!(self.0.map_width > 0);
        assert!(self.0.map_height > 0);
        let expected_len = self.0.map_width * self.0.map_height;
        assert_eq!(expected_len as usize, self.0.map.len());

        // compute automap layout data
        let w = (self.0.scr_width * MINI_MAP_WIDTH_PERCENT / 100) / self.0.map_width;
        let h = self.0.scr_height / self.0.map_height;
        self.0.mini_map_side = std::cmp::min(w, h) as i32;

        // the properties of the 3D view
        self.0.view_x = self.0.mini_map_side * (self.0.map_width as i32) + 2;
        self.0.view_width = ((self.0.scr_width as i32) - self.0.view_x - 1) | 0x0001;
        self.0.view_height = self.0.scr_height as i32;
    
        self.0
    }
}


//---------------------------------------
//  Internal stuff

#[inline]
fn add_angles_in_degrees(a1: f64, a2: f64) -> f64 {
    let new_angle = a1 + a2;
    if new_angle < 0.0 {
        new_angle + 360.0
    }
    else if new_angle >= 360.0 {
        new_angle - 360.0
    }
    else {
        new_angle
    }
}
