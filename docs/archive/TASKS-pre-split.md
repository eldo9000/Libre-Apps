# Freedom Apps — Technical Debt & Cleanup Tasks

Generated from full codebase audit (2026-04-12). Tasks are ordered within groups by impact.
Groups A–D and G are fully independent and can be dispatched to parallel agents.
Groups E–F are architectural and depend on A–D being done first (or at least not conflicting).
Group H is last — testing infra should be wired after code is stable.

---

## Group A — CI & Build Pipeline
**Unblocks everything. Do this first.**

- [ ] **A1** Fix `splice` → `fade` in `.github/workflows/release.yml`  
  Three occurrences: matrix in `build-x86_64` (line 23), matrix in `build-aarch64` (line 91), and the `for` loop in `collect-desktop-files` (line 164). Replace all three. This is the only change needed in this file.

- [ ] **A2** Add a CI check workflow for PRs and pushes to `main`  
  Create `.github/workflows/ci.yml`. Should run on `push` to `main` and `pull_request`. Steps per app:
  1. `cargo check --workspace` (from `apps/`)
  2. `cargo clippy --workspace -- -D warnings`
  3. `cargo fmt --check --all`
  4. `npm ci && npm run build` (for each of the 5 apps)
  Use a matrix the same way `release.yml` does for the frontend steps.
  Use `actions-rust-lang/setup-rust-toolchain@v1` with `cache-workspaces: apps`.

- [ ] **A3** Add `rust-toolchain.toml` at repo root  
  Pin to the current stable channel so local dev and CI don't drift. Minimal file:
  ```toml
  [toolchain]
  channel = "stable"
  components = ["rustfmt", "clippy"]
  ```

---

## Group B — Port & Version Corrections
**Quick config fixes. Independent from everything.**

- [ ] **B1** Fix Prism dev port: 1421 → 1423  
  Change port in two files:
  - `apps/prism/vite.config.js` line 13: `port: 1421` → `port: 1423`
  - `apps/prism/src-tauri/tauri.conf.json` line 7: `"devUrl": "http://localhost:1421"` → `"devUrl": "http://localhost:1423"`

- [ ] **B2** Update CLAUDE.md port table to match reality  
  The dev server ports table in `CLAUDE.md` shows Ghost at 1421 and Prism at 1423. After B1 this will be correct, but verify all 5 entries match the actual vite configs:
  - Shelf: 1420
  - Ghost: 1421
  - Stack: 1422
  - Prism: 1423
  - Fade: 1427

- [ ] **B3** Upgrade Ghost frontend deps to match other apps  
  In `apps/ghost/package.json`:
  - `"@sveltejs/vite-plugin-svelte": "^5"` → `"^7"`
  - `"vite": "^6"` → `"^8"`
  In `apps/ghost/vite.config.js`:
  - `minify: process.env.TAURI_ENV_DEBUG ? false : 'esbuild'` → `'oxc'`
  Run `npm install` in `apps/ghost/` to update `package-lock.json`.

---

## Group C — Ghost Rename / Repositioning Cleanup
**Purely textual. The rename commit 6c93eb1 missed these.**

- [ ] **C1** Update Ghost Cargo.toml description  
  `apps/ghost/src-tauri/Cargo.toml` line 4:  
  `description = "Ghost — LibreWin private browser"` → `"Ghost — LibreWin identity-randomizing browser"`

- [ ] **C2** Update Ghost .desktop file  
  `apps/ghost/io.librewin.ghost.desktop`:
  - Line 5: `Comment=LibreWin private browser — no trace, ever` → `Comment=Identity-randomizing browser — new fingerprint every session`
  - Add `WEBKIT_DISABLE_DMABUF_RENDERER=1` env wrapper to Exec (matches Shelf/Stack/Prism/Fade):  
    `Exec=ghost %U` → `Exec=env WEBKIT_DISABLE_DMABUF_RENDERER=1 ghost %U`

- [ ] **C3** Fix Ghost capability description  
  `apps/ghost/src-tauri/capabilities/default.json` line 4:  
  `"Default capabilities for Simple Web main window"` → `"Default capabilities for Ghost main window"`

---

## Group D — Accent Color Consistency
**One decision, then propagate it. Canonical value is `#0066cc` (per CLAUDE.md intent and Ghost toolbar).**

