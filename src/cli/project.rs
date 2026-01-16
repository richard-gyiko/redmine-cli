//! Project commands.

use clap::{Args, Subcommand};

use crate::client::RedmineClient;
use crate::error::Result;
use crate::models::{Project, ProjectList};

#[derive(Debug, Subcommand)]
pub enum ProjectCommand {
    /// List projects.
    List(ProjectListArgs),
    /// Get project details.
    Get(ProjectGetArgs),
}

#[derive(Debug, Args)]
pub struct ProjectListArgs {
    /// Maximum number of results.
    #[arg(long, default_value = "25")]
    pub limit: u32,
    /// Offset for pagination.
    #[arg(long, default_value = "0")]
    pub offset: u32,
}

#[derive(Debug, Args)]
pub struct ProjectGetArgs {
    /// Project ID.
    #[arg(long, conflicts_with = "identifier")]
    pub id: Option<u32>,
    /// Project identifier (slug).
    #[arg(long, conflicts_with = "id")]
    pub identifier: Option<String>,
}

/// Execute project list command.
pub async fn list(client: &RedmineClient, args: &ProjectListArgs) -> Result<ProjectList> {
    client.list_projects(args.limit, args.offset).await
}

/// Execute project get command.
pub async fn get(client: &RedmineClient, args: &ProjectGetArgs) -> Result<Project> {
    let id_or_identifier = if let Some(id) = args.id {
        id.to_string()
    } else if let Some(identifier) = &args.identifier {
        identifier.clone()
    } else {
        return Err(crate::error::AppError::validation_with_hint(
            "Either --id or --identifier is required",
            "Use `rma project get --id 1` or `rma project get --identifier my-project`",
        ));
    };

    client.get_project(&id_or_identifier).await
}
