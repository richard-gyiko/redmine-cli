//! Custom field model for issues and time entries.

use serde::{Deserialize, Serialize};

/// Custom field value from Redmine API (response format).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomField {
    pub id: u32,
    pub name: String,
    /// Value can be string, number, array (for multi-value fields), or null.
    #[serde(default)]
    pub value: serde_json::Value,
    /// Whether this is a multi-value field.
    #[serde(default)]
    pub multiple: Option<bool>,
}

/// Custom field value for API requests (write format).
/// Redmine expects: `{ "id": 5, "value": "some value" }`
///
/// Note: This currently only supports string values. Multi-value custom fields
/// (arrays) are not yet supported via the CLI.
#[derive(Debug, Clone, Serialize)]
pub struct CustomFieldValue {
    pub id: u32,
    pub value: String,
}

impl CustomFieldValue {
    /// Create a new custom field value from parsed (id, value) tuple.
    pub fn new(id: u32, value: String) -> Self {
        Self { id, value }
    }

    /// Convert a list of (id, value) tuples to CustomFieldValue vec.
    pub fn from_tuples(tuples: Vec<(u32, String)>) -> Vec<Self> {
        tuples
            .into_iter()
            .map(|(id, value)| Self::new(id, value))
            .collect()
    }
}

impl CustomField {
    /// Get the value as a display string.
    pub fn display_value(&self) -> String {
        match &self.value {
            serde_json::Value::Null => "-".to_string(),
            serde_json::Value::String(s) => {
                if s.is_empty() {
                    "-".to_string()
                } else {
                    s.clone()
                }
            }
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::Bool(b) => if *b { "Yes" } else { "No" }.to_string(),
            serde_json::Value::Array(arr) => {
                if arr.is_empty() {
                    "-".to_string()
                } else {
                    arr.iter()
                        .map(|v| match v {
                            serde_json::Value::String(s) => s.clone(),
                            _ => v.to_string(),
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                }
            }
            serde_json::Value::Object(_) => self.value.to_string(),
        }
    }
}
