//! Time entry model with activity caching.

use super::custom_field::CustomField;
use super::project::ProjectRef;
use super::user::User;
use crate::output::{
    markdown::{markdown_kv_table, markdown_table, pagination_hint},
    MarkdownOutput, Meta,
};
use serde::{Deserialize, Serialize};

/// Activity type for time entries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub id: u32,
    pub name: String,
    #[serde(default)]
    pub is_default: Option<bool>,
}

/// List of activities from API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityList {
    pub time_entry_activities: Vec<Activity>,
}

/// Time entry from Redmine API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeEntry {
    pub id: u32,
    pub hours: f64,
    #[serde(default)]
    pub comments: Option<String>,
    pub spent_on: String,
    pub activity: Activity,
    #[serde(default)]
    pub user: Option<User>,
    #[serde(default)]
    pub project: Option<ProjectRef>,
    #[serde(default)]
    pub issue: Option<TimeEntryIssue>,
    #[serde(default)]
    pub created_on: Option<String>,
    #[serde(default)]
    pub updated_on: Option<String>,
    #[serde(default)]
    pub custom_fields: Option<Vec<CustomField>>,
}

/// Simplified issue reference in time entries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeEntryIssue {
    pub id: u32,
}

/// List of time entries from API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeEntryList {
    pub time_entries: Vec<TimeEntry>,
    #[serde(default)]
    pub total_count: Option<u32>,
    #[serde(default)]
    pub offset: Option<u32>,
    #[serde(default)]
    pub limit: Option<u32>,
}

/// Wrapper for single time entry response.
#[derive(Debug, Deserialize)]
pub struct TimeEntryResponse {
    pub time_entry: TimeEntry,
}

/// New time entry creation request.
#[derive(Debug, Clone, Serialize)]
pub struct NewTimeEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<u32>,
    pub hours: f64,
    pub activity_id: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spent_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<u32>,
}

/// Wrapper for time entry creation request.
#[derive(Debug, Serialize)]
pub struct NewTimeEntryRequest {
    pub time_entry: NewTimeEntry,
}

/// Time entry update request.
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateTimeEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spent_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}

/// Wrapper for time entry update request.
#[derive(Debug, Serialize)]
pub struct UpdateTimeEntryRequest {
    pub time_entry: UpdateTimeEntry,
}

impl MarkdownOutput for Activity {
    fn to_markdown(&self, _meta: &Meta) -> String {
        let default_marker = if self.is_default.unwrap_or(false) {
            " (default)"
        } else {
            ""
        };
        format!("- **{}** (ID: {}){}\n", self.name, self.id, default_marker)
    }
}

impl MarkdownOutput for ActivityList {
    fn to_markdown(&self, _meta: &Meta) -> String {
        let mut output = String::new();
        output.push_str("## Time Entry Activities\n\n");

        if self.time_entry_activities.is_empty() {
            output.push_str("*No activities found*\n");
            return output;
        }

        let headers = &["ID", "Name", "Default"];
        let rows: Vec<Vec<String>> = self
            .time_entry_activities
            .iter()
            .map(|a| {
                vec![
                    a.id.to_string(),
                    a.name.clone(),
                    if a.is_default.unwrap_or(false) {
                        "Yes"
                    } else {
                        "-"
                    }
                    .to_string(),
                ]
            })
            .collect();

        output.push_str(&markdown_table(headers, rows));
        output
            .push_str("\n*Use activity name or ID with `rdm time create --activity <name|id>`*\n");

        output
    }
}

