//! Profile management commands.

use clap::{Args, Subcommand};
use serde::Serialize;

use crate::config::{Config, ConfigPaths, Profile, ProfileStore};
use crate::error::Result;
use crate::output::{markdown::markdown_table, MarkdownOutput, Meta};

#[derive(Debug, Subcommand)]
pub enum ProfileCommand {
    /// Add a new profile.
    Add(ProfileAdd),
    /// Set the active profile.
    Use(ProfileUse),
    /// List all profiles.
    List,
    /// Delete a profile.
    Delete(ProfileDelete),
}

#[derive(Debug, Args)]
pub struct ProfileAdd {
    /// Profile name.
    #[arg(long)]
    pub name: String,
    /// Redmine server URL.
    #[arg(long)]
    pub url: String,
    /// API key.
    #[arg(long)]
    pub api_key: String,
}

#[derive(Debug, Args)]
pub struct ProfileUse {
    /// Profile name to activate.
    pub name: String,
}

#[derive(Debug, Args)]
pub struct ProfileDelete {
    /// Profile name to delete.
    #[arg(long)]
    pub name: String,
}

#[derive(Debug, Args)]
pub struct ConfigShow {}

/// Result of profile add command.
#[derive(Debug, Clone, Serialize)]
pub struct ProfileAdded {
    pub name: String,
    pub url: String,
    pub is_active: bool,
}

impl MarkdownOutput for ProfileAdded {
    fn to_markdown(&self, _meta: &Meta) -> String {
        let mut output = String::new();
        output.push_str("## Profile Added\n\n");
        output.push_str(&format!("- **Name**: {}\n", self.name));
        output.push_str(&format!("- **URL**: {}\n", self.url));
        if self.is_active {
            output.push_str("- **Status**: Active\n");
        }
        output.push_str("\n*Use `rdm ping` to test the connection*\n");
        output
    }
}

/// Result of profile use command.
#[derive(Debug, Clone, Serialize)]
pub struct ProfileActivated {
    pub name: String,
}

impl MarkdownOutput for ProfileActivated {
    fn to_markdown(&self, _meta: &Meta) -> String {
        format!(
            "## Profile Activated\n\nNow using profile: **{}**\n",
            self.name
        )
    }
}

/// Result of profile list command.
#[derive(Debug, Clone, Serialize)]
pub struct ProfileList {
    pub profiles: Vec<ProfileInfo>,
    pub active: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProfileInfo {
    pub name: String,
    pub url: String,
    pub is_active: bool,
}

impl MarkdownOutput for ProfileList {
    fn to_markdown(&self, _meta: &Meta) -> String {
        let mut output = String::new();
        output.push_str("## Profiles\n\n");

        if self.profiles.is_empty() {
            output.push_str("*No profiles configured*\n\n");
            output.push_str("Use `rdm profile add --name <name> --url <url> --api-key <key>` to add a profile.\n");
            return output;
        }

        let headers = &["Name", "URL", "Active"];
        let rows: Vec<Vec<String>> = self
            .profiles
            .iter()
            .map(|p| {
                vec![
                    p.name.clone(),
                    p.url.clone(),
                    if p.is_active { "Yes" } else { "-" }.to_string(),
                ]
            })
            .collect();

        output.push_str(&markdown_table(headers, rows));
        output
    }
}

/// Result of profile delete command.
#[derive(Debug, Clone, Serialize)]
pub struct ProfileDeleted {
    pub name: String,
}

impl MarkdownOutput for ProfileDeleted {
    fn to_markdown(&self, _meta: &Meta) -> String {
        format!(
            "## Profile Deleted\n\nProfile **{}** has been removed.\n",
            self.name
        )
    }
}

/// Result of config show command.
#[derive(Debug, Clone, Serialize)]
pub struct ConfigInfo {
    pub url: String,
    pub api_key_redacted: String,
    pub source: String,
    pub profile_name: Option<String>,
}

impl MarkdownOutput for ConfigInfo {
    fn to_markdown(&self, _meta: &Meta) -> String {
        let mut output = String::new();
        output.push_str("## Current Configuration\n\n");
        output.push_str(&format!("- **URL**: {}\n", self.url));
        output.push_str(&format!("- **API Key**: {}\n", self.api_key_redacted));
        output.push_str(&format!("- **Source**: {}\n", self.source));
        if let Some(name) = &self.profile_name {
            output.push_str(&format!("- **Profile**: {}\n", name));
        }
        output
    }
}

/// Execute profile add command.
pub fn add_profile(args: &ProfileAdd, paths: &ConfigPaths) -> Result<ProfileAdded> {
    let mut store = ProfileStore::load(&paths.config_file)?;
    let is_first = store.profiles.is_empty();

    store.add(Profile::new(&args.name, &args.url, &args.api_key));
    store.save(&paths.config_file)?;

    Ok(ProfileAdded {
        name: args.name.clone(),
        url: args.url.clone(),
        is_active: is_first,
    })
}

/// Execute profile use command.
pub fn use_profile(args: &ProfileUse, paths: &ConfigPaths) -> Result<ProfileActivated> {
    let mut store = ProfileStore::load(&paths.config_file)?;
    store.set_active(&args.name)?;
    store.save(&paths.config_file)?;

    Ok(ProfileActivated {
        name: args.name.clone(),
    })
}

/// Execute profile list command.
pub fn list_profiles(paths: &ConfigPaths) -> Result<ProfileList> {
    let store = ProfileStore::load(&paths.config_file)?;

    let profiles = store
        .profiles
        .values()
        .map(|p| ProfileInfo {
            name: p.name.clone(),
            url: p.url.clone(),
            is_active: store.active.as_ref() == Some(&p.name),
        })
        .collect();

    Ok(ProfileList {
        profiles,
        active: store.active.clone(),
    })
}

/// Execute profile delete command.
pub fn delete_profile(args: &ProfileDelete, paths: &ConfigPaths) -> Result<ProfileDeleted> {
    let mut store = ProfileStore::load(&paths.config_file)?;
    store.delete(&args.name)?;
    store.save(&paths.config_file)?;

    Ok(ProfileDeleted {
        name: args.name.clone(),
    })
}

/// Execute config show command.
pub fn show_config(config: &Config) -> ConfigInfo {
    let source = if config.profile_name.is_some() {
        "config file"
    } else if std::env::var("REDMINE_URL").is_ok() {
        "environment variables"
    } else {
        "CLI flags"
    };

    ConfigInfo {
        url: config.url.clone(),
        api_key_redacted: config.redacted_api_key(),
        source: source.to_string(),
        profile_name: config.profile_name.clone(),
    }
}
