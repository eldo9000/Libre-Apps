# App Manifest Schema

Every Freedom App ships a `libre.manifest.json` at `apps/{name}/libre.manifest.json`.
This file declares the app's identity, version, and permission requirements. It is
machine-readable by the LibreWin install manager, which presents a permissions summary
to the user before installing or updating any app.

## Schema (version 1)

```jsonc
{
  // Schema version — increment when fields are added/removed/renamed.
  "schema_version": 1,

  // Reverse-DNS identifier. Must match tauri.conf.json → identifier.
  "id": "io.librewin.{name}",

  // Human-readable name.
  "name": "string",

  // Semver. Must match tauri.conf.json → version. CI validates this.
  "version": "0.0.0",

  // One-line description shown in the install manager.
  "description": "string",

  // Permission declarations.
  "permissions": {

    // File system access.
    "filesystem": {
      "read": true | false,
      "write": true | false,

      // Path scope tokens. Use the narrowest scope possible.
      // Tokens: "$ALL", "$HOME", "$DOCUMENTS", "$DOWNLOADS", "$PICTURES", "$MUSIC", "$VIDEOS"
      "scope": ["$TOKEN", ...]
    },

    // Network access.
    "network": {
      "outbound": true | false,  // App initiates connections
      "inbound": true | false    // App listens for connections
    },

    // External processes the app is allowed to spawn.
    // List the executable name only (no full path — PATH-resolved at runtime).
    "subprocess": ["ffmpeg", "magick", ...],

    // App embeds a webview that navigates to arbitrary URLs.
    "webview": true | false,

    // App sends desktop notifications.
    "notifications": true | false,

    // App reads/writes extended file attributes (xattr).
    "xattr": true | false
  },

  // System packages required at runtime (mirrors .desktop Runtime OS packages comment).
  // These are the apt/dnf/pacman package names on Linux.
  "runtime_deps": ["pkg1", ...]
}
```

## Build-time stamping

The CI `generate-manifests` job validates each manifest against `tauri.conf.json` and
stamps a `build` block before uploading:

```jsonc
{
  // ... all fields above, plus:
  "build": {
    "ref":  "v0.2.0",         // git tag or branch name
    "sha":  "a1b2c3d4",       // short commit SHA
    "date": "2026-04-14"      // ISO 8601
  }
}
```

Stamped manifests are signed with the same minisign key as the binaries and uploaded
as `{app}.manifest.json` + `{app}.manifest.json.minisig` in each release.

## Permission scope tokens

| Token | Resolves to |
|-------|------------|
| `$ALL` | All paths (use only when truly necessary) |
| `$HOME` | User's home directory |
| `$DOCUMENTS` | `~/Documents` |
| `$DOWNLOADS` | `~/Downloads` |
| `$PICTURES` | `~/Pictures` |
| `$MUSIC` | `~/Music` |
| `$VIDEOS` | `~/Videos` |

The install manager expands these tokens to real paths and shows them in the permissions
dialog before installation.

## Permission sets by app

| App | Filesystem | Network | Subprocess | Webview | xattr |
|-----|-----------|---------|-----------|---------|-------|
| Shelf | read+write `$ALL` | none | wine, winepath, xdg-open | no | yes |
| Stack | read+write `$HOME` | none | none | no | no |
| Prism | read `$ALL` | none | none | no | no |
| Fade | read+write `$ALL` | none | ffmpeg, magick | no | no |
| Ghost | read+write `$DOWNLOADS` | outbound | none | yes | no |

## Validation rules (enforced by CI)

1. `version` must exactly match `tauri.conf.json → version` — build fails otherwise.
2. `id` must match `tauri.conf.json → identifier`.
3. `schema_version` must be `1` (the only defined version).
4. `permissions.filesystem.scope` must be non-empty when `read` or `write` is `true`.
5. `permissions.subprocess` must be an array (empty array = no subprocess access).

## LibreWin-OS integration

The install manager reads `{app}.manifest.json` from the GitHub release to:
1. Verify the manifest signature against the baked-in public key.
2. Display a permissions summary to the user.
3. Block installation if required `runtime_deps` are unavailable on the system.

See `LibreWin-OS/install-manager/` for the consumer implementation.
