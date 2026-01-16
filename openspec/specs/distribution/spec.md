# distribution Specification

## Purpose
TBD - created by archiving change add-scoop-bucket. Update Purpose after archive.
## Requirements
### Requirement: Scoop Package Distribution
The project SHALL provide a Scoop bucket manifest for Windows package distribution.

#### Scenario: Scoop bucket discovery
- **WHEN** a user adds `https://github.com/richard-gyiko/redmine-cli` as a Scoop bucket
- **THEN** Scoop discovers the manifest at `bucket/rdm.json`
- **AND** the manifest is valid JSON conforming to Scoop schema

#### Scenario: Scoop installation
- **WHEN** a user runs `scoop install rdm` after adding the bucket
- **THEN** Scoop downloads the Windows release artifact
- **AND** extracts `rdm.exe` to the Scoop apps directory
- **AND** `rdm` is available in PATH

#### Scenario: Scoop autoupdate
- **WHEN** a new version is released with tag `v<version>`
- **THEN** the manifest autoupdate configuration generates the correct download URL
- **AND** Scoop can update to the new version via `scoop update rdm`

### Requirement: Manifest Integrity
The Scoop manifest SHALL include SHA256 hash verification for the download artifact.

#### Scenario: Hash verification
- **WHEN** Scoop downloads the release artifact
- **THEN** the download is verified against the SHA256 hash in the manifest
- **AND** installation fails if hash does not match

