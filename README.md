# Cache LRU - Rust

Implémentation d'un cache LRU (Least Recently Used) avec les 4 itérations demandées.

## Structure Simple

```
src/
├── cache.rs        - LruCache<K, V> générique (itérations 1-3)
├── trait_cache.rs  - Trait CacheOps (itération 2)
├── persistent.rs   - PersistentLruCache (itération 4)
└── lib.rs          - Exports
```

## Utilisation

### Itération 1: Valeur générique
```rust
let mut cache = LruCache::new(3);
cache.put("age".to_string(), 42);
```

### Itération 2: Avec trait
```rust
let mut cache = LruCache::new(3);
cache.insert("x".to_string(), 10);
```

### Itération 3: Clé générique
```rust
let mut cache = LruCache::new(3);
cache.put(1, "one");
```

### Itération 4: Persistance
```rust
let mut cache = PersistentLruCache::new_persistent(3, "cache.txt").unwrap();
cache.put("key".into(), "value".into());
```

## Tests

```bash
cargo test
```

9 tests (5 unitaires + 4 intégration)

## Explication pour le prof

**LruCache<K, V>** : Cache générique qui couvre les 3 premières itérations
- HashMap pour stockage O(1)
- Vec pour l'ordre LRU
- Générique sur K et V dès le départ

**CacheOps** : Trait pour abstraction

**PersistentLruCache** : Avec auto-sauvegarde fichier
