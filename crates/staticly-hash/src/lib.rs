#![no_std]

mod gouda;

pub use gouda::gouda;

mod fnv1a;

pub use fnv1a::fnv1a;

mod djb2;

pub use djb2::djb2;

mod fxhash;

pub use fxhash::fxhash;

pub mod internal {
    pub use super::gouda::gouda_in;
}
