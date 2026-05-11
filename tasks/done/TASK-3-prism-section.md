# TASK-3: PrismSection — universal media viewer gallery mockup

## Goal
`gallery/src/sections/PrismSection.svelte` has zero dashed placeholder boxes — every card contains a fully implemented, interactive Svelte 5 UI mockup that looks like a real universal media viewer app.

## Context
Prism is a universal media viewer (images, video, audio, PDF, 3D models) in the Libre family. The gallery Applications tab has a PrismSection with 5 cards (PRS-1 through PRS-5), each containing a `<div class="ph">` dashed placeholder. Replace every placeholder with real UI.

**Repo:** `/Users/eldo/Downloads/Github/Libre-Apps`  
**File to edit:** `gallery/src/sections/PrismSection.svelte`  
**Run gallery:** `cd gallery && npm run dev` (Vite-only on :1422)

**Reference files to read before starting:**
- `gallery/src/sections/MediaSection.svelte` — Transport + Timecode import and rAF playback loop pattern (copy it exactly)
- `gallery/src/lib/Card.svelte` — Card wrapper pattern
- `gallery/src/sections/NavigationSection.svelte` — PanelTabs usage pattern
- `common-js/src/tokens.css` — token reference

**Available @libre/ui components** (import from `'@libre/ui'`):
`Transport`, `Timecode`, `PanelTabs`, `Button`, `Slider`

**Design system rules:**
- Svelte 5 runes only: `$state`, `$derived`, `$effect`, `$props` — no `$:` or `createEventDispatcher`
- Tokens only — no hardcoded colors
- All `$state` for interactive demo state — no real IPC

## In scope
- `gallery/src/sections/PrismSection.svelte` — full replacement

## Out of scope
- `common-js/src/` — do not edit
- Any other gallery section or App.svelte

## Steps

1. Read `gallery/src/sections/MediaSection.svelte` in full. Copy the rAF playback loop state and functions (`playing`, `playbackRate`, `time`, `duration`, `fps`, `tick`, `startLoop`, `togglePlay`, `rewind`, `forward`, `skipStart`, `skipEnd`, `onDestroy` cleanup) into the script block. Use `duration = 245.0` and `fps = 24`.

2. Import `Transport`, `Timecode`, `PanelTabs`, `Button`, `Slider` from `'@libre/ui'`. Import `Card` from `'../lib/Card.svelte'`. Import `{ onDestroy }` from `'svelte'`.

3. Declare additional `$state`:
   - `zoom` — number, default `100` (percent)
   - `rotation` — number, default `0` (degrees, cycles through 0/90/180/270)
   - `imageTab` — string, default `'info'`
   - `pdfPage` — number, default `1`, max `12`
   - `audioVolume` — number, default `80`
   - `shortcutsOpen` — boolean, default `false`

