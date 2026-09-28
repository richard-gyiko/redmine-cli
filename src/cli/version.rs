//! Version (target version / milestone) commands.

use clap::{Args, Subcommand};

use crate::client::RedmineClient;
use crate::error::{AppError, Result};
use crate::models::VersionList;

#[derive(Debug, Subcommand)]
pub enum VersionCommand {
    /// List versions available to a project.
    List(VersionListArgs),
}

#[derive(Debug, Args)]
pub struct VersionListArgs {
    /// Project (ID or identifier).
    #[arg(long)]
    pub project: String,
}

/// Execute version list command.
pub async fn list(client: &RedmineClient, args: &VersionListArgs) -> Result<VersionList> {
    client.list_versions(&args.project).await
}

/// Resolve version arguments (IDs or names) to version IDs.
///
/// Numeric values are used as IDs directly. Names are looked up among the
/// versions available to `project`, which is required when any name is given.
pub async fn resolve_version_ids(
    client: &RedmineClient,
    project: Option<&str>,
    values: &[String],
) -> Result<Vec<u32>> {
    let mut ids = Vec::with_capacity(values.len());
    let mut versions: Option<VersionList> = None;

    for value in values.iter().map(|v| v.trim()).filter(|v| !v.is_empty()) {
        if let Ok(id) = value.parse::<u32>() {
            ids.push(id);
            continue;
        }

        let project = project.ok_or_else(|| {
            AppError::validation_with_hint(
                format!("Cannot resolve version name '{}' without a project", value),
                "Pass --project, or use the version ID (see `rdm version list --project <ID>`).",
            )
        })?;

        if versions.is_none() {
            versions = Some(client.list_versions(project).await?);
        }
        let version = versions
            .as_ref()
            .and_then(|list| list.resolve(value))
            .ok_or_else(|| {
                AppError::not_found_with_hint(
                    "Version",
                    value,
                    format!(
                        "Use `rdm version list --project {}` to see available versions.",
                        project
                    ),
                )
            })?;
        ids.push(version.id);
    }

    Ok(ids)
}
