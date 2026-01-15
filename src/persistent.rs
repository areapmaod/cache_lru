use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

/// Cache LRU avec persistance fichier (Itération 4)
///
/// # Exemples
///
/// ```no_run
/// use lru_cache::PersistentLruCache;
///
/// // Crée un cache qui se sauvegarde automatiquement
/// let mut cache = PersistentLruCache::new_persistent(3, "cache.txt").unwrap();
/// cache.put("key".to_string(), "value".to_string());
///
/// // La donnée est automatiquement sauvegardée dans cache.txt
/// ```
pub struct PersistentLruCache {
    capacity: usize,
    items: HashMap<String, String>,
    usage: Vec<String>,
    file_path: Option<String>,
}

impl PersistentLruCache {
    /// Crée un cache normal sans persistance
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            items: HashMap::new(),
            usage: Vec::new(),
            file_path: None,
        }
    }

    /// Crée un cache persistant (auto-charge et auto-sauvegarde)
    ///
    /// # Exemples
    ///
    /// ```no_run
    /// use lru_cache::PersistentLruCache;
    ///
    /// let mut cache = PersistentLruCache::new_persistent(3, "mon_cache.txt").unwrap();
    /// cache.put("user1".to_string(), "Alice".to_string());
    /// ```
    pub fn new_persistent(capacity: usize, path: &str) -> std::io::Result<Self> {
        let mut cache = Self {
            capacity,
            items: HashMap::new(),
            usage: Vec::new(),
            file_path: Some(path.to_string()),
        };

        // Charger depuis le fichier s'il existe
        if Path::new(path).exists() {
            cache.load()?;
        }

        Ok(cache)
    }

    pub fn put(&mut self, key: String, value: String) -> Option<String> {
        if self.capacity == 0 {
            return None;
        }

        let result = if let Some(old) = self.items.insert(key.clone(), value) {
            self.move_to_recent(&key);
            Some(old)
        } else {
            if self.items.len() > self.capacity {
                if let Some(lru_key) = self.usage.first().cloned() {
                    self.items.remove(&lru_key);
                    self.usage.retain(|k| k != &lru_key);
                }
            }
            self.usage.push(key);
            None
        };

        // Auto-save
        if let Some(ref path) = self.file_path {
            let _ = self.save_to(path);
        }

        result
    }

    pub fn get(&mut self, key: &str) -> Option<&String> {
        if self.items.contains_key(key) {
            self.move_to_recent(&key.to_string());
            self.items.get(key)
        } else {
            None
        }
    }

    fn move_to_recent(&mut self, key: &String) {
        self.usage.retain(|k| k != key);
        self.usage.push(key.clone());
    }

    fn save_to(&self, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        writeln!(file, "{}", self.capacity)?;

        for key in &self.usage {
            if let Some(val) = self.items.get(key) {
                writeln!(file, "{}:{}", key, val)?;
            }
        }

        Ok(())
    }

    fn load(&mut self) -> std::io::Result<()> {
        if let Some(ref path) = self.file_path.clone() {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let mut lines = reader.lines();

            if let Some(Ok(cap_line)) = lines.next() {
                self.capacity = cap_line.parse().unwrap_or(self.capacity);
            }

            for line in lines {
                if let Ok(content) = line {
                    if let Some(pos) = content.find(':') {
                        let k = content[..pos].to_string();
                        let v = content[pos + 1..].to_string();
                        self.items.insert(k.clone(), v);
                        self.usage.push(k);
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_persistent() {
        let path = "test_cache_persist.txt";

        {
            let mut cache = PersistentLruCache::new_persistent(2, path).unwrap();
            cache.put("key1".into(), "val1".into());
        }

        {
            let mut cache2 = PersistentLruCache::new_persistent(2, path).unwrap();
            assert_eq!(cache2.get("key1"), Some(&"val1".to_string()));
        }

        fs::remove_file(path).ok();
    }
}
