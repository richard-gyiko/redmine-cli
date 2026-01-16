# Change: Build Agent-First Redmine CLI

## Why
LLM/agent automation workflows need a fast, deterministic CLI to interact with Redmine. Existing tools are either interactive, produce unstructured output, or optimize for human readability over machine parsing. Agents work best with well-formatted Markdown that provides context, structure, and clear data presentation.

## What Changes
- **NEW**: Complete Rust CLI application (`rdm`) from scratch
- **NEW**: **Markdown-first output** optimized for LLM/agent consumption (default format)
- **NEW**: Profile-based configuration with env var + config file support
- **NEW**: Redmine API client with retry/backoff and rustls TLS
- **NEW**: Structured output with clear headers, tables, and contextual information
- **NEW**: Full CRUD for time entries with activity caching
- **NEW**: Issue and project read/write commands
- **NEW**: Cross-platform distribution (Windows MSVC + Linux musl)
- **NEW**: JSON output available via `--format json` for programmatic pipelines

## Impact
- Affected specs: `cli-core` (new capability)
- Affected code: Entire crate (greenfield project)
- Breaking changes: None (new project)

## Scope

### MVP Commands
1. **Diagnostics**: `ping`, `me`
2. **Profiles**: `profile add/use/list/delete`, `config show`
3. **Projects**: `project list/get`
4. **Issues**: `issue list/get/create/update`
5. **Time Entries**: `time activities list`, `time create/list/get/update/delete`

### Non-Goals (Post-MVP)
- Wiki/attachment support
- Custom query saving
- Interactive mode
- Shell completions
