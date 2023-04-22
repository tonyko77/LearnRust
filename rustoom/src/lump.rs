// Lump data holder

pub struct LumpData<'a> {
    name: String,
    raw_bytes: &'a [u8],
    // TODO to be continued ...
}

impl<'a> LumpData<'a> {
    #[inline]
    pub fn new(name: &str, raw_bytes: &'a [u8]) -> LumpData<'a> {
        LumpData { name: String::from(name), raw_bytes }
    }

    // TODO to be continued ...
}
