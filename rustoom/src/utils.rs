//  Various utilities


#[inline]
pub fn buf_to_u16(buf: &[u8]) -> u16 {
    assert!(buf.len() >= 2);

    (buf[0] as u16) |
    ((buf[1] as u16) <<  8)
}


#[inline]
pub fn buf_to_i16(buf: &[u8]) -> i16 {
    buf_to_u16(buf) as i16
}


#[inline]
pub fn buf_to_u32(buf: &[u8]) -> u32 {
    assert!(buf.len() >= 4);

    (buf[0] as u32) |
    ((buf[1] as u32) <<  8) |
    ((buf[2] as u32) << 16) |
    ((buf[3] as u32) << 24)
}
