//! Data structures for the assets loaded from files

// TODO how to interpret the map data?
//  -> looks like each map has 64*64 = 4096 words, between 0x00 and 0xFF
//      => it's JUST a 2D array :))
//  -> plane #1 seems to contain WALLS, plane #2 seems to contain THINGS
//  -> plane #3 seems to ALWAYS have 0-s => NOT USED ?!?, check SOD, WL6 etc

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
