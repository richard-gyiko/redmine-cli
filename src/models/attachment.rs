//! Attachment model and related types.

use super::user::User;
use crate::output::{
    markdown::{markdown_kv_table, markdown_table},
    MarkdownOutput, Meta,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// File attachment on an issue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub id: u32,
    pub filename: String,
    #[serde(default)]
    pub filesize: Option<u64>,
    #[serde(default)]
    pub content_type: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    pub content_url: String,
    #[serde(default)]
    pub author: Option<User>,
    #[serde(default)]
    pub created_on: Option<String>,
}

/// Wrapper for single attachment response.
#[derive(Debug, Deserialize)]
pub struct AttachmentResponse {
    pub attachment: Attachment,
}

/// Token returned from a file upload.
#[derive(Debug, Deserialize)]
pub struct UploadToken {
    pub token: String,
}

/// Wrapper for upload response.
#[derive(Debug, Deserialize)]
pub struct UploadResponse {
    pub upload: UploadToken,
}

/// Reference used to attach an uploaded file to an issue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentRef {
    pub token: String,
    pub filename: String,
    pub content_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Result type for `rdm issue attachment list`.
#[derive(Debug, Serialize)]
pub struct AttachmentList {
    pub issue_id: u32,
    pub attachments: Vec<Attachment>,
}

impl MarkdownOutput for AttachmentList {
    fn to_markdown(&self, _meta: &Meta) -> String {
        let mut output = String::new();
        output.push_str(&format!(
            "## Attachments for Issue #{} ({})\n\n",
            self.issue_id,
            self.attachments.len()
        ));

        if self.attachments.is_empty() {
            output.push_str("*No attachments.*\n");
            return output;
        }

        let headers = ["ID", "Filename", "Size", "Type", "Author"];
        let rows: Vec<Vec<String>> = self
            .attachments
            .iter()
            .map(|a| {
                vec![
                    a.id.to_string(),
                    a.filename.clone(),
                    a.filesize.map(format_bytes).unwrap_or_else(|| "-".into()),
                    a.content_type.clone().unwrap_or_else(|| "-".into()),
                    a.author
                        .as_ref()
                        .map(|u| u.name.clone())
                        .unwrap_or_else(|| "-".into()),
                ]
            })
            .collect();

        output.push_str(&markdown_table(&headers, rows));
        output.push_str("\n*Use `rdm issue attachment download --id <ID>` to download*\n");
        output
    }
}

impl MarkdownOutput for Attachment {
    fn to_markdown(&self, _meta: &Meta) -> String {
        let mut output = String::new();
        output.push_str(&format!(
            "## Attachment #{}: {}\n\n",
            self.id, self.filename
        ));

        let mut pairs = vec![
            ("ID", self.id.to_string()),
            ("Filename", self.filename.clone()),
        ];

        if let Some(ct) = &self.content_type {
            pairs.push(("Type", ct.clone()));
        }
        if let Some(size) = self.filesize {
            pairs.push(("Size", format_bytes(size)));
        }
        if let Some(desc) = &self.description {
            if !desc.is_empty() {
                pairs.push(("Description", desc.clone()));
            }
        }
        if let Some(author) = &self.author {
            pairs.push(("Author", author.name.clone()));
        }
        if let Some(created) = &self.created_on {
            pairs.push(("Created", created.clone()));
        }
        pairs.push(("URL", self.content_url.clone()));

        let pairs_ref: Vec<(&str, String)> = pairs.iter().map(|(k, v)| (*k, v.clone())).collect();
        output.push_str(&markdown_kv_table(&pairs_ref));
        output
    }
}

/// Result of a successful download.
#[derive(Debug, Serialize)]
pub struct AttachmentDownloaded {
    pub id: u32,
    pub filename: String,
    #[serde(serialize_with = "serialize_path")]
    pub saved_to: PathBuf,
    pub bytes: u64,
}

impl MarkdownOutput for AttachmentDownloaded {
    fn to_markdown(&self, _meta: &Meta) -> String {
        format!(
            "## Attachment Downloaded\n\n**{}** ({}) saved to `{}`\n",
            self.filename,
            format_bytes(self.bytes),
            self.saved_to.display()
        )
    }
}

/// Result of a successful upload + attach.
#[derive(Debug, Serialize)]
pub struct AttachmentUploaded {
    pub filename: String,
    pub issue_id: u32,
}

impl MarkdownOutput for AttachmentUploaded {
    fn to_markdown(&self, _meta: &Meta) -> String {
        format!(
            "## Attachment Uploaded\n\n**{}** attached to issue #{}.\n\n*Use `rdm issue get --id {}` to view issue*\n",
            self.filename, self.issue_id, self.issue_id
        )
    }
}

pub fn format_bytes(bytes: u64) -> String {
    if bytes >= 1_048_576 {
        format!("{:.1} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}

/// Guess MIME type from file extension.
pub fn guess_content_type(path: &std::path::Path) -> &'static str {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase())
        .as_deref()
    {
        Some("pdf") => "application/pdf",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        Some("txt") => "text/plain",
        Some("csv") => "text/csv",
        Some("json") => "application/json",
        Some("xml") => "application/xml",
        Some("zip") => "application/zip",
        Some("tar") => "application/x-tar",
        Some("gz") => "application/gzip",
        Some("doc") => "application/msword",
        Some("docx") => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        Some("xls") => "application/vnd.ms-excel",
        Some("xlsx") => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        Some("ppt") => "application/vnd.ms-powerpoint",
        Some("pptx") => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        _ => "application/octet-stream",
    }
}

fn serialize_path<S>(path: &std::path::Path, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    s.serialize_str(&path.to_string_lossy())
}
