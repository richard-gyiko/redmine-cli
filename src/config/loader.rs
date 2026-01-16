//! Configuration loader with precedence: CLI > Env > Config File.

use directories::ProjectDirs;
use std::path::PathBuf;

use super::profile::ProfileStore;
use crate::error::{AppError, Result};

/// Cross-platform configuration paths.
pub struct ConfigPaths {
    /// Base config directory.
    #[allow(dead_code)]
    pub config_dir: PathBuf,
    /// Path to the config file.
    pub config_file: PathBuf,
    /// Path to the cache directory.
    pub cache_dir: PathBuf,
}

impl ConfigPaths {
    /// Get the configuration paths for this application.
    pub fn new() -> Result<Self> {
        let proj_dirs = ProjectDirs::from("", "", "redmine-agent-cli")
            .ok_or_else(|| AppError::config("Could not determine config directory"))?;

        let config_dir = proj_dirs.config_dir().to_path_buf();
        let config_file = config_dir.join("config.toml");
        let cache_dir = proj_dirs.cache_dir().to_path_buf();

        Ok(Self {
            config_dir,
            config_file,
            cache_dir,
        })
    }
}

impl Default for ConfigPaths {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            config_dir: PathBuf::from("."),
            config_file: PathBuf::from("config.toml"),
            cache_dir: PathBuf::from(".cache"),
        })
    }
}

/// Resolved configuration with URL and API key.
#[derive(Debug, Clone)]
pub struct Config {
    /// Redmine server URL.
    pub url: String,
    /// API key for authentication.
    pub api_key: String,
    /// Profile name if loaded from config.
    pub profile_name: Option<String>,
}

impl Config {
    /// Redact the API key for display.
    pub fn redacted_api_key(&self) -> String {
        if self.api_key.len() <= 8 {
            "****".to_string()
        } else {
            format!(
                "{}...{}",
                &self.api_key[..4],
                &self.api_key[self.api_key.len() - 4..]
            )
        }
    }
}

/// Load configuration with precedence: CLI flags > Env vars > Config file.
pub fn load_config(
    cli_url: Option<&str>,
    cli_api_key: Option<&str>,
    paths: &ConfigPaths,
) -> Result<Config> {
    // 1. Try CLI flags first
    if let (Some(url), Some(api_key)) = (cli_url, cli_api_key) {
        return Ok(Config {
            url: url.to_string(),
            api_key: api_key.to_string(),
            profile_name: None,
        });
    }

    // 2. Try environment variables
    let env_url = std::env::var("REDMINE_URL").ok();
    let env_api_key = std::env::var("REDMINE_API_KEY").ok();

    // Mix CLI with env (CLI takes precedence for individual values)
    let url = cli_url.map(|s| s.to_string()).or(env_url);
    let api_key = cli_api_key.map(|s| s.to_string()).or(env_api_key);

    if let (Some(url), Some(api_key)) = (url.clone(), api_key.clone()) {
        return Ok(Config {
            url,
            api_key,
            profile_name: None,
        });
    }

    // 3. Try config file (active profile)
    let store = ProfileStore::load(&paths.config_file)?;
    if let Some(profile) = store.get_active() {
        // Allow CLI/env to override individual values from profile
        let url = url.unwrap_or_else(|| profile.url.clone());
        let api_key = api_key.unwrap_or_else(|| profile.api_key.clone());
        return Ok(Config {
            url,
            api_key,
            profile_name: Some(profile.name.clone()),
        });
    }

    // 4. Error - no credentials found
    Err(AppError::config_with_hint(
        "No Redmine credentials configured",
        "Set REDMINE_URL and REDMINE_API_KEY environment variables, or use `rma profile add` to create a profile.",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn test_paths(dir: &std::path::Path) -> ConfigPaths {
        ConfigPaths {
            config_dir: dir.to_path_buf(),
            config_file: dir.join("config.toml"),
            cache_dir: dir.join("cache"),
        }
    }

    #[test]
    fn test_cli_flags_highest_precedence() {
        let dir = tempdir().unwrap();
        let paths = test_paths(dir.path());

        // Set env vars
        std::env::set_var("REDMINE_URL", "https://env.example.com");
        std::env::set_var("REDMINE_API_KEY", "env_key");

        // CLI should override
        let config = load_config(Some("https://cli.example.com"), Some("cli_key"), &paths).unwrap();

        assert_eq!(config.url, "https://cli.example.com");
        assert_eq!(config.api_key, "cli_key");

        // Cleanup
        std::env::remove_var("REDMINE_URL");
        std::env::remove_var("REDMINE_API_KEY");
    }

    #[test]
    fn test_env_vars_over_config_file() {
        let dir = tempdir().unwrap();
        let paths = test_paths(dir.path());

        // Create config file
        let mut store = ProfileStore::default();
        store.add(super::super::profile::Profile::new(
            "test",
            "https://file.example.com",
            "file_key",
        ));
        store.save(&paths.config_file).unwrap();

        // Set env vars
        std::env::set_var("REDMINE_URL", "https://env.example.com");
        std::env::set_var("REDMINE_API_KEY", "env_key");

        let config = load_config(None, None, &paths).unwrap();

        assert_eq!(config.url, "https://env.example.com");
        assert_eq!(config.api_key, "env_key");

        // Cleanup
        std::env::remove_var("REDMINE_URL");
        std::env::remove_var("REDMINE_API_KEY");
    }

    #[test]
    fn test_config_file_fallback() {
        let dir = tempdir().unwrap();
        let paths = test_paths(dir.path());

        // Make sure env vars are not set
        std::env::remove_var("REDMINE_URL");
        std::env::remove_var("REDMINE_API_KEY");

        // Create config file
        let mut store = ProfileStore::default();
        store.add(super::super::profile::Profile::new(
            "test",
            "https://file.example.com",
            "file_key",
        ));
        store.save(&paths.config_file).unwrap();

        let config = load_config(None, None, &paths).unwrap();

        assert_eq!(config.url, "https://file.example.com");
        assert_eq!(config.api_key, "file_key");
        assert_eq!(config.profile_name, Some("test".to_string()));
    }

    #[test]
    fn test_no_config_error() {
        let dir = tempdir().unwrap();
        let paths = test_paths(dir.path());

        // Make sure env vars are not set
        std::env::remove_var("REDMINE_URL");
        std::env::remove_var("REDMINE_API_KEY");

        let result = load_config(None, None, &paths);
        assert!(result.is_err());
    }
}
