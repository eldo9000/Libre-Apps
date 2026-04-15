# The Freedom Apps — Claude Context

## Shared Engineering Standards

**Before building anything, read `Business-OS/ENGINEERING_STANDARDS.md`.** It contains shared conventions, Tauri 2 patterns, Svelte 5 patterns, design tokens, cross-platform gotchas, and known bug fixes that apply to all LibreWin apps. If you discover a new pattern or fix a non-obvious bug, document it there so Casebook, Flicker, and all Freedom Apps benefit.

---

## What This Is

The Freedom Apps are a minimal set of reliable, lightweight desktop apps that ship with LibreWin OS. They aren't trying to compete with the software users install later — they're a dependable foundation that works offline, on any hardware, from day one.

**Five apps. One repo. One design system. One mission.**

| App | Purpose | Identity |
|-----|---------|----------|
| **Shelf** | File manager | Your filesystem, your way — tags, dual-pane, keyboard-driven |
| **Stack** | Markdown editor | The note-taking app that gets out of your way |
| **Prism** | Media viewer | Open anything — images, video, audio, PDF, 3D models |
| **Fade** | Media converter | Convert, resize, and process media without leaving your desktop |
| **Ghost** | Identity-randomizing browser | Leaves no imprint — randomized fingerprint and identity every session, websites can't profile you |

**This repo is standalone.** It is not part of the LibreWin-OS repo. Apps are developed here and integrated into the OS via the ISO build pipeline.

---

## Current Status

All five apps exist as functional prototypes (v0.1.0) migrated from `Librewin-OS/apps/`. Each has a working Tauri 2 + Svelte 5 frontend and Rust backend. No app has tests yet. See `docs/ROADMAP.md` for the plan forward.

**Archive Manager is not a separate app** — archive browsing/extraction is a feature being built into Shelf.

---

## Repo Layout

```
apps/
  shelf/              File manager (Tauri 2 app)
  stack/              Markdown editor (Tauri 2 app)
  prism/              Media viewer (Tauri 2 app)
  fade/             Media converter (Tauri 2 app)
  ghost/              Privacy browser (Tauri 2 app)
  Cargo.toml          Rust workspace root
docs/
  ROADMAP.md          Master roadmap across all apps
  shelf/              Shelf specs and milestones
  stack/              Stack specs and milestones
  prism/              Prism specs and milestones
  fade/             Fade specs and milestones
  ghost/              Ghost specs and milestones
CLAUDE.md             This file
README.md             Public-facing project description
LICENSE               GPL-3.0
```

Each app follows the same internal structure:
```
apps/{name}/
  package.json        Frontend deps
  vite.config.js      Build config
  svelte.config.js    Svelte preprocessor
  index.html          Entry point
  src/                Svelte 5 frontend
    App.svelte        Window shell + titlebar
    app.css           Design tokens + Tailwind
    main.js           Mount point
    lib/              Components
  src-tauri/          Rust backend
    Cargo.toml        Rust deps
    tauri.conf.json   App config
    src/
      lib.rs          IPC commands
      main.rs         Entry point
```

---

## How to Run

```bash
cd apps/{name}
npm install
npm run tauri dev
```

Requirements: Rust toolchain (1.77+), Node 22+, system deps per app.

First build is slow (compiles Tauri + deps). After that, Vite HMR handles frontend changes instantly and Rust changes trigger a fast incremental recompile.

### Dev Server Ports

| App | Port |
|-----|------|
| Shelf | 1420 |
| Ghost | 1421 |
| Stack | 1422 |
| Prism | 1423 |
| Fade | 1427 |

---

## Tech Stack (Uniform Across All Apps)

- **Tauri 2** — window management, IPC, file system, custom protocol handlers
- **Svelte 5** — frontend (runes syntax only: `$state`, `$derived`, `$effect`, `$props`, `$bindable`)
- **Vite 8** + **Tailwind CSS 4** — build + styling
- **Rust (edition 2021, rust-version 1.77)** — backend
- **Geist** font via `@fontsource/geist`

