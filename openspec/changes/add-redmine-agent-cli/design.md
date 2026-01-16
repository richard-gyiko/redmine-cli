# Design: Agent-First Redmine CLI

## Context
Building a new CLI tool from scratch in Rust for agent/LLM automation. Must be fast, deterministic, and produce well-structured Markdown output that agents can easily parse and act upon. Target platforms: Windows x86_64 and Linux x86_64 (musl for portability).

## Goals / Non-Goals

### Goals
- Sub-100ms startup time
- Zero runtime dependencies (static linking)
- **Markdown-first output** that agents can parse and understand contextually
- JSON output available for programmatic pipelines
- Robust error handling with typed exit codes and actionable messages
- Activity name-to-ID resolution with disk caching

### Non-Goals
- Interactive wizards or prompts
- GUI or TUI
- Plugin system
- Real-time notifications
- Human-optimized "pretty" output (agents are primary consumers)

## Decisions

### Decision: Markdown as default output format
**Rationale**: LLMs and agents work best with structured text they can parse contextually. Markdown provides:
- Clear section headers for navigation
- Tables for structured data that agents can reason about
- Consistent patterns across commands
- Actionable context (IDs, follow-up commands)
- Human-readable fallback when debugging

### Decision: Use rustls instead of OpenSSL
**Rationale**: Avoids OpenSSL runtime dependency issues, especially on Linux musl targets. Pure Rust implementation is easier to cross-compile.

### Decision: Clap derive macros for CLI parsing
**Rationale**: Type-safe, generates help text, supports subcommand nesting. Industry standard for Rust CLIs.

### Decision: figment for configuration layering
**Rationale**: Supports multiple sources (env, file, CLI) with clear precedence. Works well with serde.

### Decision: chrono for date handling
**Rationale**: Mature, well-tested, handles local timezone conversion needed for `--spent-on` defaults.

### Decision: JSON envelope for `--format json`
**Rationale**: When agents need programmatic parsing (piping to jq, etc.), JSON envelope provides structured data with ok/data/meta/error fields.

### Decision: Cache activities to disk with 24h TTL
**Rationale**: Activities rarely change but are needed for every time entry. Caching reduces API calls and enables offline name resolution.

## Output Format Examples

### Markdown (Default)

**List command:**
```markdown
## Issues in project "webapp" (showing 1-3 of 47)

| ID | Subject | Status | Assignee | Updated |
|----|---------|--------|----------|---------|
| 123 | Fix login bug | In Progress | john.doe | 2024-01-15 |
| 124 | Add dark mode | New | jane.smith | 2024-01-14 |
| 125 | Update docs | Closed | john.doe | 2024-01-13 |

*Use `rma issue list --offset 3` for next page*
```

**Get command:**
```markdown
## Issue #123: Fix login bug

| Field | Value |
|-------|-------|
| Status | In Progress |
| Priority | High |
| Assignee | john.doe |
| Project | webapp |
| Created | 2024-01-10 |
| Updated | 2024-01-15 |

### Description
Users cannot log in when using SSO authentication...

*Use `rma issue update --id 123 --status closed` to close*
```

**Create/Update confirmation:**
```markdown
## Time Entry Created

| Field | Value |
|-------|-------|
| ID | 456 |
| Issue | #123 - Fix login bug |
| Hours | 2.5 |
| Activity | Development |
| Date | 2024-01-15 |
| Comment | Code review |

*Use `rma time get --id 456` to view details*
```

**Error output:**
```markdown
> **Error: NOT_FOUND**
> Issue #99999 not found
>
> The issue may have been deleted or you may not have permission to view it.
> Use `rma issue list --project <project>` to find available issues.
```

### JSON (--format json)

```json
{
  "ok": true,
  "data": { ... },
  "meta": { "total_count": 47, "limit": 25, "offset": 0, "next_offset": 25 },
  "error": null
}
```

## Crate Layout

```
src/
├── main.rs           # Entry point, clap setup
├── cli/
│   ├── mod.rs        # Command enum
│   ├── ping.rs       # Diagnostic commands
│   ├── profile.rs    # Config/profile management
│   ├── project.rs    # Project commands
│   ├── issue.rs      # Issue commands
│   └── time.rs       # Time entry commands
├── client/
│   ├── mod.rs        # RedmineClient struct
│   └── endpoints.rs  # API endpoint implementations
├── models/
│   ├── mod.rs
│   ├── issue.rs      # Issue, Tracker, Status, Priority
│   ├── project.rs    # Project
│   ├── time_entry.rs # TimeEntry, Activity
│   └── user.rs       # User
├── output/
│   ├── mod.rs
│   ├── envelope.rs   # Envelope<T>, Meta, ErrorInfo (for JSON)
│   ├── markdown.rs   # Markdown formatters for each model
│   └── format.rs     # Format trait and dispatcher
├── config/
│   ├── mod.rs
│   ├── profile.rs    # Profile struct, CRUD
│   └── loader.rs     # Precedence logic
├── cache/
│   └── mod.rs        # Activity cache with TTL
└── error.rs          # AppError enum, exit code mapping
```

## Core Types

```rust
// Output format selection
pub enum OutputFormat {
    Markdown,  // Default
    Json,
}

// For JSON output
pub struct Envelope<T> {
    pub ok: bool,
    pub data: Option<T>,
    pub meta: Meta,
    pub error: Option<ErrorInfo>,
}

pub struct Meta {
    pub total_count: Option<u32>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub next_offset: Option<u32>,
}

pub struct ErrorInfo {
    pub code: String,      // AUTH_MISSING, NOT_FOUND, etc.
    pub message: String,
    pub details: Value,    // serde_json::Value for flexibility
}

// Markdown formatting trait
pub trait MarkdownOutput {
    fn to_markdown(&self, meta: &Meta) -> String;
}

// Exit codes
pub enum ExitCode {
    Success = 0,
    Validation = 2,
    Auth = 3,
    NotFound = 4,
    ApiError = 5,
}
```

## HTTP Client Configuration

```rust
// reqwest client setup
Client::builder()
    .use_rustls_tls()
    .timeout(Duration::from_secs(30))
    .connect_timeout(Duration::from_secs(10))
    .user_agent(format!("rma/{}", env!("CARGO_PKG_VERSION")))
    .gzip(true)
    .build()
```

Retry policy: Exponential backoff for 502/503/504, max 3 attempts.

## Configuration Precedence

1. CLI flags (`--url`, `--api-key`)
2. Environment variables (`REDMINE_URL`, `REDMINE_API_KEY`)
3. Active profile from config file
4. Error if no credentials found

Config paths:
- Linux: `~/.config/redmine-agent-cli/config.toml`
- Windows: `%APPDATA%\redmine-agent-cli\config.toml`

## Risks / Trade-offs

### Risk: Markdown parsing edge cases
**Mitigation**: Use consistent patterns. Escape special characters in user content. Test with various content types.

### Risk: Activity cache staleness
**Mitigation**: 24h TTL is conservative. Add `--refresh` flag to force cache invalidation. Include cache age in meta output.

### Risk: Large response pagination
**Mitigation**: Default limit of 25, max 100. Include next page hint in Markdown output. Include `next_offset` in JSON meta.

### Risk: Redmine API inconsistencies across versions
**Mitigation**: Target Redmine 4.x+ REST API. Document minimum version. Handle missing fields gracefully with Option types.

## Migration Plan
N/A - Greenfield project.

## Open Questions
- Should we support custom fields on time entries? (Defer to post-MVP)
- Should we add `--quiet` mode that suppresses all output? (Consider for v1.1)
