---
name: redmine-cli-workflows
description: Use this skill whenever the user wants to interact with Redmine through this repository's `rdm` CLI, especially for listing or updating issues, logging time, searching Redmine data, working with profiles, using JSON output in agent pipelines, or handling Redmine custom fields. Consult this skill even if the user only says "use the CLI" or "update Redmine" without naming the exact command.
---

# Redmine CLI Workflows

Use this skill to interact efficiently with Redmine through the `rdm` CLI in this repository.

This skill is optimized for agent use:
- Prefer `rdm` over manual API calls when the user's goal fits the CLI.
- Prefer small discovery commands before mutating commands.
- Prefer machine-readable output with `--format json` when you need to inspect fields, chain commands, or make follow-up decisions.
- Prefer markdown output when the goal is a human-readable summary.

This skill is based on the repository README. If the CLI behavior appears different in the local environment, trust `rdm --help` and the local docs over assumptions.

## What the CLI covers

The README documents these command groups:

- General: `ping`, `me`, `config`
- Profiles: `profile add`, `profile use`, `profile list`, `profile delete`
- Projects: `project list`, `project get`
- Issues: `issue list`, `issue get`, `issue create`, `issue update`
- Time entries: `time list`, `time get`, `time create`, `time update`, `time delete`, `time activities list`
- Users: `user list`, `user me`

## Core operating style

Follow this pattern unless the user asks for something more direct:

1. Verify auth and target instance.
2. Discover the exact Redmine objects you need.
3. Perform the mutation only after IDs and field values are clear.
4. Read back the result if confirmation matters.

Good defaults:

- Start with `rdm ping` when setup is uncertain.
- Use `rdm config` to understand which credentials source is active.
- Use `rdm me` to confirm the acting user.
- Use `--format json` for agent decisions.
- Use `--dry-run` before create or update operations when the user wants validation or a preview.

## Authentication and configuration

The README documents this precedence order:

1. CLI flags: `--url`, `--api-key`
2. Environment variables: `REDMINE_URL`, `REDMINE_API_KEY`
3. Active profile in config

Use these setup paths:

```bash
rdm profile add --name work --url https://redmine.example.com --api-key your-api-key
rdm profile use work
rdm ping
rdm me
```

If setup is temporary or one-off, prefer CLI flags or environment variables instead of editing profiles.

## Output strategy

Use markdown by default for quick summaries:

```bash
rdm issue get --id 123
```

Use JSON when:

- you need stable fields
- you need pagination metadata
- you need to branch on command output
- you want scriptable pipelines

```bash
rdm issue get --id 123 --format json
rdm issue list --assigned-to me --status open --format json
```

JSON responses use a consistent envelope with `ok`, `data`, and `meta`, and error responses include `error`.

## Discovery workflows

Use the cheapest discovery command that narrows the target.

### Confirm identity and instance

```bash
rdm ping
rdm me
rdm config
```

### Find projects

```bash
rdm project list
rdm project get --id 1
rdm project get --identifier backend
```

### Find users

```bash
rdm user list --status active
rdm user me
```

### Find issues

Use `issue list` first. The README documents these filters:

- `--project <id>`
- `--status <open|closed|*|id>`
- `--assigned-to <me|id>`
- `--author <me|id>`
- `--tracker <id>`
- `--subject <text>`
- `--search <text>`
- `--cf <id>=<value>`

Examples:

```bash
rdm issue list --assigned-to me --status open
rdm issue list --project backend --search "authentication error"
rdm issue list --project backend --cf 5=urgent --cf 6=backend
```

Use `--subject` for exact subject matching and `--search` when the user describes partial text, keywords, or fuzzy lookup needs.

### Find time entries

The README documents these filters:

- `--project <id>`
- `--issue <id>`
- `--user <me|id>`
- `--from <YYYY-MM-DD>`
- `--to <YYYY-MM-DD>`
- `--cf <id>=<value>`
- `--group-by <field>`

Examples:

```bash
rdm time list --user me --from 2024-01-01 --to 2024-01-31
rdm time list --user me --from 2024-01-01 --to 2024-01-31 --group-by project
```