### App-Specific Dependencies

- **Shelf**: `xattr` (tags), `tauri-plugin-global-shortcut` (Super+E)
- **Stack**: `pulldown-cmark` (markdown→HTML), `rfd` (file dialogs), CodeMirror 6
- **Prism**: `pdf.js` (PDF rendering), `three.js` (3D models), custom `mvstream://` protocol
- **Fade**: FFmpeg + ImageMagick (external CLI tools)
- **Ghost**: WebKitGTK (via Tauri webview), toolbar injection script

---

## Design System

All apps share the same visual language. See `Business-OS/ENGINEERING_STANDARDS.md` for the full spec.

- **Light mode default, dark mode supported** — both first-class via CSS variables
- **Accent:** `#0066cc` via `--accent` CSS variable (user-configurable via `~/.config/librewin/accent`) — `#297acc` was the old erroneous default; all CSS `:root` blocks and Rust `get_accent` fallbacks must use `#0066cc`
- **Font:** Geist, 14px base, 1.5 line height, antialiased
- **Spacing:** 4px base unit, compact density (`py-1.5 px-3`)
- **Window chrome:** Custom titlebar, `decorations: false`, `transparent: true`, 10px border-radius, drop shadow
- **Scrollbars:** 6px width, semi-transparent thumb
- **Motion:** Minimal and functional only — 120ms fade, 200ms fly, no decorative animation
- **Theme sync:** All apps read from `~/.config/librewin/theme` and `~/.config/librewin/accent`

---

## Shared Patterns

### Tauri 2 Commands
```rust
#[tauri::command]
fn my_command(state: State<'_, AppState>) -> Result<String, String> {
    // Never hold Mutex lock when spawning threads
    // Extract data first, drop lock, then spawn
    let data = state.inner.lock().unwrap().clone();
    Ok(data)
}
```

### Svelte 5 Runes (No Legacy Syntax)
```svelte
let count = $state(0);
let doubled = $derived(count * 2);

$effect(() => {
    const unlisten = listen('event', handler);
    return () => { unlisten.then(f => f()); }; // Always return cleanup
});
```

### Window Controls (Every App)
```svelte
<!-- Custom titlebar with drag region -->
<div data-tauri-drag-region class="titlebar">
    <button onclick={() => invoke('minimize_window')}>−</button>
    <button onclick={() => invoke('toggle_maximize')}>□</button>
    <button onclick={() => invoke('close_window')}>×</button>
</div>
```

### Theme Detection (Every App)
```javascript
const theme = await invoke('get_theme');
const accent = await invoke('get_accent');
if (theme === 'dark') document.documentElement.classList.add('dark');
document.documentElement.style.setProperty('--accent', accent);
```

---

## Known Patterns & Gotchas

Non-obvious permanent truths about this codebase. Add immediately when discovered — never wait until session end.

**Accent color is `#0066cc`, not `#297acc`.**
`#297acc` was the old erroneous default. Any CSS `:root` block or Rust `get_accent` fallback still using `#297acc` is a bug. Search for it before shipping.

**Never hold a Mutex lock while spawning a thread in Tauri commands.**
Extract the data you need, drop the lock, then spawn. Holding the lock across `thread::spawn` or `tokio::spawn` causes deadlocks on re-entrant IPC calls. The code pattern in "Shared Patterns → Tauri 2 Commands" above is the correct reference.

**Svelte 5 runes only — no legacy reactive syntax.**
Never use `$:`, `createEventDispatcher`, or `export let` in any component. Only `$state`, `$derived`, `$effect`, `$props`, `$bindable`. The compiler won't always error on legacy syntax mixed with runes — it silently misbehaves.

**Custom titlebar requires four things in lockstep: `decorations: false` + `transparent: true` + 10px border-radius + drop shadow.**
Missing any one causes visual breakage. `decorations: false` without `transparent: true` leaves a white flash on open. Border-radius without a drop shadow looks disconnected on light backgrounds. All four must be set per-app in `tauri.conf.json` and the app CSS.

