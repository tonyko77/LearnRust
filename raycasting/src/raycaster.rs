//! The ray casting engine/demo.

/*
 * STILL TO DO:
 *  1. Textures
 *  2. Floor + ceiling tiles
 */

use crate::*;
use sdl2::keyboard::*;

// constant for converting degrees to radians
const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;

// adjustments for engine
const WALK_SPEED: f64 = 2.5;
const ROTATE_SPEED: f64 = 60.0;
const RUN_MULTIPLIER: f64 = 2.5;
const MIN_DISTANCE_TO_WALL: f64 = 0.25;
const HALF_HORIZ_FOV: f64 = 23.0;
const WALL_HEIGHT_SCALER: f64 = 1.0;
const MINI_MAP_WIDTH_PERCENT: i32 = 30;
const EPSILON: f64 = 0.001;

// bit flags for keys
const DO_WALK_FWD: u32 = 0x01;
const DO_WALK_BACK: u32 = 0x02;
const DO_ROT_LEFT: u32 = 0x04;
const DO_ROT_RIGHT: u32 = 0x08;
const DO_STRAFE_LEFT: u32 = 0x10;
const DO_STRAFE_RIGHT: u32 = 0x20;
const DO_RUN: u32 = 0x40;

const KEY_PAIRS: &[(Keycode, u32)] = &[
    (Keycode::W, DO_WALK_FWD),
    (Keycode::S, DO_WALK_BACK),
    (Keycode::A, DO_ROT_LEFT),
    (Keycode::D, DO_ROT_RIGHT),
    (Keycode::Q, DO_STRAFE_LEFT),
    (Keycode::E, DO_STRAFE_RIGHT),
    (Keycode::LShift, DO_RUN),
];

const MAP_EDGE: u8 = u8::MAX;
const WALL_COLORS: &[RGB] = &[MAGENTA, BROWN, CYAN, RED, GREEN, YELLOW, BLUE];
const WALL_SHADINGS: &[u32] = &[100, 80, 60, 80];

//-------------------------------------------------------

/// `RayCaster` engine. Must be built using [`RayCasterBuilder`].
pub struct RayCaster {
    scr_width: i32,
    scr_height: i32,
    map_width: i32,
    map_height: i32,
    map: Vec<u8>, // just 1s and 0s, for now (later, we can use texture indices instead of 1)
    pos_x: f64,
    pos_y: f64,
    pos_angle: f64, // angle in DEGREES
    pdx: f64,
    pdy: f64,
    mini_map_side: i32,
    view_x: i32,
    view_y: i32,
    view_width: i32,
    view_height: i32,
    keys: u32,
}

impl RayCaster {
    pub fn walk(&mut self, distance: f64) {
        self.move_and_keep_away_from_obstacles(distance, self.pdx, self.pdy);
    }

    pub fn strafe(&mut self, distance: f64) {
        // "fake" strafing by swapping pdx and pdy + changing the sign for the Y direction
        self.move_and_keep_away_from_obstacles(distance, self.pdy, -self.pdx);
    }

    pub fn rotate(&mut self, rotation_degrees: f64) {
        self.pos_angle = add_angles_in_degrees(self.pos_angle, rotation_degrees);
        self.pdx = (self.pos_angle * DEG_TO_RAD).cos();
        self.pdy = (self.pos_angle * DEG_TO_RAD).sin();
    }

    fn move_and_keep_away_from_obstacles(&mut self, distance: f64, pdx: f64, pdy: f64) {
        // perform move
        let move_x = pdx * distance;
        let move_y = pdy * distance;
        self.pos_x += move_x;
        self.pos_y += move_y;
        // compute self position + position ahead of movement direction, as integers
        let px = self.pos_x as i32;
        let py = self.pos_y as i32;
        let ax = (self.pos_x + MIN_DISTANCE_TO_WALL * move_x.signum()) as i32;
        let ay = (self.pos_y + MIN_DISTANCE_TO_WALL * move_y.signum()) as i32;
        // check for collisions on each axis
        if ax < 0 || ax >= self.map_width || self.map[(py * self.map_width + ax) as usize] != 0 {
            self.pos_x -= move_x;
        }
        if ay < 0 || ay >= self.map_height || self.map[(ay * self.map_width + px) as usize] != 0 {
            self.pos_y -= move_y;
        }
    }

