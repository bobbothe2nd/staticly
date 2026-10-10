pub const fn fxhash(bytes: &[u8]) -> u64 {
    const K: u64 = 0x517cc1b727220a95;

    let mut hash: u64 = 0;
    let mut i = 0;

    while i + 8 <= bytes.len() {
        let chunk = u64::from_le_bytes([
            bytes[i],
            bytes[i + 1],
            bytes[i + 2],
            bytes[i + 3],
            bytes[i + 4],
            bytes[i + 5],
            bytes[i + 6],
            bytes[i + 7],
        ]);

        hash = hash.rotate_left(5) ^ chunk;
        hash = hash.wrapping_mul(K);
        i += 8;
    }

    while i < bytes.len() {
        hash = hash.rotate_left(5) ^ bytes[i] as u64;
        hash = hash.wrapping_mul(K);
        i += 1;
    }

    hash
}
