## Proposal: Build a Modern, Agent-First Redmine CLI in Rust (Windows + Linux) With Full Time Entry Support

### 1) Objective

Build a **fast, modern, cross-platform CLI** for Redmine designed primarily for **LLM/agent execution** (non-interactive by default), with first-class support for **time entry management** and core issue workflows.

### 2) Primary Use Case

* An LLM/agent calls this CLI repeatedly in an automation loop.
* Must be:

  * Deterministic
  * Fast startup
  * Stable JSON outputs
  * Robust error handling
* Must run on:

  * **Windows x86_64**
  * **Linux x86_64** (including typical servers/containers)

### 3) Non-Functional Requirements

**Performance**

* Single native binary.
* Quick startup and low overhead.

**Reliability / Determinism**

* Default output **JSON** with a stable envelope schema.
* No interactive prompts unless explicitly requested.

**Security**

* API key never printed.
* Support env vars + config file + profiles.

**Compatibility**

* Use Redmine REST API (no scraping).
* Avoid instance-specific assumptions.

### 4) Technology Stack (Rust)

**Core**

* Rust 2021 edition
* **CLI:** `clap` (derive)
* **HTTP:** `reqwest` + `tokio`
* **JSON:** `serde` + `serde_json`
* **Config:** `figment` (or `config`) + `serde`
* **Directories:** `directories` (cross-platform config/cache paths)
* **Dates:** `time` (or `chrono`; pick one and standardize)
* **Logging (stderr):** `tracing` + `tracing-subscriber` (only enabled with `--debug`)
* **Retry/backoff:** `backoff` (or minimal custom exponential backoff)

**TLS choice (important for distribution)**

* Use **rustls** TLS to avoid OpenSSL runtime dependency problems.
* Configure `reqwest` features to prefer `rustls-tls` and avoid pulling in OpenSSL.

### 5) Distribution Plan (Minimal Fuss)

Deliver prebuilt binaries via GitHub Releases, built in CI:

**Targets**

* Windows: `x86_64-pc-windows-msvc`
* Linux: `x86_64-unknown-linux-musl` (preferred for portability) + rustls TLS

**Release Artifacts**

* `rma-windows-x86_64.zip` containing `rma.exe`
* `rma-linux-x86_64.tar.gz` containing `rma`
* `SHA256SUMS`

**CI/Release Automation**

* Prefer `cargo-dist` to automate packaging and release artifacts with minimal maintenance.
* Alternative: GitHub Actions matrix + manual packaging steps.

Optional later:

* Scoop manifest for Windows.
* `install.sh` / `install.ps1` one-liners.

### 6) CLI Design Principles (Agent-First)

1. **JSON output default**

   * `--format json|table` (default `json`)
2. **Stable output envelope**

   * All commands return:

     * `ok` (bool)
     * `data` (object/array or null)
     * `meta` (object)
     * `error` (object or null)
3. **Exit codes**

   * `0` success
   * `2` validation / bad args
   * `3` auth/config errors
   * `4` not found
   * `5` API/server/network errors
4. **Non-interactive by default**

   * No prompts unless `--interactive`.
5. **Idempotence and explicitness**

   * No hidden side effects; explicit flags.
6. **Debugging**

   * `--debug` writes request/response metadata to **stderr**.
   * `--dry-run` prints method/url/body and does not execute.

### 7) Authentication / Configuration

**Environment Variables**

* `REDMINE_URL`
* `REDMINE_API_KEY`
* Optional: `REDMINE_PROFILE`

**Config File**

* Linux: `~/.config/redmine-agent-cli/config.toml`
* Windows: `%APPDATA%\redmine-agent-cli\config.toml`

**Profiles**

* `profile add --name work --url ... --api-key ...`
* `profile use work`
* `profile list`
* `profile delete --name work`
* `config show`

**Precedence**

1. CLI flags
2. Env vars
3. Active profile in config
4. Error if missing

### 8) Command Tree (MVP)

Binary name: `rma` (Redmine Agent).

#### 8.1 Diagnostics

* `rma ping`

  * Verifies connectivity and auth.
* `rma me`

  * Returns authenticated user.

#### 8.2 Projects

* `rma project list [--limit N --offset N]`
* `rma project get --id <id> | --identifier <identifier>`

#### 8.3 Issues

* `rma issue get --id <id>`
* `rma issue list`

  * Filters:

    * `--project <id|identifier>`
    * `--status <open|closed|*|status_id>`
    * `--assigned-to <me|user_id>`
    * `--author <me|user_id>`
    * `--limit`, `--offset`
