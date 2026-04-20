# Libre-Apps — Claude Context

## Shared Engineering Standards

Before building anything, read `Business-OS/ENGINEERING_STANDARDS.md`. It contains shared conventions, Tauri 2 patterns, Svelte 5 patterns, design tokens, cross-platform gotchas, and known bug fixes that apply to everything built on this foundation. If you discover a new pattern or fix a non-obvious bug, document it there so every downstream app benefits.

---

## What This Repo Is (as of April 2026)

**Libre-Apps is the shared foundation for the Libre family of desktop apps.** It is no longer a monorepo. The five apps (Shelf, Stack, Prism, Fade, Ghost) each live in their own standalone repo under `eldo9000/`. This repo holds the code and standards they all consume.

| App | Repo | Role |
|-----|------|------|
| Shelf | [Shelf-App](https://github.com/eldo9000/Shelf-App) | File manager |
| Stack | [Stack-App](https://github.com/eldo9000/Stack-App) | Markdown editor |
| Prism | [Prism-App](https://github.com/eldo9000/Prism-App) | Universal media viewer |
| Fade  | [Fade-App](https://github.com/eldo9000/Fade-App)   | Media converter |
| Ghost | [Ghost-App](https://github.com/eldo9000/Ghost-App) | Privacy browser |

The split happened at commit `532a620`. Any stray `apps/` directory in your working tree is leftover local build output (node_modules, dist, src-tauri targets) — it is untracked and not part of the repo.

---

## Repo Layout

```
common/            librewin-common — shared Rust crate
  Cargo.toml
  src/
    config.rs      ~/.config/librewin/{theme,accent} helpers
    media.rs       file classification
    os.rs          platform helpers
    window.rs      titlebar + window control commands
    xattr.rs       extended attribute helpers

common-js/         @libre/ui — shared Svelte 5 component library
  src/
    components/    Button, Dialog, Input, Menu, Tabs, Titlebar, Tooltip, WindowFrame, ...
    api/           commands.ts, dialogs.ts
    theme.js       theme init helper
    tokens.css     design tokens (CSS variables)
  scripts/
    create-libre-app.js   scaffold a new app against this foundation

docs/              shared specs and per-app milestone docs
  ROADMAP.md       master roadmap across the family
  {app}/M*.md      milestone docs for each app (authoritative here, read by app repos)
  specs/SIGNING.md signing + release infrastructure
  investigations/  archived INVESTIGATION-LOG files

Business-OS/       engineering standards (submodule / vendored)
.github/workflows/ ci.yml (foundation lints) + release.yml (cross-repo app release pipeline)
Cargo.toml         workspace = [common]; apps excluded
package.json       root tooling — eslint, stylelint, prettier over common-js/
rust-toolchain.toml
justfile           top-level build orchestration
keys/              minisign signing metadata
```

**Note:** `package.json` intentionally omits `"workspaces"`. A stale `"workspaces": ["apps/*"]` declaration used to live here (left over from the pre-split monorepo) and broke downstream CI: release.yml checks out app repos into `apps/{name}/`, and an ancestor `workspaces` declaration made npm treat them as workspace members, which mis-resolved `@libre/ui: file:./common-js` as a registry dep. Removed 2026-04-20. If you see an empty `apps/` show up locally, it's just stale build directories — do not commit files into it.

---

## How consumers pull from this repo

**`librewin-common` (Rust):** each app's `src-tauri/Cargo.toml` pins a git dep:
```toml
librewin-common = { git = "https://github.com/eldo9000/Libre-Apps.git", rev = "<sha>" }
```
CI for app repos needs `CARGO_DEPS_TOKEN` with read access to this repo. Bump the `rev` deliberately when you want to pull updates.

**`@libre/ui` (Svelte):** each app repo **vendors** `common-js/` as a snapshot at its root and references it via `"@libre/ui": "file:./common-js"`. When `common-js/` changes here, downstream repos need to re-sync. A helper script lives at `common-js/scripts/create-libre-app.js`.

---

## How to work on this repo

There is no app to run here. Validation is lint + type-check + (eventually) unit tests on the shared crate and UI package.

```bash
cargo check --workspace        # checks librewin-common
cargo fmt --check --all
cargo clippy --workspace -- -D warnings

npm install
npm run lint                   # eslint over common-js/src
npm run lint:css               # stylelint over common-js/src CSS + svelte
npm run fmt                    # prettier write
```

To actually exercise a change end-to-end, pin a downstream app repo at your local commit (or a pushed SHA) and run `tauri dev` over there.

---

## Design System

Light mode default, dark mode supported. Both first-class via CSS variables.
- **Accent:** `#0066cc` via `--accent` (user-configurable via `~/.config/librewin/accent`). `#297acc` was the old erroneous default — any remnants are bugs.
- **Font:** Geist, 14px base, 1.5 line height, antialiased.
- **Spacing:** 4px base unit, compact density.
- **Window chrome:** custom titlebar, `decorations: false`, `transparent: true`, 10px border-radius, drop shadow.
- **Scrollbars:** 6px, semi-transparent thumb.
- **Motion:** functional only — 120ms fade, 200ms fly.
- **Theme sync:** apps read from `~/.config/librewin/{theme,accent}` via `librewin_common` helpers — never directly.

Source of truth for tokens: `common-js/src/tokens.css`. Source of truth for shared components: `common-js/src/components/`.

---

## Known Patterns & Gotchas

Non-obvious permanent truths. Add immediately when discovered.

**Accent color is `#0066cc`, not `#297acc`.**
`#297acc` was the old erroneous default. Any CSS `:root` block or Rust `get_accent` fallback still using `#297acc` is a bug.

**Never hold a Mutex lock while spawning a thread in Tauri commands.**
Extract the data, drop the lock, then spawn. Holding across `thread::spawn` / `tokio::spawn` deadlocks on re-entrant IPC.

**Svelte 5 runes only — no legacy reactive syntax.**
Never use `$:`, `createEventDispatcher`, or `export let`. Only `$state`, `$derived`, `$effect`, `$props`, `$bindable`. The compiler silently misbehaves on mixed syntax.

**Custom titlebar requires four things in lockstep: `decorations: false` + `transparent: true` + 10px border-radius + drop shadow.**
Missing any one causes visual breakage. All four must be set per-app.

**Theme sync reads from `~/.config/librewin/theme` and `~/.config/librewin/accent` via `librewin_common` helpers.**
Do not read those files directly from app code — use `get_theme()` / `get_accent()` IPC commands backed by `librewin_common`. Consistent fallback (`light` / `#0066cc`).

**Linux dev renderer is WebKit2GTK 4.1 — declare all system deps in `tauri.conf.json`.**
Undeclared deps install fine on dev machines but break in CI and ISO builds. macOS uses a different WebKit — Linux rendering bugs won't surface in `tauri dev` on macOS.

**`$effect` cleanup must always return the unlisten function — not call it.**
`return () => { unlisten.then(f => f()); }`. Forgetting the return leaks listeners.

**CI lint job needs system deps installed before `cargo clippy`.**
WebKit2GTK / GTK headers are required to compile Tauri crates even for lint. Silent breakage if missing — not a Rust toolchain problem.

**`release.yml` depends on `CARGO_DEPS_TOKEN` reaching every downstream app repo, and shell-uppercases app names.**
Token rotation or a multi-word app name will break the pipeline without warning. Flagged in commit `3baafbf`.

---

## LibreWin OS Integration Contract

Downstream app repos publish per-release:
- `{app}-x86_64` and `{app}-aarch64` — Tauri binaries
- `{app}.desktop` — .desktop file + MIME declarations (source of truth for OS integration)

The `.desktop` file is the contract between each app and `LibreWin-OS`. `LibreWin-OS/build/fetch-apps.sh` downloads it alongside the binary.

Binary install names (do not rename without updating LibreWin-OS `apps.sh` / `branding.sh`):

| App | Installed as in `/usr/local/bin/` |
|-----|----------------------------------|
| shelf | `shelf` |
| ghost | `ghost` |
| prism | `librewin-prism` |
| stack | `librewin-stack` |
| fade  | `fade` |

All release binaries and `.desktop` files are signed with minisign. Production public key lives in `README.md` and is baked into LibreWin OS. See `docs/specs/SIGNING.md`.

---

## Workflow

- Don't burn tokens reading entire codebases — read this file, then `README.md`, then dive in for specific tasks.
- One concern at a time. Changes to `common/` vs `common-js/` usually don't mix.
- A change to `common/` or `common-js/` is a breaking change from downstream repos' perspective. Think about whether you need to bump their pinned rev / re-vendor the snapshot, and mention it in the commit body.
- Commit small. Every bump downstream has to re-verify, so churn is costly.
- Check `docs/ROADMAP.md` before starting substantive work.
