# The Freedom Apps — Master Roadmap

## Overview

Five apps. Three phases. The goal is shipping a cohesive, polished suite of desktop apps that makes LibreWin OS feel complete on day one.

Each app has its own milestone docs in `docs/{app}/`. This roadmap is the high-level view of how they fit together.

---

## Current State (Pre-Phase 1)

All five apps were built as prototypes inside `Librewin-OS/apps/` and have been migrated to this standalone repo. Here's where each one stands:

| App | Maturity | Core Features | Tests | Key Gaps |
|-----|----------|--------------|-------|----------|
| **Shelf** | Beta | File browsing, tags, dual-pane, tabs, search, quick convert, Wine support | None | Archive support missing, no batch ops, no Shelf Mode |
| **Stack** | Beta | CodeMirror editor, multi-tab, export HTML/TXT/PDF, outline panel | None | Search/Format panels are placeholders, no syntax highlighting, no live preview |
| **Prism** | Beta | Image/video/audio/PDF/3D viewing, zoom, rotation, streaming protocol | None | No metadata viewer, no slideshow, no playlist |
| **Splice** | Alpha | Image/video/audio conversion, batch queue, presets, progress reporting | None | No cancel/pause, no preview, no AI features, no parallel encoding |
| **Ghost** | Alpha | Custom titlebar, address bar, navigation, download handler, search links | None | Privacy features NOT implemented (just relies on WebKitGTK defaults), no settings UI |

**Common gaps across all apps:** Zero test coverage. No CI pipeline. No linting/formatting enforcement.

---

## Phase 1 — Foundation & MVP Completion

**Goal:** Every app is genuinely useful as a daily driver. No placeholder UI. No features that don't work. Testing infrastructure in place.

### Shared Work (All Apps)
- [ ] Set up Cargo workspace (`apps/Cargo.toml`)
- [ ] Add CI workflow (GitHub Actions: build all 5 apps on Linux)
- [ ] Add `rustfmt` + `clippy` to CI
- [ ] Add `prettier` + basic lint config for Svelte/JS
- [ ] Establish vitest config for frontend smoke tests
- [ ] Establish `#[cfg(test)]` patterns for Rust unit tests

### Per-App Phase 1 Milestones

**Shelf** → [docs/shelf/M1.md](shelf/M1.md), [M2.md](shelf/M2.md)
- M1: Archive manager integration (browse/extract ZIP, 7Z, TAR inline)
- M2: Core polish (batch operations, file preview thumbnails, hot-plug volumes, error handling)

**Stack** → [docs/stack/M1.md](stack/M1.md), [M2.md](stack/M2.md)
- M1: Fix broken panels (search wired to CodeMirror, format panel functional)
- M2: Markdown experience (syntax highlighting in editor, markdown shortcuts)

**Prism** → [docs/prism/M1.md](prism/M1.md)
- M1: Metadata viewer + drag-and-drop + keyboard shortcut overlay

**Splice** → [docs/splice/M1.md](splice/M1.md), [M2.md](splice/M2.md)
- M1: Production hardening (cancel jobs, stderr logging, file info dialog)
- M2: Preview + parallel encoding

**Ghost** → [docs/ghost/M1.md](ghost/M1.md), [M2.md](ghost/M2.md)
- M1: Implement actual privacy features (session isolation, cookie clearing, tracker blocking)
- M2: Settings UI + browser fundamentals (zoom, fullscreen, find-in-page)

---

## Phase 2 — Power Features & Integration

**Goal:** Each app earns its reputation. Features that make users say "this is better than what I had on Windows."

### Per-App Phase 2 Milestones

**Shelf** → [docs/shelf/M3.md](shelf/M3.md)
- M3: Shelf Mode (global floating palette), xdg-desktop-portal backend, Wine drag-bridge

**Stack** → [docs/stack/M3.md](stack/M3.md)
- M3: Live preview, spell check, tab drag-to-break windows

**Prism** → [docs/prism/M2.md](prism/M2.md)
- M2: Slideshow mode, playlist/queue, batch export

**Splice** → [docs/splice/M3.md](splice/M3.md)
- M3: AI features Phase 1 (noise removal, upscaling, auto-captions)

**Ghost** → [docs/ghost/M3.md](ghost/M3.md)
- M3: Security audit, CSP enforcement, WebRTC leak prevention

---

## Phase 3 — Pro Tiers & Polish

**Goal:** Free versions are excellent. Pro features are worth paying for. Ship-ready for LibreWin 1.0.

Pro tiers come at $3–5 one-time purchase. The line between free and pro is the "minimalist ethos boundary" — when a feature strains the focused, simple identity of an app, it goes in Pro.

### Per-App Phase 3 Milestones

**Shelf** → [docs/shelf/M4.md](shelf/M4.md)
- M4: Pro features (network drives, advanced batch ops, sync, plugin system)

**Stack** → [docs/stack/M4.md](stack/M4.md)
- M4: Pro features (version history, PDF/DOCX export, templates, sync)

**Prism** → [docs/prism/M3.md](prism/M3.md)
- M3: Pro features (annotation, RAW adjustments, advanced slideshow)

**Splice** → [docs/splice/M4.md](splice/M4.md)
- M4: AI features Phase 2 (stem separation, face restoration, frame interpolation, background removal)

**Ghost** — No pro tier. Ghost stays free and minimal.

---

## Principles

1. **Free versions must be genuinely good.** Pro tiers are for power users who want more, not for users who want it to work properly.
2. **One design system.** Every app looks and feels like it belongs to the same family.
3. **Offline first.** No network calls. No telemetry. No accounts. (Ghost is the exception — it's a browser.)
4. **Ship together.** The Freedom Apps are a bundle. They release together with LibreWin OS.
5. **Test what ships.** No feature merges without at least smoke tests for the critical path.