## Mutation workflows

Mutations should usually follow discovery.

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

### Log time

Against an issue:

```bash
rdm time create --issue 123 --hours 2.5 --activity Development --comment "Implemented feature X"
```

Against a project:

```bash
rdm time create --project 1 --hours 1.0 --activity Meeting --comment "Sprint planning"
```

For `time create`, the activity can be given as a name or an ID. If the valid activity is uncertain, resolve it first:

```bash
rdm time activities list
```

### Update or delete time

```bash
rdm time update --id 456 --hours 3.0 --comment "Adjusted after review"
rdm time delete --id 456
```

## Custom fields

This CLI explicitly supports Redmine custom fields in the README.

Use repeatable `--cf <id>=<value>` arguments when:

- filtering issues
- filtering time entries
- creating issues with custom field values
- updating issues with custom field values
- grouping time entries by a custom field

Examples:

```bash
rdm issue create \
  --project 1 \
  --subject "High priority task" \
  --cf 5=urgent \
  --cf 6=backend
```

```bash
rdm issue update --id 123 --cf 5=normal --cf 7="Q2 2024"
```

```bash
rdm issue list --project backend --cf 5=urgent --cf 6=backend
rdm time list --from 2024-01-01 --to 2024-01-31 --cf 9=client-a
rdm time list --from 2024-01-01 --to 2024-01-31 --group-by cf_9
```

Important practice:

- Treat custom field IDs as instance-specific Redmine configuration.
- Do not invent custom field IDs or allowed values.
- If the user does not know the IDs, first look for existing examples in prior commands, project docs, or issue/time output the user already has.

## Grouping and reporting

For time analysis, prefer server filtering first, then grouping:

```bash
rdm time list --user me --from 2024-01-01 --to 2024-01-31 --group-by project
```

Documented group values are:

- `user`
- `project`
- `activity`
- `issue`
- `spent_on`
- `cf_<id>`

This is useful for:

- monthly summaries
- project breakdowns
- activity breakdowns
- custom field reporting

## Agent-efficient patterns

Prefer these patterns:

- Use `rdm issue list ... --format json` before `rdm issue get` when you need to select one issue from many.
- Use `rdm time activities list` before `rdm time create` if the activity name is uncertain.
- Use `rdm project list` or `rdm user list` to resolve IDs before create and update operations.
- Use `--dry-run` when constructing non-trivial issue create requests.
- Use `--debug` only when troubleshooting; it is not the normal path for routine work.

Avoid these mistakes:

- Jumping straight to `issue update` without first confirming the issue ID.
- Guessing project IDs, user IDs, tracker IDs, priority IDs, status IDs, or custom field IDs.
- Using markdown output when you need exact downstream parsing.
- Treating the CLI as interactive; it is documented as non-interactive and agent-first.

## Examples by intent

### "Show my open work"

```bash
rdm issue list --assigned-to me --status open
```

### "Find the authentication bug in backend"

```bash
rdm issue list --project backend --search "authentication error"
```

### "Create an issue with custom fields"

```bash
rdm issue create \
  --project 1 \
  --subject "Backend follow-up" \
  --cf 5=urgent \
  --cf 6=backend
```

### "Log time for today"

```bash
rdm time create --issue 123 --hours 2.5 --activity Development --comment "Implemented feature X"
```

### "Summarize my month by project"

```bash
rdm time list --user me --from 2024-01-01 --to 2024-01-31 --group-by project
```

### "Use it in a pipeline"

```bash
rdm ping --format json
rdm issue get --id 123 --format json
```

## Error handling expectations

The README documents stable exit codes:

- `0` success
- `2` validation or argument error
- `3` authentication or configuration error
- `4` resource not found
- `5` API, server, or network error

Use those semantics when building scripts or agent loops around `rdm`.

## When not to rely on this skill alone

Escalate to local help output or repository docs when:

- the user asks for a command or flag not covered in the README
- the local binary behavior appears different from the README
- the user needs exact field names from a JSON payload you have not inspected yet

In those cases, verify with:

```bash
rdm --help
rdm issue --help
rdm time --help
```
