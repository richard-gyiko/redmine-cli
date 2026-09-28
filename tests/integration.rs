//! Integration tests for the rdm CLI.

mod common;

use assert_cmd::Command;
use common::*;
use predicates::prelude::*;

fn get_binary() -> Command {
    Command::cargo_bin("rdm").unwrap()
}

// ============================================================================
// Project Commands
// ============================================================================

#[tokio::test]
async fn test_project_list() {
    let server = start_mock_server().await;
    mock_projects_list().mount(&server).await;

    let mut cmd = get_binary();
    cmd.env("APPDATA", std::env::temp_dir())
        .env("LOCALAPPDATA", std::env::temp_dir())
        .args(["--url", &server.uri(), "--api-key", "test-api-key"])
        .arg("project")
        .arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Test Project"));
}

#[tokio::test]
async fn test_project_list_json() {
    let server = start_mock_server().await;
    mock_projects_list().mount(&server).await;

    let mut cmd = get_binary();
    cmd.env("APPDATA", std::env::temp_dir())
        .env("LOCALAPPDATA", std::env::temp_dir())
        .args(["--url", &server.uri(), "--api-key", "test-api-key"])
        .args(["--format", "json"])
        .arg("project")
        .arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"ok\": true"))
        .stdout(predicate::str::contains("\"name\": \"Test Project\""));
}

#[tokio::test]
async fn test_project_get() {
    let server = start_mock_server().await;
    mock_project_get().mount(&server).await;

    let mut cmd = get_binary();
    cmd.env("APPDATA", std::env::temp_dir())
        .env("LOCALAPPDATA", std::env::temp_dir())
        .args(["--url", &server.uri(), "--api-key", "test-api-key"])
        .arg("project")
        .arg("get")
        .args(["--identifier", "test-project"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Test Project"));
}

// ============================================================================
// Issue Commands
// ============================================================================

#[tokio::test]
async fn test_issue_list() {
    let server = start_mock_server().await;
    mock_issues_list().mount(&server).await;

    let mut cmd = get_binary();
    cmd.env("APPDATA", std::env::temp_dir())
        .env("LOCALAPPDATA", std::env::temp_dir())
        .args(["--url", &server.uri(), "--api-key", "test-api-key"])
        .arg("issue")
        .arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Test Issue"))
        .stdout(predicate::str::contains("123"));
}

#[tokio::test]
async fn test_issue_list_json() {
    let server = start_mock_server().await;
    mock_issues_list().mount(&server).await;

    let mut cmd = get_binary();
    cmd.env("APPDATA", std::env::temp_dir())
        .env("LOCALAPPDATA", std::env::temp_dir())
        .args(["--url", &server.uri(), "--api-key", "test-api-key"])
        .args(["--format", "json"])
        .arg("issue")
        .arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"ok\": true"))
        .stdout(predicate::str::contains("\"subject\": \"Test Issue\""));
}

