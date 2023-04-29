//! The ray casting engine/demo.

use crate::*;
use sdl2::keyboard::*;


// constants for converting degrees to radians
const FULL_CIRCLE: f64 = std::f64::consts::PI / 180.0; 

// TO BE ADJUSTED
const MINI_MAP_WIDTH_PERCENT: u32 = 40;
const WALK_SPEED: f64 = 4.0;
const STRAFE_SPEED: f64 = WALK_SPEED;
const ROTATE_SPEED: f64 = 150.0;
const MIN_DISTANCE_TO_WALL: f64 = 0.1;
const MINIMAP_DIRECTION_LEN: f64 = 10.0;
//const HALF_HORIZ_FOV: f64 = 45.0;

const     DO_WALK_FWD: u32 = 0x0001;
const    DO_WALK_BACK: u32 = 0x0002;
const  DO_STRAFE_LEFT: u32 = 0x0004;
const DO_STRAFE_RIGHT: u32 = 0x0008;
const     DO_ROT_LEFT: u32 = 0x0010;
const    DO_ROT_RIGHT: u32 = 0x0020;
//const          DO_RUN: u32 = 0x0040;
//const          DO_USE: u32 = 0x0080;
//const        DO_SHOOT: u32 = 0x0100;

const MAP_EMPTY: u8 = 0;
const MAP_EDGE: u8 = u8::MAX;

