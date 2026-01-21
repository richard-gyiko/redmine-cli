# Change: Enhance CLI with Search, Grouping, and Custom Fields

## Why

The current CLI has basic filtering but lacks important capabilities for real-world Redmine workflows:
1. Time entries show user info in detail view but not in list view, making it hard to review team time
2. No way to search issues by subject/title (partial or full match)
3. No aggregation or grouping for time entries - users can't see time breakdowns by project, user, activity, etc.
4. Custom fields are completely unsupported despite being heavily used in most Redmine installations

## What Changes

### 1. Display User in Time Entry List
- Add user column to `rdm time list` markdown table output
- User info already exists in API response, just not displayed

### 2. Issue Subject Search
- Add `--subject` filter to `rdm issue list` for exact match
- Add `--search` filter for partial/fuzzy text search via Redmine's `/search.json` endpoint
- Support both approaches since Redmine API handles them differently

### 3. Time Entry Grouping and Totals
- Add summary section showing total hours at end of `rdm time list`
- Add `--group-by <field>` flag supporting: `user`, `project`, `activity`, `issue`, `spent_on`, or custom field ID (`cf_<id>`)
- Group results and show subtotals per group with grand total

### 4. Custom Field Support
- Add `custom_fields` to Issue and TimeEntry models
- Display custom fields in detail views (`rdm issue get`, `rdm time get`)
- Support `--cf <id>=<value>` filter in `rdm issue list` and `rdm time list`
- Support `--group-by cf_<id>` for time entry grouping

### 5. User Search (Limited)
- **Note**: Redmine API does not support user search/filtering
- Add `rdm user list [--status active|registered|locked]` to list users
- Client-side filtering can be added later if needed

## Impact

- **Affected specs**: `cli-core`
- **Affected code**:
  - `src/models/time_entry.rs` - add custom_fields, update markdown output
  - `src/models/issue.rs` - add custom_fields, update markdown output
  - `src/cli/time.rs` - add --group-by flag
  - `src/cli/issue.rs` - add --subject, --search, --cf filters
  - `src/cli/user.rs` (new) - user list command
  - `src/client/endpoints.rs` - add search endpoint, custom field filters

## API Research Summary

| Feature | Redmine API Support | Implementation |
|---------|---------------------|----------------|
| Time entry user info | Yes - included in response | Display in list |
| User search by name | **No** - only status filter | Fetch all + client filter |
| Issue subject filter | Partial - exact match only | Use `subject=` parameter |
| Issue text search | Yes - via `/search.json` | New endpoint integration |
| Custom fields in response | Yes - `custom_fields` array | Parse and display |
| Custom field filters | Yes - `cf_<id>=<value>` | Add to query params |
| Server-side grouping | **No** | Client-side grouping |

## Risks

- **Performance**: User list requires fetching all users (pagination) for client-side filtering. Mitigated by status filter.
- **Custom field types**: Multiple value types exist (string, list, date, etc.). May need type-aware formatting.
- **API variations**: Some undocumented parameters vary by Redmine version. Testing recommended.
