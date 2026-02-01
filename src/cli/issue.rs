//! Issue commands.

use clap::{Args, Subcommand};
use serde::Serialize;

use super::parse_custom_fields;
use crate::client::{endpoints::IssueFilters, RedmineClient};
use crate::error::Result;
use crate::models::{CustomFieldValue, Issue, IssueList, NewIssue, UpdateIssue};
use crate::output::{markdown::markdown_kv_table, MarkdownOutput, Meta};

#[derive(Debug, Subcommand)]
pub enum IssueCommand {
    /// List issues.
    List(IssueListArgs),
    /// Get issue details.
    Get(IssueGetArgs),
    /// Create a new issue.
    Create(IssueCreateArgs),
    /// Update an issue.
    Update(IssueUpdateArgs),
}

#[derive(Debug, Args)]
pub struct IssueListArgs {
    /// Filter by project (ID or identifier).
    #[arg(long)]
    pub project: Option<String>,
    /// Filter by status (ID, "open", "closed", or "*").
    #[arg(long)]
    pub status: Option<String>,
    /// Filter by assignee (ID or "me").
    #[arg(long)]
    pub assigned_to: Option<String>,
    /// Filter by author (ID or "me").
    #[arg(long)]
    pub author: Option<String>,
    /// Filter by tracker ID.
    #[arg(long)]
    pub tracker: Option<String>,
    /// Filter by exact subject match.
    #[arg(long)]
    pub subject: Option<String>,
    /// Search issues by text (searches subject and description).
    #[arg(long)]
    pub search: Option<String>,
    /// Filter by custom field value (format: id=value, repeatable).
    #[arg(long = "cf", value_name = "ID=VALUE")]
    pub custom_fields: Vec<String>,
    /// Maximum number of results.
    #[arg(long, default_value = "25")]
    pub limit: u32,
    /// Offset for pagination.
    #[arg(long, default_value = "0")]
    pub offset: u32,
}

#[derive(Debug, Args)]
pub struct IssueGetArgs {
    /// Issue ID.
    #[arg(long)]
    pub id: u32,
}

#[derive(Debug, Args)]
pub struct IssueCreateArgs {
    /// Project ID.
    #[arg(long)]
    pub project: u32,
    /// Issue subject.
    #[arg(long)]
    pub subject: String,
    /// Issue description.
    #[arg(long)]
    pub description: Option<String>,
    /// Tracker ID.
    #[arg(long)]
    pub tracker: Option<u32>,
    /// Status ID.
    #[arg(long)]
    pub status: Option<u32>,
    /// Priority ID.
    #[arg(long)]
    pub priority: Option<u32>,
    /// Assignee ID.
    #[arg(long)]
    pub assigned_to: Option<u32>,
    /// Start date (YYYY-MM-DD).
    #[arg(long)]
    pub start_date: Option<String>,
    /// Due date (YYYY-MM-DD).
    #[arg(long)]
    pub due_date: Option<String>,
    /// Estimated hours.
    #[arg(long)]
    pub estimated_hours: Option<f64>,
    /// Set custom field value (format: id=value, repeatable).
    #[arg(long = "cf", value_name = "ID=VALUE")]
    pub custom_fields: Vec<String>,
}

#[derive(Debug, Args)]
pub struct IssueUpdateArgs {
    /// Issue ID.
    #[arg(long)]
    pub id: u32,
    /// New subject.
    #[arg(long)]
    pub subject: Option<String>,
    /// New description.
    #[arg(long)]
    pub description: Option<String>,
    /// New status ID.
    #[arg(long)]
    pub status: Option<u32>,
    /// New priority ID.
    #[arg(long)]
    pub priority: Option<u32>,
    /// New assignee ID.
    #[arg(long)]
    pub assigned_to: Option<u32>,
    /// Done percentage (0-100).
    #[arg(long)]
    pub done_ratio: Option<u32>,
    /// Add a note/comment.
    #[arg(long)]
    pub notes: Option<String>,
    /// Set custom field value (format: id=value, repeatable).
    #[arg(long = "cf", value_name = "ID=VALUE")]
    pub custom_fields: Vec<String>,
}

