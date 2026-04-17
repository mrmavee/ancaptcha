//! Thread-safe singleton for asset variation caching.

pub mod cache;

pub use cache::{AssetCache, get_cache, init_cache, init_with_intensity};
