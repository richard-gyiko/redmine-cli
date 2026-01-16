//! User model.

use crate::output::{markdown::markdown_kv_table, MarkdownOutput, Meta};
use serde::{Deserialize, Serialize};

/// User reference (embedded in other objects).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    #[serde(default)]
    pub login: Option<String>,
}

/// Current user response from /users/current.json.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentUser {
    pub id: u32,
    pub login: String,
    pub firstname: String,
    pub lastname: String,
    #[serde(default)]
    pub mail: Option<String>,
    #[serde(default)]
    pub admin: Option<bool>,
    #[serde(default)]
    pub created_on: Option<String>,
    #[serde(default)]
    pub last_login_on: Option<String>,
}

impl CurrentUser {
    /// Get the full name.
    pub fn full_name(&self) -> String {
        format!("{} {}", self.firstname, self.lastname)
    }
}

/// Wrapper for API response.
#[derive(Debug, Deserialize)]
pub struct CurrentUserResponse {
    pub user: CurrentUser,
}

impl MarkdownOutput for CurrentUser {
    fn to_markdown(&self, _meta: &Meta) -> String {
        let mut output = String::new();
        output.push_str(&format!("## Current User: {}\n\n", self.full_name()));

        let mut pairs = vec![
            ("ID", self.id.to_string()),
            ("Login", self.login.clone()),
            ("Name", self.full_name()),
        ];

        if let Some(mail) = &self.mail {
            pairs.push(("Email", mail.clone()));
        }

        if let Some(admin) = self.admin {
            pairs.push(("Admin", if admin { "Yes" } else { "No" }.to_string()));
        }

        if let Some(created) = &self.created_on {
            pairs.push(("Created", created.clone()));
        }

        if let Some(last_login) = &self.last_login_on {
            pairs.push(("Last Login", last_login.clone()));
        }

        let pairs_ref: Vec<(&str, String)> = pairs.iter().map(|(k, v)| (*k, v.clone())).collect();
        output.push_str(&markdown_kv_table(&pairs_ref));

        output
    }
}
