pub fn fnv1a(bytes: &[u8], seed: Option<u64>) -> u64 {
    const OFFSET_BASIS: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;

    let mut hash = seed.unwrap_or(OFFSET_BASIS);

    for &byte in bytes {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }

    hash
}
