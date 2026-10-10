use std::{collections::HashMap, fmt::Debug};

use staticly_hash::{djb2, fnv1a, fxhash, internal::gouda_in};

const TABLE_BITS: u32 = 7;

#[derive(Clone, Copy)]
struct Score {
    collisions: usize,
    max_bucket: usize,
    used_buckets: usize,
    spread: usize,
    full_hash_collisions: usize,
}

impl Debug for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "collisions={} max_bucket={} used={} spread={} full_collisions={}",
            self.collisions,
            self.max_bucket,
            self.used_buckets,
            self.spread,
            self.full_hash_collisions
        )
    }
}

fn score(hash: impl Fn(&[u8]) -> u64) -> Score {
    let bucket_count = 1usize << TABLE_BITS;
    let mask = bucket_count - 1;

    let mut buckets = vec![0usize; bucket_count];
    let mut hashes = HashMap::<u64, usize>::new();

    for keyword in KEYWORDS {
        let h = hash(keyword.as_bytes());

        buckets[(h as usize) & mask] += 1;
        *hashes.entry(h).or_default() += 1;
    }

    let used_buckets = buckets.iter().filter(|&&n| n != 0).count();

    let collisions = buckets.iter().map(|&n| n.saturating_sub(1)).sum();

    let max_bucket = buckets.iter().copied().max().unwrap_or(0);

    let spread = buckets
        .iter()
        .map(|&n| n.saturating_sub(1) * n.saturating_sub(1))
        .sum();

    let full_hash_collisions = hashes.values().map(|&n| n.saturating_sub(1)).sum();

    Score {
        collisions,
        max_bucket,
        used_buckets,
        spread,
        full_hash_collisions,
    }
}

#[inline]
fn random_u64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E3779B97F4A7C15);

    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

fn better(a: Score, b: Score) -> bool {
    a.collisions < b.collisions
        || (a.collisions == b.collisions && a.max_bucket < b.max_bucket)
        || (a.collisions == b.collisions && a.max_bucket == b.max_bucket && a.spread < b.spread)
        || (a.collisions == b.collisions
            && a.max_bucket == b.max_bucket
            && a.spread == b.spread
            && a.full_hash_collisions < b.full_hash_collisions)
}

fn main() {
    const ITERATIONS: usize = 5_000_000;

    let mut rng = 0x1234_5678_9ABC_DEF0;

    let mut best = Score {
        collisions: usize::MAX,
        max_bucket: usize::MAX,
        used_buckets: 0,
        spread: usize::MAX,
        full_hash_collisions: usize::MAX,
    };

    let mut best_constants = (0, 0, 0, 0);

    for _ in 0..ITERATIONS {
        let p0 = random_u64(&mut rng);
        let p1 = random_u64(&mut rng);
        let p2 = random_u64(&mut rng);
        let rot = random_u64(&mut rng) & 0x1f;

        let s = score(|bytes| gouda_in(bytes, p0, p1, p2, rot as u32));

        if better(s, best) {
            best = s;
            best_constants = (p0, p1, p2, rot);

            println!("P0 = 0x{p0:016X}, P1 = 0x{p1:016X}, P2 = 0x{p2:016X}, ROT = {rot} | {s:?}");

            if s.collisions == 0 && s.full_hash_collisions == 0 {
                break;
            }
        }
    }

    let (p0, p1, p2, rot) = best_constants;

    println!();
    println!("BEST:");
    println!("P0 = 0x{p0:016X}");
    println!("P1 = 0x{p1:016X}");
    println!("P2 = 0x{p2:016X}");
    println!("ROT = {rot}");
    println!("{best:?}");

    println!();
    println!("const CHEESE_P0: u64 = 0x{p0:016X};");
    println!("const CHEESE_P1: u64 = 0x{p1:016X};");
    println!("const CHEESE_P2: u64 = 0x{p2:016X};");
    println!("const CHEESE_ROT: u32 = {rot};");

    println!("FNV-1a: {:?}", score(|bytes| fnv1a(bytes)));
    println!("FxHash: {:?}", score(|bytes| fxhash(bytes)));
    println!("DJB2: {:?}", score(|bytes| djb2(bytes)));

    println!();
    println!("Buckets:");

    let mask = (1usize << TABLE_BITS) - 1;
    let mut buckets: Vec<Vec<&str>> = vec![Vec::new(); mask + 1];

    for keyword in KEYWORDS {
        let h = gouda_in(keyword.as_bytes(), p0, p1, p2, rot as u32);
        buckets[(h as usize) & mask].push(keyword);
    }

    for (i, bucket) in buckets.iter().enumerate() {
        if !bucket.is_empty() {
            println!("{i:3}: {bucket:?}");
        }
    }
}

const KEYWORDS: &[&str] = &[
    "fn", "def", "let", "mut", "pub", "struct", "enum", "impl", "trait", "return", "match", "if",
    "else", "while", "loop", "for", "use", "mod", "const", "static", "type", "where", "async",
    "await", "move", "crate", "self", "add", "addx", "mul", "mulx", "sub", "subx", "div", "divx",
    "super", "in", "then", "var", "shared", "global", "num", "elseif", "elif", "or", "and", "xor",
    "nor", "nand", "xnor", "shl", "shr", "rol", "ror", "rolx", "rorx", "hadd", "hmul", "hsub",
    "hdiv", "f32", "f64", "f16", "f128", "u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32",
    "i64", "i128", "usize", "isize", "bool",
];
