//! The ray casting engine/demo.

use crate::*;
use sdl2::keyboard::*;

// constant for converting degrees to radians
const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;

// adjustments for engine
const WALK_SPEED: f64 = 2.5;
const STRAFE_SPEED: f64 = WALK_SPEED;
const ROTATE_SPEED: f64 = 60.0;
const MIN_DISTANCE_TO_WALL: f64 = 0.2;
const HALF_HORIZ_FOV: f64 = 20.0;
const WALL_HEIGHT_SCALER: f64 = 1.0;
const MINI_MAP_WIDTH_PERCENT: u32 = 30;
const EPSILON: f64 = 0.001;

// bit flags for keys
const DO_WALK_FWD: u32 = 0x0001;
const DO_WALK_BACK: u32 = 0x0002;
const DO_STRAFE_LEFT: u32 = 0x0004;
const DO_STRAFE_RIGHT: u32 = 0x0008;
const DO_ROT_LEFT: u32 = 0x0010;
const DO_ROT_RIGHT: u32 = 0x0020;
//const DO_RUN: u32 = 0x0040;
//const DO_USE: u32 = 0x0080;
//const DO_SHOOT: u32 = 0x0100;

const MAP_EMPTY: u8 = 0;
const MAP_EDGE: u8 = u8::MAX;
const WALL_COLORS: &[RGB] = &[MAGENTA, BROWN, CYAN, RED, GREEN, YELLOW, BLUE];
const WALL_SHADINGS: &[u32] = &[100, 80, 60, 80];

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
        let w = self.map_width as i32;
        let h = self.map_height as i32;

        // move + check collision on X
        let move_x = pdx * distance;
        self.pos_x += move_x;
        let xx = if move_x < 0.0 {
            (self.pos_x - MIN_DISTANCE_TO_WALL) as i32
        } else {
            (self.pos_x + MIN_DISTANCE_TO_WALL) as i32
        };
        let coll_x = (xx < 0) || (xx >= w) || {
            let yl = (self.pos_y - MIN_DISTANCE_TO_WALL) as i32;
            let yh = (self.pos_y + MIN_DISTANCE_TO_WALL) as i32;
            yl < 0
                || yh >= h
                || self.map[(yl * w + xx) as usize] != MAP_EMPTY
                || self.map[(yh * w + xx) as usize] != MAP_EMPTY
        };
        if coll_x {
            // collision on X => restore X coordinate
            self.pos_x -= move_x;
        }

        // move + check collision on Y
        let move_y = pdy * distance;
        self.pos_y += move_y;
        let yy = if move_y < 0.0 {
            (self.pos_y - MIN_DISTANCE_TO_WALL) as i32
        } else {
            (self.pos_y + MIN_DISTANCE_TO_WALL) as i32
        };
        let coll_y = (yy < 0) || (yy >= h) || {
            let xl = (self.pos_x - MIN_DISTANCE_TO_WALL) as i32;
            let xh = (self.pos_x + MIN_DISTANCE_TO_WALL) as i32;
            xl < 0
                || xh >= w
                || self.map[(yy * w + xl) as usize] != MAP_EMPTY
                || self.map[(yy * w + xh) as usize] != MAP_EMPTY
        };
        if coll_y {
            // collision on Y => restore Y coordinate
            self.pos_y -= move_y;
        }
    }

    fn draw_mini_map(&self, painter: &mut dyn Painter) {
        let ms = self.mini_map_side;
        for x in 0..self.map_width as i32 {
            for y in 0..self.map_height as i32 {
                let idx = (y * (self.map_width as i32) + x) as usize;
                let color = Self::get_wall_color(self.map[idx], 0);
                painter.fill_rect(x * ms + 1, y * ms + 1, ms - 1, ms - 1, color);
            }
        }
    }

    fn draw_3d_view(&self, painter: &mut dyn Painter) {
        let width = self.scr_width as i32;

        // draw the view horizon
        let half_height = self.view_height / 2;
        for y in 0..=half_height {
            let shade_up = (y * 100 / half_height) as u8;
            let shade_down = 50 + (shade_up / 2);
            painter.draw_horiz_line(
                self.view_x,
                width - 1,
                y + self.view_y,
                RGB::from(shade_up, 128, 128),
            );
            painter.draw_horiz_line(
                self.view_x,
                width - 1,
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
            if (x & 0x07) == 0 {
                let ray_x = ((fov_angle * DEG_TO_RAD).cos() * dist * (ms as f64)) as i32;
                let ray_y = ((fov_angle * DEG_TO_RAD).sin() * dist * (ms as f64)) as i32;
                painter.draw_line(px, py, px + ray_x, py + ray_y, GREEN);
            }
            // draw the result of the ray cast on the 3D view
            let fish_eye_rectified_dist = dist * ((self.pos_angle - fov_angle) * DEG_TO_RAD).cos();
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
        let dirx = (self.pdx * (self.mini_map_side as f64) * 0.3) as i32;
        let diry = (self.pdy * (self.mini_map_side as f64) * 0.3) as i32;
        painter.draw_line(px, py, px + dirx, py + diry, LIGHT_YELLOW);
    }

    /// Computes: distance to wall, wall color index, wall orientation(0=N, 1=W, 2=S, 3=E).
    /// Thanks to [javidx9 a.k.a. olc](https://www.youtube.com/watch?v=NbSee-XM7WA)
    fn compute_ray(&self, angle: f64) -> (f64, u8, u8) {
        let sin = (angle * DEG_TO_RAD).sin();
        let cos = (angle * DEG_TO_RAD).cos();
        let map_w = self.map_width as i32;
        let map_h = self.map_height as i32;
        let mut map_x = self.pos_x as i32;
        let mut map_y = self.pos_y as i32;
        let mut map_idx = map_y * map_w + map_x;

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
                let m = if map_x < 0 || map_x >= map_w {
                    MAP_EDGE
                } else {
                    self.map[map_idx as usize]
                };
                if m != MAP_EMPTY {
                    return (dist_x, m, orient_x);
                }
                // continue on the X axis
                dist_x += scale_x;
            } else {
                // moving on the Y axis
                map_y += dir_y;
                map_idx += dir_y * map_w;
                let m = if map_y < 0 || map_y >= map_h {
                    MAP_EDGE
                } else {
                    self.map[map_idx as usize]
                };
                if m != MAP_EMPTY {
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
        match *key {
            Keycode::W => self.keys |= DO_WALK_FWD,
            Keycode::S => self.keys |= DO_WALK_BACK,
            Keycode::A => self.keys |= DO_ROT_LEFT,
            Keycode::D => self.keys |= DO_ROT_RIGHT,
            Keycode::Q => self.keys |= DO_STRAFE_LEFT,
            Keycode::E => self.keys |= DO_STRAFE_RIGHT,
            _ => {}
        }
    }

    #[inline]
    fn handle_key_up(&mut self, key: &Keycode) {
        match *key {
            Keycode::W => self.keys &= !DO_WALK_FWD,
            Keycode::S => self.keys &= !DO_WALK_BACK,
            Keycode::A => self.keys &= !DO_ROT_LEFT,
            Keycode::D => self.keys &= !DO_ROT_RIGHT,
            Keycode::Q => self.keys &= !DO_STRAFE_LEFT,
            Keycode::E => self.keys &= !DO_STRAFE_RIGHT,
            _ => {}
        }
    }

    #[inline]
    fn get_wall_color(wall: u8, orientation: u8) -> RGB {
        if wall == MAP_EMPTY || wall == MAP_EDGE {
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

            Event::KeyDown {
                keycode: Some(key), ..
            } => {
                self.handle_key_down(key);
            }

            Event::KeyUp {
                keycode: Some(key), ..
            } => {
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
        // handle movement
        if self.is_key_pressed(DO_WALK_FWD) {
            self.walk(elapsed_time * WALK_SPEED);
        }
        if self.is_key_pressed(DO_WALK_BACK) {
            self.walk(-elapsed_time * WALK_SPEED);
        }

        if self.is_key_pressed(DO_STRAFE_LEFT) {
            self.strafe(elapsed_time * STRAFE_SPEED);
        }
        if self.is_key_pressed(DO_STRAFE_RIGHT) {
            self.strafe(-elapsed_time * STRAFE_SPEED);
        }

        if self.is_key_pressed(DO_ROT_LEFT) {
            self.rotate(-elapsed_time * ROTATE_SPEED);
        }
        if self.is_key_pressed(DO_ROT_RIGHT) {
            self.rotate(elapsed_time * ROTATE_SPEED);
        }

        true
    }

    fn paint(&self, painter: &mut dyn Painter) {
        // clear the screen ...
        painter.fill_rect(
            0,
            0,
            self.scr_width as i32,
            self.scr_height as i32,
            DARK_GREY,
        );
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
                'A'..='Z' => {
                    // wall
                    self.0.map[idx] = 1 + (ch as u8) - ('A' as u8);
                    idx += 1;
                }
                '.' => {
                    // empty space
                    self.0.map[idx] = MAP_EMPTY;
                    idx += 1;
                }
                '@' => {
                    // player position
                    let y = (idx as u32) / self.0.map_width;
                    let x = (idx as u32) - y * self.0.map_width;
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
        self.0.mini_map_side = std::cmp::min(w, h) as i32;

        // the properties of the 3D view
        self.0.view_x = self.0.mini_map_side * (self.0.map_width as i32) + 2;
        self.0.view_width = (self.0.scr_width as i32) - self.0.view_x;
        self.0.view_height = self.0.scr_height as i32;

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
