# Staticly

Instead of writing huge match statements or iterators over strings for parsing, you can *staticly* switch themm using macros and constant hashes.

## Macros

`static_set!`

Declares a `StaticSet` for efficiently checking if it contains a string.

`static_map!`

Declares a `StaticMap` for getting an unrelated value based on the string key.

`static_enum!`

Declares or implements methods on an enum like a lightweight, hash-accelerated `#[derive(strum::EnumString)]`.

`switch!`

Matches over a runtime value in near-constant time via hashing like `static_map!` with expressions instead of values.

`hash!`

Declares a compile-time hash from a string literal.

`unique!`

Errors if any provided literals are equal. Used internally by other macros.

## Performance

A normal Rust `match` statement has performance that scales with the amount of arms there are. Staticly performance scales with the amount of hash collisions on the string and the length of the input.

For few keys, this can have significantly worse performance. As possible keys and the length of keys scale, Staticly can offer huge increases in performance.

In one benchmark where all 64 keys are the same length:

```
switch_8_keys/direct/hit_first
    time:   [2.7926 ns 2.9141 ns 3.0452 ns]
switch_8_keys/hash/hit_first
    time:   [12.053 ns 12.238 ns 12.412 ns]
switch_8_keys/precompute/hit_first
    time:   [2.7717 ns 3.1705 ns 3.6354 ns]
switch_8_keys/direct/hit_middle
    time:   [15.155 ns 16.584 ns 18.301 ns]
switch_8_keys/hash/hit_middle
    time:   [5.4854 ns 6.0965 ns 6.8647 ns]
switch_8_keys/precompute/hit_middle
    time:   [1.4419 ns 1.4677 ns 1.5049 ns]
switch_8_keys/direct/hit_last
    time:   [21.927 ns 23.777 ns 26.200 ns]
switch_8_keys/hash/hit_last
    time:   [4.7602 ns 4.8431 ns 5.0102 ns]
switch_8_keys/precompute/hit_last
    time:   [1.2737 ns 1.3033 ns 1.3583 ns]
switch_8_keys/direct/miss
    time:   [738.19 ps 753.32 ps 782.03 ps]
switch_8_keys/hash/miss
    time:   [4.1286 ns 4.1839 ns 4.2899 ns]
switch_8_keys/precompute/miss
    time:   [1.4889 ns 1.6291 ns 1.8030 ns]
```

## Usage

```rust
use staticly::{hash, static_enum, static_map, static_set, switch, algo::fnv1a};

static_map! {
    HTTP_STATUS: u64 = {
        "OK" => 200,
        "Created" => 201,
        "No Content" => 204,
        "Bad Request" => 400,
        "Not Found" => 404,
        "Internal Server Error" => 500,
    };
}

static_set! {
    HTTP_METHODS = [
        "GET",
        "POST",
    ];
}

static_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum Color {
        Red = "red",
        Green = "green",
        Blue = "blue",
    }
}

fn main() {
    const GET_HASH: u64 = hash!("get");
    const GREEN_HASH: u64 = Color::Green.hash();
    const BLUE_STR: &str = Color::Blue.as_str();

    assert_eq!(GET_HASH, fnv1a(b"get"));

    assert!(!HTTP_STATUS.is_empty());
    assert_eq!(HTTP_STATUS.len(), 6);
    assert!(HTTP_STATUS.contains_key("OK"));

    assert_eq!(HTTP_STATUS.get("OK"), Some(200));
    assert_eq!(HTTP_STATUS.get_hash(hash!("Not Found")), Some(404));

    println!("status:");

    for (name, status) in HTTP_STATUS.iter() {
        println!("  {name} = {status}");
    }

    assert!(!HTTP_METHODS.is_empty());
    assert_eq!(HTTP_METHODS.len(), 2);
    assert!(HTTP_METHODS.contains("GET"));

    println!("\nmethods:");

    for name in HTTP_METHODS.iter() {
        println!("  {name}");
    }

    let val = switch!("123", {
        "123" => 123,
        _ => 0,
    });
    assert_eq!(val, 123);

    assert_eq!("red".parse(), Ok(Color::Red));
    assert_eq!(BLUE_STR, "blue");
    assert_eq!(GREEN_HASH, hash!("green"));
}
```
