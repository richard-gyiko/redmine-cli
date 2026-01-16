//! Output format selection and dispatching.

use clap::ValueEnum;
use serde::Serialize;

use super::markdown::format_error_markdown;
use super::{Envelope, ErrorInfo, Meta};
use crate::error::AppError;

/// Output format selection.
#[derive(Debug, Clone, Copy, Default, ValueEnum, PartialEq, Eq)]
pub enum OutputFormat {
    /// Markdown output (default, optimized for LLM/agent consumption).
    #[default]
    Markdown,
    /// JSON output (envelope format for programmatic pipelines).
    Json,
}

/// Trait for outputting results in the selected format.
pub trait Format {
    /// Format success output.
    fn format_success<T: Serialize + super::MarkdownOutput>(&self, data: T, meta: Meta) -> String;

    /// Format error output.
    fn format_error(&self, error: &AppError) -> String;
}

impl Format for OutputFormat {
    fn format_success<T: Serialize + super::MarkdownOutput>(&self, data: T, meta: Meta) -> String {
        match self {
            OutputFormat::Markdown => data.to_markdown(&meta),
            OutputFormat::Json => {
                let envelope = Envelope::success_with_meta(data, meta);
                serde_json::to_string_pretty(&envelope).unwrap_or_else(|e| {
                    format!(
                        "{{\"ok\":false,\"error\":{{\"code\":\"JSON_ERROR\",\"message\":\"{}\"}}}}",
                        e
                    )
                })
            }
        }
    }

    fn format_error(&self, error: &AppError) -> String {
        match self {
            OutputFormat::Markdown => format_error_markdown(error),
            OutputFormat::Json => {
                let envelope: Envelope<()> = Envelope::<()>::error(ErrorInfo::from(error));
                serde_json::to_string_pretty(&envelope).unwrap_or_else(|e| {
                    format!(
                        "{{\"ok\":false,\"error\":{{\"code\":\"JSON_ERROR\",\"message\":\"{}\"}}}}",
                        e
                    )
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize)]
    struct TestData {
        id: u32,
        name: String,
    }

    impl super::super::MarkdownOutput for TestData {
        fn to_markdown(&self, _meta: &Meta) -> String {
            format!("## Test: {}\n\nID: {}", self.name, self.id)
        }
    }

    #[test]
    fn test_markdown_format() {
        let format = OutputFormat::Markdown;
        let data = TestData {
            id: 1,
            name: "test".to_string(),
        };
        let output = format.format_success(data, Meta::default());
        assert!(output.contains("## Test: test"));
        assert!(output.contains("ID: 1"));
    }

    #[test]
    fn test_json_format() {
        let format = OutputFormat::Json;
        let data = TestData {
            id: 1,
            name: "test".to_string(),
        };
        let output = format.format_success(data, Meta::default());
        assert!(output.contains("\"ok\": true"));
        assert!(output.contains("\"id\": 1"));
        assert!(output.contains("\"name\": \"test\""));
    }

    #[test]
    fn test_json_error_format() {
        let format = OutputFormat::Json;
        let error = AppError::not_found("Issue", "123");
        let output = format.format_error(&error);
        assert!(output.contains("\"ok\": false"));
        assert!(output.contains("NOT_FOUND"));
    }
}
