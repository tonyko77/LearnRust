//! Screen utilities for keeping resolution info and
//! computing based on that the FOV/aspect ratio data

#[derive(Debug, Clone)]
pub struct Screen {
    // TODO which of these are really needed?
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f64,
    pub dist_from_screen: f64,
    // TODO are these others needed?
    // pub fov: f32,
    // pub half_width: f32,
    // pub half_height: f32,
    // pub near_plane: f32,
    // pub far_plane: f32,
    // pub near_plane_height: f32,
    // pub near_plane_width: f32,
    // pub far_plane_height: f32,
    // pub far_plane_width: f32,
    // pub near_plane_top_left: Vec3,
    // pub near_plane_top_right: Vec3,
    // pub near_plane_bottom_left: Vec3,
    // pub near_plane_bottom_right: Vec3,
    // pub far_plane_top_left: Vec3,
    // pub far_plane_top_right: Vec3,
    // pub far_plane_bottom_left: Vec3,
    // pub far_plane_bottom_right: Vec3,
}

impl Screen {
    pub fn new(width: u32, height: u32) -> Self {
        let wf = width as f64;
        let hf = height as f64;
        let aspect_ratio = wf / hf;
        let dist_from_screen = (height as f64) * 2.0 / 3.0;
        assert!(dist_from_screen > 1.0);
        Self {
            width,
            height,
            aspect_ratio,
            dist_from_screen,
        }
    }

    // TODO return angles in radians? degrees? BOM??
    // (perhaps it is easiest to just keep angles in radians/f64, so we can call atan() etc on them)
    // -> for now, just return radians
    #[inline(always)]
    pub fn screen_x_to_angle(&self, x: i32) -> f64 {
        let dx = (x - (self.width as i32 / 2)).abs() as f64;
        (dx / self.dist_from_screen).atan()
    }

    #[inline(always)]
    pub fn fov_deg(&self) -> f64 {
        let hfov = self.screen_x_to_angle(0);
        hfov * 2.0 * 180.0 / std::f64::consts::PI
    }
}