* `rma issue create`

  * Required:

    * `--project <id|identifier>`
    * `--subject <text>`
  * Optional:

    * `--description <text>`
    * `--assigned-to <me|user_id>`
    * `--priority <name|id>`
    * `--status <name|id>`
    * `--tracker <name|id>`
    * `--cf <key=value>` (repeatable)
* `rma issue update --id <id>`

  * Optional:

    * `--subject`, `--description`, `--status`, `--assigned-to`, `--priority`, `--notes`, `--cf key=value`

#### 8.4 Time Entries (Core Requirement: Full CRUD)

* `rma time activities list`

  * Lists available time entry activities with IDs and names.
  * Must cache results on disk with TTL (default 24h).

* `rma time create`

  * Required:

    * `--hours <float>` (> 0)
    * `--spent-on <YYYY-MM-DD>` (default today local time)
    * `--activity <name|id>` (resolve name to ID; use cache)
  * One of:

    * `--issue <issue_id>` OR `--project <project_id|identifier>`
  * Optional:

    * `--comment <text>`
    * `--user <me|user_id>` (attempt if API allows; otherwise report in meta)

* `rma time list`

  * Filters:

    * `--issue <id>`
    * `--project <id|identifier>`
    * `--user <me|user_id>`
    * `--from <YYYY-MM-DD>`
    * `--to <YYYY-MM-DD>`
    * `--limit`, `--offset`

* `rma time get --id <time_entry_id>`

* `rma time update --id <time_entry_id>`

  * Fields:

    * `--hours`
    * `--spent-on`
    * `--activity <name|id>`
    * `--comment`

* `rma time delete --id <time_entry_id>`

### 9) Output Contract (Canonical JSON)

**Success**

```json
{
  "ok": true,
  "data": {},
  "meta": {},
  "error": null
}
```

**Failure**

```json
{
  "ok": false,
  "data": null,
  "meta": {},
  "error": {
    "code": "AUTH_MISSING|NOT_FOUND|VALIDATION|API_ERROR|NETWORK_ERROR",
    "message": "summary",
    "details": {}
  }
}
```

**List command meta**

* `meta.total_count`
* `meta.limit`
* `meta.offset`
* `meta.next_offset` (null if none)

### 10) Rust Implementation Guidance (Structure)

**Crate layout**

* `src/main.rs` – CLI entry
* `src/cli/` – clap commands and argument parsing
* `src/client/` – `RedmineClient` + request helpers
* `src/models/` – serde structs for Redmine objects (Issue, Project, TimeEntry, Activity)
* `src/output/` – envelope types + JSON/table formatting
* `src/config/` – profile/config management + env precedence
* `src/cache/` – activity cache file + TTL logic
* `src/error.rs` – unified error type + mapping to exit codes

**Core types**

* `Envelope<T>`
* `Meta`
* `ErrorInfo`
* `RedmineClient` with:

  * base_url
  * api_key
  * request timeout
  * retry policy

**HTTP requirements**

* timeouts (connect + overall)
* retries for 502/503/504 + transient network errors (max 3)
* `User-Agent: rma/<version>`
* gzip enabled

### 11) Validation Rules

* Date format: `YYYY-MM-DD`
* hours: float > 0
* mutually exclusive args enforced (`--issue` vs `--project`)
* activity name resolution:

  * if unknown, return VALIDATION error suggesting `rma time activities list`

### 12) Testing Strategy (Acceptance Criteria)

**Unit tests**

* config precedence
* argument validation
* envelope serialization
* exit code mapping

**Integration tests**

* Use `wiremock` (or `httpmock`) to simulate Redmine endpoints.
* Golden JSON output tests to enforce schema stability.
* Must cover:

  * time activities caching + resolution
  * time create/list/get/update/delete
  * issue list/get/create/update
  * project list/get

**Definition of Done**

* All MVP commands implemented.
* JSON envelope consistent across commands.
* Time entry CRUD fully functional and tested.
* Prebuilt binaries produced for Windows + Linux musl and attached to release with checksums.

### 13) Milestones

1. **Foundation**

   * clap CLI skeleton, config/profiles, client, envelope, ping/me.
2. **Time Entries**

   * activities list + cache, CRUD for time entries.
3. **Issues/Projects**

   * project list/get, issue list/get/create/update.
4. **Hardening + Release**

   * retries, debug/dry-run, CI builds, release artifacts.