- [ ] **D1** Fix Rust `get_accent` defaults in all 5 apps  
  In each app's `src-tauri/src/lib.rs`, the `get_accent` function falls back to `"#297acc"`. Change to `"#0066cc"` in:
  - `apps/shelf/src-tauri/src/lib.rs`
  - `apps/stack/src-tauri/src/lib.rs`
  - `apps/prism/src-tauri/src/lib.rs`
  - `apps/fade/src-tauri/src/lib.rs`
  - `apps/ghost/src-tauri/src/lib.rs`

- [ ] **D2** Fix CSS `--accent` defaults in all app.css files  
  In each app's `src/app.css`, the `:root` block sets `--accent: #297acc`. Change to `#0066cc` in:
  - `apps/shelf/src/app.css`
  - `apps/stack/src/app.css`
  - `apps/prism/src/app.css` (if present — verify)
  - `apps/fade/src/app.css` (if present — verify)
  - `apps/ghost/src/app.css` (if present — verify)

- [ ] **D3** Fix Ghost's injected toolbar hardcode  
  `apps/ghost/src-tauri/src/lib.rs` line 54: `var ACCENT = '#0066cc';` — already correct, verify nothing else needs changing in TOOLBAR_INIT_SCRIPT.

- [ ] **D4** Update CLAUDE.md design system section  
  The CLAUDE.md already says `#0066cc` — but add a note that `#297acc` was the old erroneous default, and that all CSS/Rust defaults should match.

---

## Group E — Shared Infrastructure
**Architectural work. Reduces ~100 lines of duplication. Do after A–D.**

- [ ] **E1** Create `Business-OS/ENGINEERING_STANDARDS.md`  
  CLAUDE.md's very first instruction says "read this before building anything" and links here — but the file doesn't exist. Create it and seed it with the patterns that are already duplicated across apps:
  - Canonical design tokens (accent, surface, text, border CSS variables)
  - Window chrome recipe (titlebar, drag region, resize strip, controls)
  - Theme/accent init pattern (the `onMount` block)
  - Svelte 5 runes rule (no legacy syntax)
  - WebKit dmabuf env workaround
  - Known Tauri 2 gotchas (capability schema path, `windows_subsystem`, etc.)

- [ ] **E2** Add `[workspace.dependencies]` to `apps/Cargo.toml`  
  Currently every app's Cargo.toml re-declares `tauri = "2"`, `serde = ...`, `serde_json = "1"` independently. Move shared deps to workspace-level so version bumps happen in one place:
  ```toml
  [workspace.dependencies]
  tauri = { version = "2", features = [] }
  serde = { version = "1", features = ["derive"] }
  serde_json = "1"
  ```
  Then each app's Cargo.toml references `tauri = { workspace = true }` etc.

- [ ] **E3** Extract shared Rust crate `librewin-common`  
  Create `apps/common/` with a Cargo.toml `name = "librewin-common"` added to workspace members. Move `get_theme()` and `get_accent()` here (currently copy-pasted into all 5 lib.rs files). Each app adds `librewin-common = { path = "../../common" }` and calls `librewin_common::get_theme()` etc.

- [ ] **E4** Extract shared JS theme init  
  Create `apps/common-js/theme.js` exporting `initTheme(invoke)`. Currently the same 12-line `onMount` block (invoke get_theme, toggle dark class, listen for media query changes) is copy-pasted into all 5 App.svelte files. Each app imports and calls it. If using npm workspaces, add `"workspaces": ["../common-js"]` to each package.json.

---

## Group F — Code Quality
**Medium-effort fixes. Independent from E but related to E3 (E3 should go first if doing both).**

- [ ] **F1** Migrate Shelf's `QuickFiles.svelte` from legacy to Svelte 5 runes  
  `apps/shelf/src/lib/QuickFiles.svelte` is the only `.svelte` file in the repo using legacy syntax. Two violations:
  - Line 34: `export let dualPane = false;` → `let { dualPane = $bindable(false) } = $props();`
  - Lines 63–64: `$: ap = panes[activePaneIdx];` and `$: t = ap.tabs[ap.activeTabIdx];` → `let ap = $derived(panes[activePaneIdx]);` and `let t = $derived(ap.tabs[ap.activeTabIdx]);`
  Audit the rest of the file for any other `$:` reactive statements and convert them.

