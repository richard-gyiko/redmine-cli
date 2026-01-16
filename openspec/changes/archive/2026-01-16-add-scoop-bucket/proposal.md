# Change: Add Scoop Bucket for Windows Distribution

## Why
Windows users currently must manually download releases from GitHub. Scoop is the de facto package manager for CLI tools on Windows, enabling simple installation with `scoop install rdm`.

## What Changes
- Add `bucket/rdm.json` manifest to the main repository
- Scoop auto-discovers manifests in the `bucket/` subdirectory
- Users can add the repo as a bucket and install directly

## Impact
- Affected specs: New `distribution` capability (documents packaging/installation)
- Affected code: No code changes—adds manifest file only
- New files: `bucket/rdm.json`

## User Experience

### Before
```powershell
# Manual download from GitHub Releases
Invoke-WebRequest -Uri "https://github.com/.../rdm-x86_64-pc-windows-msvc.zip" -OutFile rdm.zip
Expand-Archive rdm.zip -DestinationPath C:\tools
# Manually add to PATH
```

### After
```powershell
scoop bucket add rdm https://github.com/richard-gyiko/redmine-cli
scoop install rdm
# Done - rdm is in PATH
```

## Alternatives Considered
1. **Separate repo for bucket** - More maintenance overhead, no benefit
2. **Submit to scoop-extras** - Good future option but requires established user base
