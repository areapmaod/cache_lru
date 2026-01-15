use crate::cache::LruCache;
use std::hash::Hash;

/// Trait pour les opérations de cache (Itération 2)
pub trait CacheOps<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V>;
    fn retrieve(&mut self, key: &K) -> Option<&V>;
    fn size(&self) -> usize;
}

impl<K, V> CacheOps<K, V> for LruCache<K, V>
where
    K: Hash + Eq + Clone,
{
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.put(key, value)
    }

    fn retrieve(&mut self, key: &K) -> Option<&V> {
        self.get(key)
    }

    fn size(&self) -> usize {
        self.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_usage() {
        let mut cache = LruCache::new(2);
        cache.insert("x".to_string(), 10);

        assert_eq!(cache.retrieve(&"x".to_string()), Some(&10));
        assert_eq!(cache.size(), 1);
    }
}
