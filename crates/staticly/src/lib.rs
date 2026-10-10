<<<<<<< HEAD
//! Macros for hashing strings at compile time

#![forbid(unsafe_code, missing_docs)]
=======
#![forbid(unsafe_code)]
>>>>>>> ba5e60cfceba11284255adf2578415dda091a2e9
#![no_std]

pub mod map;

pub use staticly_hash as algo;

pub use staticly_macros::*;

/// Declares a constant static map
///
/// ```rust
/// staticly::static_map! {
///     HTTP_STATUS: u64 = {
///         "OK" => 200,
///         "Created" => 201,
///         "No Content" => 204,
///         "Bad Request" => 400,
///         "Not Found" => 404,
///         "Internal Server Error" => 500,
///     };
/// }
///
/// assert!(!HTTP_STATUS.is_empty());
/// assert_eq!(HTTP_STATUS.len(), 6);
/// assert!(HTTP_STATUS.contains_key("OK"));
///
/// assert_eq!(HTTP_STATUS.get("OK"), Some(200));
/// assert_eq!(HTTP_STATUS.get_hash(staticly::algo::gouda(b"Not Found")), Some(404));
/// ```
#[macro_export]
macro_rules! static_map {
    ($(
        $vis:vis $name:ident: $ty:ty = {$(
            $key:literal => $val:expr
        ),*$(,)?};
    )*) => {
        $(
            $crate::unique! {$(
                $key
            ),*}

            $vis static $name: $crate::map::StaticMap<$ty, { 0$(+ {
                let _ = $val;
                1
            })* }> = {
                const LEN: usize = 0$(+ {
                    let _ = $val;
                    1
                })*;

                fn contains_key(key: &str) -> bool {
                    match $crate::algo::gouda(key.as_bytes()) {
                        $(
                            $crate::hash!($key) if key == $key => true,
                        )*
                        _ => false,
                    }
                }

                fn get(key: &str) -> Option<$ty> {
                    match $crate::algo::gouda(key.as_bytes()) {
                        $(
                            $crate::hash!($key) if key == $key => Some($val),
                        )*
                        _ => None,
                    }
                }

                fn get_hash(hash: u64) -> Option<$ty> {
                    match hash {
                        $(
                            $crate::hash!($key) => Some($val),
                        )*
                        _ => None,
                    }
                }

                fn iter(index: usize) -> Option<(&'static str, $ty)> {
                    if index >= LEN {
                        return None;
                    }

                    Some([$(
                        ($key, $val),
                    )*][index])
                }

                $crate::map::StaticMap::new(get, get_hash, iter, contains_key)
            };
        )*
    };
}

/// Declares a constant static set
///
/// ```rust
/// staticly::static_set! {
///     HTTP_METHODS = [
///         "GET",
///         "POST",
///     ];
/// }
///
/// assert!(!HTTP_METHODS.is_empty());
/// assert_eq!(HTTP_METHODS.len(), 2);
/// assert!(HTTP_METHODS.contains("GET"));
/// ```
#[macro_export]
macro_rules! static_set {
    ($(
        $vis:vis $name:ident = [$(
            $key:literal
        ),*$(,)?];
    )*) => {
        $(
            $crate::unique! {$(
                $key
            ),*}

            $vis static $name: $crate::map::StaticSet<{ 0$(+ {
                let _ = $key;
                1
            })* }> = {
                const LEN: usize = 0$(+ {
                    let _ = $key;
                    1
                })*;

                fn contains(key: &str) -> bool {
                    match $crate::algo::gouda(key.as_bytes()) {
                        $(
                            $crate::hash!($key) if key == $key => true,
                        )*
                        _ => false,
                    }
                }

                fn iter(index: usize) -> Option<&'static str> {
                    if index >= LEN {
                        return None;
                    }

                    Some([$(
                        $key,
                    )*][index])
                }

                $crate::map::StaticSet::new(iter, contains)
            };
        )*
    };
}

/// Single match statement accelerated with hashing
///
/// ```rust
/// let val = staticly::switch!("123", {
///     "123" => 123,
///     "456" => 456,
///     "789" => 789,
///     _ => 0,
/// });
///
/// assert_eq!(val, 123);
///```
#[macro_export]
macro_rules! switch {
    ($var:expr, {
        $(
            $key:literal => $eval:expr,
        )*
        _ => $default:expr$(,)?
    }) => {{
        $crate::unique! {$(
            $key
        ),*}

        match $crate::algo::gouda($var.as_bytes()) {
            $(
                $crate::hash!($key) if $var == $key => $eval,
            )*
            _ => $default,
        }
    }};

    (
        $var:expr,
        $(
            $key:literal => $eval:expr,
        )*
        _ => $default:expr$(,)?
    ) => {
        switch!($var, {
            $key => $eval,
            _ => #default
        })
    };
}

/// Declares an enum and/or implements methods/constructors for it
///
/// ```rust
/// staticly::static_enum! {
///     #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
///     enum Color {
///         Red = "red",
///         Green = "green",
///         Blue = "blue",
///     }
/// }
///
/// const GREEN_HASH: u64 = Color::Green.hash();
/// const BLUE_STR: &str = Color::Blue.as_str();
///
/// assert_eq!("red".parse(), Ok(Color::Red));
/// assert_eq!(BLUE_STR, "blue");
/// assert_eq!(GREEN_HASH, staticly::hash!("green"));
/// ```
///
/// or
///
/// ```rust
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// enum Color {
///     Red,
///     Blue,
///     Green,
/// }
///
/// staticly::static_enum! {
///     impl Color {
///         Red = "red",
///         Green = "green",
///         Blue = "blue",
///     }
/// }
///
/// const GREEN_HASH: u64 = Color::Green.hash();
/// const BLUE_STR: &str = Color::Blue.as_str();
///
/// assert_eq!("red".parse(), Ok(Color::Red));
/// assert_eq!(BLUE_STR, "blue");
/// assert_eq!(GREEN_HASH, staticly::hash!("green"));
/// ```
#[macro_export]
macro_rules! static_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $(
                $variant:ident = $key:literal
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis enum $name {
            $($variant),*
        }

        $crate::static_enum! {
            impl $name {
                $($variant = $key),*
            }
        }
    };

    (
        impl $ty:ty {
            $(
                $variant:ident = $key:literal
            ),* $(,)?
        }
    ) => {
        $crate::unique! {$(
            $key
        ),*}

        impl ::core::str::FromStr for $ty {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match $crate::algo::gouda(s.as_bytes()) {
                    $(
                        $crate::hash!($key) if s == $key => Ok(Self::$variant),
                    )*
                    _ => Err(()),
                }
            }
        }

        impl From<$ty> for &'static str {
            fn from(val: $ty) -> Self {
                val.as_str()
            }
        }

        impl $ty {
            pub const fn from_hash(hash: u64) -> Option<Self> {
                match hash {
                    $(
                        $crate::hash!($key) => Some(Self::$variant),
                    )*
                    _ => None,
                }
            }

            pub const fn as_str(self) -> &'static str {
                match self {
                    $(
                        Self::$variant => $key,
                    )*
                }
            }

            pub const fn hash(self) -> u64 {
                match self {
                    $(
                        Self::$variant => $crate::hash!($key),
                    )*
                }
            }
        }
    };
}

#[allow(dead_code)]
#[doc = include_str!("../README.md")]
fn test() {}
