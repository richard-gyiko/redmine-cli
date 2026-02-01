## ADDED Requirements

### Requirement: Time Entry List User Display
The time entry list command SHALL display the user who logged each entry in the output table.

#### Scenario: Time list shows user column
- **WHEN** `rdm time list --project myproject` is executed
- **THEN** the output table includes a User column showing the name of who logged each entry

#### Scenario: Time list with multiple users
- **WHEN** `rdm time list --from 2024-01-01 --to 2024-01-31` is executed
- **THEN** each row shows the respective user who created that time entry

### Requirement: Issue Subject Search
The CLI SHALL support searching issues by subject text with both exact and partial matching options.

#### Scenario: Exact subject match filter
- **WHEN** `rdm issue list --subject "Login bug"` is executed
- **THEN** only issues with subject exactly matching "Login bug" are returned

#### Scenario: Partial text search
- **WHEN** `rdm issue list --search "login"` is executed
- **THEN** issues containing "login" in subject or description are returned via Redmine search

#### Scenario: Search with project scope
- **WHEN** `rdm issue list --search "error" --project myproject` is executed
- **THEN** only issues in myproject containing "error" are returned

### Requirement: Time Entry Grouping
The CLI SHALL support grouping time entries by a specified field with subtotals per group and a grand total.

#### Scenario: Group by user
- **WHEN** `rdm time list --project myproject --group-by user` is executed
- **THEN** entries are grouped by user with subtotal hours per user and grand total at end

#### Scenario: Group by project
- **WHEN** `rdm time list --user me --group-by project` is executed
- **THEN** entries are grouped by project with subtotal hours per project and grand total at end

#### Scenario: Group by activity
- **WHEN** `rdm time list --from 2024-01-01 --to 2024-01-31 --group-by activity` is executed
- **THEN** entries are grouped by activity type with subtotal hours per activity and grand total at end

#### Scenario: Group by date
- **WHEN** `rdm time list --user me --group-by spent_on` is executed
- **THEN** entries are grouped by date with subtotal hours per date and grand total at end

#### Scenario: Group by custom field
- **WHEN** `rdm time list --project myproject --group-by cf_5` is executed
- **THEN** entries are grouped by custom field ID 5 value with subtotals per value and grand total

### Requirement: Time Entry Summary Totals
The time entry list command SHALL always display a summary of total hours at the end of the output.

#### Scenario: List shows total hours
- **WHEN** `rdm time list --user me --from 2024-01-01 --to 2024-01-07` is executed
- **THEN** the output includes a total hours summary (e.g., "**Total: 32.5 hours**")

### Requirement: Custom Field Support
The CLI SHALL support custom fields in issue and time entry operations including display, filtering, and grouping.

#### Scenario: Issue detail shows custom fields
- **WHEN** `rdm issue get --id 123` is executed for an issue with custom fields
- **THEN** custom fields are displayed in the output with their names and values

#### Scenario: Time entry detail shows custom fields
- **WHEN** `rdm time get --id 456` is executed for a time entry with custom fields
- **THEN** custom fields are displayed in the output with their names and values

#### Scenario: Filter issues by custom field
- **WHEN** `rdm issue list --cf 5=urgent` is executed
- **THEN** only issues where custom field ID 5 equals "urgent" are returned

#### Scenario: Filter time entries by custom field
- **WHEN** `rdm time list --cf 10=billable` is executed
- **THEN** only time entries where custom field ID 10 equals "billable" are returned

#### Scenario: Multiple custom field filters
- **WHEN** `rdm issue list --cf 5=urgent --cf 6=backend` is executed
- **THEN** only issues matching both custom field criteria are returned

### Requirement: User List Command
The CLI SHALL provide a command to list users from Redmine with basic filtering options.

#### Scenario: List all active users
- **WHEN** `rdm user list` is executed
- **THEN** active users are returned in a Markdown table with ID, Login, Name, and Email columns

#### Scenario: Filter users by status
- **WHEN** `rdm user list --status locked` is executed
- **THEN** only users with locked status are returned