impl MarkdownOutput for TimeEntry {
    fn to_markdown(&self, _meta: &Meta) -> String {
        let mut output = String::new();
        output.push_str(&format!("## Time Entry #{}\n\n", self.id));

        let mut pairs = vec![
            ("ID", self.id.to_string()),
            ("Hours", format!("{:.2}", self.hours)),
            ("Activity", self.activity.name.clone()),
            ("Date", self.spent_on.clone()),
        ];

        if let Some(issue) = &self.issue {
            pairs.push(("Issue", format!("#{}", issue.id)));
        }

        if let Some(project) = &self.project {
            pairs.push(("Project", project.name.clone()));
        }

        if let Some(user) = &self.user {
            pairs.push(("User", user.name.clone()));
        }

        if let Some(comments) = &self.comments {
            if !comments.is_empty() {
                pairs.push(("Comment", comments.clone()));
            }
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

        output.push_str(&format!(
            "\n*Use `rdm time update --id {}` to modify or `rdm time delete --id {}` to remove*\n",
            self.id, self.id
        ));

        output
    }
}

impl MarkdownOutput for TimeEntryList {
    fn to_markdown(&self, meta: &Meta) -> String {
        let mut output = String::new();

        let total = meta.total_count.unwrap_or(self.time_entries.len() as u32);
        let offset = meta.offset.unwrap_or(0);
        let showing_end = offset + self.time_entries.len() as u32;

        output.push_str(&format!(
            "## Time Entries (showing {}-{} of {})\n\n",
            offset + 1,
            showing_end,
            total
        ));

        if self.time_entries.is_empty() {
            output.push_str("*No time entries found*\n");
            return output;
        }

        // Calculate total hours
        let total_hours: f64 = self.time_entries.iter().map(|t| t.hours).sum();

        let headers = &[
            "ID", "Date", "Hours", "User", "Activity", "Issue", "Comment",
        ];
        let rows: Vec<Vec<String>> = self
            .time_entries
            .iter()
            .map(|t| {
                vec![
                    t.id.to_string(),
                    t.spent_on.clone(),
                    format!("{:.2}", t.hours),
                    t.user
                        .as_ref()
                        .map(|u| truncate_name(&u.name, 15))
                        .unwrap_or_else(|| "-".to_string()),
                    t.activity.name.clone(),
                    t.issue
                        .as_ref()
                        .map(|i| format!("#{}", i.id))
                        .unwrap_or_else(|| "-".to_string()),
                    truncate_comment(t.comments.as_deref().unwrap_or("-")),
                ]
            })
            .collect();

        output.push_str(&markdown_table(headers, rows));
        output.push_str(&format!("\n**Total: {:.2} hours**\n", total_hours));

        if let Some(hint) = pagination_hint("rdm time list ", meta) {
            output.push('\n');
            output.push_str(&hint);
            output.push('\n');
        }

        output
    }
}

fn truncate_comment(s: &str) -> String {
    let s = s.replace('\n', " ");
    if s.chars().count() <= 30 {
        s
    } else {
        let truncated: String = s.chars().take(27).collect();
        format!("{}...", truncated)
    }
}

fn truncate_name(s: &str, max_len: usize) -> String {
    if s.chars().count() <= max_len {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max_len - 3).collect();
        format!("{}...", truncated)
    }
}

/// Message for successful time entry creation.
#[derive(Debug, Clone, Serialize)]
pub struct TimeEntryCreated {
    pub time_entry: TimeEntry,
}

impl MarkdownOutput for TimeEntryCreated {
    fn to_markdown(&self, _meta: &Meta) -> String {
        let t = &self.time_entry;
        let mut output = String::new();
        output.push_str("## Time Entry Created\n\n");

        let mut pairs = vec![
            ("ID", t.id.to_string()),
            ("Hours", format!("{:.2}", t.hours)),
            ("Activity", t.activity.name.clone()),
            ("Date", t.spent_on.clone()),
        ];

        if let Some(issue) = &t.issue {
            pairs.push(("Issue", format!("#{}", issue.id)));
        }

        if let Some(project) = &t.project {
            pairs.push(("Project", project.name.clone()));
        }

        if let Some(comments) = &t.comments {
            if !comments.is_empty() {
                pairs.push(("Comment", comments.clone()));
            }
        }

        let pairs_ref: Vec<(&str, String)> = pairs.iter().map(|(k, v)| (*k, v.clone())).collect();
        output.push_str(&markdown_kv_table(&pairs_ref));

        output.push_str(&format!(
            "\n*Use `rdm time get --id {}` to view details*\n",
            t.id
        ));

        output
    }
}

/// Message for successful time entry update.
#[derive(Debug, Clone, Serialize)]
pub struct TimeEntryUpdated {
    pub time_entry: TimeEntry,
}

impl MarkdownOutput for TimeEntryUpdated {
    fn to_markdown(&self, _meta: &Meta) -> String {
        let t = &self.time_entry;
        let mut output = String::new();
        output.push_str("## Time Entry Updated\n\n");

        let pairs = [
            ("ID", t.id.to_string()),
            ("Hours", format!("{:.2}", t.hours)),
            ("Activity", t.activity.name.clone()),
            ("Date", t.spent_on.clone()),
        ];

        let pairs_ref: Vec<(&str, String)> = pairs.iter().map(|(k, v)| (*k, v.clone())).collect();
        output.push_str(&markdown_kv_table(&pairs_ref));

        output
    }
}

/// Message for successful time entry deletion.
#[derive(Debug, Clone, Serialize)]
pub struct TimeEntryDeleted {
    pub id: u32,
}

impl MarkdownOutput for TimeEntryDeleted {
    fn to_markdown(&self, _meta: &Meta) -> String {
        format!(
            "## Time Entry Deleted\n\nTime entry #{} has been deleted.\n",
            self.id
        )
    }
}

/// Field to group time entries by.
#[derive(Debug, Clone)]
pub enum GroupByField {
    User,
    Project,
    Activity,
    Issue,
    SpentOn,
    CustomField(u32),
}

