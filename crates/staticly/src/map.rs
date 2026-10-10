//! Implementations of [`StaticMap`] and [`StaticSet`] for no-std

/// Like `std::collections::HashMap`, but with compile-time defined keys/values
#[derive(Debug, Clone)]
pub struct StaticMap<V, const LEN: usize> {
    get_func: fn(&str) -> Option<V>,
    get_hash_func: fn(u64) -> Option<V>,
    iter_func: fn(usize) -> Option<(&'static str, V)>,
    contains_key_func: fn(&str) -> bool,
}

impl<V, const LEN: usize> StaticMap<V, LEN> {
    /// The length of this map, equal to `LEN`
    pub const LEN: usize = LEN;

    /// Constructs a new map from a VTable.
    ///
    /// This is used internally by the `static_map` macro and should not
    /// be used directly.
    #[inline(always)]
    pub const fn new(
        get_func: fn(&str) -> Option<V>,
        get_hash_func: fn(u64) -> Option<V>,
        iter_func: fn(usize) -> Option<(&'static str, V)>,
        contains_key_func: fn(&str) -> bool,
    ) -> Self {
        Self {
            get_func,
            get_hash_func,
            iter_func,
            contains_key_func,
        }
    }

    /// Gets the value with matching key as `query`.
    ///
    /// This is collision safe and will check against the query on
    /// hash equality.
    #[inline(always)]
    pub fn get(&self, query: &str) -> Option<V> {
        (self.get_func)(query)
    }

    /// Gets an unspecified value matching the provided hash.
    ///
    /// This is NOT collision-safe because it has no query to
    /// check where there are multiple equal hashes. This just
    /// returns the first one it finds.
    #[inline(always)]
    pub fn get_hash(&self, hash: u64) -> Option<V> {
        (self.get_hash_func)(hash)
    }

    /// Checks if the key is included in the static map.
    #[inline(always)]
    pub fn contains_key(&self, query: &str) -> bool {
        (self.contains_key_func)(query)
    }

    /// Gets the length of this map, equal to `LEN`
    #[inline(always)]
    pub const fn len(&self) -> usize {
        LEN
    }

    /// Checks if this map has any keys, equal to `LEN == 0`
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        LEN == 0
    }

    /// Converts the static map into a static iterator
    #[inline(always)]
<<<<<<< HEAD
    pub const fn iter(&self) -> StaticMapIter<V> {
=======
    pub fn into_iter(&self) -> StaticMapIter<V> {
>>>>>>> ba5e60cfceba11284255adf2578415dda091a2e9
        StaticMapIter {
            next_func: self.iter_func,
            index: 0,
        }
    }

    /// Converts the static map into a static iterator over values
    #[inline(always)]
<<<<<<< HEAD
    pub const fn values(&self) -> Values<V> {
=======
    pub fn into_values(&self) -> Values<V> {
>>>>>>> ba5e60cfceba11284255adf2578415dda091a2e9
        Values {
            next_func: self.iter_func,
            index: 0,
        }
    }

    /// Converts the static map into a static iterator over keys
    #[inline(always)]
<<<<<<< HEAD
    pub const fn keys(&self) -> Keys<V> {
=======
    pub fn into_keys(&self) -> Keys<V> {
>>>>>>> ba5e60cfceba11284255adf2578415dda091a2e9
        Keys {
            next_func: self.iter_func,
            index: 0,
        }
    }

    /// Converts the static map into a static iterator
    #[inline(always)]
    pub const fn into_iter(self) -> StaticMapIter<V> {
        self.iter()
    }

    /// Converts the static map into a static iterator over values
    #[inline(always)]
    pub fn into_values(self) -> Values<V> {
        self.values()
    }

    /// Converts the static map into a static iterator over keys
    #[inline(always)]
    pub const fn into_keys(self) -> Keys<V> {
        self.keys()
    }
}

/// Like `std::collections::HashSet`, but with compile-time defined keys
#[derive(Debug, Clone)]
pub struct StaticSet<const LEN: usize> {
    iter_func: fn(usize) -> Option<&'static str>,
    contains_func: fn(&str) -> bool,
}

impl<const LEN: usize> StaticSet<LEN> {
    /// The length of this map, equal to `LEN`
    pub const LEN: usize = LEN;

    /// Constructs a new map from a VTable.
    ///
    /// This is used internally by the `static_map` macro and should not
    /// be used directly.
    #[inline(always)]
    pub const fn new(
        iter_func: fn(usize) -> Option<&'static str>,
        contains_func: fn(&str) -> bool,
    ) -> Self {
        Self {
            iter_func,
            contains_func,
        }
    }

    /// Checks if the key is included in the static map.
    #[inline(always)]
    pub fn contains(&self, query: &str) -> bool {
        (self.contains_func)(query)
    }

    /// Gets the length of this map, equal to `LEN`
    #[inline(always)]
    pub const fn len(&self) -> usize {
        LEN
    }

    /// Checks if this map has any keys, equal to `LEN == 0`
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        LEN == 0
    }

    /// Converts the static map into a static iterator
    #[inline(always)]
<<<<<<< HEAD
    pub const fn iter(&self) -> StaticSetIter {
=======
    pub fn into_iter(&self) -> StaticSetIter {
>>>>>>> ba5e60cfceba11284255adf2578415dda091a2e9
        StaticSetIter {
            next_func: self.iter_func,
            index: 0,
        }
    }

    /// Converts the static map into a static iterator
    #[inline(always)]
    pub const fn into_iter(&self) -> StaticSetIter {
        self.iter()
    }
}

/// Iterator over key/value pairs of a static map
#[derive(Debug, Clone)]
pub struct StaticMapIter<V> {
    next_func: fn(usize) -> Option<(&'static str, V)>,
    index: usize,
}

impl<V> Iterator for StaticMapIter<V> {
    type Item = (&'static str, V);

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;
        (self.next_func)(index)
    }
}

/// Iterator over keys of a static set
#[derive(Debug, Clone)]
pub struct StaticSetIter {
    next_func: fn(usize) -> Option<&'static str>,
    index: usize,
}

impl Iterator for StaticSetIter {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;
        (self.next_func)(index)
    }
}

/// Static iterator over values
#[derive(Debug, Clone)]
pub struct Values<V> {
    next_func: fn(usize) -> Option<(&'static str, V)>,
    index: usize,
}

impl<V> Iterator for Values<V> {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;
        (self.next_func)(index).map(|x| x.0)
    }
}

/// Static iterator over keys
#[derive(Debug, Clone)]
pub struct Keys<V> {
    next_func: fn(usize) -> Option<(&'static str, V)>,
    index: usize,
}

impl<V> Iterator for Keys<V> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;
        (self.next_func)(index).map(|x| x.1)
    }
}
