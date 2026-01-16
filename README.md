# rdm - Redmine CLI

An agent-first Redmine CLI with markdown-optimized output.

## Features

- **Markdown-first output** - Default format optimized for LLM/agent consumption
- **JSON envelope format** - Structured output for programmatic pipelines with `--format json`
- **Full CRUD operations** - Issues, time entries, and projects
- **Multiple profiles** - Manage credentials for different Redmine instances
- **Activity caching** - 24-hour TTL cache for time entry activities
- **Standardized exit codes** - Predictable error handling for scripts
- **Cross-platform static binaries** - Linux (musl) and Windows (MSVC)

## Installation

### Windows (Scoop)

```powershell
scoop bucket add rdm https://github.com/richard-gyiko/redmine-cli
scoop install rdm
```

### From GitHub Releases

Download the latest binary for your platform from [GitHub Releases](https://github.com/richard-gyiko/redmine-cli/releases).

### From Source

```bash
# Clone the repository
git clone https://github.com/richard-gyiko/redmine-cli.git
cd redmine-cli

# Install with cargo
cargo install --path .
```

## Quick Start

### 1. Set up credentials

**Option A: Environment variables**

```bash
export REDMINE_URL=https://redmine.example.com
export REDMINE_API_KEY=your-api-key
```

**Option B: Create a profile**

```bash
rdm profile add --name work --url https://redmine.example.com --api-key your-api-key
```

### 2. Test the connection

```bash
rdm ping
```

### 3. View your user info

```bash
rdm me
```

### 4. List your open issues

```bash
rdm issue list --assigned-to me --status open
```

### 5. Log time

```bash
rdm time create --issue 123 --hours 2.5 --activity Development --comment "Implemented feature X"
```

## Commands Reference

### General

| Command | Description |
|---------|-------------|
| `rdm ping` | Check connection and authentication |
| `rdm me` | Show current user information |
| `rdm config` | Show current configuration |

### Profile Management

| Command | Description |
|---------|-------------|
| `rdm profile add` | Add a new profile |
| `rdm profile use <name>` | Set the active profile |
| `rdm profile list` | List all profiles |
| `rdm profile delete` | Delete a profile |

### Projects

| Command | Description |
|---------|-------------|
| `rdm project list` | List projects |
| `rdm project get` | Get project details |

### Issues

| Command | Description |
|---------|-------------|
| `rdm issue list` | List issues with filters |
| `rdm issue get` | Get issue details |
| `rdm issue create` | Create a new issue |
| `rdm issue update` | Update an existing issue |

### Time Entries

| Command | Description |
|---------|-------------|
| `rdm time list` | List time entries |
| `rdm time get` | Get time entry details |
| `rdm time create` | Create a time entry |
| `rdm time update` | Update a time entry |
| `rdm time delete` | Delete a time entry |
| `rdm time activities list` | List available activities |

## Configuration

### Environment Variables

| Variable | Description |
|----------|-------------|
| `REDMINE_URL` | Redmine server URL |
| `REDMINE_API_KEY` | Your Redmine API key |

### Profiles

Profiles are stored in the configuration file and allow managing multiple Redmine instances:

```bash
# Add profiles
rdm profile add --name work --url https://work.redmine.com --api-key key1
rdm profile add --name personal --url https://personal.redmine.com --api-key key2

# Switch between profiles
rdm profile use personal

# List profiles
rdm profile list
```

### Configuration Precedence

1. CLI flags (`--url`, `--api-key`)
2. Environment variables (`REDMINE_URL`, `REDMINE_API_KEY`)
3. Config file (active profile)

## Output Formats

### Markdown (default)

The default output format is markdown, optimized for readability and LLM consumption:

```bash
$ rdm issue get --id 123
```

```markdown
## Issue #123

| Field | Value |
|-------|-------|
| Subject | Fix login bug |
| Project | Backend |
| Status | In Progress |
| Priority | High |
| Assignee | John Doe |
```

### JSON (`--format json`)

Use the `--format json` flag for structured output:

```bash
$ rdm issue get --id 123 --format json
```

```json
{
  "ok": true,
  "data": {
    "id": 123,
    "subject": "Fix login bug",
    "project": { "id": 1, "name": "Backend" },
    "status": { "id": 2, "name": "In Progress" },
    "priority": { "id": 3, "name": "High" }
  },
  "meta": {}
}
```

For list operations, the envelope includes pagination metadata:

```json
{
  "ok": true,
  "data": { "issues": [...] },
  "meta": {
    "total_count": 150,
    "limit": 25,
    "offset": 0,
    "next_offset": 25
  }
}
```

Error responses follow the same envelope structure:

```json
{
  "ok": false,
  "error": {
    "code": "NOT_FOUND",
    "message": "Not found: Issue #999"
  },
  "meta": {}
}
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 2 | Validation/argument error |
| 3 | Authentication/configuration error |
| 4 | Resource not found |
| 5 | API/server/network error |

## Examples

### List my open issues assigned to me

```bash
rdm issue list --assigned-to me --status open
```

### List issues in a specific project

```bash
rdm issue list --project backend --status "*"
```

### Create an issue

```bash
rdm issue create \
  --project 1 \
  --subject "Implement user authentication" \
  --description "Add OAuth2 support" \
  --tracker 1 \
  --priority 2 \
  --assigned-to 5
```

### Update an issue

```bash
rdm issue update --id 123 --status 3 --done-ratio 50 --notes "Halfway done"
```

### Log time to an issue

```bash
rdm time create --issue 123 --hours 2.5 --activity Development
```

### Log time to a project (no issue)

```bash
rdm time create --project 1 --hours 1.0 --activity Meeting --comment "Sprint planning"
```

### List time entries for a date range

```bash
rdm time list --user me --from 2024-01-01 --to 2024-01-31
```

### Use in shell scripts

```bash
#!/bin/bash
set -e

# Check connection
rdm ping --format json | jq -e '.ok' > /dev/null

# Get issue and extract status
STATUS=$(rdm issue get --id 123 --format json | jq -r '.data.status.name')
echo "Issue status: $STATUS"
```

### Dry run mode

Preview what would be sent without executing:

```bash
rdm issue create --project 1 --subject "Test" --dry-run
```

### Debug mode

Enable debug logging to stderr:

```bash
rdm --debug issue list
```

## Building from Source

### Prerequisites

- Rust 1.70 or later

### Build

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

The binary will be at `target/release/rdm` (or `target/release/rdm.exe` on Windows).

### Run Tests

```bash
cargo test
```

### Cross-compilation Targets

The project is configured for:

- `x86_64-unknown-linux-musl` - Linux static binary
- `x86_64-pc-windows-msvc` - Windows binary

## License

MIT
