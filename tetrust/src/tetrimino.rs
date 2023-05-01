//! Tetrimino data and functionalities.

// constant with the data for generating all 7 tetriminoes
const TETRIMINOES: &'static [Tetrimino] = &[
    internal_build_tetrimino('O', 0b_0000_0110_0110),
    internal_build_tetrimino('I', 0b_0000_1111_0000),
    internal_build_tetrimino('T', 0b_0100_1110_0000),
    internal_build_tetrimino('S', 0b_0110_1100_0000),
    internal_build_tetrimino('Z', 0b_1100_0110_0000),
    internal_build_tetrimino('J', 0b_1000_1110_0000),
    internal_build_tetrimino('L', 0b_0010_1110_0000),
];

/// Data structure for a tetrimino.
#[derive(Clone, Debug)]
pub struct Tetrimino {
    x: [i8; 16],
    y: [i8; 16],
    x_ofs: i16,
    y_ofs: i16,
    rotation: u8,
    color_idx: u8,
    name: u8,
}

impl Tetrimino {
    pub fn from_index(idx: usize) -> Self {
        assert!(idx < TETRIMINOES.len(), "Invalid Tetromino index: {idx}");
        let mut tetr = TETRIMINOES[idx].clone();
        tetr.color_idx = idx as u8;
        tetr
    }

    #[inline]
    pub fn name(&self) -> char {
        self.name as char
    }

    #[inline]
    pub fn color_idx(&self) -> usize {
        self.color_idx as usize
    }

    #[inline]
    pub fn x(&self, idx: usize) -> i32 {
        assert!(idx < 4);
        let cidx = idx + ((self.rotation as usize) << 2);
        (self.x[cidx] as i32) + (self.x_ofs as i32)
    }

    #[inline]
    pub fn y(&self, idx: usize) -> i32 {
        assert!(idx < 4);
        let cidx = idx + ((self.rotation as usize) << 2);
        (self.y[cidx] as i32) + (self.y_ofs as i32)
    }

    #[inline]
    pub fn slide(&mut self, delta_x: i32, delta_y: i32) {
        self.x_ofs += delta_x as i16;
        self.y_ofs += delta_y as i16;
    }

    #[inline]
    pub fn rotate_cw(&mut self) {
        self.rotation = (self.rotation + 1) & 0x03;
    }

    #[inline]
    pub fn rotate_ccw(&mut self) {
        self.rotation = (self.rotation + 3) & 0x03;
    }
}

/// Internal function, for building a tetrimino.
/// It *must* be `const fn`, so that it can be called at compile time
/// (which is why we cannot use `for` loops).
const fn internal_build_tetrimino(name: char, bits: u32) -> Tetrimino {
    // check for pieces which are 3 squares wide
    // (this info is needed for computing the rotations)
    let is_3_width = (name != 'O') && (name != 'I');

    let mut tetr = Tetrimino {
        x: [0; 16],
        y: [0; 16],
        x_ofs: 0,
        y_ofs: (is_3_width as i16) - 1,
        rotation: 0,
        color_idx: 0,
        name: name as u8,
    };

    // compute positions for the initial rotation
    let mut mask: u32 = 0b_1000_0000_0000;
    let mut idx: usize = 0;
    let mut xy: i8 = 0;
    // `for` cannot be used in const fn
    // - see: https://github.com/rust-lang/rust/issues/87575
    while mask > 0 {
        if (bits & mask) != 0 {
            tetr.x[idx] = xy & 0x03;
            tetr.y[idx] = (xy >> 2) & 0x03;
            idx += 1;
        }
        mask >>= 1;
        xy += 1;
    }

    // compute all rotations
    let rotation_transformer = 3 - (is_3_width as i8);
    idx = 4;
    while idx < 16 {
        tetr.x[idx] = rotation_transformer - tetr.y[idx - 4];
        tetr.y[idx] = tetr.x[idx - 4];
        idx += 1;
    }

    tetr
}
