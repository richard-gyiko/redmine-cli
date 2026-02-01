//! Issue model with related types.

use super::custom_field::{CustomField, CustomFieldValue};
use super::project::ProjectRef;
use super::user::User;
use crate::output::{
    markdown::{markdown_kv_table, markdown_table, pagination_hint},
    MarkdownOutput, Meta,
};
use serde::{Deserialize, Serialize};

/// Tracker (Bug, Feature, etc).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tracker {
    pub id: u32,
    pub name: String,
}

/// Issue status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Status {
    pub id: u32,
    pub name: String,
    #[serde(default)]
    pub is_closed: Option<bool>,
}

/// Issue priority.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Priority {
    pub id: u32,
    pub name: String,
}

/// Issue from Redmine API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: u32,
    pub subject: String,
    #[serde(default)]
    pub description: Option<String>,
    pub project: ProjectRef,
    #[serde(default)]
    pub tracker: Option<Tracker>,
    pub status: Status,
    pub priority: Priority,
    #[serde(default)]
    pub author: Option<User>,
    #[serde(default)]
    pub assigned_to: Option<User>,
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub due_date: Option<String>,
    #[serde(default)]
    pub done_ratio: Option<u32>,
    #[serde(default)]
    pub estimated_hours: Option<f64>,
    #[serde(default)]
    pub spent_hours: Option<f64>,
    #[serde(default)]
    pub created_on: Option<String>,
    #[serde(default)]
    pub updated_on: Option<String>,
    #[serde(default)]
    pub custom_fields: Option<Vec<CustomField>>,
}

/// List of issues from API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueList {
    pub issues: Vec<Issue>,
    #[serde(default)]
    pub total_count: Option<u32>,
    #[serde(default)]
    pub offset: Option<u32>,
    #[serde(default)]
    pub limit: Option<u32>,
}

/// Wrapper for single issue response.
#[derive(Debug, Deserialize)]
pub struct IssueResponse {
    pub issue: Issue,
}

/// New issue creation request.
#[derive(Debug, Clone, Serialize)]
pub struct NewIssue {
    pub project_id: u32,
    pub subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracker_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_hours: Option<f64>,
    /// Custom field values for the issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldValue>>,
}

/// Wrapper for issue creation request.
#[derive(Debug, Serialize)]
pub struct NewIssueRequest {
    pub issue: NewIssue,
}

/// Issue update request.
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateIssue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracker_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_hours: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done_ratio: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Custom field values to update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldValue>>,
}

/// Wrapper for issue update request.
#[derive(Debug, Serialize)]
pub struct UpdateIssueRequest {
    pub issue: UpdateIssue,
}

impl MarkdownOutput for Issue {
    fn to_markdown(&self, _meta: &Meta) -> String {
        let mut output = String::new();
        output.push_str(&format!("## Issue #{}: {}\n\n", self.id, self.subject));

        let mut pairs = vec![
            ("ID", self.id.to_string()),
            ("Subject", self.subject.clone()),
            ("Project", self.project.name.clone()),
            ("Status", self.status.name.clone()),
            ("Priority", self.priority.name.clone()),
        ];

        if let Some(tracker) = &self.tracker {
            pairs.push(("Tracker", tracker.name.clone()));
        }

        if let Some(assignee) = &self.assigned_to {
            pairs.push(("Assignee", assignee.name.clone()));
        }

        if let Some(author) = &self.author {
            pairs.push(("Author", author.name.clone()));
        }

        if let Some(start) = &self.start_date {
            pairs.push(("Start Date", start.clone()));
        }

        if let Some(due) = &self.due_date {
            pairs.push(("Due Date", due.clone()));
        }

        if let Some(done) = self.done_ratio {
            pairs.push(("Done", format!("{}%", done)));
        }

        if let Some(estimated) = self.estimated_hours {
            pairs.push(("Estimated", format!("{:.2}h", estimated)));
        }

        if let Some(spent) = self.spent_hours {
            pairs.push(("Spent", format!("{:.2}h", spent)));
        }

        if let Some(created) = &self.created_on {
            pairs.push(("Created", created.clone()));
        }

        if let Some(updated) = &self.updated_on {
            pairs.push(("Updated", updated.clone()));
        }

        let pairs_ref: Vec<(&str, String)> = pairs.iter().map(|(k, v)| (*k, v.clone())).collect();
        output.push_str(&markdown_kv_table(&pairs_ref));

        // Display custom fields if present
        if let Some(custom_fields) = &self.custom_fields {
            if !custom_fields.is_empty() {
                output.push_str("\n### Custom Fields\n\n");
                let cf_pairs: Vec<(&str, String)> = custom_fields
                    .iter()
                    .map(|cf| (cf.name.as_str(), cf.display_value()))
                    .collect();
                output.push_str(&markdown_kv_table(&cf_pairs));
            }
        }

        if let Some(desc) = &self.description {
            if !desc.is_empty() {
                output.push_str("\n### Description\n\n");
                output.push_str(desc);
                output.push('\n');
            }
        }

        output.push_str(&format!(
            "\n*Use `rdm issue update --id {}` to modify this issue*\n",
            self.id
        ));

        output
    }
}

impl MarkdownOutput for IssueList {
    fn to_markdown(&self, meta: &Meta) -> String {
        let mut output = String::new();

        let total = meta.total_count.unwrap_or(self.issues.len() as u32);
        let offset = meta.offset.unwrap_or(0);
        let showing_end = offset + self.issues.len() as u32;

        output.push_str(&format!(
            "## Issues (showing {}-{} of {})\n\n",
            offset + 1,
            showing_end,
            total
        ));

        if self.issues.is_empty() {
            output.push_str("*No issues found*\n");
            return output;
        }

        let headers = &["ID", "Subject", "Status", "Priority", "Assignee", "Updated"];
        let rows: Vec<Vec<String>> = self
            .issues
            .iter()
            .map(|i| {
                vec![
                    i.id.to_string(),
                    truncate(&i.subject, 40),
                    i.status.name.clone(),
                    i.priority.name.clone(),
                    i.assigned_to
                        .as_ref()
                        .map(|u| u.name.clone())
                        .unwrap_or_else(|| "-".to_string()),
                    i.updated_on.clone().unwrap_or_else(|| "-".to_string()),
                ]
            })
            .collect();

        output.push_str(&markdown_table(headers, rows));

        if let Some(hint) = pagination_hint("rdm issue list ", meta) {
            output.push('\n');
            output.push_str(&hint);
            output.push('\n');
        }

        output
    }
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

/// Search result from Redmine search API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: u32,
    pub title: String,
    #[serde(rename = "type")]
    pub result_type: String,
    pub url: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub datetime: Option<String>,
}

/// Search results response from API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    pub results: Vec<SearchResult>,
    #[serde(default)]
    pub total_count: Option<u32>,
    #[serde(default)]
    pub offset: Option<u32>,
    #[serde(default)]
    pub limit: Option<u32>,
}
