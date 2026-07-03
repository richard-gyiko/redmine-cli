//! Saved query commands.

use clap::{Args, Subcommand, ValueEnum};

use crate::client::RedmineClient;
use crate::error::Result;
use crate::models::QueryList;
use crate::output::{markdown::markdown_table, MarkdownOutput, Meta};

#[derive(Debug, Subcommand)]
pub enum QueryCommand {
    /// List saved (custom) queries.
    List(QueryListArgs),
}

/// Which kind of saved query to list.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum QueryType {
    /// Issue queries (Redmine default).
    Issue,
    /// Time entry queries.
    Time,
}

impl QueryType {
    /// Map to Redmine's `type` parameter value.
    pub fn as_api_value(&self) -> &'static str {
        match self {
            Self::Issue => "IssueQuery",
            Self::Time => "TimeEntryQuery",
        }
    }
}

#[derive(Debug, Args)]
pub struct QueryListArgs {
    /// Query type to list (defaults to issue queries).
    #[arg(long = "type", value_enum)]
    pub query_type: Option<QueryType>,
}

/// Execute query list command.
pub async fn list(client: &RedmineClient, args: &QueryListArgs) -> Result<QueryList> {
    client
        .list_queries(args.query_type.map(|t| t.as_api_value()))
        .await
}

impl MarkdownOutput for QueryList {
    fn to_markdown(&self, _meta: &Meta) -> String {
        let mut output = String::new();
        let total = self.total_count.unwrap_or(self.queries.len() as u32);

        output.push_str(&format!("## Saved Queries ({})\n\n", total));

        if self.queries.is_empty() {
            output.push_str("*No queries found*\n");
            return output;
        }

        let headers = &["ID", "Name", "Scope", "Visibility"];
        let rows: Vec<Vec<String>> = self
            .queries
            .iter()
            .map(|q| {
                vec![
                    q.id.to_string(),
                    q.name.clone(),
                    match q.project_id {
                        Some(id) => format!("project {}", id),
                        None => "global".to_string(),
                    },
                    if q.is_public { "public" } else { "private" }.to_string(),
                ]
            })
            .collect();

        output.push_str(&markdown_table(headers, rows));
        output.push_str(
            "\n*Run with `--query-id <ID>` on `issue list` or `time list`. \
             Project-scoped queries also need `--project <ID>`.*\n",
        );
        output
    }
}
