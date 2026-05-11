# TASK-2: FadeSection — media converter gallery mockup

## Goal
`gallery/src/sections/FadeSection.svelte` has zero dashed placeholder boxes — every card contains a fully implemented, interactive Svelte 5 UI mockup that looks like a real media converter app.

## Context
Fade is a batch media converter app (audio/video/image) in the Libre family. The gallery Applications tab has a FadeSection with 7 cards (FD-1 through FD-7), each containing a `<div class="ph">` dashed placeholder. Replace every placeholder with real UI.

**Repo:** `/Users/eldo/Downloads/Github/Libre-Apps`  
**File to edit:** `gallery/src/sections/FadeSection.svelte`  
**Run gallery:** `cd gallery && npm run dev` (Vite-only on :1422)

**Reference files to read before starting:**
- `gallery/src/lib/panels/FadePanel.svelte` — exact style patterns for the fd-seg buttons, ir-* collapsible sections, bitrate mode toggle, range slider. Reuse these class names verbatim.
- `gallery/src/lib/Card.svelte` — Card wrapper pattern (already used in the file)
- `common-js/src/tokens.css` — token reference, including `.seg-active` / `.seg-inactive` classes

**Available @libre/ui components** (import from `'@libre/ui'`):
`Button`, `Input`, `Select`, `Slider`, `SegmentedControl`, `Checkbox`

**Design system rules:**
- Svelte 5 runes only: `$state`, `$derived`, `$effect`, `$props` — no `$:` or `createEventDispatcher`
- Tokens only — no hardcoded colors. Use `var(--surface)`, `var(--surface-raised)`, `var(--surface-panel)`, `var(--border)`, `var(--border-subtle)`, `var(--text-primary)`, `var(--text-secondary)`, `var(--text-muted)`, `var(--accent)`
- No hardcoded hex colors (the `.ir-dot` `background:#555` in FadePanel is a known exception — you may copy it)
- All `$state` for interactive demo state — no real IPC

## In scope
- `gallery/src/sections/FadeSection.svelte` — full replacement

## Out of scope
- `common-js/src/` — do not edit any shared component
- `gallery/src/lib/panels/FadePanel.svelte` — do not edit
- Any other gallery section or App.svelte

## Steps

1. Read `gallery/src/lib/panels/FadePanel.svelte` in full. Note the exact CSS for `.fd-seg`, `.fd-seg-btn`, `.fd-seg-first`, `.fd-seg-last`, `.fd-seg-on`, `.fd-range`, `.fd-range-labels`, `.ir`, `.ir-section`, `.ir-sec-hd`, `.ir-row`, `.ir-lbl`. Copy these styles into FadeSection's `<style>` block.

2. At the top of the script block, import `Button`, `Input`, `Select`, `SegmentedControl` from `'@libre/ui'`. Import `Card` from `'../lib/Card.svelte'`.

3. Declare `$state`:
   - `queueItems` — array of 4 objects `{ name, ext, size, status, progress }` where status is one of `'done'`, `'converting'`, `'queued'`, `'error'`
   - `selectedFormat` — string, default `'mp4'`
   - `outputPath` — string, default `'~/Downloads/Converted'`
   - `videoCodec` — `'h264'`
   - `videoBitrate` — `4` (Mbps)
   - `videoRes` — `'1080p'`
   - `audioCodec` — `'aac'`
   - `audioBitrate` — `192`
   - `audioSampleRate` — `'44100'`
   - `audioChannels` — `'stereo'`
   - `audioMode` — `'cbr'`
   - `videoCollapsed` — `false`
   - `audioCollapsed` — `false`