    fn draw_mini_map(&self, painter: &mut dyn Painter) {
        let ms = self.mini_map_side;
        for x in 0..self.map_width {
            for y in 0..self.map_height {
                let idx = (y * self.map_width + x) as usize;
                let color = Self::get_wall_color(self.map[idx], 0);
                painter.fill_rect(x * ms + 1, y * ms + 1, ms - 1, ms - 1, color);
            }
        }
    }

    fn draw_3d_view(&self, painter: &mut dyn Painter) {
        // draw the view horizon
        let half_height = self.view_height / 2;
        for y in 0..=half_height {
            let shade_up = (y * 100 / half_height) as u8;
            let shade_down = 50 + (shade_up / 2);
            painter.draw_horiz_line(
                self.view_x,
                self.scr_width - 1,
                y + self.view_y,
                RGB::from(shade_up, 128, 128),
            );
            painter.draw_horiz_line(
                self.view_x,
                self.scr_width - 1,
                self.view_height - y + self.view_y,
                RGB::from(shade_down, shade_down, shade_down),
            );
        }
    }

    fn draw_rays(&self, painter: &mut dyn Painter) {
        // player position on the mini map
        let ms = self.mini_map_side;
        let px = (self.pos_x * (ms as f64)) as i32;
        let py = (self.pos_y * (ms as f64)) as i32;
        // Half FOV, corrected for screen aspect ratio
        let chhf = HALF_HORIZ_FOV * (self.view_width as f64) / (self.view_height as f64);

        // cast rays to draw the walls
        let mut fov_angle = add_angles_in_degrees(self.pos_angle, -chhf);
        let fov_step = 2.0 * chhf / (self.view_width as f64);
        for x in 0..self.view_width {
            let (dist, wall, orientation) = self.compute_ray(fov_angle);
            let color = Self::get_wall_color(wall, orientation);
            // draw SOME of the rays on the mini map
            if (x & 0x0F) == 0 {
                let ray_x = ((fov_angle * DEG_TO_RAD).cos() * dist * (ms as f64)) as i32;
                let ray_y = ((fov_angle * DEG_TO_RAD).sin() * dist * (ms as f64)) as i32;
                painter.draw_line(px, py, px + ray_x, py + ray_y, color);
            }
            // rectify the ray distance, to avoid the "fish eye" distortion
            // - see: https://gamedev.stackexchange.com/questions/97574/how-can-i-fix-the-fisheye-distortion-in-my-raycast-renderer
            let fish_eye_rectified_dist = dist * ((self.pos_angle - fov_angle) * DEG_TO_RAD).cos();
            // draw the result of the ray cast on the 3D view
            let s = WALL_HEIGHT_SCALER / fish_eye_rectified_dist;
            if s > 0.01 {
                let h = if s >= 1.0 {
                    self.view_height
                } else {
                    (s * (self.view_height as f64)) as i32
                };
                let y = (self.view_height - h) >> 1;
                painter.draw_vert_line(x + self.view_x, y, y + h, color);
            }
            // move to next ray
            fov_angle += fov_step;
        }

        // after the rays, draw the player on the mini map
        // (so it appears over the rays)
        painter.fill_circle(px, py, 2, LIGHT_YELLOW);
        // draw the player's direction
        let delta_x = (self.pdx * (self.mini_map_side as f64) * 0.3) as i32;
        let delta_y = (self.pdy * (self.mini_map_side as f64) * 0.3) as i32;
        painter.draw_line(px, py, px + delta_x, py + delta_y, LIGHT_YELLOW);
    }

