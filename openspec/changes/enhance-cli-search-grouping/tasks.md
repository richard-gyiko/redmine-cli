# Implementation Tasks

## 1. Display User in Time Entry List
- [x] 1.1 Update `TimeEntryList::to_markdown()` to include User column in table
- [x] 1.2 Ensure user name is properly truncated for table readability
- [x] 1.3 Update golden tests for time list output

## 2. Custom Field Support (Foundation)
- [x] 2.1 Add `CustomField` struct to models (`id`, `name`, `value`, `multiple`)
- [x] 2.2 Add `custom_fields: Option<Vec<CustomField>>` to `Issue` model
- [x] 2.3 Add `custom_fields: Option<Vec<CustomField>>` to `TimeEntry` model
- [x] 2.4 Display custom fields in `Issue::to_markdown()` detail view
- [x] 2.5 Display custom fields in `TimeEntry::to_markdown()` detail view
- [x] 2.6 Add `--cf <id>=<value>` filter argument to `IssueListArgs`
- [x] 2.7 Implement custom field filter in `IssueFilters` query building
- [x] 2.8 Add `--cf <id>=<value>` filter argument to `TimeListArgs`
- [x] 2.9 Implement custom field filter for time entries query building

## 3. Issue Subject Search
- [x] 3.1 Add `--subject <text>` argument to `IssueListArgs` for exact match
- [x] 3.2 Implement subject filter in `IssueFilters` query params
- [x] 3.3 Add `--search <text>` argument to `IssueListArgs` for text search
- [x] 3.4 Implement `/search.json` endpoint integration in client
- [x] 3.5 Create `SearchResult` model for search response
- [x] 3.6 ~~Add `rdm issue search` command~~ Covered by `--search` flag on `issue list`

## 4. Time Entry Grouping and Totals
- [x] 4.1 Add `--group-by <field>` argument to `TimeListArgs`
- [x] 4.2 Define allowed group-by values: user, project, activity, issue, spent_on, cf_<id>
- [x] 4.3 Implement client-side grouping logic in time list handler
- [x] 4.4 Create `GroupedTimeEntries` model with groups and totals
- [x] 4.5 Implement `GroupedTimeEntries::to_markdown()` with subtotals and grand total
- [x] 4.6 Update `TimeEntryList::to_markdown()` to always show total hours summary

## 5. User List Command
- [x] 5.1 Create `src/cli/user.rs` with `UserCommand` enum
- [x] 5.2 Add `UserListArgs` with `--status` and `--limit/--offset` options
- [x] 5.3 Implement `list_users()` endpoint in client
- [x] 5.4 Create `UserList` model with markdown output
- [x] 5.5 Register `user` subcommand in main CLI

## 6. Testing
- [x] 6.1 Unit tests pass (32 tests)
- [x] 6.2 Golden tests pass (19/20 - 1 pre-existing failure unrelated to this change)
- [x] 6.3 Build succeeds with only unused import warnings (for public API exports)

## 7. Documentation
- [x] 7.1 Update README with new commands and options
- [x] 7.2 Add examples for common search and grouping use cases