4. Implement each card:

   **FD-1 — Drop Zone** (full width, ~128px):
   - Centered dashed border box (border: `2px dashed var(--border)`, border-radius 12px, background `color-mix(in srgb, var(--surface-raised) 60%, transparent)`)
   - Upload icon SVG (arrow-up or cloud-upload, 28px, `var(--text-muted)`)
   - Text: "Drop files to convert" (14px, `var(--text-secondary)`) + "or click to browse" (12px, `var(--text-muted)`)
   - Hover: border changes to `var(--accent)`, background gets slight accent tint
   - `$state` `dragOver = false` — toggle on mouseenter/mouseleave for the hover style

   **FD-2 — Queue List** (~380px wide, ~320px tall):
   - Header row: "4 files" label (left) + "Clear done" button (right, small, `var(--text-muted)`)
   - 4 queue rows, each 60px tall with:
     - File type icon (colored square 28×28px: video=accent-tinted, audio=green-tinted using `color-mix(in srgb, #4ade80 30%, var(--surface-raised))`)
     - Filename + original size (secondary)
     - Status badge: `done` = green text "Done", `converting` = accent text "Converting…" + progress bar below, `queued` = muted "Queued", `error` = red-ish "Error"
     - Remove × button on right (visible on row hover)
   - The `converting` item shows a thin progress bar (8px tall, full width of the row, accent fill at 60%)

   **FD-3 — Format Preset Cards** (~280px wide, ~320px tall):
   - Title: "Output Format" (10px uppercase `var(--text-muted)`)
   - 6 preset buttons in a 2-column grid: MP4, MP3, WebP, GIF, MOV, WAV
   - Each button: 80×52px, centered label (13px), format badge (9px uppercase), border `var(--border)`, border-radius 8px
   - Selected format gets accent border + `color-mix(in srgb, var(--accent) 8%, var(--surface-raised))` background
   - `onclick` updates `selectedFormat`

   **FD-4 — Output Path Picker** (full width, ~44px):
   - Flex row: folder icon (SVG, 16px, `var(--text-muted)`) + path text (`outputPath`, monospace 12px, `var(--text-secondary)`, `flex:1`) + "Browse…" button
   - Border: `1px solid var(--border)`, border-radius 6px, padding `8px 12px`, background `var(--surface)`

   **FD-5 — Conversion Progress Row** (full width, ~64px):
   - Filename on left (the `converting` item from queue), percent right
   - Full-width progress bar below: track = `var(--border)`, fill = `var(--accent)`, height 4px, border-radius 2px, animated via CSS `animation: progress-pulse 2s ease-in-out infinite`
   - Speed readout: "2.4× realtime — 00:01:23 remaining" in 11px `var(--text-muted)`
   - Define `@keyframes progress-pulse` that nudges the fill width between 58% and 65%

   **FD-6 — Video Codec Options** (~300px wide, ~280px tall):
   - Collapsible section (same ir-sec-hd pattern from FadePanel):
     - "Video" section header with chevron
     - Codec: fd-seg buttons [H.264, H.265, VP9] bound to `videoCodec`
     - Bitrate (Mbps): `<input type="range">` 1–20 bound to `videoBitrate` + value label
     - Resolution: fd-seg buttons [720p, 1080p, 4K] bound to `videoRes`
     - Frame Rate: row with label + `<select>` [23.976, 24, 25, 29.97, 60]
     - Two-pass encoding: checkbox row

   **FD-7 — Audio Codec Options** (~300px wide, ~280px tall):
   - Same collapsible section pattern:
     - "Audio" section header with chevron
     - Codec: fd-seg [AAC, MP3, FLAC, Opus] bound to `audioCodec`
     - Bitrate Mode: fd-seg [CBR, VBR] bound to `audioMode` — when CBR show bitrate row, when VBR show quality slider (copy pattern from FadePanel exactly)
     - Sample Rate: fd-seg [44.1k, 48k] bound to `audioSampleRate`
     - Channels: fd-seg [Mono, Stereo, Source] bound to `audioChannels`

5. Write the `<style>` block: copy the full fd-seg + ir-* CSS from FadePanel.svelte, add any additional classes needed (queue row, format card, drop zone, progress bar).

6. Run `cd /Users/eldo/Downloads/Github/Libre-Apps/gallery && npm run dev` — verify no console errors, all 7 cards show real content.

7. Run `cd /Users/eldo/Downloads/Github/Libre-Apps && npm run lint` — fix any ESLint errors.

## Success signal
- `npm run dev` starts without errors
- Applications → Fade section: 7 cards, none with a dashed placeholder
- FD-2 queue list shows 4 items with distinct status styles
- FD-3 clicking a format preset updates the selection highlight
- `npm run lint` exits 0

## Notes
- The fd-seg-on active state uses `color: #fff` in dark mode and `color: var(--text-primary)` in light mode — the FadePanel.svelte uses `:global(html:not(.dark)) .ir .fd-seg-on { color: var(--text-primary); }` — replicate this pattern (scope it to `.section` instead of `.ir` if needed).
- Do not use `<SegmentedControl>` for the fd-seg buttons in FLCK-6/7 — manually build them with fd-seg-btn classes to match FadePanel's exact visual style.
- For the `@keyframes progress-pulse`, animating `width` is fine since this is a mock demo element, not a real layout concern.
