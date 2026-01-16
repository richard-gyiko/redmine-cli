# Tasks: Build Agent-First Redmine CLI

## 1. Foundation
- [x] 1.1 Initialize Cargo project with dependencies (clap, reqwest, serde, figment, tokio, chrono, tracing, directories, backoff)
- [x] 1.2 Implement unified error type (`AppError`) with exit code mapping
- [x] 1.3 Implement output envelope types (`Envelope<T>`, `Meta`, `ErrorInfo`)
- [x] 1.4 Implement JSON/table output formatting
- [x] 1.5 Implement config loader with env/file/CLI precedence
- [x] 1.6 Implement profile CRUD operations
- [x] 1.7 Implement `RedmineClient` with retry/backoff and rustls TLS
- [x] 1.8 Implement `rdm ping` command
- [x] 1.9 Implement `rdm me` command
- [x] 1.10 Implement `rdm profile add/use/list/delete` commands
- [x] 1.11 Implement `rdm config show` command

## 2. Time Entries
- [x] 2.1 Implement Activity model and cache with TTL
- [x] 2.2 Implement `rdm time activities list` command
- [x] 2.3 Implement TimeEntry model
- [x] 2.4 Implement `rdm time create` command with activity name resolution
- [x] 2.5 Implement `rdm time list` command with filters
- [x] 2.6 Implement `rdm time get` command
- [x] 2.7 Implement `rdm time update` command
- [x] 2.8 Implement `rdm time delete` command

## 3. Projects
- [x] 3.1 Implement Project model
- [x] 3.2 Implement `rdm project list` command with pagination
- [x] 3.3 Implement `rdm project get` command (by id or identifier)

## 4. Issues
- [x] 4.1 Implement Issue model with related types (Tracker, Status, Priority)
- [x] 4.2 Implement `rdm issue list` command with filters
- [x] 4.3 Implement `rdm issue get` command
- [x] 4.4 Implement `rdm issue create` command
- [x] 4.5 Implement `rdm issue update` command

## 5. Hardening & Release
- [x] 5.1 Add `--debug` flag with request/response tracing to stderr
- [x] 5.2 Add `--dry-run` flag that prints request without executing
- [x] 5.3 Add unit tests for config precedence
- [ ] 5.4 Add unit tests for argument validation
- [x] 5.5 Add unit tests for envelope serialization
- [x] 5.6 Add unit tests for exit code mapping
- [ ] 5.7 Add integration tests with wiremock for time entry CRUD
- [ ] 5.8 Add integration tests for issue commands
- [ ] 5.9 Add integration tests for project commands
- [ ] 5.10 Add golden JSON output tests
- [ ] 5.11 Setup GitHub Actions CI for Windows and Linux musl builds
- [ ] 5.12 Configure cargo-dist for release automation
- [ ] 5.13 Create initial release with binaries and SHA256SUMS
