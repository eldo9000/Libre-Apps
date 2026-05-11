# TASK-1: FlickerSection — NLE gallery mockup

## Goal
`gallery/src/sections/FlickerSection.svelte` has zero dashed placeholder boxes — every card contains a fully implemented, interactive Svelte 5 UI mockup that looks like a real NLE (non-linear video editor).

## Context
Flicker is a non-linear video editor app in the Libre family. The gallery Applications tab has a FlickerSection that currently shows 13 cards (FLCK-1 through FLCK-13), each containing a `<div class="ph">` dashed placeholder. Replace every placeholder with real UI.

**Repo:** `/Users/eldo/Downloads/Github/Libre-Apps`  
**File to edit:** `gallery/src/sections/FlickerSection.svelte`  
**Run gallery:** `cd gallery && npm run dev` (Vite-only on :1422, sufficient for layout work)

**Reference files to read before starting:**
- `gallery/src/lib/Card.svelte` — Card wrapper pattern (already used in the file)
- `gallery/src/lib/panels/FlickerInspector.svelte` — exact style patterns (ir-*, collapsible sections, ir-sec-hd, ir-row, ir-lbl, ir-num, ir-dot, ir-chev) — reuse these CSS class names and patterns verbatim
- `gallery/src/sections/MediaSection.svelte` — how Transport + Timecode are imported and wired
- `gallery/src/sections/NavigationSection.svelte` — how GlobalTabs + PanelTabs are imported
- `common-js/src/tokens.css` — token reference

**Available @libre/ui components** (import from `'@libre/ui'`):
`Transport`, `Timecode`, `GlobalTabs`, `PanelTabs`, `SegmentedControl`, `Button`, `Slider`

**Design system rules:**
- Svelte 5 runes only: `$state`, `$derived`, `$effect`, `$props` — no `$:` or `createEventDispatcher`
- Tokens only — no hardcoded colors. Use `var(--surface)`, `var(--surface-raised)`, `var(--surface-panel)`, `var(--border)`, `var(--border-subtle)`, `var(--text-primary)`, `var(--text-secondary)`, `var(--text-muted)`, `var(--accent)`, `var(--radius)`
- Hover states: `color-mix(in srgb, var(--surface-raised) 85%, var(--text-primary))`
- No local `--ir-*` variable override blocks — use global tokens directly
- All `$state` for interactive demo state — no real IPC

## In scope
- `gallery/src/sections/FlickerSection.svelte` — full replacement

## Out of scope
- `common-js/src/` — do not edit any shared component
- `gallery/src/lib/panels/FlickerInspector.svelte` — do not edit
- Any other gallery section
- `gallery/src/App.svelte` — card IDs are already registered, no changes needed

## Steps

1. Read `gallery/src/lib/panels/FlickerInspector.svelte` and `gallery/src/sections/MediaSection.svelte` in full. Note the exact CSS class patterns used in FlickerInspector (ir-section, ir-sec-hd, ir-row, ir-lbl, etc.) — replicate them in FlickerSection's local `<style>`.

2. At the top of the script block, import: `Transport`, `Timecode`, `GlobalTabs`, `PanelTabs`, `SegmentedControl` from `'@libre/ui'`. Also import `Card` from `'../lib/Card.svelte'` and `{ onDestroy }` from `'svelte'`.

3. Declare all `$state` for the Transport/Timecode mock: `playing`, `playbackRate`, `time` (same rAF loop pattern as MediaSection.svelte). Use `duration = 312.8` and `fps = 24`.