#### Scenario: Paginate user list
- **WHEN** `rdm user list --limit 10 --offset 20` is executed
- **THEN** users 21-30 are returned with pagination metadata

## MODIFIED Requirements

### Requirement: Time Entry Commands
The CLI SHALL provide full CRUD for time entries:
- `time create --hours <float> --activity <name|id>` with one of `--issue <id>` or `--project <id>`, optional: `--spent-on`, `--comment`, `--user`
- `time list` with filters: `--issue`, `--project`, `--user`, `--from`, `--to`, `--limit`, `--offset`, `--cf <id>=<value>`, `--group-by <field>`
- `time get --id <id>`
- `time update --id <id>` with optional: `--hours`, `--spent-on`, `--activity`, `--comment`
- `time delete --id <id>`

#### Scenario: Create time entry with activity name
- **WHEN** `rdm time create --issue 123 --hours 2.5 --activity Development --comment "Code review"` is executed
- **THEN** a time entry is created and the response confirms with entry ID and summary

#### Scenario: Create time entry defaults to today
- **WHEN** `rdm time create --issue 123 --hours 1 --activity Design` is executed without `--spent-on`
- **THEN** the time entry is created with today's date

#### Scenario: List time entries with date range
- **WHEN** `rdm time list --user me --from 2024-01-01 --to 2024-01-31` is executed
- **THEN** the response contains time entries in a Markdown table with user column and totals

#### Scenario: Delete time entry
- **WHEN** `rdm time delete --id 456` is executed
- **THEN** the time entry is deleted and success is confirmed with entry ID

#### Scenario: List time entries with custom field filter
- **WHEN** `rdm time list --cf 10=billable --from 2024-01-01` is executed
- **THEN** only time entries matching the custom field filter are returned

#### Scenario: List time entries grouped by user
- **WHEN** `rdm time list --project myproject --group-by user` is executed
- **THEN** time entries are grouped by user with subtotals and grand total

### Requirement: Issue Commands
The CLI SHALL provide issue commands:
- `issue list` with filters: `--project`, `--status`, `--assigned-to`, `--author`, `--tracker`, `--subject`, `--search`, `--cf <id>=<value>`, `--limit`, `--offset`
- `issue get --id <id>`
- `issue create --project <id> --subject <text>` with optional: `--description`, `--assigned-to`, `--priority`, `--status`, `--tracker`, `--cf <key=value>`
- `issue update --id <id>` with optional: `--subject`, `--description`, `--status`, `--assigned-to`, `--priority`, `--notes`, `--cf <key=value>`

#### Scenario: List issues with filters
- **WHEN** `rdm issue list --project myproject --status open --assigned-to me` is executed
- **THEN** the response contains issues matching all filter criteria in a Markdown table

#### Scenario: List issues by subject
- **WHEN** `rdm issue list --subject "Critical bug in login"` is executed
- **THEN** issues with exact subject match are returned

#### Scenario: Search issues by text
- **WHEN** `rdm issue list --search "authentication error"` is executed
- **THEN** issues containing the search text are returned

#### Scenario: List issues with custom field filter
- **WHEN** `rdm issue list --project myproject --cf 5=high` is executed
- **THEN** only issues where custom field 5 equals "high" are returned

#### Scenario: Create issue with required fields
- **WHEN** `rdm issue create --project myproject --subject "New bug"` is executed
- **THEN** a new issue is created and the response confirms with issue ID and details

#### Scenario: Update issue with notes
- **WHEN** `rdm issue update --id 123 --status closed --notes "Fixed in commit abc"` is executed
- **THEN** the issue is updated and the response confirms the changes

#### Scenario: Create issue with custom fields
- **WHEN** `rdm issue create --project 1 --subject "New task" --cf 5=urgent --cf 6=backend` is executed
- **THEN** a new issue is created with the specified custom field values

#### Scenario: Update issue with custom fields
- **WHEN** `rdm issue update --id 123 --cf 5=normal` is executed
- **THEN** the issue's custom field is updated to the new value
