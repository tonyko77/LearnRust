//! Data structures for the assets loaded from files

// TODO how to interpret the map data?
//  -> looks like each map has 64*64 = 4096 words, between 0x00 and 0xFF
//      => it's JUST a 2D array :))
//  -> plane #1 seems to contain WALLS, plane #2 seems to contain THINGS
//  -> plane #3 seems to ALWAYS have 0-s => NOT USED ?!?, check SOD, WL6 etc

//-----------------------

/// Graphics types
// TODO is this really needed ??
pub enum GfxType {
    WALL,
    SPRITE,
    PIC,
}

/// Graphics - contains walls, sprites and miscellaneous (fonts, PICs etc)
/// Each pic is stored as columns, then rows (flipped)
pub struct GfxData {
    pub gtype: GfxType,
    pub width: u16,
    pub height: u16,
    pub pixels: Vec<u8>,
}

impl GfxData {
    pub fn new_wall(pixels: Vec<u8>) -> Self {
        let wh = if pixels.is_empty() { 0 } else { 64 };
        Self::new(GfxType::WALL, wh, wh, pixels)
    }

    pub fn new_sprite(pixels: Vec<u8>) -> Self {
        let wh = if pixels.is_empty() { 0 } else { 64 };
        Self::new(GfxType::SPRITE, wh, wh, pixels)
    }

    pub fn new_pic(width: u16, height: u16, pixels: Vec<u8>) -> Self {
        Self::new(GfxType::PIC, width, height, pixels)
    }

    fn new(gtype: GfxType, width: u16, height: u16, pixels: Vec<u8>) -> Self {
        assert_eq!((width * height) as usize, pixels.len());
        Self {
            gtype,
            width,
            height,
            pixels,
        }
    }
}

//-----------------------

pub struct FontData {
    pub font_height: u16,
    pub space_width: u16,
    pub char_pixels: Vec<Vec<u8>>,
}

impl FontData {
    pub fn new(font_height: u16, space_width: u16) -> Self {
        Self {
            font_height,
            space_width,
            char_pixels: Vec::with_capacity(100),
        }
    }

    #[inline]
    pub fn add_char_data(&mut self, pixels: Vec<u8>) {
        self.char_pixels.push(pixels)
    }

    pub fn char_width(&self, ch: u8) -> u16 {
        if ch == 32 {
            return self.space_width;
        }
        if ch >= 33 {
            let idx = (ch - 33) as usize;
            if idx < self.char_pixels.len() {
                return (self.char_pixels[idx].len() as u16) / self.font_height;
            }
        }
        // not a valid char => no width
        0
    }
}

//-----------------------

/// Map data - contains walls/doors and things.
pub struct MapData {
    name: String,
    width: u16,
    height: u16,
    walls: Vec<u16>,
    things: Vec<u16>,
}

impl MapData {
    pub fn new(name: String, width: u16, height: u16, walls: Vec<u16>, things: Vec<u16>) -> Self {
        // some silly checks - seem to be valid for WL1, WL6 and SOD
        assert!(name.len() > 0);
        // check that maps are always 64 x 64
        assert_eq!(64, width);
        assert_eq!(64, height);
        // check that planes have exactly 64*64 = 4096 words
        assert_eq!(4096, walls.len());
        assert_eq!(4096, things.len());
        // check that all wall IDs are <= 0xFF
        let wallsok = walls.iter().all(|w| *w <= 0xFF);
        assert!(wallsok);
        // check that all thing IDs are <= 0x1FF
        let thingsok = things.iter().all(|t| *t <= 0x1FF);
        assert!(thingsok);

        Self {
            name,
            width,
            height,
            walls,
            things,
        }
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn width(&self) -> i32 {
        self.width as i32
    }

    #[inline]
    pub fn height(&self) -> i32 {
        self.height as i32
    }

    #[inline]
    pub fn wall(&self, x: i32, y: i32) -> u16 {
        self.safe_item_from_array(x, y, &self.walls)
    }

    #[inline]
    pub fn thing(&self, x: i32, y: i32) -> u16 {
        self.safe_item_from_array(x, y, &self.things)
    }

    fn safe_item_from_array(&self, x: i32, y: i32, vect: &Vec<u16>) -> u16 {
        let w = self.width as i32;
        let h = self.height as i32;
        if x >= 0 && y >= 0 && x < w && y < h {
            let idx = (y * w + x) as usize;
            vect[idx]
        } else {
            0
        }
    }
}
