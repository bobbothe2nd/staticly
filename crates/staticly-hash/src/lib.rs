#![no_std]

mod fnv1a;

pub use fnv1a::fnv1a;

mod djb2;

pub use djb2::djb2;

mod fxhash;

pub use fxhash::fxhash;
