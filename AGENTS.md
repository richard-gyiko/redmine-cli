<!-- OPENSPEC:START -->
# OpenSpec Instructions

These instructions are for AI assistants working in this project.

Always open `@/openspec/AGENTS.md` when the request:
- Mentions planning or proposals (words like proposal, spec, change, plan)
- Introduces new capabilities, breaking changes, architecture shifts, or big performance/security work
- Sounds ambiguous and you need the authoritative spec before coding

Use `@/openspec/AGENTS.md` to learn:
- How to create and apply change proposals
- Spec format and conventions
- Project structure and guidelines

Keep this managed block so 'openspec update' can refresh the instructions.

<!-- OPENSPEC:END -->

## Release Checklist

Before creating a new release:

1. **Bump version in `Cargo.toml`** - Update the `version` field
2. **Update `Cargo.lock`** - Run `cargo check` to regenerate
3. **Commit and push** - `git commit -m "chore: Bump version to x.y.z"`
4. **Create release on GitHub** - Tag should match version (e.g., `v0.1.1`)
5. **Wait for CI** - Release workflow builds binaries and uploads to release
6. **Update Scoop manifest** - Update `bucket/rdm.json` with:
   - New `version` field
   - New `hash` from `SHA256SUMS` release asset