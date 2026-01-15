use std::collections::HashMap;
use std::hash::Hash;

/// Cache LRU générique K → V
///
/// Itérations 1-3: Valeur générique, Clé générique, Trait
///
/// # Exemples
///
/// ```
/// use lru_cache::LruCache;
///
/// let mut cache = LruCache::new(3);
/// cache.put("A".to_string(), "valeur_a".to_string());
/// cache.put("B".to_string(), "valeur_b".to_string());
/// cache.put("C".to_string(), "valeur_c".to_string());
///
/// // Cache plein: [A, B, C]
/// cache.put("D".to_string(), "valeur_d".to_string());
///
/// // A est évincé, cache: [B, C, D]
/// assert_eq!(cache.get(&"A".to_string()), None);
/// assert_eq!(cache.get(&"B".to_string()), Some(&"valeur_b".to_string()));
/// ```
pub struct LruCache<K, V>
where
    K: Hash + Eq + Clone,
{
    capacity: usize,
    items: HashMap<K, V>,
    usage: Vec<K>,
}

impl<K, V> LruCache<K, V>
where
    K: Hash + Eq + Clone,
{
    /// Crée un nouveau cache LRU
    ///
    /// # Exemples
    ///
    /// ```
    /// use lru_cache::LruCache;
    ///
    /// let mut cache: LruCache<String, i32> = LruCache::new(3);
    /// assert_eq!(cache.len(), 0);
    /// ```
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            items: HashMap::new(),
            usage: Vec::new(),
        }
    }

    /// Insère une paire clé-valeur
    ///
    /// Retourne l'ancienne valeur si la clé existait déjà.
    ///
    /// # Exemples
    ///
    /// ```
    /// use lru_cache::LruCache;
    ///
    /// let mut cache = LruCache::new(2);
    /// assert_eq!(cache.put("x".to_string(), 10), None);
    /// assert_eq!(cache.put("x".to_string(), 20), Some(10)); // mise à jour
    /// ```
    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        if self.capacity == 0 {
            return None;
        }

        // Mise à jour si existe
        if let Some(old_value) = self.items.insert(key.clone(), value) {
            self.move_to_recent(&key);
            return Some(old_value);
        }

        // Éviction si plein
        if self.items.len() > self.capacity {
            if let Some(lru_key) = self.usage.first().cloned() {
                self.items.remove(&lru_key);
                self.usage.retain(|k| k != &lru_key);
            }
        }

        self.usage.push(key);
        None
    }

    /// Récupère une valeur et marque la clé comme récemment utilisée
    ///
    /// # Exemples
    ///
    /// ```
    /// use lru_cache::LruCache;
    ///
    /// let mut cache = LruCache::new(2);
    /// cache.put("key".to_string(), "value".to_string());
    ///
    /// assert_eq!(cache.get(&"key".to_string()), Some(&"value".to_string()));
    /// assert_eq!(cache.get(&"missing".to_string()), None);
    /// ```
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.items.contains_key(key) {
            self.move_to_recent(key);
            self.items.get(key)
        } else {
            None
        }
    }

    fn move_to_recent(&mut self, key: &K) {
        self.usage.retain(|k| k != key);
        self.usage.push(key.clone());
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_string() {
        let mut cache = LruCache::new(2);
        cache.put("a".to_string(), "1".to_string());
        cache.put("b".to_string(), "2".to_string());

        assert_eq!(cache.get(&"a".to_string()), Some(&"1".to_string()));
    }

    #[test]
    fn test_generic_value() {
        let mut cache = LruCache::new(2);
        cache.put("age".to_string(), 42);

        assert_eq!(cache.get(&"age".to_string()), Some(&42));
    }

    #[test]
    fn test_generic_key() {
        let mut cache = LruCache::new(2);
        cache.put(1, "one");
        cache.put(2, "two");

        assert_eq!(cache.get(&1), Some(&"one"));
    }

    #[test]
    fn test_eviction() {
        let mut cache = LruCache::new(2);
        cache.put(1, "a");
        cache.put(2, "b");
        cache.put(3, "c"); // évince 1

        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), Some(&"b"));
    }
}
