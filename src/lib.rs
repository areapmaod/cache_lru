//! # Cache LRU
//!
//! Implémentation d'un cache LRU (Least Recently Used) en Rust.
//!
//! Le cache évince automatiquement les éléments les moins récemment utilisés.

mod cache;
mod trait_cache;
mod persistent;

pub use cache::LruCache;
pub use trait_cache::CacheOps;
pub use persistent::PersistentLruCache;
