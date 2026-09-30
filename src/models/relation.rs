//! Issue relation models (`/issues/{id}/relations.json`, `/relations/{id}.json`).

use crate::output::{markdown::markdown_table, MarkdownOutput, Meta};
use serde::{Deserialize, Serialize};

/// Minimal issue reference (embedded in issues as `parent`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueRef {
    pub id: u32,
}

/// Relation between two issues, in Redmine's shape.
///
/// Redmine stores each relation once; from the `issue_to_id` side it is
/// returned unchanged, so readers must consider direction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub id: u32,
    pub issue_id: u32,
    pub issue_to_id: u32,
    pub relation_type: String,
    /// Delay in days (only for `precedes`/`follows`).
    #[serde(default)]
    pub delay: Option<i32>,
}

impl Relation {
    /// Human-readable description of this relation from `from_issue`'s side,
    /// e.g. `blocks #42` or `blocked by #7 (delay 2d)`.
    pub fn describe_from(&self, from_issue: u32) -> String {
        let t = self.relation_type.as_str();
        let (label, other) = if self.issue_id == from_issue || self.issue_to_id != from_issue {
            (forward_label(t), self.issue_to_id)
        } else {
            (inverse_label(t), self.issue_id)
        };
        let mut s = format!("{} #{}", label, other);
        if let Some(d) = self.delay {
            s.push_str(&format!(" (delay {}d)", d));
        }
        s
    }
}

fn forward_label(t: &str) -> String {
    match t {
        "relates" => "related to",
        "duplicates" => "duplicates",
        "duplicated" => "duplicated by",
        "blocks" => "blocks",
        "blocked" => "blocked by",
        "precedes" => "precedes",
        "follows" => "follows",
        "copied_to" => "copied to",
        "copied_from" => "copied from",
        other => other,
    }
    .to_string()
}

fn inverse_label(t: &str) -> String {
    match t {
        "relates" => "related to",
        "duplicates" => "duplicated by",
        "duplicated" => "duplicates",
        "blocks" => "blocked by",
        "blocked" => "blocks",
        "precedes" => "follows",
        "follows" => "precedes",
        "copied_to" => "copied from",
        "copied_from" => "copied to",
        other => other,
    }
    .to_string()
}

/// Response of `GET /issues/{id}/relations.json`.
#[derive(Debug, Deserialize)]
pub struct RelationsResponse {
    pub relations: Vec<Relation>,
}

/// Wrapper for single relation response.
#[derive(Debug, Deserialize)]
pub struct RelationResponse {
    pub relation: Relation,
}

/// New relation request body.
#[derive(Debug, Clone, Serialize)]
pub struct NewRelation {
    pub issue_to_id: u32,
    pub relation_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<i32>,
}

/// Wrapper for relation creation request.
#[derive(Debug, Serialize)]
pub struct NewRelationRequest {
    pub relation: NewRelation,
}

/// Result type for `rdm issue relation list`.
#[derive(Debug, Serialize)]
pub struct RelationList {
    pub issue_id: u32,
    pub relations: Vec<Relation>,
}

impl MarkdownOutput for RelationList {
    fn to_markdown(&self, _meta: &Meta) -> String {
        let mut output = String::new();
        output.push_str(&format!(
            "## Relations for Issue #{} ({})\n\n",
            self.issue_id,
            self.relations.len()
        ));

        if self.relations.is_empty() {
            output.push_str("*No relations.*\n");
            return output;
        }

        let headers = ["ID", "Relation", "Type", "From", "To", "Delay"];
        let rows: Vec<Vec<String>> = self
            .relations
            .iter()
            .map(|r| {
                vec![
                    r.id.to_string(),
                    r.describe_from(self.issue_id),
                    r.relation_type.clone(),
                    format!("#{}", r.issue_id),
                    format!("#{}", r.issue_to_id),
                    r.delay.map(|d| d.to_string()).unwrap_or_else(|| "-".into()),
                ]
            })
            .collect();

        output.push_str(&markdown_table(&headers, rows));
        output.push_str("\n*Use `rdm issue relation remove --relation-id <ID>` to remove*\n");
        output
    }
}

/// Result of `rdm issue relation add`.
#[derive(Debug, Serialize)]
pub struct RelationCreated {
    pub relation: Relation,
}

impl MarkdownOutput for RelationCreated {
    fn to_markdown(&self, _meta: &Meta) -> String {
        let r = &self.relation;
        format!(
            "## Relation Created\n\nRelation #{}: #{} {}.\n\n*Use `rdm issue relation list --id {}` to view relations*\n",
            r.id,
            r.issue_id,
            r.describe_from(r.issue_id),
            r.issue_id
        )
    }
}

/// Result of `rdm issue relation remove`.
#[derive(Debug, Serialize)]
pub struct RelationDeleted {
    pub relation_id: u32,
    pub deleted: bool,
}

impl MarkdownOutput for RelationDeleted {
    fn to_markdown(&self, _meta: &Meta) -> String {
        format!(
            "## Relation Removed\n\nRelation #{} has been removed.\n",
            self.relation_id
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rel(issue_id: u32, issue_to_id: u32, t: &str, delay: Option<i32>) -> Relation {
        Relation {
            id: 1,
            issue_id,
            issue_to_id,
            relation_type: t.into(),
            delay,
        }
    }

    #[test]
    fn describe_forward() {
        assert_eq!(rel(1, 2, "blocks", None).describe_from(1), "blocks #2");
        assert_eq!(
            rel(1, 2, "precedes", Some(3)).describe_from(1),
            "precedes #2 (delay 3d)"
        );
    }

    #[test]
    fn describe_inverse() {
        assert_eq!(rel(1, 2, "blocks", None).describe_from(2), "blocked by #1");
        assert_eq!(rel(1, 2, "precedes", None).describe_from(2), "follows #1");
        assert_eq!(
            rel(1, 2, "copied_to", None).describe_from(2),
            "copied from #1"
        );
        assert_eq!(rel(1, 2, "relates", None).describe_from(2), "related to #1");
    }

    #[test]
    fn delay_serializes_as_null_when_absent() {
        let v = serde_json::to_value(rel(1, 2, "relates", None)).unwrap();
        assert!(v["delay"].is_null());
        assert_eq!(v["relation_type"], "relates");
    }
}
