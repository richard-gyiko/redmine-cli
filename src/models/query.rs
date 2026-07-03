//! Saved query (custom query) models.

use serde::{Deserialize, Serialize};

/// A saved/custom query as returned by `/queries.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    pub id: u32,
    pub name: String,
    #[serde(default)]
    pub is_public: bool,
    /// Project the query is scoped to, or `None` for a global query.
    #[serde(default)]
    pub project_id: Option<u32>,
}

/// List of saved queries from `/queries.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryList {
    pub queries: Vec<Query>,
    #[serde(default)]
    pub total_count: Option<u32>,
    #[serde(default)]
    pub offset: Option<u32>,
    #[serde(default)]
    pub limit: Option<u32>,
}
