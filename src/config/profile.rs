//! Profile management for multiple Redmine instances.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::error::{AppError, Result};

/// A single Redmine profile with connection details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    /// Display name for the profile.
    pub name: String,
    /// Redmine server URL.
    pub url: String,
    /// API key for authentication.
    pub api_key: String,
}

impl Profile {
    /// Create a new profile.
    pub fn new(
        name: impl Into<String>,
        url: impl Into<String>,
        api_key: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            url: url.into(),
            api_key: api_key.into(),
        }
    }

    /// Redact the API key for display.
    #[allow(dead_code)]
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

/// Storage for multiple profiles.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfileStore {
    /// The currently active profile name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<String>,
    /// Map of profile name to profile.
    #[serde(default)]
    pub profiles: HashMap<String, Profile>,
}

impl ProfileStore {
    /// Load profile store from a TOML file.
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(path)?;
        let store: ProfileStore = toml::from_str(&content)?;
        Ok(store)
    }

    /// Save profile store to a TOML file.
    pub fn save(&self, path: &Path) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)
            .map_err(|e| AppError::config(format!("Failed to serialize config: {}", e)))?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Add or update a profile.
    pub fn add(&mut self, profile: Profile) {
        let name = profile.name.clone();
        self.profiles.insert(name.clone(), profile);
        // If this is the first profile, make it active
        if self.active.is_none() {
            self.active = Some(name);
        }
    }

    /// Delete a profile by name.
    pub fn delete(&mut self, name: &str) -> Result<()> {
        if !self.profiles.contains_key(name) {
            return Err(AppError::not_found_with_hint(
                "Profile",
                name,
                "Use `rdm profile list` to see available profiles.",
            ));
        }
        self.profiles.remove(name);
        // Clear active if we deleted the active profile
        if self.active.as_deref() == Some(name) {
            self.active = self.profiles.keys().next().cloned();
        }
        Ok(())
    }

    /// Set the active profile.
    pub fn set_active(&mut self, name: &str) -> Result<()> {
        if !self.profiles.contains_key(name) {
            return Err(AppError::not_found_with_hint(
                "Profile",
                name,
                "Use `rdm profile list` to see available profiles.",
            ));
        }
        self.active = Some(name.to_string());
        Ok(())
    }

    /// Get the active profile.
    pub fn get_active(&self) -> Option<&Profile> {
        self.active
            .as_ref()
            .and_then(|name| self.profiles.get(name))
    }

    /// Get a profile by name.
    #[allow(dead_code)]
    pub fn get(&self, name: &str) -> Option<&Profile> {
        self.profiles.get(name)
    }

    /// List all profile names.
    #[allow(dead_code)]
    pub fn list(&self) -> Vec<&str> {
        self.profiles.keys().map(|s| s.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_profile_redacted_key() {
        let profile = Profile::new("test", "https://example.com", "abcd1234efgh5678");
        assert_eq!(profile.redacted_api_key(), "abcd...5678");
    }

    #[test]
    fn test_profile_redacted_key_short() {
        let profile = Profile::new("test", "https://example.com", "short");
        assert_eq!(profile.redacted_api_key(), "****");
    }

    #[test]
    fn test_profile_store_add() {
        let mut store = ProfileStore::default();
        store.add(Profile::new("work", "https://work.example.com", "key1"));
        assert_eq!(store.active, Some("work".to_string()));
        assert!(store.profiles.contains_key("work"));
    }

    #[test]
    fn test_profile_store_save_load() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("config.toml");

        let mut store = ProfileStore::default();
        store.add(Profile::new("test", "https://example.com", "apikey"));
        store.save(&path).unwrap();

        let loaded = ProfileStore::load(&path).unwrap();
        assert_eq!(loaded.active, Some("test".to_string()));
        assert!(loaded.profiles.contains_key("test"));
    }

    #[test]
    fn test_profile_store_delete() {
        let mut store = ProfileStore::default();
        store.add(Profile::new("work", "https://work.example.com", "key1"));
        store.add(Profile::new("home", "https://home.example.com", "key2"));
        store.set_active("work").unwrap();

        store.delete("work").unwrap();
        assert!(!store.profiles.contains_key("work"));
        // Active should switch to remaining profile
        assert_eq!(store.active, Some("home".to_string()));
    }
}
