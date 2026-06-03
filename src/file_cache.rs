//! Index entry and crate file cache helpers

use std::fs::{create_dir_all, metadata, read, write, File};
use std::io::Write;
use std::path::Path;

use log::{debug, error, trace};

use super::{CrateInfo, IndexEntry};

/// Caches the crate package file on the local filesystem.
pub fn cache_store_crate(dir: &Path, crate_info: &CrateInfo, data: &[u8]) {
    let crate_file_path = dir.join(crate_info.to_file_path());
    debug!("cache: storing crate file: {}", crate_file_path.display());

    // Create all parent directories first.
    if let Err(e) = create_dir_all(crate_file_path.parent().unwrap()) {
        error!("cache: failed to create crate directory: {e}");
        return;
    }

    match write(&crate_file_path, data) {
        Ok(()) => debug!(
            "cache: stored crate file: {} ({} bytes)",
            crate_file_path.display(),
            data.len()
        ),
        Err(e) => error!(
            "cache: failed to write crate file {}: {e}",
            crate_file_path.display()
        ),
    }
}

/// Fetches the cached crate package file from the local filesystem, if present.
pub fn cache_fetch_crate(dir: &Path, crate_info: &CrateInfo) -> Option<Vec<u8>> {
    let path = dir.join(crate_info.to_file_path());
    trace!("cache: looking for crate file: {}", path.display());
    match read(&path) {
        Ok(data) => {
            debug!(
                "cache: found crate file: {} ({} bytes)",
                path.display(),
                data.len()
            );
            Some(data)
        }
        Err(e) => {
            trace!("cache: crate file not found: {}: {e}", path.display());
            None
        }
    }
}

/// Caches the index entry file on the local filesystem.
pub fn cache_store_index_entry(dir: &Path, entry: &IndexEntry, data: &[u8]) {
    let entry_file_path = dir.join(entry.to_file_path());
    debug!(
        "cache: storing index entry file: {}",
        entry_file_path.display()
    );

    if let Err(e) = create_dir_all(entry_file_path.parent().unwrap()) {
        error!("cache: failed to create index directory: {e}");
        return;
    }

    let mut file = match File::create(&entry_file_path) {
        Ok(f) => f,
        Err(e) => {
            error!(
                "cache: failed to create index entry file {}: {e}",
                entry_file_path.display()
            );
            return;
        }
    };

    if let Err(e) = file.write_all(data) {
        error!(
            "cache: failed to write index entry data to {}: {e}",
            entry_file_path.display()
        );
        return;
    }

    debug!(
        "cache: stored index entry file: {} ({} bytes)",
        entry_file_path.display(),
        data.len()
    );

    // Set the cache file mtime according to the Last-Modified HTTP metadata.
    if let Some(mtime) = entry.mtime() {
        if let Err(e) = file.set_modified(mtime) {
            error!(
                "cache: failed to set index entry file mtime for {}: {e}",
                entry_file_path.display()
            );
        } else {
            debug!(
                "cache: set index entry file mtime for {}: {:?}",
                entry_file_path.display(),
                mtime
            );
        }
    }
}

/// Fetches the cached index entry file from the local filesystem, if present.
pub fn cache_fetch_index_entry(dir: &Path, entry: &IndexEntry) -> Option<Vec<u8>> {
    let path = dir.join(entry.to_file_path());
    trace!("cache: looking for index entry file: {}", path.display());
    match read(&path) {
        Ok(data) => {
            debug!(
                "cache: found index entry file: {} ({} bytes)",
                path.display(),
                data.len()
            );
            Some(data)
        }
        Err(e) => {
            trace!("cache: index entry file not found: {}: {e}", path.display());
            None
        }
    }
}

/// Tries to recreate the missing index entry metadata from the cache file metadata.
pub fn cache_try_find_index_entry(dir: &Path, name: &str) -> Option<IndexEntry> {
    let mut entry = IndexEntry::new(name);
    let path = dir.join(entry.to_file_path());
    trace!("cache: looking for index file metadata: {}", path.display());

    let mtime = metadata(&path).ok()?.modified().ok()?;

    debug!(
        "cache: recreated index entry metadata from file mtime: {}: {:?}",
        path.display(),
        mtime
    );
    entry.set_mtime(mtime);

    Some(entry)
}
