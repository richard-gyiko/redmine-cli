//! Data models for Redmine API responses.

mod custom_field;
mod issue;
mod project;
mod time_entry;
mod user;

// Re-export for public API (may not be used internally but available for consumers)
#[allow(unused_imports)]
pub use custom_field::CustomField;
pub use issue::{
    Issue, IssueList, IssueResponse, NewIssue, NewIssueRequest, UpdateIssue, UpdateIssueRequest,
};
// Re-export for internal use by client/endpoints.rs
pub(crate) use issue::SearchResults;
pub use project::{Project, ProjectList, ProjectResponse};
pub use time_entry::{
    Activity, ActivityList, GroupByField, GroupedTimeEntries, NewTimeEntry, NewTimeEntryRequest,
    TimeEntry, TimeEntryCreated, TimeEntryDeleted, TimeEntryList, TimeEntryResponse,
    TimeEntryUpdated, UpdateTimeEntry, UpdateTimeEntryRequest,
};
pub use user::{CurrentUser, CurrentUserResponse};
// Re-export for public API
#[allow(unused_imports)]
pub use user::User;