4. Implement each card:

   **PRS-1 — Media Viewer (Image)** (~480px wide, ~340px tall):
   - Top toolbar row: filename label left ("mountain_vista.jpg"), PanelTabs right with tabs `[{id:'info', label:'Info'}, {id:'edit', label:'Edit'}, {id:'share', label:'Share'}]` bound to `imageTab`
   - Main image area (16:9, background `color-mix(in srgb, var(--surface) 40%, #000)`): centered SVG mountain-silhouette illustration using simple paths in `var(--text-muted)` tones
   - Bottom toolbar row: zoom-out button (−) + zoom display (`{zoom}%`) + zoom-in button (+) + separator + rotate button (↺) + fit button (⊡) + fullscreen button
   - Zoom +/− updates `zoom` state (clamp 10–400), rotate increments `rotation` by 90

   **PRS-2 — 3D Model Viewer** (~360px wide, ~320px tall):
   - Dark viewport area (`color-mix(in srgb, var(--surface) 30%, #000)`, border-radius 6px)
   - Inside: SVG wireframe cube (8 vertices projected to 2D isometric view using SVG lines, `var(--accent)` color at 60% opacity)
   - Axis indicator in bottom-left corner: 3 colored lines for X/Y/Z (small, 20px)
   - Bottom toolbar: orbit/pan/zoom mode buttons (3 icon buttons), "Reset View" text button
   - Filename: "model_v3.glb" in 11px `var(--text-muted)` above the viewport

   **PRS-3 — PDF Viewer** (~340px wide, ~440px tall):
   - Top toolbar: prev page (‹) + "Page {pdfPage} of 12" + next page (›) + zoom controls
   - Page area: white/light surface with subtle shadow, constrained inner width (~260px), simulated page content using horizontal bars (text lines: alternating 100%/80%/60% width bars in `var(--border)`, ~6px tall, spaced 10px apart)
   - First "line" simulates a heading (taller, `var(--text-muted)` at more opacity)
   - Clicking prev/next updates `pdfPage` (clamp 1–12)

   **PRS-4 — Audio Player** (~400px wide, ~100px tall):
   - Single compact row layout:
     - Left: audio file icon (SVG waveform symbol, 32×32, accent color)
     - Center column: filename "ambient_loop.flac" (13px primary) + artist/album "Unknown · FLAC 24-bit 96kHz" (11px muted)
     - Then: `<Transport {playing} {playbackRate} onTogglePlay={togglePlay} onRewind={rewind} onForward={forward} onSkipStart={skipStart} onSkipEnd={skipEnd} />` (no fullscreen)
     - Below transport: `<Timecode {time} {duration} fps={0} />` (fps=0 shows MM:SS format — or use fps=1 if 0 causes issues)
   - Right: volume icon + `<input type="range" min="0" max="100" bind:value={audioVolume} class="vol-slider" />`

   **PRS-5 — Shortcuts Overlay** (~560px wide, ~400px tall):
   - Simulates a keyboard shortcuts overlay panel (like pressing ? in the app)
   - Header: "Keyboard Shortcuts" (16px, primary) + close × button
   - 3 columns of shortcut groups:
     - **Navigation:** Space Play/Pause, ← Prev frame, → Next frame, J Rewind, L Forward, Home First frame, End Last frame
     - **View:** + Zoom in, − Zoom out, 0 Fit to window, R Rotate 90°, F Fullscreen, I Toggle info
     - **Files:** O Open file, ⌘← Prev file, ⌘→ Next file, Del Move to trash
   - Each row: `<kbd>` styled key chip (background `var(--surface-raised)`, border `var(--border)`, border-radius 4px, monospace 11px, padding 2px 6px) + description (12px `var(--text-secondary)`)
   - Group headers: 9px uppercase `var(--text-muted)` with bottom border

5. Write the `<style>` block with all needed classes. Key patterns:
   - `.viewer-toolbar` — flex row, align-center, gap 8px, padding 6px 10px, border-bottom/top 1px solid var(--border)
   - `.viewer-area` — flex 1, display flex, align/justify center, overflow hidden
   - `.kbd` — inline-block, padding 2px 6px, border 1px solid var(--border), border-radius 4px, font-family monospace, font-size 11px, background var(--surface-raised)
   - `.vol-slider` — width 80px, accent-color var(--accent)

6. Run `cd /Users/eldo/Downloads/Github/Libre-Apps/gallery && npm run dev` — verify all 5 cards render with real content and no console errors.

7. Run `cd /Users/eldo/Downloads/Github/Libre-Apps && npm run lint` — fix any ESLint errors.

## Success signal
- `npm run dev` starts without errors
- Applications → Prism section: 5 cards, none with a dashed placeholder
- PRS-4 audio player transport plays/pauses and Timecode advances
- PRS-1 zoom + / − buttons update the zoom display
- `npm run lint` exits 0

## Notes
- For `Timecode` with audio (no real FPS concept), pass `fps={1}` — this makes it display as MM:SS.FF which is close enough for a mock. Alternatively pass `fps={0}` and verify it doesn't crash; if it does, use `fps={1}`.
- The SVG wireframe cube for PRS-2 can be simple: draw the 12 edges of a cube in isometric projection (top face, front face, right face). Static SVG is fine — no rotation needed.
- For the PRS-3 "page content" simulation, use a series of `<div class="text-line">` elements with varying widths, not actual text. This avoids font rendering differences.
- The shortcuts overlay (PRS-5) is purely static display — no `shortcutsOpen` toggle needed since it's always shown inside the Card body.