    /// Computes: distance to wall, wall color index, wall orientation(0=N, 1=W, 2=S, 3=E).
    /// Thanks to [javidx9 a.k.a. olc](https://www.youtube.com/watch?v=NbSee-XM7WA)
    fn compute_ray(&self, angle: f64) -> (f64, u8, u8) {
        let sin = (angle * DEG_TO_RAD).sin();
        let cos = (angle * DEG_TO_RAD).cos();
        let mut map_x = self.pos_x as i32;
        let mut map_y = self.pos_y as i32;
        let mut map_idx = map_y * self.map_width + map_x;

        let (mut dist_x, scale_x, dir_x, orient_x) = if cos > EPSILON {
            // looking RIGHT
            let d = self.pos_x.floor() + 1.0 - self.pos_x;
            (d / cos, 1.0 / cos, 1, 3_u8)
        } else if cos < -EPSILON {
            // looking LEFT
            let d = self.pos_x.floor() - self.pos_x;
            (d / cos, -1.0 / cos, -1, 1_u8)
        } else {
            // straight vertical => no hits on the X axis
            (f64::MAX, 0.0, 0, 0_u8)
        };

        let (mut dist_y, scale_y, dir_y, orient_y) = if sin > EPSILON {
            // looking DOWN
            let d = self.pos_y.floor() + 1.0 - self.pos_y;
            (d / sin, 1.0 / sin, 1, 2_u8)
        } else if sin < -EPSILON {
            // looking UP
            let d = self.pos_y.floor() - self.pos_y;
            (d / sin, -1.0 / sin, -1, 0_u8)
        } else {
            // straight horizontal => no hits on the Y axis
            (f64::MAX, 0.0, 0, 0_u8)
        };

        loop {
            if dist_x < dist_y {
                // moving on the X axis
                map_x += dir_x;
                map_idx += dir_x;
                let m = if map_x < 0 || map_x >= self.map_width {
                    MAP_EDGE
                } else {
                    self.map[map_idx as usize]
                };
                if m != 0 {
                    return (dist_x, m, orient_x);
                }
                // continue on the X axis
                dist_x += scale_x;
            } else {
                // moving on the Y axis
                map_y += dir_y;
                map_idx += dir_y * self.map_width;
                let m = if map_y < 0 || map_y >= self.map_height {
                    MAP_EDGE
                } else {
                    self.map[map_idx as usize]
                };
                if m != 0 {
                    return (dist_y, m, orient_y);
                }
                // continue on the Y axis
                dist_y += scale_y;
            }
        }
    }

    #[inline]
    fn is_key_pressed(&self, flag: u32) -> bool {
        (self.keys & flag) != 0
    }

    #[inline]
    fn handle_key_down(&mut self, key: &Keycode) {
        for p in KEY_PAIRS {
            if *key == p.0 {
                self.keys |= p.1;
            }
        }
    }

    #[inline]
    fn handle_key_up(&mut self, key: &Keycode) {
        for p in KEY_PAIRS {
            if *key == p.0 {
                self.keys &= !p.1;
            }
        }
    }

    #[inline]
    fn get_wall_color(wall: u8, orientation: u8) -> RGB {
        if wall == 0 || wall == MAP_EDGE {
            BLACK
        } else {
            let color = WALL_COLORS[(wall as usize) % WALL_COLORS.len()];
            let shading = WALL_SHADINGS[(orientation as usize) % WALL_SHADINGS.len()];
            RGB {
                r: ((color.r as u32) * shading / 100) as u8,
                g: ((color.g as u32) * shading / 100) as u8,
                b: ((color.b as u32) * shading / 100) as u8,
            }
        }
    }
}

//-------------------------------------------------------

