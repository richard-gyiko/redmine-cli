//! Custom field model for issues and time entries.

use serde::{Deserialize, Serialize};

/// Custom field value from Redmine API.
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
