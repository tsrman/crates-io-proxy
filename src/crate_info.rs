//! Rust crate information helpers

use std::fmt::{Display, Formatter, Result};
use std::path::PathBuf;

use log::{debug, trace};

/// Crate download API endpoint suffix
const DOWNLOAD_API_ENDPOINT: &str = "/download";

/// Rust crate information structure
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CrateInfo {
    name: String,
    version: String,
}

impl Display for CrateInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} v{}", self.name, self.version)
    }
}

impl CrateInfo {
    /// Creates a new crate information object.
    #[must_use]
    pub fn new(name: &str, version: &str) -> Self {
        CrateInfo {
            name: name.to_owned(),
            version: version.to_owned(),
        }
    }

    /// Gets the crate name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Extracts crate information from the download API URL path.
    #[must_use]
    pub fn try_from_download_url(url: &str) -> Option<Self> {
        trace!("crate_info: parsing download URL: '{url}'");

        let name_version = url.strip_suffix(DOWNLOAD_API_ENDPOINT)?;
        trace!("crate_info: stripped suffix, remaining: '{name_version}'");

        let mut i = name_version.split('/');
        let result = match (i.next(), i.next(), i.next()) {
            (Some(name), Some(version), None) => {
                debug!("crate_info: parsed crate info: name='{name}', version='{version}'");
                Some(CrateInfo::new(name, version))
            }
            other => {
                debug!(
                    "crate_info: failed to parse download URL '{url}': unexpected segments: {:?}",
                    other
                );
                None
            }
        };
        result
    }

    /// Builds the crate download URL (relative).
    #[must_use]
    pub fn to_download_url(&self) -> String {
        format!(
            "{name}/{version}{DOWNLOAD_API_ENDPOINT}",
            name = self.name,
            version = self.version
        )
    }

    /// Builds the crate file name for cache storage.
    #[must_use]
    pub fn to_file_name(&self) -> String {
        format!("{}-{}.crate", self.name, self.version)
    }

    /// Builds the relative crate file path for cache storage.
    #[must_use]
    pub fn to_file_path(&self) -> PathBuf {
        PathBuf::from(self.name()).join(self.to_file_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_download_url() {
        assert_eq!(
            CrateInfo::try_from_download_url("foo/1.0.0/download"),
            Some(CrateInfo::new("foo", "1.0.0"))
        );
        assert_eq!(
            CrateInfo::try_from_download_url("my-crate/2.1.0-rc1/download"),
            Some(CrateInfo::new("my-crate", "2.1.0-rc1"))
        );
    }

    #[test]
    fn test_try_from_download_url_invalid() {
        assert_eq!(CrateInfo::try_from_download_url(""), None);
        assert_eq!(CrateInfo::try_from_download_url("foo/1.0.0"), None);
        assert_eq!(
            CrateInfo::try_from_download_url("foo/1.0.0/extra/download"),
            None
        );
        assert_eq!(CrateInfo::try_from_download_url("foo/download"), None);
    }

    #[test]
    fn test_to_download_url() {
        let info = CrateInfo::new("foo", "1.0.0");
        assert_eq!(info.to_download_url(), "foo/1.0.0/download");
    }

    #[test]
    fn test_to_file_name() {
        let info = CrateInfo::new("foo", "1.0.0");
        assert_eq!(info.to_file_name(), "foo-1.0.0.crate");
    }

    #[test]
    fn test_to_file_path() {
        let info = CrateInfo::new("foo", "1.0.0");
        assert_eq!(info.to_file_path(), PathBuf::from("foo/foo-1.0.0.crate"));
    }
}
