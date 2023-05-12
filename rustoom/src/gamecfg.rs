//! Immutable game data:
//! * screen resolution and FOV
//! * parsed WAD
//! * Font
//! * Graphics (patches, flats, textures)
//! * Palette (and colormap)

use crate::{angle::Angle, font::Font, graphics::Graphics, palette::Palette, WadData};
use std::rc::Rc;

pub struct GameConfig(Rc<InternalGameData>);

impl GameConfig {
    pub fn new(wad_data: WadData, scr_width: i32, scr_height: i32) -> Self {
        assert!(scr_width > 0);
        assert!(scr_height > 0);
        assert!(wad_data.map_count() > 0);

        let igd = InternalGameData {
            wad_data,
            scr_width,
            scr_height,
            dist_from_screen: compute_dist_from_screen(scr_height),
        };
        GameConfig(Rc::new(igd))
    }

    #[inline]
    pub fn scr_width(&self) -> i32 {
        self.0.scr_width
    }

    #[inline]
    pub fn scr_height(&self) -> i32 {
        self.0.scr_height
    }

    #[inline]
    pub fn wad(&self) -> &WadData {
        &self.0.wad_data
    }

    #[inline]
    pub fn palette(&self) -> &Palette {
        self.0.wad_data.palette()
    }

    #[inline]
    pub fn graphics(&self) -> &Graphics {
        self.0.wad_data.graphics()
    }

    #[inline]
    pub fn font(&self) -> &Font {
        self.0.wad_data.font()
    }

    #[inline]
    pub fn half_fov(&self) -> Angle {
        self.screen_x_to_angle(self.0.scr_width as i32)
    }

    #[inline]
    pub fn aspect_ratio(&self) -> f64 {
        let wf = self.0.scr_width as f64;
        let hf = self.0.scr_height as f64;
        wf / hf
    }

    // TODO: is this used as is? do I need to make it abs()?
    // or should I make angle positive towards left and negative towards right?
    #[inline]
    pub fn screen_x_to_angle(&self, x: i32) -> Angle {
        let dx = (x - (self.0.scr_width as i32 / 2)).abs() as f64;
        let rad = (dx / self.0.dist_from_screen).atan();
        Angle::from_radians(rad)
    }
}

impl Clone for GameConfig {
    fn clone(&self) -> Self {
        let rc_clone = Rc::clone(&self.0);
        Self(rc_clone)
    }
}

//-------------------

struct InternalGameData {
    wad_data: WadData,
    scr_width: i32,
    scr_height: i32,
    dist_from_screen: f64,
}

/// Compute distance from screen, assuming a 4/3 aspect ratio and a 90 degrees FOV,
// based on screen height (as if width would be 4/3 of height)
#[inline]
fn compute_dist_from_screen(height: i32) -> f64 {
    let dist_from_screen = (height as f64) * 2.0 / 3.0;
    assert!(dist_from_screen > 1.0);
    dist_from_screen
}
