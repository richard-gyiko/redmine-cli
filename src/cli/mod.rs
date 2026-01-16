//! CLI command definitions.

pub mod issue;
pub mod ping;
pub mod profile;
pub mod project;
pub mod time;

use crate::output::OutputFormat;
use clap::{Parser, Subcommand};

/// Agent-first Redmine CLI with markdown-first output.
#[derive(Debug, Parser)]
#[command(name = "rma", version, about, long_about = None)]
pub struct Cli {
    /// Output format (markdown or json).
    #[arg(
        long,
        short = 'f',
        value_enum,
        default_value = "markdown",
        global = true
    )]
    pub format: OutputFormat,

    /// Redmine server URL (overrides env/config).
    #[arg(long, env = "REDMINE_URL", global = true)]
    pub url: Option<String>,

    /// Redmine API key (overrides env/config).
    #[arg(long, env = "REDMINE_API_KEY", global = true)]
    pub api_key: Option<String>,

    /// Enable debug output to stderr.
    #[arg(long, global = true)]
    pub debug: bool,

    /// Print request without executing.
    #[arg(long, global = true)]
    pub dry_run: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Check connection and authentication.
    Ping,

    /// Show current user information.
    Me,

    /// Manage configuration profiles.
    #[command(subcommand)]
    Profile(profile::ProfileCommand),

    /// Show current configuration.
    Config(profile::ConfigShow),

    /// Project commands.
    #[command(subcommand)]
    Project(project::ProjectCommand),

    /// Issue commands.
    #[command(subcommand)]
    Issue(issue::IssueCommand),

    /// Time entry commands.
    #[command(subcommand)]
    Time(time::TimeCommand),
}
