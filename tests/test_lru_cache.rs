use lru_cache::*;

#[test]
fn test_basic_usage() {
    let mut cache = LruCache::new(3);
    cache.put("a".to_string(), "1".to_string());
    cache.put("b".to_string(), "2".to_string());

    assert_eq!(cache.get(&"a".to_string()), Some(&"1".to_string()));
}

#[test]
fn test_generic_types() {
    let mut cache = LruCache::new(2);
    cache.put(1, "one");
    cache.put(2, "two");

    assert_eq!(cache.get(&1), Some(&"one"));
}

#[test]
fn test_with_trait() {
    let mut cache = LruCache::new(2);
    CacheOps::insert(&mut cache, "x".to_string(), 10);

    assert_eq!(CacheOps::retrieve(&mut cache, &"x".to_string()), Some(&10));
}

#[test]
fn test_persistence() {
    use std::fs;
    let path = "test_integration.txt";

    {
        let mut cache = PersistentLruCache::new_persistent(2, path).unwrap();
        cache.put("foo".into(), "bar".into());
    }

    {
        let mut cache = PersistentLruCache::new_persistent(2, path).unwrap();
        assert_eq!(cache.get("foo"), Some(&"bar".to_string()));
    }

    fs::remove_file(path).ok();
}