impl GroupByField {
    /// Parse a group-by field from string.
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "user" => Some(Self::User),
            "project" => Some(Self::Project),
            "activity" => Some(Self::Activity),
            "issue" => Some(Self::Issue),
            "spent_on" | "date" => Some(Self::SpentOn),
            _ if s.starts_with("cf_") => s[3..].parse().ok().map(Self::CustomField),
            _ => None,
        }
    }

    /// Get the display name for this field.
    pub fn display_name(&self) -> String {
        match self {
            Self::User => "User".to_string(),
            Self::Project => "Project".to_string(),
            Self::Activity => "Activity".to_string(),
            Self::Issue => "Issue".to_string(),
            Self::SpentOn => "Date".to_string(),
            Self::CustomField(id) => format!("Custom Field {}", id),
        }
    }
}

/// A group of time entries with a name and subtotal.
#[derive(Debug, Clone, Serialize)]
pub struct TimeEntryGroup {
    pub name: String,
    pub entries: Vec<TimeEntry>,
    pub subtotal: f64,
}

/// Grouped time entries for display.
#[derive(Debug, Clone, Serialize)]
pub struct GroupedTimeEntries {
    pub group_by: String,
    pub groups: Vec<TimeEntryGroup>,
    pub total_hours: f64,
    pub total_count: u32,
}

impl GroupedTimeEntries {
    /// Create grouped time entries from a list.
    pub fn from_entries(entries: Vec<TimeEntry>, field: &GroupByField) -> Self {
        use std::collections::BTreeMap;

        let mut groups_map: BTreeMap<String, Vec<TimeEntry>> = BTreeMap::new();

        for entry in entries {
            let key = match field {
                GroupByField::User => entry
                    .user
                    .as_ref()
                    .map(|u| u.name.clone())
                    .unwrap_or_else(|| "Unknown".to_string()),
                GroupByField::Project => entry
                    .project
                    .as_ref()
                    .map(|p| p.name.clone())
                    .unwrap_or_else(|| "Unknown".to_string()),
                GroupByField::Activity => entry.activity.name.clone(),
                GroupByField::Issue => entry
                    .issue
                    .as_ref()
                    .map(|i| format!("#{}", i.id))
                    .unwrap_or_else(|| "No Issue".to_string()),
                GroupByField::SpentOn => entry.spent_on.clone(),
                GroupByField::CustomField(cf_id) => entry
                    .custom_fields
                    .as_ref()
                    .and_then(|cfs| cfs.iter().find(|cf| cf.id == *cf_id))
                    .map(|cf| cf.display_value())
                    .unwrap_or_else(|| "-".to_string()),
            };

            groups_map.entry(key).or_default().push(entry);
        }

        let mut total_hours = 0.0;
        let mut total_count = 0u32;
        let groups: Vec<TimeEntryGroup> = groups_map
            .into_iter()
            .map(|(name, entries)| {
                let subtotal: f64 = entries.iter().map(|e| e.hours).sum();
                total_hours += subtotal;
                total_count += entries.len() as u32;
                TimeEntryGroup {
                    name,
                    entries,
                    subtotal,
                }
            })
            .collect();

        Self {
            group_by: field.display_name(),
            groups,
            total_hours,
            total_count,
        }
    }
}

impl MarkdownOutput for GroupedTimeEntries {
    fn to_markdown(&self, _meta: &Meta) -> String {
        let mut output = String::new();
        output.push_str(&format!(
            "## Time Entries by {} ({} entries)\n\n",
            self.group_by, self.total_count
        ));

        if self.groups.is_empty() {
            output.push_str("*No time entries found*\n");
            return output;
        }

        for group in &self.groups {
            output.push_str(&format!(
                "### {} ({:.2} hours)\n\n",
                group.name, group.subtotal
            ));

            let headers = &[
                "ID", "Date", "Hours", "User", "Activity", "Issue", "Comment",
            ];
            let rows: Vec<Vec<String>> = group
                .entries
                .iter()
                .map(|t| {
                    vec![
                        t.id.to_string(),
                        t.spent_on.clone(),
                        format!("{:.2}", t.hours),
                        t.user
                            .as_ref()
                            .map(|u| truncate_name(&u.name, 15))
                            .unwrap_or_else(|| "-".to_string()),
                        t.activity.name.clone(),
                        t.issue
                            .as_ref()
                            .map(|i| format!("#{}", i.id))
                            .unwrap_or_else(|| "-".to_string()),
                        truncate_comment(t.comments.as_deref().unwrap_or("-")),
                    ]
                })
                .collect();

            output.push_str(&markdown_table(headers, rows));
            output.push('\n');
        }

        output.push_str(&format!("**Grand Total: {:.2} hours**\n", self.total_hours));

        output
    }
}
