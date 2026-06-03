//! Index entry file metadata cache helpers

use std::collections::BTreeMap;
use std::sync::RwLock;

use log::{debug, trace};

use super::IndexEntry;

/// Volatile registry index entry metadata cache
static INDEX_CACHE: RwLock<BTreeMap<String, IndexEntry>> = RwLock::new(BTreeMap::new());

/// Caches the index entry metadata in memory.
pub fn metadata_store_index_entry(entry: &IndexEntry) {
    let name = entry.name().to_owned();
    debug!("metadata_cache: storing metadata for '{}'", name);
    INDEX_CACHE.write().unwrap().insert(name, entry.clone());
}

/// Fetches the cached index entry metadata from memory.
pub fn metadata_fetch_index_entry(name: &str) -> Option<IndexEntry> {
    trace!("metadata_cache: looking up metadata for '{}'", name);
    let result = INDEX_CACHE.read().unwrap().get(name).map(ToOwned::to_owned);
    if result.is_some() {
        debug!("metadata_cache: metadata cache hit for '{}'", name);
    } else {
        trace!("metadata_cache: metadata cache miss for '{}'", name);
    }
    result
}

/// Erases the cached index entry metadata from memory.
pub fn metadata_invalidate_index_entry(entry: &IndexEntry) {
    let name = entry.name();
    debug!("metadata_cache: invalidating metadata for '{}'", name);
    INDEX_CACHE.write().unwrap().remove(name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_store_and_fetch() {
        let entry = IndexEntry::new("test-crate");
        metadata_store_index_entry(&entry);
        assert!(metadata_fetch_index_entry("test-crate").is_some());
    }

    #[test]
    fn test_metadata_fetch_miss() {
        assert!(metadata_fetch_index_entry("nonexistent").is_none());
    }

    #[test]
    fn test_metadata_invalidate() {
        let entry = IndexEntry::new("temp-crate");
        metadata_store_index_entry(&entry);
        assert!(metadata_fetch_index_entry("temp-crate").is_some());
        metadata_invalidate_index_entry(&entry);
        assert!(metadata_fetch_index_entry("temp-crate").is_none());
    }
}
