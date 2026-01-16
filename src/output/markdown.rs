//! Markdown formatting trait and implementations.

use super::{ErrorInfo, Meta};

/// Trait for types that can render as Markdown.
pub trait MarkdownOutput {
    /// Render this item as Markdown.
    fn to_markdown(&self, meta: &Meta) -> String;
}

/// Format an error as Markdown blockquote.
pub fn format_error_markdown(error: &crate::error::AppError) -> String {
    let mut output = String::new();
    output.push_str(&format!("> **Error: {}**\n", error.code()));
    output.push_str(&format!("> {}\n", error));
    if let Some(hint) = error.hint() {
        output.push_str(">\n");
        output.push_str(&format!("> {}\n", hint));
    }
    output
}

/// Format an ErrorInfo as Markdown blockquote.
#[allow(dead_code)]
pub fn format_error_info_markdown(error: &ErrorInfo) -> String {
    let mut output = String::new();
    output.push_str(&format!("> **Error: {}**\n", error.code));
    output.push_str(&format!("> {}\n", error.message));
    if let Some(details) = &error.details {
        if let Some(hint) = details.get("hint").and_then(|v| v.as_str()) {
            output.push_str(">\n");
            output.push_str(&format!("> {}\n", hint));
        }
    }
    output
}

/// Helper to create a Markdown table from headers and rows.
pub fn markdown_table(headers: &[&str], rows: Vec<Vec<String>>) -> String {
    let mut output = String::new();

    // Header row
    output.push('|');
    for header in headers {
        output.push_str(&format!(" {} |", header));
    }
    output.push('\n');

    // Separator row
    output.push('|');
    for _ in headers {
        output.push_str("----|");
    }
    output.push('\n');

    // Data rows
    for row in rows {
        output.push('|');
        for cell in row {
            output.push_str(&format!(" {} |", cell));
        }
        output.push('\n');
    }

    output
}

/// Helper to create a key-value Markdown table.
pub fn markdown_kv_table(pairs: &[(&str, String)]) -> String {
    let mut output = String::new();
    output.push_str("| Field | Value |\n");
    output.push_str("|-------|-------|\n");
    for (key, value) in pairs {
        output.push_str(&format!("| {} | {} |\n", key, value));
    }
    output
}

/// Helper to add a pagination hint.
pub fn pagination_hint(command: &str, meta: &Meta) -> Option<String> {
    meta.next_offset
        .map(|next| format!("*Use `{}--offset {}` for next page*", command, next))
}
