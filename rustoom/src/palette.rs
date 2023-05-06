//! Palette and color handling/mapping

use crate::ColorMapper;
use crate::RGB;

pub struct Palette {
    colormaps: Vec<u8>,
    palletes: Vec<u8>,
    cmap_cnt: usize,
    cmap_selection: usize,
    pal_cnt: usize,
    pal_selection: usize,
}

impl Palette {
    pub fn new() -> Self {
        Palette {
            colormaps: vec![],
            palletes: vec![],
            cmap_cnt: 0,
            cmap_selection: 0,
            pal_cnt: 0,
            pal_selection: 0,
        }
    }

    pub fn init_palettes(&mut self, bytes: &[u8]) {
        self.palletes = bytes.to_vec();
        self.pal_selection = 0;
        self.pal_cnt = bytes.len() / 768;
    }

    pub fn init_colormaps(&mut self, bytes: &[u8]) {
        self.colormaps = bytes.to_vec();
        self.cmap_selection = 0;
        self.cmap_cnt = bytes.len() / 256;
    }

    #[inline]
    pub fn is_palette_initialized(&self) -> bool {
        self.pal_cnt > 0
    }

    #[inline]
    pub fn is_colormap_initialized(&self) -> bool {
        self.cmap_cnt > 0
    }

    pub fn select_palette(&mut self, pal: usize) -> Result<(), String> {
        if pal >= self.pal_cnt {
            Err(format!("Invalid palette index: {pal} >= {}", self.pal_cnt))
        } else {
            self.pal_selection = pal * 768;
            Ok(())
        }
    }

    pub fn select_colormap(&mut self, cmap: usize) -> Result<(), String> {
        if cmap >= self.cmap_cnt {
            Err(format!(
                "Invalid colormap index: {cmap} >= {}",
                self.cmap_cnt
            ))
        } else {
            self.cmap_selection = cmap * 256;
            Ok(())
        }
    }
}

impl ColorMapper for Palette {
    fn byte2rgb(&self, color: u8) -> RGB {
        if self.cmap_cnt == 0 || self.pal_cnt == 0 {
            // data is NOT SET !!
            // => just grayscale it :/
            RGB::from(color, color, color)
        } else {
            // get palette index from color map ...
            let cmap_idx = self.cmap_selection + (color as usize);
            let pal_entry = 3 * (self.colormaps[cmap_idx] as usize);
            // and find out the palette location of r, g, b
            let pal_idx = self.pal_selection + pal_entry;
            let r = self.palletes[pal_idx];
            let g = self.palletes[pal_idx + 1];
            let b = self.palletes[pal_idx + 2];
            RGB::from(r, g, b)
        }
    }
}