#[tokio::test]
async fn test_issue_get() {
    let server = start_mock_server().await;
    mock_issue_get().mount(&server).await;

    let mut cmd = get_binary();
    cmd.env("APPDATA", std::env::temp_dir())
        .env("LOCALAPPDATA", std::env::temp_dir())
        .args(["--url", &server.uri(), "--api-key", "test-api-key"])
        .arg("issue")
        .arg("get")
        .args(["--id", "123"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Test Issue"))
        .stdout(predicate::str::contains("#123"));
}

#[tokio::test]
async fn test_issue_list_query_id_project_scoped() {
    // A --query-id with --project must route through /projects/{id}/issues.json.
    let server = start_mock_server().await;
    mock_project_issues_query().mount(&server).await;

    let mut cmd = get_binary();
    cmd.env("APPDATA", std::env::temp_dir())
        .env("LOCALAPPDATA", std::env::temp_dir())
        .args(["--url", &server.uri(), "--api-key", "test-api-key"])
        .arg("issue")
        .arg("list")
        .args(["--query-id", "5", "--project", "10"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Test Issue"));
}

#[tokio::test]
async fn test_issue_get_shows_target_version() {
    let server = start_mock_server().await;
    mock_issue_get().mount(&server).await;

    let mut cmd = get_binary();
    cmd.env("APPDATA", std::env::temp_dir())
        .env("LOCALAPPDATA", std::env::temp_dir())
        .args(["--url", &server.uri(), "--api-key", "test-api-key"])
        .arg("issue")
        .arg("get")
        .args(["--id", "123"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Target Version"))
        .stdout(predicate::str::contains("Milestone 1 (#116)"));
}

#[tokio::test]
async fn test_issue_get_json_includes_fixed_version() {
    let server = start_mock_server().await;
    mock_issue_get().mount(&server).await;

    let mut cmd = get_binary();
    cmd.env("APPDATA", std::env::temp_dir())
        .env("LOCALAPPDATA", std::env::temp_dir())
        .args(["--url", &server.uri(), "--api-key", "test-api-key"])
        .args(["--format", "json"])
        .arg("issue")
        .arg("get")
        .args(["--id", "123"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"fixed_version\""))
        .stdout(predicate::str::contains("\"name\": \"Milestone 1\""));
}

#[tokio::test]
async fn test_issue_list_version_filter_by_name_and_id() {
    // Names resolve via the project's versions, mix with IDs, and join with `|`.
    let server = start_mock_server().await;
    mock_versions_list().mount(&server).await;
    mock_project_issues_by_version().mount(&server).await;

    let mut cmd = get_binary();
    cmd.env("APPDATA", std::env::temp_dir())
        .env("LOCALAPPDATA", std::env::temp_dir())
        .args(["--url", &server.uri(), "--api-key", "test-api-key"])
        .arg("issue")
        .arg("list")
        .args(["--project", "10", "--version", "milestone 1,117"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Target Version"))
        .stdout(predicate::str::contains("Milestone 1"));
}

#[tokio::test]
async fn test_issue_list_version_name_requires_project() {
    let server = start_mock_server().await;

    let mut cmd = get_binary();
    cmd.env("APPDATA", std::env::temp_dir())
        .env("LOCALAPPDATA", std::env::temp_dir())
        .args(["--url", &server.uri(), "--api-key", "test-api-key"])
        .arg("issue")
        .arg("list")
        .args(["--version", "Milestone 1"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("without a project"));
}

#[tokio::test]
async fn test_issue_list_version_unknown_name() {
    let server = start_mock_server().await;
    mock_versions_list().mount(&server).await;

    let mut cmd = get_binary();
    cmd.env("APPDATA", std::env::temp_dir())
        .env("LOCALAPPDATA", std::env::temp_dir())
        .args(["--url", &server.uri(), "--api-key", "test-api-key"])
        .arg("issue")
        .arg("list")
        .args(["--project", "10", "--version", "Milestone 9"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Milestone 9"));
}

#[tokio::test]
async fn test_issue_update_version_by_name() {
    // A name resolves within the issue's project, then is sent as fixed_version_id.
    let server = start_mock_server().await;
    mock_issue_get().mount(&server).await;
    mock_versions_list().mount(&server).await;
    mock_issue_update_version(117).mount(&server).await;

    let mut cmd = get_binary();
    cmd.env("APPDATA", std::env::temp_dir())
        .env("LOCALAPPDATA", std::env::temp_dir())
        .args(["--url", &server.uri(), "--api-key", "test-api-key"])
        .arg("issue")
        .arg("update")
        .args(["--id", "123", "--version", "Milestone 2"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Issue #123 has been updated"));
}

#[tokio::test]
async fn test_issue_update_version_by_id() {
    let server = start_mock_server().await;
    mock_issue_update_version(116).mount(&server).await;

    let mut cmd = get_binary();
    cmd.env("APPDATA", std::env::temp_dir())
        .env("LOCALAPPDATA", std::env::temp_dir())
        .args(["--url", &server.uri(), "--api-key", "test-api-key"])
        .arg("issue")
        .arg("update")
        .args(["--id", "123", "--version", "116"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Issue #123 has been updated"));
}

// ============================================================================
// Version Commands
// ============================================================================

#[tokio::test]
async fn test_version_list() {
    let server = start_mock_server().await;
    mock_versions_list().mount(&server).await;

    let mut cmd = get_binary();
    cmd.env("APPDATA", std::env::temp_dir())
        .env("LOCALAPPDATA", std::env::temp_dir())
        .args(["--url", &server.uri(), "--api-key", "test-api-key"])
        .arg("version")
        .arg("list")
        .args(["--project", "10"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Milestone 1"))
        .stdout(predicate::str::contains("Milestone 2"))
        .stdout(predicate::str::contains("116"))
        .stdout(predicate::str::contains("2024-03-01"))
        .stdout(predicate::str::contains("closed"));
}

// ============================================================================
// Query Commands
// ============================================================================

#[tokio::test]
async fn test_query_list() {
    let server = start_mock_server().await;
    mock_queries_list().mount(&server).await;

    let mut cmd = get_binary();
    cmd.env("APPDATA", std::env::temp_dir())
        .env("LOCALAPPDATA", std::env::temp_dir())
        .args(["--url", &server.uri(), "--api-key", "test-api-key"])
        .arg("query")
        .arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Global Query"))
        .stdout(predicate::str::contains("Project Query"))
        .stdout(predicate::str::contains("global"))
        .stdout(predicate::str::contains("project 10"));
}

#[tokio::test]
async fn test_query_list_type_time_json() {
    let server = start_mock_server().await;
    mock_queries_list().mount(&server).await;

    let mut cmd = get_binary();
    cmd.env("APPDATA", std::env::temp_dir())
        .env("LOCALAPPDATA", std::env::temp_dir())
        .args(["--url", &server.uri(), "--api-key", "test-api-key"])
        .args(["--format", "json"])
        .arg("query")
        .arg("list")
        .args(["--type", "time"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"ok\": true"))
        .stdout(predicate::str::contains("\"name\": \"Global Query\""));
}

// ============================================================================
// Time Entry Commands
// ============================================================================

#[tokio::test]
async fn test_time_list() {
    let server = start_mock_server().await;
    mock_time_entries_list().mount(&server).await;

    let mut cmd = get_binary();
    cmd.env("APPDATA", std::env::temp_dir())
        .env("LOCALAPPDATA", std::env::temp_dir())
        .args(["--url", &server.uri(), "--api-key", "test-api-key"])
        .arg("time")
        .arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("456"))
        .stdout(predicate::str::contains("2.50"))
        .stdout(predicate::str::contains("Development"));
}

#[tokio::test]
async fn test_time_list_json() {
    let server = start_mock_server().await;
    mock_time_entries_list().mount(&server).await;

    let mut cmd = get_binary();
    cmd.env("APPDATA", std::env::temp_dir())
        .env("LOCALAPPDATA", std::env::temp_dir())
        .args(["--url", &server.uri(), "--api-key", "test-api-key"])
        .args(["--format", "json"])
        .arg("time")
        .arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("\"ok\": true"))
        .stdout(predicate::str::contains("\"hours\": 2.5"));
}

#[tokio::test]
async fn test_time_list_query_id_project_scoped() {
    // A --query-id with --project must route through /projects/{id}/time_entries.json.
    let server = start_mock_server().await;
    mock_project_time_entries_query().mount(&server).await;

    let mut cmd = get_binary();
    cmd.env("APPDATA", std::env::temp_dir())
        .env("LOCALAPPDATA", std::env::temp_dir())
        .args(["--url", &server.uri(), "--api-key", "test-api-key"])
        .arg("time")
        .arg("list")
        .args(["--query-id", "112", "--project", "113"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("456"))
        .stdout(predicate::str::contains("Development"));
}

#[tokio::test]
async fn test_time_get() {
    let server = start_mock_server().await;
    mock_time_entry_get().mount(&server).await;

    let mut cmd = get_binary();
    cmd.env("APPDATA", std::env::temp_dir())
        .env("LOCALAPPDATA", std::env::temp_dir())
        .args(["--url", &server.uri(), "--api-key", "test-api-key"])
        .arg("time")
        .arg("get")
        .args(["--id", "456"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Time Entry #456"))
        .stdout(predicate::str::contains("2.50"));
}

#[tokio::test]
async fn test_time_delete() {
    let server = start_mock_server().await;
    mock_time_entry_delete().mount(&server).await;

    let mut cmd = get_binary();
    cmd.env("APPDATA", std::env::temp_dir())
        .env("LOCALAPPDATA", std::env::temp_dir())
        .args(["--url", &server.uri(), "--api-key", "test-api-key"])
        .arg("time")
        .arg("delete")
        .args(["--id", "456"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Time Entry Deleted"));
}

#[tokio::test]
async fn test_time_activities_list() {
    let server = start_mock_server().await;
    mock_activities().mount(&server).await;

    let mut cmd = get_binary();
    cmd.env("APPDATA", std::env::temp_dir())
        .env("LOCALAPPDATA", std::env::temp_dir())
        .args(["--url", &server.uri(), "--api-key", "test-api-key"])
        .arg("time")
        .arg("activities")
        .arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Development"))
        .stdout(predicate::str::contains("Design"))
        .stdout(predicate::str::contains("Testing"));
}

// ============================================================================
// Me Command
// ============================================================================

#[tokio::test]
async fn test_me() {
    let server = start_mock_server().await;
    mock_current_user().mount(&server).await;

    let mut cmd = get_binary();
    cmd.env("APPDATA", std::env::temp_dir())
        .env("LOCALAPPDATA", std::env::temp_dir())
        .args(["--url", &server.uri(), "--api-key", "test-api-key"])
        .arg("me");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("testuser"))
        .stdout(predicate::str::contains("Test User"));
}

// ============================================================================
// Ping Command
// ============================================================================

#[tokio::test]
async fn test_ping() {
    let server = start_mock_server().await;
    mock_current_user().mount(&server).await;

    let mut cmd = get_binary();
    cmd.env("APPDATA", std::env::temp_dir())
        .env("LOCALAPPDATA", std::env::temp_dir())
        .args(["--url", &server.uri(), "--api-key", "test-api-key"])
        .arg("ping");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Connection Status"))
        .stdout(predicate::str::contains("ok"));
}

// ============================================================================
// Error Handling
// ============================================================================

#[test]
fn test_missing_credentials() {
    let mut cmd = get_binary();
    cmd.env("APPDATA", std::env::temp_dir())
        .env("LOCALAPPDATA", std::env::temp_dir())
        .env_remove("REDMINE_URL")
        .env_remove("REDMINE_API_KEY")
        .arg("ping");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No Redmine credentials"));
}

#[test]
fn test_help() {
    let mut cmd = get_binary();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Agent-first Redmine CLI"))
        .stdout(predicate::str::contains("ping"))
        .stdout(predicate::str::contains("issue"))
        .stdout(predicate::str::contains("time"));
}

#[test]
fn test_version() {
    let mut cmd = get_binary();
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("rdm"));
}

// ============================================================================
// Profile Commands
// ============================================================================

#[test]
fn test_profile_list_empty() {
    let temp = tempfile::tempdir().unwrap();
    let mut cmd = get_binary();
    cmd.env("APPDATA", temp.path())
        .env("LOCALAPPDATA", temp.path())
        .env_remove("REDMINE_URL")
        .env_remove("REDMINE_API_KEY")
        .arg("profile")
        .arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No profiles"));
}
