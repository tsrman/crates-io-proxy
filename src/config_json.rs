//! Sparse registry configuration file helpers

use log::debug;

use super::{ProxyConfig, CRATES_API_PATH};

/// Registry configuration file endpoint path
const CONFIG_JSON_ENDPOINT: &str = "config.json";

/// Checks for the registry configuration file download endpoint.
#[must_use]
pub fn is_config_json_url(index_url: &str) -> bool {
    let result = index_url == CONFIG_JSON_ENDPOINT;
    if result {
        debug!("config_json: matched config.json endpoint for '{index_url}'");
    }
    result
}

/// Dynamically generates the registry configuration file contents.
#[must_use]
pub(super) fn gen_config_json_file(config: &ProxyConfig) -> String {
    // Generate the crate download API URL pointing to this same proxy server.
    let dl_url = config
        .proxy_url
        .join(CRATES_API_PATH)
        .expect("invalid proxy server URL");

    // Cargo can not handle trailing slashes in `config.json`.
    let dl = dl_url.as_str().trim_end_matches('/');
    let api = config.upstream_url.as_str().trim_end_matches('/');

    let json = format!(r#"{{"dl":"{dl}","api":"{api}"}}"#);
    debug!("config_json: generated config.json: dl='{dl}', api='{api}'");
    json
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::Duration;
    use url::Url;

    fn test_config() -> ProxyConfig {
        ProxyConfig {
            index_url: Url::parse("https://index.crates.io/").unwrap(),
            upstream_url: Url::parse("https://crates.io/").unwrap(),
            proxy_url: Url::parse("http://localhost:3080/").unwrap(),
            index_dir: PathBuf::from("/tmp/index"),
            crates_dir: PathBuf::from("/tmp/crates"),
            cache_ttl: Duration::from_secs(3600),
        }
    }

    #[test]
    fn test_is_config_json_url() {
        assert!(is_config_json_url("config.json"));
        assert!(!is_config_json_url("config.json.bak"));
        assert!(!is_config_json_url("index/config.json"));
        assert!(!is_config_json_url(""));
    }

    #[test]
    fn test_gen_config_json_file() {
        let config = test_config();
        let json = gen_config_json_file(&config);
        assert_eq!(
            json,
            r#"{"dl":"http://localhost:3080/api/v1/crates","api":"https://crates.io"}"#
        );
    }
}