impl GraphicsLoop for RayCaster {
    fn handle_event(&mut self, event: &Event) -> bool {
        // check keys
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::X),
                keymod: modd,
                ..
            } => {
                if modd.contains(Mod::LALTMOD) {
                    return false;
                }
            }

            Event::KeyDown { keycode: Some(key), .. } => {
                self.handle_key_down(key);
            }

            Event::KeyUp { keycode: Some(key), .. } => {
                self.handle_key_up(key);
            }

            _ => {}
        }

        // exit on LAlt + X
        if let Event::KeyDown {
            keycode: Some(Keycode::X),
            keymod: modd,
            ..
        } = event
        {
            if modd.contains(Mod::LALTMOD) {
                return false;
            }
        }

        true
    }

    fn update_state(&mut self, elapsed_time: f64) -> bool {
        let mult = if self.is_key_pressed(DO_RUN) {
            RUN_MULTIPLIER
        } else {
            1.0
        };

        // handle movement
        if self.is_key_pressed(DO_WALK_FWD) {
            self.walk(mult * WALK_SPEED * elapsed_time);
        }
        if self.is_key_pressed(DO_WALK_BACK) {
            self.walk(-mult * WALK_SPEED * elapsed_time);
        }

        if self.is_key_pressed(DO_STRAFE_LEFT) {
            self.strafe(mult * WALK_SPEED * elapsed_time);
        }
        if self.is_key_pressed(DO_STRAFE_RIGHT) {
            self.strafe(-mult * WALK_SPEED * elapsed_time);
        }

        if self.is_key_pressed(DO_ROT_LEFT) {
            self.rotate(-mult * ROTATE_SPEED * elapsed_time);
        }
        if self.is_key_pressed(DO_ROT_RIGHT) {
            self.rotate(mult * ROTATE_SPEED * elapsed_time);
        }

        true
    }

    fn paint(&self, painter: &mut dyn Painter) {
        // clear the screen ...
        painter.fill_rect(0, 0, self.scr_width, self.scr_height, DARK_GREY);
        // ... and draw everything
        self.draw_mini_map(painter);
        self.draw_3d_view(painter);
        self.draw_rays(painter);
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
                pos_angle: 270.0,
                pdx: 0.0,
                pdy: 0.0,
                mini_map_side: 0,
                view_x: 0,
                view_y: 0,
                view_width: 0,
                view_height: 0,
                keys: 0,
            },
        }
    }

    #[inline]
    pub fn map_size(&mut self, map_width: i32, map_height: i32) -> &mut Self {
        assert!(map_width > 0);
        assert!(map_height > 0);
        self.0.map_width = map_width;
        self.0.map_height = map_height;
        self
    }

    #[inline]
    pub fn scr_size(&mut self, scr_width: i32, scr_height: i32) -> &mut Self {
        assert!(scr_width > 0);
        assert!(scr_height > 0);
        self.0.scr_width = scr_width;
        self.0.scr_height = scr_height;
        self
    }

    pub fn map_from_str(&mut self, map_data: &str) -> &mut Self {
        assert!(self.0.map_width > 0);
        assert!(self.0.map_height > 0);

        // create an empty map
        let map_len = self.0.map_width * self.0.map_height;
        self.0.map = vec![0; map_len as usize];

        // fill the map based on the characters from the string
        let mut idx: i32 = 0;
        for ch in map_data.chars() {
            match ch {
                'A'..='Z' => {
                    // wall
                    self.0.map[idx as usize] = 1 + (ch as u8) - ('A' as u8);
                    idx += 1;
                }
                '.' => {
                    // empty space
                    self.0.map[idx as usize] = 0;
                    idx += 1;
                }
                '@' => {
                    // player position
                    let y = idx / self.0.map_width;
                    let x = idx - y * self.0.map_width;
                    self.0.pos_x = (x as f64) + 0.5;
                    self.0.pos_y = (y as f64) + 0.5;
                    idx += 1;
                }
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
        self.0.mini_map_side = std::cmp::min(w, h);

        // the properties of the 3D view
        self.0.view_x = self.0.mini_map_side * (self.0.map_width) + 2;
        self.0.view_width = self.0.scr_width - self.0.view_x;
        self.0.view_height = self.0.scr_height;

        // pre-compute rotation data
        self.0.rotate(0.0);

        self.0
    }
}

//-------------------------------------------------------
//  Internal stuff

#[inline]
fn add_angles_in_degrees(a1: f64, a2: f64) -> f64 {
    let new_angle = a1 + a2;
    if new_angle < 0.0 {
        new_angle + 360.0
    } else if new_angle >= 360.0 {
        new_angle - 360.0
    } else {
        new_angle
    }
}
