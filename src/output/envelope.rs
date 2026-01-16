//! JSON envelope types for `--format json` output.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// JSON output envelope wrapping all responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Envelope<T> {
    /// Whether the operation succeeded.
    pub ok: bool,
    /// The response data (null on error).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    /// Metadata about the response.
    pub meta: Meta,
    /// Error information (null on success).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorInfo>,
}

impl<T> Envelope<T> {
    /// Create a success envelope with data.
    #[allow(dead_code)]
    pub fn success(data: T) -> Self {
        Self {
            ok: true,
            data: Some(data),
            meta: Meta::default(),
            error: None,
        }
    }

    /// Create a success envelope with data and metadata.
    pub fn success_with_meta(data: T, meta: Meta) -> Self {
        Self {
            ok: true,
            data: Some(data),
            meta,
            error: None,
        }
    }

    /// Create an error envelope.
    pub fn error(error: ErrorInfo) -> Envelope<()> {
        Envelope {
            ok: false,
            data: None,
            meta: Meta::default(),
            error: Some(error),
        }
    }
}

/// Metadata about the response (pagination, etc).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Meta {
    /// Total count of items (for list responses).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<u32>,
    /// Limit used for this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Offset used for this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    /// Next offset for pagination (if more results exist).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<u32>,
}

impl Meta {
    /// Create metadata for a paginated response.
    pub fn paginated(total_count: u32, limit: u32, offset: u32) -> Self {
        let next_offset = if offset + limit < total_count {
            Some(offset + limit)
        } else {
            None
        };
        Self {
            total_count: Some(total_count),
            limit: Some(limit),
            offset: Some(offset),
            next_offset,
        }
    }
}

/// Error information for failed responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    /// Error code (e.g., "NOT_FOUND", "AUTH_ERROR").
    pub code: String,
    /// Human-readable error message.
    pub message: String,
    /// Additional error details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
}

impl ErrorInfo {
    /// Create a new error info.
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    /// Create error info with details.
    #[allow(dead_code)]
    pub fn with_details(
        code: impl Into<String>,
        message: impl Into<String>,
        details: Value,
    ) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: Some(details),
        }
    }
}

impl From<&crate::error::AppError> for ErrorInfo {
    fn from(err: &crate::error::AppError) -> Self {
        let mut info = ErrorInfo::new(err.code(), err.to_string());
        if let Some(hint) = err.hint() {
            info.details = Some(serde_json::json!({ "hint": hint }));
        }
        info
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_envelope() {
        let envelope = Envelope::success(vec![1, 2, 3]);
        assert!(envelope.ok);
        assert!(envelope.data.is_some());
        assert!(envelope.error.is_none());
    }

    #[test]
    fn test_error_envelope() {
        let envelope: Envelope<()> =
            Envelope::<()>::error(ErrorInfo::new("NOT_FOUND", "Issue not found"));
        assert!(!envelope.ok);
        assert!(envelope.data.is_none());
        assert!(envelope.error.is_some());
    }

    #[test]
    fn test_meta_pagination() {
        let meta = Meta::paginated(100, 25, 0);
        assert_eq!(meta.total_count, Some(100));
        assert_eq!(meta.limit, Some(25));
        assert_eq!(meta.offset, Some(0));
        assert_eq!(meta.next_offset, Some(25));
    }

    #[test]
    fn test_meta_pagination_last_page() {
        let meta = Meta::paginated(100, 25, 75);
        assert_eq!(meta.next_offset, None);
    }

    #[test]
    fn test_envelope_json_serialization() {
        let envelope =
            Envelope::success_with_meta(serde_json::json!({"id": 123}), Meta::paginated(1, 25, 0));
        let json = serde_json::to_string(&envelope).unwrap();
        assert!(json.contains("\"ok\":true"));
        assert!(json.contains("\"id\":123"));
    }
}
