//! Common test utilities.

use wiremock::matchers::{header, method, path, path_regex};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Start a mock Redmine server.
pub async fn start_mock_server() -> MockServer {
    MockServer::start().await
}

/// Create a mock for the current user endpoint.
pub fn mock_current_user() -> Mock {
    Mock::given(method("GET"))
        .and(path("/users/current.json"))
        .and(header("X-Redmine-API-Key", "test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "user": {
                "id": 1,
                "login": "testuser",
                "firstname": "Test",
                "lastname": "User",
                "mail": "test@example.com",
                "admin": false,
                "created_on": "2024-01-01T00:00:00Z",
                "last_login_on": "2024-01-15T12:00:00Z"
            }
        })))
}

/// Create a mock for the activities endpoint.
pub fn mock_activities() -> Mock {
    Mock::given(method("GET"))
        .and(path("/enumerations/time_entry_activities.json"))
        .and(header("X-Redmine-API-Key", "test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "time_entry_activities": [
                {"id": 1, "name": "Development", "is_default": true},
                {"id": 2, "name": "Design", "is_default": false},
                {"id": 3, "name": "Testing", "is_default": false}
            ]
        })))
}

/// Create a mock for the projects list endpoint.
pub fn mock_projects_list() -> Mock {
    Mock::given(method("GET"))
        .and(path_regex(r"/projects\.json.*"))
        .and(header("X-Redmine-API-Key", "test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "projects": [
                {
                    "id": 1,
                    "name": "Test Project",
                    "identifier": "test-project",
                    "description": "A test project",
                    "status": 1,
                    "is_public": true,
                    "created_on": "2024-01-01T00:00:00Z",
                    "updated_on": "2024-01-15T12:00:00Z"
                }
            ],
            "total_count": 1,
            "offset": 0,
            "limit": 25
        })))
}

/// Create a mock for getting a single project.
pub fn mock_project_get() -> Mock {
    Mock::given(method("GET"))
        .and(path_regex(r"/projects/[^/]+\.json"))
        .and(header("X-Redmine-API-Key", "test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "project": {
                "id": 1,
                "name": "Test Project",
                "identifier": "test-project",
                "description": "A test project",
                "status": 1,
                "is_public": true,
                "created_on": "2024-01-01T00:00:00Z",
                "updated_on": "2024-01-15T12:00:00Z"
            }
        })))
}

/// Create a mock for the issues list endpoint.
pub fn mock_issues_list() -> Mock {
    Mock::given(method("GET"))
        .and(path_regex(r"/issues\.json.*"))
        .and(header("X-Redmine-API-Key", "test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "issues": [
                {
                    "id": 123,
                    "subject": "Test Issue",
                    "project": {"id": 1, "name": "Test Project", "identifier": "test-project"},
                    "status": {"id": 1, "name": "New"},
                    "priority": {"id": 2, "name": "Normal"},
                    "author": {"id": 1, "name": "Test User"},
                    "created_on": "2024-01-01T00:00:00Z",
                    "updated_on": "2024-01-15T12:00:00Z"
                }
            ],
            "total_count": 1,
            "offset": 0,
            "limit": 25
        })))
}

/// Create a mock for getting a single issue.
pub fn mock_issue_get() -> Mock {
    Mock::given(method("GET"))
        .and(path_regex(r"/issues/\d+\.json"))
        .and(header("X-Redmine-API-Key", "test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "issue": {
                "id": 123,
                "subject": "Test Issue",
                "description": "This is a test issue",
                "project": {"id": 1, "name": "Test Project", "identifier": "test-project"},
                "status": {"id": 1, "name": "New"},
                "priority": {"id": 2, "name": "Normal"},
                "tracker": {"id": 1, "name": "Bug"},
                "author": {"id": 1, "name": "Test User"},
                "created_on": "2024-01-01T00:00:00Z",
                "updated_on": "2024-01-15T12:00:00Z"
            }
        })))
}

/// Create a mock for time entries list endpoint.
pub fn mock_time_entries_list() -> Mock {
    Mock::given(method("GET"))
        .and(path_regex(r"/time_entries\.json.*"))
        .and(header("X-Redmine-API-Key", "test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "time_entries": [
                {
                    "id": 456,
                    "hours": 2.5,
                    "comments": "Test comment",
                    "spent_on": "2024-01-15",
                    "activity": {"id": 1, "name": "Development"},
                    "user": {"id": 1, "name": "Test User"},
                    "issue": {"id": 123},
                    "created_on": "2024-01-15T12:00:00Z",
                    "updated_on": "2024-01-15T12:00:00Z"
                }
            ],
            "total_count": 1,
            "offset": 0,
            "limit": 25
        })))
}

/// Create a mock for getting a single time entry.
pub fn mock_time_entry_get() -> Mock {
    Mock::given(method("GET"))
        .and(path_regex(r"/time_entries/\d+\.json"))
        .and(header("X-Redmine-API-Key", "test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "time_entry": {
                "id": 456,
                "hours": 2.5,
                "comments": "Test comment",
                "spent_on": "2024-01-15",
                "activity": {"id": 1, "name": "Development"},
                "user": {"id": 1, "name": "Test User"},
                "project": {"id": 1, "name": "Test Project", "identifier": "test-project"},
                "issue": {"id": 123},
                "created_on": "2024-01-15T12:00:00Z",
                "updated_on": "2024-01-15T12:00:00Z"
            }
        })))
}

/// Create a mock for creating a time entry.
pub fn mock_time_entry_create() -> Mock {
    Mock::given(method("POST"))
        .and(path("/time_entries.json"))
        .and(header("X-Redmine-API-Key", "test-api-key"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "time_entry": {
                "id": 789,
                "hours": 1.5,
                "comments": "New entry",
                "spent_on": "2024-01-16",
                "activity": {"id": 1, "name": "Development"},
                "user": {"id": 1, "name": "Test User"},
                "issue": {"id": 123},
                "created_on": "2024-01-16T12:00:00Z",
                "updated_on": "2024-01-16T12:00:00Z"
            }
        })))
}

/// Create a mock for updating a time entry (PUT returns no body, then we GET).
pub fn mock_time_entry_update() -> Mock {
    Mock::given(method("PUT"))
        .and(path_regex(r"/time_entries/\d+\.json"))
        .and(header("X-Redmine-API-Key", "test-api-key"))
        .respond_with(ResponseTemplate::new(200))
}

/// Create a mock for deleting a time entry.
pub fn mock_time_entry_delete() -> Mock {
    Mock::given(method("DELETE"))
        .and(path_regex(r"/time_entries/\d+\.json"))
        .and(header("X-Redmine-API-Key", "test-api-key"))
        .respond_with(ResponseTemplate::new(200))
}