- [ ] **F2** Add `svelte.config.js` to Stack  
  Stack is the only app without one. Create `apps/stack/svelte.config.js` identical to the others:
  ```js
  import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
  export default { preprocess: vitePreprocess() };
  ```

- [ ] **F3** Replace Fade's homemade UUID with `uuid` crate  
  `apps/fade/src-tauri/src/lib.rs` lines 696–715: hand-rolled `uuid_v4()` using time XOR. Not random enough. Add `uuid = { version = "1", features = ["v4"] }` to Fade's Cargo.toml and replace the function body with `uuid::Uuid::new_v4().to_string()`.

- [ ] **F4** Capture stderr in Shelf's quick-convert commands  
  `apps/shelf/src-tauri/src/lib.rs` lines 377 and 468: `.stderr(Stdio::null())`. When ffmpeg/magick fail, the error event only says `"{cmd} failed"`. Change to `.stderr(Stdio::piped())`, capture the output, and include it in the error payload. Matches Fade's approach.

- [ ] **F5** Decouple Shelf from Fade's preset schema  
  `apps/shelf/src-tauri/src/lib.rs` lines 282–494: Shelf defines its own `FadePreset` struct and re-implements `run_fade_preset` and `quick_convert`. This duplicates Fade's code and creates a silent schema contract. Once E3 exists, move `FadePreset` to `librewin-common`. Until then, at minimum add a code comment explaining the coupling risk and noting that both `FadePreset` structs must stay in sync.

---

## Group G — Documentation & Housekeeping
**Small, safe, cosmetic. Fully parallel with everything else.**

- [ ] **G1** Soften README — remove archive support claim  
  `README.md` line 10 says Shelf has "Built-in archive support (ZIP, 7Z, TAR)". The roadmap lists this as a Phase-1 gap. Change to "Archive browsing/extraction (ZIP, 7Z, TAR) coming in v0.2" or similar.

- [ ] **G2** Fix `.gitignore` duplicate lines  
  `.gitignore` has `.DS_Store` and `**/.DS_Store` duplicated (lines 2–3 and 9–10). Remove the duplicate block.

- [ ] **G3** Fix Prism capability description  
  `apps/prism/src-tauri/capabilities/default.json` line 4: `"Default capabilities for Simple Web main window"` → `"Default capabilities for Prism main window"` (orphan text from an earlier rename).

- [ ] **G4** Standardize panic messages in `run()` entry points  
  Some apps say `"error while running tauri application"`, others say `"error while running stack"` / `"error while running ghost"`. Standardize to `"error while running {appname}"` across all 5 `run()` functions in lib.rs.

- [ ] **G5** Create `SESSION-STATUS.md` at repo root  
  Per project convention (CLAUDE.md session-continuity rules). Initialize with current state: audit complete, fixing technical debt before Phase 1 work begins.

---

## Group H — Testing Infrastructure
**Do last. Depends on code being stable (F done, E done if possible).**

- [x] **H1** Add Rust unit tests to each app  
  At minimum, one `#[cfg(test)]` module per lib.rs testing the most critical command. Examples:
  - Shelf: `list_dir` on a temp dir, `get_tags`/`set_tags` roundtrip
  - Stack: `export_html` produces valid HTML from markdown
  - Prism: `category_for_ext` and `mime_for_ext` spot-checks
  - Fade: `build_output_path`, `validate_suffix`, `media_type_for`
  - Ghost: no stateful commands, skip or test `get_theme` fallback

- [x] **H2** Add vitest to each app  
  Create `apps/{app}/src/tests/` with one smoke test per app verifying the UI mounts without errors. Add `"test": "vitest run"` to each `package.json`. Use `@testing-library/svelte` if needed.

- [x] **H3** Wire tests into CI  
  Add a `test` step to the CI check workflow created in A2:
  - `cargo test --workspace` (Rust)
  - `npm run test` per app (vitest)

---

## Parallel dispatch plan

Groups that are **fully independent** and safe to run in parallel:

| Agent 1 | Agent 2 | Agent 3 | Agent 4 |
|---------|---------|---------|---------|
| A1, A2, A3 | B1, B2, B3 | C1, C2, C3 | D1, D2, D3, D4 |
| G1, G2, G3, G4, G5 | F2, F3, F4 | — | — |

**After the above are merged:** E1–E4 together (shared infra), then F1 and F5.  
**After E+F:** H1–H3 (tests).
