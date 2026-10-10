pub const fn djb2(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 5381;
    let mut i = 0;

    while i < bytes.len() {
        hash = hash.wrapping_mul(33).wrapping_add(bytes[i] as u64);
        i += 1;
    }

    hash
}
