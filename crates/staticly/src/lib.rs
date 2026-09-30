#![no_std]

pub mod map;

pub mod algo {
    //! Runtime hashing algorithms

    pub use staticly_hash::*;
}

pub use staticly_macros::*;

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

            $vis static $name: $crate::map::StaticMap<&str, $ty, { 0$(+ {
                let _ = $val;
                1
            })* }> = {
                const LEN: usize = 0$(+ {
                    let _ = $val;
                    1
                })*;

                fn hash(key: &str) -> u64 {
                    $crate::algo::fnv1a(key.as_bytes(), None)
                }

                fn contains_key(key: &&str) -> bool {
                    match hash(*key) {
                        $(
                            $crate::hash!($key) if *key == $key => true,
                        )*
                        _ => false,
                    }
                }

                fn get(key: &str) -> Option<$ty> {
                    match hash(key) {
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

            $vis static $name: $crate::map::StaticSet<&str, { 0$(+ {
                let _ = $key;
                1
            })* }> = {
                const LEN: usize = 0$(+ {
                    let _ = $key;
                    1
                })*;

                fn hash(key: &str) -> u64 {
                    $crate::algo::fnv1a(key.as_bytes(), None)
                }

                fn contains(key: &&str) -> bool {
                    match hash(*key) {
                        $(
                            $crate::hash!($key) if *key == $key => true,
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

        match $crate::algo::fnv1a($var.as_bytes(), None) {
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