4. Implement each card:

   **FLCK-1 — Node Strip** (full width, ~52px tall):
   - Horizontal scrollable strip of 6 clip thumbnail boxes
   - Each clip: 80×44px colored rectangle (use `var(--surface-raised)` + `var(--border)`) with a clip name label below (10px, `var(--text-muted)`)
   - Clip types: V1 (video, blue-tinted border using `color-mix(in srgb, var(--accent) 40%, var(--border))`), A1 (audio, subtler), etc.
   - Active clip gets accent border

   **FLCK-2 — Timeline** (full width, ~140px tall):
   - Time ruler at top: row of tick marks with frame numbers (0, 24, 48, 72, 96, 120) in 10px monospace `var(--text-muted)`
   - 3 track lanes below: V1 (video), A1 (audio left), A2 (audio right)
   - Each lane: 28px tall, label on left (40px), clip blocks filling the rest
   - V1: 2 clip blocks with titles; A1/A2: waveform-style blocks (use repeating CSS gradient to fake a waveform)
   - Playhead: a 1px accent-colored vertical line positioned at `(time / duration * 100)%` across the track area

   **FLCK-3 — Transport Bar** (full width, ~48px):
   - Left: `<Timecode {time} {duration} {fps} />`
   - Center: `<Transport {playing} {playbackRate} onTogglePlay={togglePlay} onRewind={rewind} onForward={forward} onSkipStart={skipStart} onSkipEnd={skipEnd} showFullscreen />`
   - Right: zoom level display ("100%") + `<SegmentedControl options={trackZoomOptions} bind:value={trackZoom} variant="sliding" size="sm" />`
   - Layout: flex row, space-between, align-items center

   **FLCK-4 — Compositor** (full width, ~340px):
   - Dark surface area (`var(--surface)` border, dark background `color-mix(in srgb, var(--surface) 60%, #000)` for the canvas area)
   - 16:9 aspect-ratio inner frame (use `aspect-ratio: 16/9`, max-width 540px)
   - Inside the frame: a gradient placeholder simulating a video frame (dark gradient)
   - Bottom of compositor: `<PanelTabs tabs={compositorTabs} bind:active={compositorTab} />` where compositorTabs = `[{id:'color', label:'Color'}, {id:'effects', label:'Effects'}, {id:'audio', label:'Audio'}]`

   **FLCK-5 — Color Wheel** (~220×220px):
   - SVG-based HSV color wheel: outer ring = hue gradient (conic-gradient), inner triangle or just a circle
   - Use a `<div>` with `border-radius: 50%`, `background: conic-gradient(red, yellow, lime, cyan, blue, magenta, red)`, width/height 160px
   - Current color swatch below: 32×16px box filled with `var(--accent)`
   - Label: "Hue 210°  Sat 80%  Val 90%" in 10px monospace

   **FLCK-6 — Curves Editor** (~260×260px):
   - SVG canvas: 200×200px, dark background `var(--surface)`, grid lines
   - Grid: 4×4 lines using `var(--border-subtle)`
   - Diagonal baseline: gray line from (0,200) to (200,0)
   - RGB curves: 3 colored SVG paths (use opacity-appropriate stroke colors close to red/green/blue but via `color-mix`) — just static S-curve shapes
   - Channel selector at top: small buttons R G B A (use PanelTabs or manual buttons)

   **FLCK-7 — Scopes** (~360×220px):
   - Two scope views side by side: Waveform (left, ~180px wide) + Vectorscope (right, ~120×120px circle)
   - Waveform: SVG with random-looking waveform lines using `var(--accent)` at low opacity
   - Vectorscope: SVG circle with `conic-gradient` hints and a scatter of dots near center
   - Labels: "Waveform" / "Vectorscope" in 9px uppercase `var(--text-muted)`

   **FLCK-8 — Color Grading Tab** (full width, ~320px):
   - `<GlobalTabs tabs={colorTabs} bind:active={colorTab} />` at top where colorTabs = Primary / Secondary / LUTs / HSL
   - Below: when `colorTab === 'primary'`, show 3 lift/gamma/gain wheels (reuse the color wheel circle style at 80px each) side by side with labels
   - When other tabs: show a placeholder label saying e.g. "Secondary grading — select a qualifier"

   **FLCK-9 — Audio Mixer** (full width, ~200px):
   - 5 mixer strips side by side: A1, A2, A3, Music, Master
   - Each strip: 80px wide, vertical fader (input type=range, orient vertical via `writing-mode: vertical-lr`), pan knob (a small circular div), mute/solo buttons, VU meter (3 green segments + 1 yellow + 1 red using `var(--accent)` shades)
   - Strip labels at bottom

   **FLCK-10 — Media Pool** (~340×280px):
   - Search bar at top (same `.search-compact` pattern from NavigationSection)
   - Grid of 6 media item thumbnails (3 across): each 90×60px with aspect-ratio box, filename below
   - Mix of "video" (darker with play triangle) and "audio" (waveform icon) items
   - Selected item gets accent border

   **FLCK-11 — Folder Tree** (~180×280px):
   - Tree list: Project Root, ├─ Footage (expanded, 3 items), ├─ Music (collapsed), └─ Exports (collapsed)
   - Each row: 28px, triangle chevron + folder icon (SVG) + name
   - Use the ir-sec-hd style for rows (hover effect, small chevron)

   **FLCK-12 — Clip Inspector** (~200×320px):
   - Render a minimal version of the FlickerInspector content inline (do NOT import the panel component)
   - Show: clip name at top, then 3 collapsible sections: Transform (pos X/Y, scale, rotation, opacity), Composite (blend mode select), Speed (speed %, reverse checkbox)
   - Reuse ir-* CSS classes exactly as in FlickerInspector.svelte

   **FLCK-13 — Export / Delivery** (~480×320px):
   - Left column (~200px): format preset list (H.264 MP4, ProRes 422, DNxHD, WebM VP9, GIF) — each a clickable row, active gets accent border
   - Right column: settings for selected format:
     - Resolution: SegmentedControl (1080p / 720p / 4K)
     - Frame rate: Select (23.976 / 24 / 25 / 29.97 / 60)
     - Quality: Slider (0–100, bind to $state)
     - Output path row with a mock path string + "Browse" button
   - Bottom: "Export" Button (variant="primary", full width)

