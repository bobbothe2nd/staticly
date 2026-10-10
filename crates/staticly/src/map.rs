#[derive(Debug)]
pub struct StaticMap<V, const LEN: usize> {
    get_func: fn(&str) -> Option<V>,
    get_hash_func: fn(u64) -> Option<V>,
    iter_func: fn(usize) -> Option<(&'static str, V)>,
    contains_key_func: fn(&str) -> bool,
}

impl<V, const LEN: usize> StaticMap<V, LEN> {
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

    #[inline(always)]
    pub const fn len(&self) -> usize {
        LEN
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        LEN == 0
    }

    #[inline(always)]
    pub fn into_iter(&self) -> StaticMapIter<V> {
        StaticMapIter {
            next_func: self.iter_func,
            index: 0,
        }
    }

    #[inline(always)]
    pub fn into_values(&self) -> Values<V> {
        Values {
            next_func: self.iter_func,
            index: 0,
        }
    }

    #[inline(always)]
    pub fn into_keys(&self) -> Keys<V> {
        Keys {
            next_func: self.iter_func,
            index: 0,
        }
    }

    #[inline(always)]
    pub fn iter(&self) -> StaticMapIter<V> {
        self.into_iter()
    }

    #[inline(always)]
    pub fn values(&self) -> Values<V> {
        self.into_values()
    }

    #[inline(always)]
    pub fn keys(&self) -> Keys<V> {
        self.into_keys()
    }
}
#[derive(Debug)]
pub struct StaticSet<const LEN: usize> {
    iter_func: fn(usize) -> Option<&'static str>,
    contains_func: fn(&str) -> bool,
}

impl<const LEN: usize> StaticSet<LEN> {
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

    #[inline(always)]
    pub const fn len(&self) -> usize {
        LEN
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        LEN == 0
    }

    #[inline(always)]
    pub fn into_iter(&self) -> StaticSetIter {
        StaticSetIter {
            next_func: self.iter_func,
            index: 0,
        }
    }

    #[inline(always)]
    pub fn iter(&self) -> StaticSetIter {
        self.into_iter()
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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