/// Result of issue creation.
#[derive(Debug, Clone, Serialize)]
pub struct IssueCreated {
    pub issue: Issue,
}

impl MarkdownOutput for IssueCreated {
    fn to_markdown(&self, _meta: &Meta) -> String {
        let i = &self.issue;
        let mut output = String::new();
        output.push_str("## Issue Created\n\n");

        let pairs = [
            ("ID", i.id.to_string()),
            ("Subject", i.subject.clone()),
            ("Project", i.project.name.clone()),
            ("Status", i.status.name.clone()),
            ("Priority", i.priority.name.clone()),
        ];

        let pairs_ref: Vec<(&str, String)> = pairs.iter().map(|(k, v)| (*k, v.clone())).collect();
        output.push_str(&markdown_kv_table(&pairs_ref));

        output.push_str(&format!(
            "\n*Use `rdm issue get --id {}` to view full details*\n",
            i.id
        ));
        output
    }
}

/// Result of issue update.
#[derive(Debug, Clone, Serialize)]
pub struct IssueUpdated {
    pub id: u32,
}

impl MarkdownOutput for IssueUpdated {
    fn to_markdown(&self, _meta: &Meta) -> String {
        format!("## Issue Updated\n\nIssue #{} has been updated.\n\n*Use `rdm issue get --id {}` to view changes*\n", self.id, self.id)
    }
}

/// Parse custom field arguments into CustomFieldValue vec, or None if empty.
fn parse_custom_field_values(args: &[String]) -> Result<Option<Vec<CustomFieldValue>>> {
    if args.is_empty() {
        Ok(None)
    } else {
        let parsed = parse_custom_fields(args)?;
        Ok(Some(CustomFieldValue::from_tuples(parsed)))
    }
}

/// Execute issue list command.
pub async fn list(client: &RedmineClient, args: &IssueListArgs) -> Result<IssueList> {
    // Parse custom field filters
    let custom_fields = parse_custom_fields(&args.custom_fields)?;

    let filters = IssueFilters {
        project: args.project.clone(),
        status: args.status.clone(),
        assigned_to: args.assigned_to.clone(),
        author: args.author.clone(),
        tracker: args.tracker.clone(),
        subject: args.subject.clone(),
        custom_fields,
        limit: args.limit,
        offset: args.offset,
    };

    // If search is specified, use search endpoint instead
    if let Some(query) = &args.search {
        return client
            .search_issues(query, args.project.as_deref(), args.limit, args.offset)
            .await;
    }

    client.list_issues(filters).await
}

/// Execute issue get command.
pub async fn get(client: &RedmineClient, args: &IssueGetArgs) -> Result<Issue> {
    client.get_issue(args.id).await
}

/// Execute issue create command.
pub async fn create(client: &RedmineClient, args: &IssueCreateArgs) -> Result<IssueCreated> {
    let custom_fields = parse_custom_field_values(&args.custom_fields)?;

    let issue = NewIssue {
        project_id: args.project,
        subject: args.subject.clone(),
        description: args.description.clone(),
        tracker_id: args.tracker,
        status_id: args.status,
        priority_id: args.priority,
        assigned_to_id: args.assigned_to,
        start_date: args.start_date.clone(),
        due_date: args.due_date.clone(),
        estimated_hours: args.estimated_hours,
        custom_fields,
    };

    let created = client.create_issue(issue).await?;
    Ok(IssueCreated { issue: created })
}

/// Execute issue update command.
pub async fn update(client: &RedmineClient, args: &IssueUpdateArgs) -> Result<IssueUpdated> {
    let custom_fields = parse_custom_field_values(&args.custom_fields)?;

    let update = UpdateIssue {
        subject: args.subject.clone(),
        description: args.description.clone(),
        status_id: args.status,
        priority_id: args.priority,
        assigned_to_id: args.assigned_to,
        done_ratio: args.done_ratio,
        notes: args.notes.clone(),
        custom_fields,
        ..Default::default()
    };

    client.update_issue(args.id, update).await?;
    Ok(IssueUpdated { id: args.id })
}
