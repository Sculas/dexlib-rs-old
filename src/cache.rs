use lru::LruCache;
use parking_lot::Mutex;
use std::{cmp::Eq, hash::Hash, num::NonZeroUsize, sync::Arc};

/// LRU cache that provides interior mutability
pub(crate) struct Cache<K, V> {
    inner: Arc<Mutex<LruCache<K, V>>>,
}

impl<K: Hash + Eq, V: Clone> Cache<K, V> {
    /// Get a new instance of cache with the given capacity
    pub(crate) fn new(cap: usize) -> Self {
        Self {
            inner: Arc::new(Mutex::new(LruCache::new(NonZeroUsize::new(cap).unwrap()))),
        }
    }

    /// Get a reference to the value at key from the cache, if found
    pub(crate) fn get(&self, key: &K) -> Option<V> {
        self.inner.lock().get(key).map(std::clone::Clone::clone)
    }

    /// Insert a new key value pair into the cache
    pub(crate) fn put(&self, key: K, value: V) {
        self.inner.lock().put(key, value);
    }
}

impl<K, V> Clone for Cache<K, V> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
