# Libre-Apps — Claude Context

## Shared Engineering Standards

Before building anything, read `Business-OS/ENGINEERING_STANDARDS.md`. It contains shared conventions, Tauri 2 patterns, Svelte 5 patterns, design tokens, cross-platform gotchas, and known bug fixes that apply to everything built on this foundation. If you discover a new pattern or fix a non-obvious bug, document it there so every downstream app benefits.

---

## What This Repo Is (as of April 2026)

**Libre-Apps is the shared foundation for the Libre family of desktop apps.** It is no longer a monorepo. The five apps (Shelf, Stack, Prism, Fade, Avalanche) each live in their own standalone repo under `eldo9000/`. This repo holds the code and standards they all consume.

| App | Repo | Role |
|-----|------|------|
| Shelf | [Shelf-App](https://github.com/eldo9000/Shelf-App) | File manager |
| Stack | [Stack-App](https://github.com/eldo9000/Stack-App) | Markdown editor |
| Prism | [Prism-App](https://github.com/eldo9000/Prism-App) | Universal media viewer |
| Fade  | [Fade-App](https://github.com/eldo9000/Fade-App)   | Media converter |
| Avalanche | [Avalanche-App](https://github.com/eldo9000/Avalanche-App) | Privacy browser |

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

gallery/           Live component showcase — primary daily workspace (see ## Gallery)
  src/
    App.svelte     shell layout + sections registry
    sections/      one .svelte per nav section (Demo Tiles, Tokens, Typography, …)
    lib/
      AppPanels.svelte   collapsible right sidebar + panels registry
      Card.svelte         component wrapper with click-to-focus
      ThemeLab.svelte     HSV color editor (bottom-left dock)
      focus.svelte.js     focus state store + Tauri write_focus bridge
      panels/        per-app panel components (FlickerInspector, FadePanel, TurboTalkPanel)
  src-tauri/       Tauri backend for gallery (write_focus, write_preset, read_preset, save_theme)
  .focus.json      transient — written by click-to-focus, read by agent (git-ignored)

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

**Most day-to-day work happens in `gallery/`** — see the `## Gallery` section below. The root-level tooling is for linting and type-checking the shared packages (`common/` and `common-js/`).

```bash
# Shared package validation
cargo check --workspace        # checks librewin-common
cargo fmt --check --all
cargo clippy --workspace -- -D warnings

npm install
npm run lint                   # eslint over common-js/src
npm run lint:css               # stylelint over common-js/src CSS + svelte
npm run fmt                    # prettier write
```

To exercise a `common/` or `common-js/` change end-to-end: pin a downstream app repo at your local commit (or a pushed SHA) and run `tauri dev` over there. Or validate it directly in the gallery.

---

## Design System

Light mode default, dark mode supported. Both first-class via CSS variables.
- **Accent:** `#2884c9` via `--accent` (user-configurable via `~/.config/librewin/accent`). `#297acc` and `#0066cc` were previous values — any remnants are bugs.
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

**Accent color is `#2884c9` (HSV 206°/80%/79%), hover `#2373b0`.**
`#297acc` and `#0066cc` were previous values — both are wrong. Any CSS `:root` block or Rust `get_accent` fallback not using `#2884c9` is a bug. The Rust fallback in `librewin-common` also needs updating if touched.

**Never hold a Mutex lock while spawning a thread in Tauri commands.**
Extract the data, drop the lock, then spawn. Holding across `thread::spawn` / `tokio::spawn` deadlocks on re-entrant IPC.

**Svelte 5 runes only — no legacy reactive syntax.**
Never use `$:`, `createEventDispatcher`, or `export let`. Only `$state`, `$derived`, `$effect`, `$props`, `$bindable`. The compiler silently misbehaves on mixed syntax.

**Custom titlebar requires four things in lockstep: `decorations: false` + `transparent: true` + 10px border-radius + drop shadow.**
Missing any one causes visual breakage. All four must be set per-app.

**Theme sync reads from `~/.config/librewin/theme` and `~/.config/librewin/accent` via `librewin_common` helpers.**
Do not read those files directly from app code — use `get_theme()` / `get_accent()` IPC commands backed by `librewin_common`. Consistent fallback (`light` / `#2884c9`).

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

## Gallery

The gallery is a Svelte 5 + Tauri app that showcases and stress-tests every `@libre/ui` component. It is the **primary daily workspace** in this repo. Running it is how you see and iterate on shared components visually.

### Running the gallery

```bash
cd gallery
npm run tauri dev   # full Tauri app — required for click-to-focus, preset save/load
npm run dev         # Vite-only on :1422 — faster; fine for pure CSS/layout work
```

Use `npm run tauri dev` any time the click-to-focus system or ThemeLab save/load is involved.

### Shell layout

```
.shell (flex row, 100vh)
  ├─ .sidebar (200px, fixed width)
  │    ├─ header: "◈ Libre UI" logo + dark-mode toggle
  │    └─ nav: one button per section, auto-highlights on scroll
  ├─ .content (flex: 1, overflow-y scroll)
  │    └─ .zoom-wrap — all sections rendered sequentially, separated by borders
  └─ AppPanels — collapsible right sidebar (304px open / 28px collapsed)

.dock (position: fixed, bottom-left, 200px wide — always visible)
  ├─ ThemeLab — HSV accent editor
  └─ dock-footer — zoom controls (−, N%, +)
```

The sidebar nav has extra bottom padding so items scroll past the fixed dock without being obscured.

### Sections

Registered in `gallery/src/App.svelte` as a `sections` array. Each entry maps to a component in `gallery/src/sections/`:

| id | Label | File |
|----|-------|------|
| `demo` | Demo Tiles | `DemoTilesSection.svelte` |
| `tokens` | Tokens | `TokensSection.svelte` |
| `typography` | Typography | `TypographySection.svelte` |
| `buttons` | Buttons & Actions | `ButtonsSection.svelte` |
| `form` | Form Controls | `FormSection.svelte` |
| `feedback` | Feedback | `FeedbackSection.svelte` |
| `navigation` | Navigation | `NavigationSection.svelte` |
| `layout` | Layout | `LayoutSection.svelte` |

**To add a section:** create `gallery/src/sections/MySec.svelte`, import it in `App.svelte`, push `{ id, label, component }` into the `sections` array. Nav highlight and scroll-to are automatic.

### Right sidebar panels

Application panels (simulated settings UIs from real Libre apps) are registered in `gallery/src/lib/AppPanels.svelte` as a `PANELS` array. Each entry maps to a component in `gallery/src/lib/panels/`:

| id | App | Label | File |
|----|-----|-------|------|
| `flicker-inspector` | Flicker | Inspector | `FlickerInspector.svelte` |
| `fade-mp3` | Fade | MP3 | `FadePanel.svelte` |
| `turbotalk-settings` | TurboTalk | Settings | `TurboTalkPanel.svelte` |

**To add a panel:** create the component in `panels/`, import it in `AppPanels.svelte`, push `{ id, app, label, component }` into `PANELS`. Tab switching is automatic.

**Panel CSS rules:** panels must not define local `--ir-*` variable blocks. Use global design tokens directly (`var(--surface)`, `var(--border)`, `var(--text-primary/secondary/muted)`, `var(--accent)`). For hover/intermediate values: `color-mix(in srgb, var(--surface) 94%, var(--text-primary))` — this auto-adapts to both themes without any per-panel override block.

### Card component

Every component demo is wrapped in a `<Card>`:

```svelte
<Card id="BTN-1" label="Primary" sourceFile="common-js/src/components/Button.svelte">
  <!-- demo content -->
</Card>
```

Props:
- `id` — unique across the entire gallery; drives click-to-focus. Prefix by section: `BTN`, `FORM`, `NAV`, `SEG`, `LAY`, `TYP`, `DEMO`, etc.
- `label` — human-readable name in the card header and focus bar
- `sourceFile` — optional; shown in the token inspect overlay
- `component` — optional; stored in focus data, not rendered

Card interactions: click header → single focus, Shift+click header → multi-select, Shift+click body → CSS token inspector overlay.

### ThemeLab

Always-visible color editor pinned to the bottom-left dock.

**Light HSV sliders (H / S / V):** define the base accent color for light mode.

**Dark Δ sliders (ΔH / ΔS / ΔV):** ±20° / ±20pp shift applied on top of the light accent to derive the dark-mode accent. The two dot swatches next to "Dark Δ" preview the current light/dark pair.

**Accent token override mechanism:** `ThemeLab` sets four CSS properties on `:root` — `--accent-light`, `--accent-light-hover`, `--accent-dark`, `--accent-dark-hover`. Its own `:global(html:not(.dark))` and `:global(html.dark)` rules (specificity 0-1-1, beats `tokens.css` `:root` at 0-0-1) assign `--accent: var(--accent-light/dark)`. Theme switching is pure CSS — no MutationObserver.

**Persistence:** `localStorage` key `libre-theme-lab` stores `{ accent, deltaH, deltaS, deltaV }`. Zoom persists under `libre-gallery-zoom`.

### Click-to-focus agent workflow

The gallery has a built-in point-and-select system. When the user says "fix the one I marked" or "remove the two I selected", **read `gallery/.focus.json` first**.

```bash
cat gallery/.focus.json
```

**Schema — single card:**
```json
{ "id": "BTN-1", "label": "Primary", "sourceFile": null, "component": null, "focusedAt": "2026-05-09T18:42:00.000Z" }
```

**Schema — multiple cards:**
```json
[
  { "id": "SEG-5", "label": "Sliding / md", "sourceFile": null, "component": null, "focusedAt": "…" },
  { "id": "SEG-6", "label": "Sliding / sm", "sourceFile": null, "component": null, "focusedAt": "…" }
]
```

**File absent** → nothing is focused.

Use `id` to find the card in source:

```bash
grep -r "SEG-5" gallery/src/sections/
```

Use `label` to confirm you have the right element. The right sidebar focus bar shows the current selection; a `◉` pip and accent border appear on focused cards in the UI.

---

## Workflow

- Don't burn tokens reading entire codebases — read this file, then `README.md`, then dive in for specific tasks.
- One concern at a time. Changes to `common/` vs `common-js/` usually don't mix.
- A change to `common/` or `common-js/` is a breaking change from downstream repos' perspective. Think about whether you need to bump their pinned rev / re-vendor the snapshot, and mention it in the commit body.
- Commit small. Every bump downstream has to re-verify, so churn is costly.
- Check `docs/ROADMAP.md` before starting substantive work.
