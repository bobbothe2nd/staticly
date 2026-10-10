use core::hint::black_box;
use criterion::{
    criterion_group, criterion_main, Criterion,
};
use staticly::{hash, static_map};

fn bench_switch(c: &mut Criterion) {
    for i in 0..64 {
        let key = format!("KEY_{i:02}");
        assert_eq!(direct_match(&key), hashed_match(&key));
    }

    let mut group = c.benchmark_group("switch_8_keys");

    group.bench_function("direct/hit_first", |b| {
        b.iter(|| direct_match(black_box("KEY_00")))
    });

    group.bench_function("hash/hit_first", |b| {
        b.iter(|| hashed_match(black_box("KEY_00")))
    });

    group.bench_function("precompute/hit_first", |b| {
        b.iter(|| precomputed_hashed_match(black_box(hash!("KEY_00"))))
    });

    group.bench_function("direct/hit_middle", |b| {
        b.iter(|| direct_match(black_box("KEY_31")))
    });

    group.bench_function("hash/hit_middle", |b| {
        b.iter(|| hashed_match(black_box("KEY_31")))
    });

    group.bench_function("precompute/hit_middle", |b| {
        b.iter(|| precomputed_hashed_match(black_box(hash!("KEY_31"))))
    });

    group.bench_function("direct/hit_last", |b| {
        b.iter(|| direct_match(black_box("KEY_63")))
    });

    group.bench_function("hash/hit_last", |b| {
        b.iter(|| hashed_match(black_box("KEY_63")))
    });

    group.bench_function("precompute/hit_last", |b| {
        b.iter(|| precomputed_hashed_match(black_box(hash!("KEY_63"))))
    });

    group.bench_function("direct/miss", |b| {
        b.iter(|| direct_match(black_box("UNKNOWN")))
    });

    group.bench_function("hash/miss", |b| {
        b.iter(|| hashed_match(black_box("UNKNOWN")))
    });

    group.bench_function("precompute/miss", |b| {
        b.iter(|| precomputed_hashed_match(black_box(hash!("UNKNOWN"))))
    });

    group.finish();
}

criterion_group!(benches, bench_switch);
criterion_main!(benches);

fn direct_match(s: &str) -> u8 {
    match s {
        "KEY_00" => 0,
        "KEY_01" => 1,
        "KEY_02" => 2,
        "KEY_03" => 3,
        "KEY_04" => 4,
        "KEY_05" => 5,
        "KEY_06" => 6,
        "KEY_07" => 7,
        "KEY_08" => 8,
        "KEY_09" => 9,
        "KEY_10" => 10,
        "KEY_11" => 11,
        "KEY_12" => 12,
        "KEY_13" => 13,
        "KEY_14" => 14,
        "KEY_15" => 15,
        "KEY_16" => 16,
        "KEY_17" => 17,
        "KEY_18" => 18,
        "KEY_19" => 19,
        "KEY_20" => 20,
        "KEY_21" => 21,
        "KEY_22" => 22,
        "KEY_23" => 23,
        "KEY_24" => 24,
        "KEY_25" => 25,
        "KEY_26" => 26,
        "KEY_27" => 27,
        "KEY_28" => 28,
        "KEY_29" => 29,
        "KEY_30" => 30,
        "KEY_31" => 31,
        "KEY_32" => 32,
        "KEY_33" => 33,
        "KEY_34" => 34,
        "KEY_35" => 35,
        "KEY_36" => 36,
        "KEY_37" => 37,
        "KEY_38" => 38,
        "KEY_39" => 39,
        "KEY_40" => 40,
        "KEY_41" => 41,
        "KEY_42" => 42,
        "KEY_43" => 43,
        "KEY_44" => 44,
        "KEY_45" => 45,
        "KEY_46" => 46,
        "KEY_47" => 47,
        "KEY_48" => 48,
        "KEY_49" => 49,
        "KEY_50" => 50,
        "KEY_51" => 51,
        "KEY_52" => 52,
        "KEY_53" => 53,
        "KEY_54" => 54,
        "KEY_55" => 55,
        "KEY_56" => 56,
        "KEY_57" => 57,
        "KEY_58" => 58,
        "KEY_59" => 59,
        "KEY_60" => 60,
        "KEY_61" => 61,
        "KEY_62" => 62,
        "KEY_63" => 63,
        _ => 255,
    }
}

static_map! {
    KEYS: u8 = {
        "KEY_00" => 0,
        "KEY_01" => 1,
        "KEY_02" => 2,
        "KEY_03" => 3,
        "KEY_04" => 4,
        "KEY_05" => 5,
        "KEY_06" => 6,
        "KEY_07" => 7,
        "KEY_08" => 8,
        "KEY_09" => 9,
        "KEY_10" => 10,
        "KEY_11" => 11,
        "KEY_12" => 12,
        "KEY_13" => 13,
        "KEY_14" => 14,
        "KEY_15" => 15,
        "KEY_16" => 16,
        "KEY_17" => 17,
        "KEY_18" => 18,
        "KEY_19" => 19,
        "KEY_20" => 20,
        "KEY_21" => 21,
        "KEY_22" => 22,
        "KEY_23" => 23,
        "KEY_24" => 24,
        "KEY_25" => 25,
        "KEY_26" => 26,
        "KEY_27" => 27,
        "KEY_28" => 28,
        "KEY_29" => 29,
        "KEY_30" => 30,
        "KEY_31" => 31,
        "KEY_32" => 32,
        "KEY_33" => 33,
        "KEY_34" => 34,
        "KEY_35" => 35,
        "KEY_36" => 36,
        "KEY_37" => 37,
        "KEY_38" => 38,
        "KEY_39" => 39,
        "KEY_40" => 40,
        "KEY_41" => 41,
        "KEY_42" => 42,
        "KEY_43" => 43,
        "KEY_44" => 44,
        "KEY_45" => 45,
        "KEY_46" => 46,
        "KEY_47" => 47,
        "KEY_48" => 48,
        "KEY_49" => 49,
        "KEY_50" => 50,
        "KEY_51" => 51,
        "KEY_52" => 52,
        "KEY_53" => 53,
        "KEY_54" => 54,
        "KEY_55" => 55,
        "KEY_56" => 56,
        "KEY_57" => 57,
        "KEY_58" => 58,
        "KEY_59" => 59,
        "KEY_60" => 60,
        "KEY_61" => 61,
        "KEY_62" => 62,
        "KEY_63" => 63,
    };
}

fn hashed_match(s: &str) -> u8 {
    KEYS.get(s).unwrap_or(255)
}

fn precomputed_hashed_match(hash: u64) -> u8 {
    KEYS.get_hash(hash).unwrap_or(255)
}