const WALL_COLORS: &[RGB] = &[
    MAGENTA,
    BROWN,
    CYAN,
    RED,
    GREEN,
    YELLOW,
    BLUE,
];
const WALL_SHADINGS: &[u32] = &[100, 80, 50, 70];


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
        self.pdx = (self.pos_angle * FULL_CIRCLE).cos();
        self.pdy = (self.pos_angle * FULL_CIRCLE).sin();
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
            yl < 0 || yh >= h ||
            self.map[(yl * w + xx) as usize] != MAP_EMPTY ||
            self.map[(yh * w + xx) as usize] != MAP_EMPTY
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
            xl < 0 || xh >= w ||
            self.map[(yy * w + xl) as usize] != MAP_EMPTY ||
            self.map[(yy * w + xh) as usize] != MAP_EMPTY
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
            painter.draw_horiz_line(self.view_x, width-1,
                y + self.view_y,
                RGB::from(shade_up, 128, 128));
            painter.draw_horiz_line(self.view_x, width-1,
                self.view_height - y + self.view_y,
                RGB::from(shade_down, shade_down, shade_down));
        }
    }

    fn draw_rays(&self, painter: &mut dyn Painter) {
        // player position on the mini map
        let ms = self.mini_map_side;
        let px = (self.pos_x * (ms as f64)) as i32;
        let py = (self.pos_y * (ms as f64)) as i32;

        // cast rays to draw the walls
        //self.cast_and_draw_ray(0.0, self.view_x + self.view_width / 2);
        // TODO only one ray for now 
        let delta_angle = 0.0;
        let angle = add_angles_in_degrees(self.pos_angle, delta_angle);
        let (dist, wall, orientation) = self.compute_ray(angle);
        let color = Self::get_wall_color(wall, orientation);

        // draw the ray on the mini map
        let ray_x = ((angle * FULL_CIRCLE).cos() * dist * (ms as f64)) as i32;
        let ray_y = ((angle * FULL_CIRCLE).sin() * dist * (ms as f64)) as i32;
        painter.draw_line(px, py, px + ray_x, py + ray_y, color);

        // draw the result of the ray cast on the 3D view
        // TODO ...


        // after the rays, draw the player on the mini map
        // (so it appears over the rays)
        painter.fill_circle(px, py, 2, LIGHT_YELLOW);
        // draw the player's direction
        let dirx = (self.pdx * MINIMAP_DIRECTION_LEN) as i32;
        let diry = (self.pdy * MINIMAP_DIRECTION_LEN) as i32;
        painter.draw_line(px, py, px + dirx, py + diry, ORANGE);
    }

    /// Computes: distance to wall, wall color index, wall orientation(0=N, 1=W, 2=S, 3=E)
    fn compute_ray(&self, angle: f64) -> (f64, u8, u8) {
        if angle > 359.9 || angle < 0.1 {
            // looking east
            self.compute_orthogonal_ray(1, 0, 3)
        }
        else if angle > 89.9 && angle < 90.1 {
            // looking south (IMPORTANT: Y axis is pointing DOWN)
            self.compute_orthogonal_ray(0, 1, 2)
        }
        else if angle > 179.9 && angle < 180.1 {
            // looking west
            self.compute_orthogonal_ray(-1, 0, 1)
        }
        else if angle > 269.9 && angle < 270.1 {
            // looking north (IMPORTANT: Y axis is pointing DOWN)
            self.compute_orthogonal_ray(0, -1, 0)
        }
        else {
            // looking at an angle => safe to use tan()
            let atan = -1.0 / angle.tan();

            // compute directions
            let (dx, dy) = 
                if angle < 90.0 { (1, 1) }
                else if angle < 180.0 { (-1, 1) }
                else if angle < 180.0 { (-1, -1) }
                else { (1, -1) };

            // !! TODO implement this !!
            (0.0, 0, 0)
        }
    }

    fn compute_orthogonal_ray(&self, dx: i32, dy: i32, orientation: u8) -> (f64, u8, u8) {
        let mut x = (self.pos_x) as i32;
        let mut y = (self.pos_y) as i32;

        let mut dist: f64 =
            if dy == 0 {
                if dx < 0 {
                    self.pos_x - (x as f64)
                }
                else {
                    ((x + 1) as f64) - self.pos_x
                }
            }
            else {
                if dy < 0 {
                    self.pos_y - (y as f64)
                }
                else {
                    ((y + 1) as f64) - self.pos_y
                }
            };

        let mut wall: u8 = 0;
        let w = self.map_width as i32;
        let h = self.map_height as i32;
        x += dx;
        y += dy;

        while x >= 0 && x < w && y >= 0 && y < h {
            let idx = (y * w + x) as usize;
            wall = self.map[idx];
            if wall > 0 {
                break;
            }
            dist += 1.0;
            x += dx;
            y += dy;
        }

        (dist, wall, orientation)
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
            Keycode::A => self.keys |= DO_STRAFE_LEFT,
            Keycode::D => self.keys |= DO_STRAFE_RIGHT,
            Keycode::Q => self.keys |= DO_ROT_LEFT,
            Keycode::E => self.keys |= DO_ROT_RIGHT,
            _ => { } 
        }
    }

    #[inline]
    fn handle_key_up(&mut self, key: &Keycode) {
        match *key {
            Keycode::W => self.keys &= !DO_WALK_FWD,
            Keycode::S => self.keys &= !DO_WALK_BACK,
            Keycode::A => self.keys &= !DO_STRAFE_LEFT,
            Keycode::D => self.keys &= !DO_STRAFE_RIGHT,
            Keycode::Q => self.keys &= !DO_ROT_LEFT,
            Keycode::E => self.keys &= !DO_ROT_RIGHT,
            _ => { } 
        }
    }

    #[inline]
    fn get_wall_color(wall: u8, orientation: u8) -> RGB {
        if wall == MAP_EMPTY || wall == MAP_EDGE {
            BLACK
        }
        else {
            let color = WALL_COLORS[(wall as usize) % WALL_COLORS.len()];
            let shading = WALL_SHADINGS[(orientation as usize) % WALL_SHADINGS.len()];
            RGB {
                r: Self::shade_wall_color(color.r, shading),
                g: Self::shade_wall_color(color.g, shading),
                b: Self::shade_wall_color(color.b, shading),
            }
        }
    }

    #[inline]
    fn shade_wall_color(rgb: u8, shading: u32) -> u8 {
        ((rgb as u32) * shading / 100) as u8
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
        painter.fill_rect(0, 0,
            self.scr_width as i32, self.scr_height as i32,
            DARK_GREY);
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
                    self.0.map[idx] = MAP_EMPTY;
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
        self.0.view_width = (self.0.scr_width as i32) - self.0.view_x;
        self.0.view_height = self.0.scr_height as i32;
    
        // pre-compute rotation data
        self.0.rotate(0.0);

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
