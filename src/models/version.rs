//! Version (target version / milestone) models.

use super::project::ProjectRef;
use crate::output::{markdown::markdown_table, MarkdownOutput, Meta};
use serde::{Deserialize, Serialize};

/// Minimal version reference (embedded in issues as `fixed_version`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionRef {
    pub id: u32,
    pub name: String,
}

/// Version from Redmine API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    pub id: u32,
    pub name: String,
    /// Owning project (may differ from the queried project for shared versions).
    #[serde(default)]
    pub project: Option<ProjectRef>,
    #[serde(default)]
    pub description: Option<String>,
    /// `open`, `locked` or `closed`.
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub due_date: Option<String>,
    #[serde(default)]
    pub sharing: Option<String>,
    #[serde(default)]
    pub created_on: Option<String>,
    #[serde(default)]
    pub updated_on: Option<String>,
}

/// List of versions from `/projects/{id}/versions.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionList {
    pub versions: Vec<Version>,
    #[serde(default)]
    pub total_count: Option<u32>,
}

impl VersionList {
    /// Resolve a version by ID or name (case-insensitive).
    pub fn resolve(&self, id_or_name: &str) -> Option<&Version> {
        if let Ok(id) = id_or_name.parse::<u32>() {
            if let Some(v) = self.versions.iter().find(|v| v.id == id) {
                return Some(v);
            }
        }
        let needle = id_or_name.to_lowercase();
        self.versions
            .iter()
            .find(|v| v.name.to_lowercase() == needle)
    }
}

impl MarkdownOutput for VersionList {
    fn to_markdown(&self, _meta: &Meta) -> String {
        let mut output = String::new();
        let total = self.total_count.unwrap_or(self.versions.len() as u32);

        output.push_str(&format!("## Versions ({})\n\n", total));

        if self.versions.is_empty() {
            output.push_str("*No versions found*\n");
            return output;
        }

        let headers = &["ID", "Name", "Status", "Due Date", "Project"];
        let rows: Vec<Vec<String>> = self
            .versions
            .iter()
            .map(|v| {
                vec![
                    v.id.to_string(),
                    v.name.clone(),
                    v.status.clone().unwrap_or_else(|| "-".to_string()),
                    v.due_date.clone().unwrap_or_else(|| "-".to_string()),
                    v.project
                        .as_ref()
                        .map(|p| p.name.clone())
                        .unwrap_or_else(|| "-".to_string()),
                ]
            })
            .collect();

        output.push_str(&markdown_table(headers, rows));
        output.push_str(
            "\n*Filter issues with `rdm issue list --project <ID> --version <ID|NAME>`*\n",
        );
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list() -> VersionList {
        serde_json::from_value(serde_json::json!({
            "versions": [
                {"id": 116, "name": "Milestone 1", "status": "open"},
                {"id": 117, "name": "Milestone 2", "status": "closed"}
            ],
            "total_count": 2
        }))
        .unwrap()
    }

    #[test]
    fn resolve_by_id() {
        assert_eq!(list().resolve("117").map(|v| v.id), Some(117));
    }

    #[test]
    fn resolve_by_name_case_insensitive() {
        assert_eq!(list().resolve("milestone 1").map(|v| v.id), Some(116));
    }

    #[test]
    fn resolve_unknown() {
        assert!(list().resolve("Milestone 3").is_none());
        assert!(list().resolve("999").is_none());
    }
}
