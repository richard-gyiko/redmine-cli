//! User commands.

use clap::{Args, Subcommand, ValueEnum};
use serde::Serialize;

use crate::client::RedmineClient;
use crate::error::Result;
use crate::output::{
    markdown::{markdown_table, pagination_hint},
    MarkdownOutput, Meta,
};

#[derive(Debug, Subcommand)]
pub enum UserCommand {
    /// List users.
    List(UserListArgs),
    /// Get current user info (alias for 'rdm me').
    Me,
}

#[derive(Debug, Args)]
pub struct UserListArgs {
    /// Filter by status (active, registered, locked).
    #[arg(long, value_enum)]
    pub status: Option<UserStatus>,
    /// Maximum number of results.
    #[arg(long, default_value = "25")]
    pub limit: u32,
    /// Offset for pagination.
    #[arg(long, default_value = "0")]
    pub offset: u32,
}

/// User status filter.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum UserStatus {
    /// Active users (status=1).
    Active,
    /// Registered but not activated (status=2).
    Registered,
    /// Locked users (status=3).
    Locked,
}

impl UserStatus {
    /// Get the numeric status value for the API.
    pub fn as_api_value(&self) -> u32 {
        match self {
            Self::Active => 1,
            Self::Registered => 2,
            Self::Locked => 3,
        }
    }
}

/// Full user details from /users.json endpoint.
#[derive(Debug, Clone, Serialize, serde::Deserialize)]
pub struct UserDetails {
    pub id: u32,
    pub login: String,
    pub firstname: String,
    pub lastname: String,
    #[serde(default)]
    pub mail: Option<String>,
    #[serde(default)]
    pub created_on: Option<String>,
    #[serde(default)]
    pub last_login_on: Option<String>,
    #[serde(default)]
    pub status: Option<u32>,
}

impl UserDetails {
    /// Get the full name.
    pub fn full_name(&self) -> String {
        format!("{} {}", self.firstname, self.lastname)
    }

    /// Get status as a display string.
    pub fn status_display(&self) -> &'static str {
        match self.status {
            Some(1) => "Active",
            Some(2) => "Registered",
            Some(3) => "Locked",
            _ => "Unknown",
        }
    }
}

/// List of users from API.
#[derive(Debug, Clone, Serialize, serde::Deserialize)]
pub struct UserList {
    pub users: Vec<UserDetails>,
    #[serde(default)]
    pub total_count: Option<u32>,
    #[serde(default)]
    pub offset: Option<u32>,
    #[serde(default)]
    pub limit: Option<u32>,
}

impl MarkdownOutput for UserList {
    fn to_markdown(&self, meta: &Meta) -> String {
        let mut output = String::new();

        let total = meta.total_count.unwrap_or(self.users.len() as u32);
        let offset = meta.offset.unwrap_or(0);
        let showing_end = offset + self.users.len() as u32;

        output.push_str(&format!(
            "## Users (showing {}-{} of {})\n\n",
            offset + 1,
            showing_end,
            total
        ));

        if self.users.is_empty() {
            output.push_str("*No users found*\n");
            return output;
        }

        let headers = &["ID", "Login", "Name", "Email", "Status"];
        let rows: Vec<Vec<String>> = self
            .users
            .iter()
            .map(|u| {
                vec![
                    u.id.to_string(),
                    u.login.clone(),
                    u.full_name(),
                    u.mail.clone().unwrap_or_else(|| "-".to_string()),
                    u.status_display().to_string(),
                ]
            })
            .collect();

        output.push_str(&markdown_table(headers, rows));

        if let Some(hint) = pagination_hint("rdm user list ", meta) {
            output.push('\n');
            output.push_str(&hint);
            output.push('\n');
        }

        output
    }
}

/// Execute user list command.
pub async fn list(client: &RedmineClient, args: &UserListArgs) -> Result<UserList> {
    client
        .list_users(
            args.status.map(|s| s.as_api_value()),
            args.limit,
            args.offset,
        )
        .await
}