**Theme sync reads from `~/.config/librewin/theme` and `~/.config/librewin/accent` via `librewin_common` helpers.**
Do not read these files directly from app code — use `get_theme()` and `get_accent()` IPC commands backed by librewin_common. This ensures consistent fallback behavior (`light` / `#0066cc`) across all apps.

**Linux dev renderer is WebKit2GTK 4.1 — declare all system deps in `tauri.conf.json`.**
Undeclared deps install fine on dev machines (already present) but break in CI and ISO builds. macOS uses a different WebKit — Linux-specific rendering bugs won't surface in `tauri dev` on macOS.

**Fade: FFmpeg and ImageMagick must be in PATH — Tauri does not bundle them.**
On macOS dev, set PATH explicitly in the shell that runs `tauri dev`. On Linux, ensure packages are declared in the `.desktop` file's `# Runtime OS packages:` comment and in `LibreWin-OS/build/modules/packages.sh`. A missing binary produces a cryptic "command not found" from inside the Rust process, not a Tauri error.

**`$effect` cleanup must always return the unlisten function — not call it.**
`return () => { unlisten.then(f => f()); }` — the cleanup runs when the component unmounts. Forgetting the return leaks event listeners and causes ghost event handlers after navigation.

<!-- Add new entries below this line -->

---

## Cross-Platform Notes

- **Linux (primary):** WebKit2GTK 4.1 renderer. Declare system deps in `tauri.conf.json`. FFmpeg/ImageMagick must be in PATH.
- **macOS:** Test production `.app` bundles, not just `tauri dev`. FFmpeg needs explicit PATH config.
- **Windows:** `windows_subsystem = "windows"` in `main.rs`. Use `Path::join()`, never hardcode `/`.

---

## LibreWin OS Integration Contract

Each release of this repo publishes these assets for every app:
- `{app}-x86_64` and `{app}-aarch64` — the Tauri binaries
- `{app}.desktop` — `.desktop` file and MIME type declarations (source of truth for OS integration)

**The `.desktop` file is the contract between this repo and LibreWin-OS.**
`LibreWin-OS/build/fetch-apps.sh` downloads it alongside the binary and installs it into the ISO.

### When you change OS integration for an app:
1. Edit `apps/{app}/io.librewin.{app}.desktop` — this is the source of truth
2. Tag a new release — the CI uploads it automatically
3. No changes needed in LibreWin-OS (it will pick up the new file on next build)

### When an app gains a new system runtime dependency:
1. Add/update the `# Runtime OS packages: ...` comment at the top of `apps/{app}/io.librewin.{app}.desktop`
2. **Also open a PR in `eldo9000/LibreWin-OS`** to add the package to `build/modules/packages.sh`
   under the "Freedom App runtime deps" comment block. The OS won't install deps it doesn't know about.

### Binary install names (do not rename without updating LibreWin-OS `apps.sh`/`branding.sh`):
| App | Installed as in `/usr/local/bin/` |
|-----|----------------------------------|
| shelf | `shelf` |
| ghost | `ghost` |
| prism | `librewin-prism` |
| stack | `librewin-stack` |
| fade | `fade` |

### Local development:
```bash
cd apps/{name}
npm install
npx tauri dev
```
Runs the full app natively on macOS or Linux. Linux-specific system calls (kscreen-doctor, nmcli, etc.)
are only in Settings — all Freedom Apps work on macOS for development purposes.

---

## Workflow

- **Don't burn tokens reading entire codebases** — read this file and app-specific docs first. Only dive into code for specific tasks.
- **One app at a time.** Focus on the app you're working on. Don't make cross-app changes unless updating shared patterns.
- **Commit at least once per milestone** — don't let work accumulate.
- **Update milestone docs** when completing work — mark tasks done, note any discoveries.
- **Check `docs/ROADMAP.md`** to understand where each app stands before starting work.

