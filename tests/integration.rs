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
