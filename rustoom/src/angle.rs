//! Common structures, used in many places.

use crate::map_items::Vertex;
use std::{
    f64::consts::PI,
    ops::{Add, Div, Mul, Sub},
};

/// Angle representation - kept as radians, for easier trigonometry.
/// Also implements useful operations for angles.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Angle(f64);

impl Angle {
    #[inline]
    pub fn from_radians(rad: f64) -> Self {
        const PI2: f64 = 2.0 * PI;
        let rad = rad % PI2;
        if rad >= 0.0 {
            Self(rad)
        } else {
            Self(rad + PI2)
        }
    }

    /// Things use degrees as angles, from 0 (east) to 90 (north), 180 (west), 270 (south), up to 359.
    #[inline]
    pub fn from_degrees(deg: i32) -> Self {
        Self::from_radians((deg as f64) * PI / 180.0)
    }

    /// Segment angles go from 0 (east), through 32768 (west, half a circle), to 65535 (almost full circle).
    #[inline]
    pub fn from_segment_angle(seg_angle: u16) -> Self {
        Self::from_radians((seg_angle as f64) * PI / 32768.0)
    }

    #[inline]
    pub fn from_vector(orig: Vertex, dir: Vertex) -> Self {
        Self::from_vector_delta(dir.x - orig.x, dir.y - orig.y)
    }

    #[inline]
    pub fn from_vector_delta(dx: i32, dy: i32) -> Self {
        if dx != 0 {
            // safe to use arctan
            Self::from_radians(((dy as f64) / (dx as f64)).atan())
        } else if dy > 0 {
            // vector points straight up => 90 degrees, or 1/2 PI
            Self(PI * 0.5)
        } else {
            // vector points straight down => 270 degrees, or 3/2 PI
            Self(PI * 1.5)
        }
    }

    #[inline]
    pub fn rad(&self) -> f64 {
        self.0
    }

    #[inline]
    pub fn deg(&self) -> i32 {
        (self.0 * 180.0 / PI + 0.03125) as i32
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

impl Mul<f64> for Angle {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::from_radians(self.0 * rhs)
    }
}

impl Div<f64> for Angle {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::from_radians(self.0 / rhs)
    }
}
