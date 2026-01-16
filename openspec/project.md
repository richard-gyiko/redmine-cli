# Project Context

## Purpose
Build a fast, modern, cross-platform CLI for Redmine (`rma`) designed primarily for LLM/agent execution with first-class support for time entry management and core issue workflows.

## Tech Stack
- Rust 2021 edition
- CLI: `clap` (derive)
- HTTP: `reqwest` + `tokio` with `rustls-tls`
- JSON: `serde` + `serde_json`
- Config: `figment` + `serde`
- Directories: `directories` (cross-platform config/cache paths)
- Dates: `chrono`
- Logging: `tracing` + `tracing-subscriber`
- Retry: `backoff`

## Project Conventions

### Code Style
- Follow Rust idioms and clippy recommendations
- Use `Result<T, E>` for fallible operations
- Derive traits where possible (`serde::Serialize`, `serde::Deserialize`, `clap::Args`)
- Module-per-concern organization

### Architecture Patterns
- Command pattern via clap subcommands
- Single `RedmineClient` for all API interactions
- Unified `Envelope<T>` response wrapper
- Exit codes mapped from error types

### Testing Strategy
- Unit tests for config precedence, validation, serialization
- Integration tests with `wiremock` for API mocking
- Golden JSON output tests for schema stability

### Git Workflow
- Feature branches off main
- Conventional commits (feat:, fix:, docs:, refactor:)
- PR-based workflow

## Domain Context
- Redmine is a project management web application with REST API
- Time entries track work hours against issues or projects
- Activities are predefined categories for time entries (Development, Design, etc.)
- Projects contain issues, issues have trackers/statuses/priorities

## Important Constraints
- Non-interactive by default (agent-first)
- JSON output with stable envelope schema
- API key never printed to stdout/stderr
- Cross-platform: Windows x86_64 + Linux x86_64 musl

## External Dependencies
- Redmine REST API (v4.x+ recommended)
- No external runtime dependencies (static binary with rustls)
