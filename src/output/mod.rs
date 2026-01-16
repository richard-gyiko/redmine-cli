//! Output formatting module - Markdown default, JSON envelope available.

mod envelope;
mod format;
pub mod markdown;

pub use envelope::{Envelope, ErrorInfo, Meta};
pub use format::{Format, OutputFormat};
pub use markdown::MarkdownOutput;
