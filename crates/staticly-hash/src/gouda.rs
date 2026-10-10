/// Custom hash optimized for parsers and performance
///
/// Exact behavior is unstable and may change with new breaking version releases
#[inline(always)]
pub const fn gouda(bytes: &[u8]) -> u64 {
    const CHEESE_P0: u64 = 0xCEC7DF260FB7CBBE;
    const CHEESE_P1: u64 = 0xCF4084650DE68D3F;
    const CHEESE_P2: u64 = 0x93544412AABC9E1A;
    const CHEESE_ROT: u32 = 28;

    gouda_in(
        bytes,
        CHEESE_P0,
        CHEESE_P1,
        CHEESE_P2,
        CHEESE_ROT,
    )
}

#[inline(always)]
pub const fn gouda_in(
    bytes: &[u8],
    p0: u64,
    p1: u64,
    p2: u64,
    rot: u32,
) -> u64 {
    let mut p = bytes.as_ptr();
    let mut len = bytes.len();

    let mut h = (len as u64).wrapping_mul(p0);

    while len >= 4 {
        let x = unsafe { p.cast::<u32>().read_unaligned() }.to_le() as u64;

        h ^= x;
        h = h.rotate_left(rot);

        p = unsafe { p.add(4) };
        len -= 4;
    }

    if len >= 2 {
        h ^= unsafe { p.cast::<u16>().read_unaligned() }.to_le() as u64;

        p = unsafe { p.add(2) };
        len -= 2;
    }

    if len != 0 {
        h ^= unsafe { p.read() } as u64;
    }

    h ^= h >> 29;
    h = h.wrapping_mul(p1);
    h ^= h >> 32;
    h ^= h.wrapping_sub(p2) << 17;

    h
}
