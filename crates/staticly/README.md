# Staticly

Instead of writing huge match statements or iterators over strings for parsing, you can *staticly* switch them using macros and constant hashes.

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

```text
switch_8_keys/direct/hit_first
    time:   [1.0556 ns 1.0740 ns 1.1079 ns]
switch_8_keys/hash/hit_first
    time:   [4.4498 ns 4.5774 ns 4.7857 ns]
switch_8_keys/precompute/hit_first
    time:   [1.2675 ns 1.3272 ns 1.4215 ns]
switch_8_keys/direct/hit_middle
    time:   [11.189 ns 12.104 ns 13.225 ns]
switch_8_keys/hash/hit_middle
    time:   [4.7657 ns 4.8567 ns 4.9867 ns]
switch_8_keys/precompute/hit_middle
    time:   [1.4312 ns 1.4495 ns 1.4767 ns]
switch_8_keys/direct/hit_last
    time:   [21.119 ns 22.894 ns 25.172 ns]
switch_8_keys/hash/hit_last
    time:   [4.7508 ns 4.9732 ns 5.3117 ns]
switch_8_keys/precompute/hit_last
    time:   [1.4373 ns 1.5033 ns 1.6057 ns]
switch_8_keys/direct/miss
    time:   [965.84 ps 1.0253 ns 1.1114 ns]
switch_8_keys/hash/miss
    time:   [3.9438 ns 4.2044 ns 4.5475 ns]
switch_8_keys/precompute/miss
    time:   [1.4327 ns 1.5430 ns 1.6971 ns]
```

## Usage

```rust
use staticly::{hash, static_enum, static_map, static_set, switch, algo::gouda};

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

const GET_HASH: u64 = hash!("get");
const GREEN_HASH: u64 = Color::Green.hash();
const BLUE_STR: &str = Color::Blue.as_str();

assert_eq!(GET_HASH, gouda(b"get"));

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
```

Gouda is a small, non-cryptographic hash optimized for static string dispatch. It's also a really good cheese to put on hashbrowns :).