5. Write the `<style>` block. Mirror the ir-* class set from FlickerInspector exactly (copy the CSS verbatim for: `.ir`, `.ir-section`, `.ir-sec-hd`, `.ir-dot`, `.ir-sec-title`, `.ir-chev`, `.ir-row`, `.ir-lbl`, `.ir-num`, `.ir-val`, `.ir-sel`, `.ir-check`). Add additional classes for timeline, compositor, mixer strips, media pool.

6. Run `cd /Users/eldo/Downloads/Github/Libre-Apps/gallery && npm run dev` — verify no console errors and all 13 cards render with real content (no dashed boxes).

7. Run `cd /Users/eldo/Downloads/Github/Libre-Apps && npm run lint` — fix any ESLint errors before declaring done.

## Success signal
- `npm run dev` in `gallery/` starts without errors
- Navigating to the Applications tab → Flicker section shows 13 cards, none with a dashed placeholder box
- Transport card (FLCK-3) plays/pauses and the Timecode and timeline playhead advance in sync
- `npm run lint` exits 0

## Notes
- The rAF playback loop: copy the exact pattern from `gallery/src/sections/MediaSection.svelte` lines 13–54. Import `onDestroy` from `'svelte'` for cleanup.
- FlickerInspector uses hardcoded `style="background:#555"` on `.ir-dot` — this is acceptable, it's a visual accent dot not a semantic color. Keep it.
- For the vertical fader in the mixer, use `<input type="range" orient="vertical" style="writing-mode: vertical-lr; direction: rtl;">` — this works cross-browser in WebKit.
- `SegmentedControl` with `variant="sliding"` requires `options` array with `{value, label}` shape (optionally `icon`).
- `GlobalTabs` takes `tabs={[{id, label}]}` and `bind:active`.
- `PanelTabs` takes `tabs={[{id, label}]}` and `bind:active`.
