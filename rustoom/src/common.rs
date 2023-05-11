//! Common structures, used in many places.

use std::{
    f64::consts::PI,
    ops::{Add, Sub},
};

/// A Vertex is a point in the 2D top-view space of a level map.<br/>
/// **Note:** the Y axis goes *upwards* (towards North), like in a normal xOy system,
/// and not like on screen, where the Y axis goes downwards.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Vertex {
    pub x: i32,
    pub y: i32,
}

impl Vertex {
    #[inline]
    pub fn add(&self, other: &Vertex) -> Vertex {
        Vertex {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    #[inline]
    pub fn sub(&self, other: &Vertex) -> Vertex {
        Vertex {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    #[inline]
    pub fn scale(&self, mul: i32, div: i32) -> Vertex {
        Vertex {
            x: self.x * mul / div,
            y: self.y * mul / div,
        }
    }
}

/// Screen class - keeps resolution info and computes based on that the FOV/aspect ratio data.
#[derive(Debug, Clone)]
pub struct Screen {
    width: u32,
    height: u32,
    dist_from_screen: f64,
}

impl Screen {
    pub fn new(width: u32, height: u32) -> Self {
        // compute distance assuming a 4/3 aspect ratio and a 90 degrees FOV,
        // based on screen height (as if width would be 4/3 of height)
        let dist_from_screen = (height as f64) * 2.0 / 3.0;
        assert!(dist_from_screen > 1.0);
        Self {
            width,
            height,
            dist_from_screen,
        }
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    // TODO: is this used as is? do I need to make it abs()?
    // or should I make angle positive towards left and negative towards right?
    #[inline]
    pub fn screen_x_to_angle(&self, x: i32) -> Angle {
        let dx = (x - (self.width as i32 / 2)).abs() as f64;
        let rad = (dx / self.dist_from_screen).atan();
        Angle::from_radians(rad)
    }

    #[inline]
    pub fn fov(&self) -> Angle {
        let hfov = self.screen_x_to_angle(0);
        hfov + hfov
    }

    #[inline]
    pub fn aspect_ratio(&self) -> f64 {
        let wf = self.width as f64;
        let hf = self.height as f64;
        wf / hf
    }
}

/// Angle representation - kept as radians, for easier trigonometry.
/// Also implements useful operations for angles.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Angle(f64);

impl Angle {
    #[inline]
    pub fn from_radians(rad: f64) -> Self {
        Self(rad)
    }

    /// Things use degrees as angles, from 0 (east) to 90 (north), 180 (west), 270 (south), up to 359.
    #[inline]
    pub fn from_degrees(deg: i32) -> Self {
        // restrict degrees to [0 .. 360)
        let xdeg = ((deg % 360) + if deg < 0 { 360 } else { 0 }) as f64;
        // convert degrees to radians
        let rad = xdeg * PI / 180.0;
        Self(rad)
    }

    /// Segment angles go from 0 (east), through 32768 (west, half a circle), to 65535 (almost full circle).
    #[inline]
    pub fn from_segment_angle(seg_angle: u16) -> Self {
        let rad = (seg_angle as f64) * PI / 32768.0;
        Self(rad)
    }

    #[inline]
    pub fn from_vector(orig: Vertex, dir: Vertex) -> Self {
        Self::from_vector_delta(dir.x - orig.x, dir.y - orig.y)
    }

    #[inline]
    pub fn from_vector_delta(dx: i32, dy: i32) -> Self {
        let rad = if dx == 0 {
            if dy > 0 {
                PI * 0.5
            } else {
                PI * 1.5
            }
        } else {
            ((dy as f64) / (dx as f64)).atan()
        };
        Self(rad)
    }

    #[inline]
    pub fn rad(&self) -> f64 {
        self.0
    }

    pub fn deg(&self) -> i32 {
        let deg = (self.0 * 180.0 / PI) as i32;
        // restrict degrees to [0 .. 360)
        (deg % 360) + if deg < 0 { 360 } else { 0 }
    }
}

impl Default for Angle {
    fn default() -> Self {
        Self(0.0)
    }
}

impl Add for Angle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Angle {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
