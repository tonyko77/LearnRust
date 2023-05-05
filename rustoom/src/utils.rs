//  Various utilities

#[inline]
pub fn buf_to_u16(buf: &[u8]) -> u16 {
    assert!(buf.len() >= 2);

    (buf[0] as u16) | ((buf[1] as u16) << 8)
}

#[inline]
pub fn buf_to_i16(buf: &[u8]) -> i16 {
    buf_to_u16(buf) as i16
}

#[inline]
pub fn buf_to_u32(buf: &[u8]) -> u32 {
    assert!(buf.len() >= 4);

    (buf[0] as u32) | ((buf[1] as u32) << 8) | ((buf[2] as u32) << 16) | ((buf[3] as u32) << 24)
}

pub fn buf_to_lump_name<'a>(buf: &'a [u8]) -> Result<&'a str, String> {
    const ERR: &str = "Invalid lump name";
    assert!(buf.len() >= 8);
    // dismiss all null bytes at the end, ...
    let mut name_end = 7;
    while (name_end > 0) && (0 == buf[name_end]) {
        name_end -= 1;
    }
    // take the remaining bytes as the name, ...
    let name_bytes = &buf[0..=name_end];
    // and also check if the last byte is null
    if buf[name_end] == 0 {
        Err(ERR.to_string())
    } else {
        std::str::from_utf8(name_bytes).map_err(|_| ERR.to_string())
    }
}

/// Convert a lump name into a 64 bit integer, for easier use as key in a hashmap.
/// Since lumps should only use digits, upper case letters and a few simbols
/// => they fall into the range 32-95 (0x20-0x5F)
/// => it is safe to pick only the lower 6 bits of each ASCII character (byte).
pub fn hash_lump_name(name: &str) -> u64 {
    let mut k = 0_u64;
    for b in name.bytes() {
        k = (k << 6) + ((b & 0x3F) as u64);
    }
    k
}
