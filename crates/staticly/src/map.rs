#[derive(Debug)]
pub struct StaticMap<K, V, const LEN: usize> {
    get_func: fn(K) -> Option<V>,
    get_hash_func: fn(u64) -> Option<V>,
    iter_func: fn(usize) -> Option<(K, V)>,
    contains_key_func: fn(&K) -> bool,
}

impl<K, V, const LEN: usize> StaticMap<K, V, LEN> {
    pub const LEN: usize = LEN;

    /// Constructs a new map from a VTable.
    ///
    /// This is used internally by the `static_map` macro and should not
    /// be used directly.
    #[inline(always)]
    pub const fn new(
        get_func: fn(K) -> Option<V>,
        get_hash_func: fn(u64) -> Option<V>,
        iter_func: fn(usize) -> Option<(K, V)>,
        contains_key_func: fn(&K) -> bool,
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
    pub fn get<Q: Into<K>>(&self, query: Q) -> Option<V> {
        (self.get_func)(query.into())
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
    pub fn contains_key<Q: Into<K>>(&self, query: Q) -> bool {
        (self.contains_key_func)(&query.into())
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
    pub fn into_iter(&self) -> StaticMapIter<K, V> {
        StaticMapIter { next_func: self.iter_func, index: 0 }
    }

    #[inline(always)]
    pub fn into_values(&self) -> Values<K, V> {
        Values { next_func: self.iter_func, index: 0 }
    }

    #[inline(always)]
    pub fn into_keys(&self) -> Keys<K, V> {
        Keys { next_func: self.iter_func, index: 0 }
    }

    #[inline(always)]
    pub fn iter(&self) -> StaticMapIter<K, V> {
        self.into_iter()
    }

    #[inline(always)]
    pub fn values(&self) -> Values<K, V> {
        self.into_values()
    }

    #[inline(always)]
    pub fn keys(&self) -> Keys<K, V> {
        self.into_keys()
    }
}
#[derive(Debug)]
pub struct StaticSet<K, const LEN: usize> {
    iter_func: fn(usize) -> Option<K>,
    contains_func: fn(&K) -> bool,
}

impl<K, const LEN: usize> StaticSet<K, LEN> {
    pub const LEN: usize = LEN;

    /// Constructs a new map from a VTable.
    ///
    /// This is used internally by the `static_map` macro and should not
    /// be used directly.
    #[inline(always)]
    pub const fn new(
        iter_func: fn(usize) -> Option<K>,
        contains_func: fn(&K) -> bool,
    ) -> Self {
        Self {
            iter_func,
            contains_func,
        }
    }

    /// Checks if the key is included in the static map.
    #[inline(always)]
    pub fn contains<Q: Into<K>>(&self, query: Q) -> bool {
        (self.contains_func)(&query.into())
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
    pub fn into_iter(&self) -> StaticSetIter<K> {
        StaticSetIter { next_func: self.iter_func, index: 0 }
    }

    #[inline(always)]
    pub fn iter(&self) -> StaticSetIter<K> {
        self.into_iter()
    }
}

#[derive(Debug)]
pub struct StaticMapIter<K, V> {
    next_func: fn(usize) -> Option<(K, V)>,
    index: usize,
}

impl<K, V> Iterator for StaticMapIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;
        (self.next_func)(index)
    }
}

#[derive(Debug)]
pub struct StaticSetIter<K> {
    next_func: fn(usize) -> Option<K>,
    index: usize,
}

impl<K> Iterator for StaticSetIter<K> {
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;
        (self.next_func)(index)
    }
}

#[derive(Debug)]
pub struct Values<K, V> {
    next_func: fn(usize) -> Option<(K, V)>,
    index: usize,
}

impl<K, V> Iterator for Values<K, V> {
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;
        (self.next_func)(index).map(|x| x.0)
    }
}

#[derive(Debug)]
pub struct Keys<K, V> {
    next_func: fn(usize) -> Option<(K, V)>,
    index: usize,
}

impl<K, V> Iterator for Keys<K, V> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;
        (self.next_func)(index).map(|x| x.1)
    }
}
