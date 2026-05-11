# TASK-5: StackSection — markdown editor gallery mockup

## Goal
`gallery/src/sections/StackSection.svelte` has zero dashed placeholder boxes — every card contains a fully implemented, interactive Svelte 5 UI mockup that looks like a real Markdown editor app.

## Context
Stack is a Markdown editor app in the Libre family (CodeMirror editor, multi-tab, outline panel, export). The gallery Applications tab has a StackSection with 6 cards (STK-1 through STK-6), each containing a `<div class="ph">` dashed placeholder. Replace every placeholder with real UI.

**Repo:** `/Users/eldo/Downloads/Github/Libre-Apps`  
**File to edit:** `gallery/src/sections/StackSection.svelte`  
**Run gallery:** `cd gallery && npm run dev` (Vite-only on :1422)

**Reference files to read before starting:**
- `gallery/src/lib/Card.svelte` — Card wrapper pattern
- `gallery/src/sections/NavigationSection.svelte` — Tabs component usage (horizontal tabs with panel snippets)
- `common-js/src/tokens.css` — token reference

**Available @libre/ui components** (import from `'@libre/ui'`):
`Button`, `Input`, `Tabs`, `SegmentedControl`, `Checkbox`

**Design system rules:**
- Svelte 5 runes only: `$state`, `$derived`, `$effect` — no `$:` or `createEventDispatcher`
- Tokens only — no hardcoded colors
- All `$state` for interactive demo state — no real IPC

## In scope
- `gallery/src/sections/StackSection.svelte` — full replacement

## Out of scope
- `common-js/src/` — do not edit
- Any other gallery section or App.svelte

## Steps

1. Import `Button`, `Input`, `Tabs`, `SegmentedControl` from `'@libre/ui'`. Import `Card` from `'../lib/Card.svelte'`.

2. Declare `$state`:
   - `activeDocTab` — string, default `'notes'`
   - `editorMode` — string, default `'edit'` (for SegmentedControl: edit/split/preview)
   - `editorContent` — string with mock markdown text (see below)
   - `activeHeading` — string, default `'h2-architecture'`
   - `searchQuery` — string, default `''`
   - `replaceQuery` — string, default `''`
   - `bold` — boolean, default `false`
   - `italic` — boolean, default `false`
   - `treeExpanded` — object, default `{ notes: true, docs: false, archive: false }`

   Seed `editorContent` with this mock markdown string (use a template literal):
   ```
   # Libre Apps — Architecture Notes\n\n## Overview\n\nThe shared foundation provides a consistent design system across all five apps.\n\n## Components\n\nEvery component uses CSS custom properties for theming.\n\n### Button\n\nPrimary, secondary, and ghost variants.\n\n## Token System\n\nDesign tokens are the single source of truth for all visual values.
   ```

3. Implement each card:

   **STK-1 — Editor Toolbar** (full width, ~44px tall):
   - Left group: document tab strip — 3 clickable tab pills with file names ("notes.md", "roadmap.md", "ideas.md") — active tab is white/raised, others muted; small × close button on each
   - Right group: SegmentedControl with `variant="sliding"` size="sm", options `[{value:'edit', label:'Edit'}, {value:'split', label:'Split'}, {value:'preview', label:'Preview'}]` bound to `editorMode`
   - Between groups: formatting toolbar (bold B, italic I, heading H, list •, ordered 1., code `, link 🔗 — each a small 26×26 icon button, `background: none`, hover surface tint)
   - The B and I buttons toggle `bold`/`italic` state with accent color when active

   **STK-2 — Document Tree** (~200px wide, ~400px tall):
   - Header: "Documents" (9px uppercase muted) + new-file button (+)
   - 3 tree sections: "Notes" (expanded), "Docs" (collapsed), "Archive" (collapsed)
   - Each section header: chevron + folder icon + label (click toggles `treeExpanded[key]`)
   - Notes (expanded): 4 file items indented — notes.md (active/accent border-left), roadmap.md, ideas.md, scratch.md
   - Docs/Archive: collapsed, no children visible
   - Each file row: 28px, doc icon (SVG) + filename (12px) + modified indicator (dot) for unsaved changes
   - Footer: "4 files · 12 KB" in 10px muted

   **STK-3 — Markdown Editor** (~400px wide, ~400px tall):
   - Simulated code editor area with line numbers
   - Left gutter: 32px wide, right-aligned line numbers in 10px monospace `var(--text-muted)`, lines 1–18
   - Right content: the `editorContent` split by `\n`, rendered as colored text lines:
     - Lines starting with `#`: large, `var(--text-primary)`, 15px bold
     - Lines starting with `##`/`###`: `var(--text-primary)`, 14px, bold
     - Lines starting with `*` or `-`: muted accent bullet
     - Empty lines: 8px tall spacer
     - Normal lines: 13px `var(--text-secondary)`
   - Cursor blink: a `|` character in accent color after the current line (CSS `animation: blink 1s step-end infinite`)
   - Scrollable vertically; background `var(--surface)`, monospace font
   - Do NOT use `<textarea>` — render the mock content as static divs

   **STK-4 — Preview Pane** (~360px wide, ~400px tall):
   - Rendered HTML preview of the same mock markdown content
   - Background `var(--surface)`, padding 20px
   - Style headings: h1 (20px, bold, primary, border-bottom), h2 (16px bold primary), h3 (14px bold primary)
   - Style paragraphs: 13px, `var(--text-secondary)`, line-height 1.6
   - Style `code`: inline monospace `var(--surface-raised)` background, `var(--accent)` color, border-radius 3px, padding 1px 4px
   - Show a "Rendered Preview" label in 9px uppercase muted at the very top, then the content below

   **STK-5 — Outline Panel** (~220px wide, ~280px tall):
   - Header: "Outline" (9px uppercase muted)
   - List of heading items extracted from the mock content:
     - H1: "Libre Apps — Architecture Notes" (indent 0)
     - H2: "Overview" (indent 1, `var(--text-secondary)`)
     - H2: "Components" (indent 1)
     - H3: "Button" (indent 2, `var(--text-muted)`)
     - H2: "Token System" (indent 1)
   - Active heading (H2 "Architecture") gets accent left border + primary text
   - Click updates `activeHeading` state
   - Each row: 26px, small h-level indicator ("H1"/"H2"/"H3" in 8px monospace muted box), heading text (12px), hover tint

   **STK-6 — Search Bar** (~320px wide, ~auto height — show both find and replace rows):
   - This is a find-and-replace panel, not a search input
   - Two rows stacked:
     - Row 1: magnifier icon + "Find" `<input>` bound to `searchQuery` + match count badge ("3 of 12" in 10px muted) + prev (↑) / next (↓) buttons + close button (×)
     - Row 2: replace icon + "Replace" `<input>` bound to `replaceQuery` + "Replace" button + "Replace All" button
   - Container: border 1px solid var(--border), border-radius 8px, background var(--surface-raised), padding 4px 8px
   - Label "Find & Replace" in 9px uppercase muted as a header above the two rows
   - Below: options checkboxes row — "Match case" + "Whole word" + "Regex" (use `<input type="checkbox" class="fade-check">` — this class is defined in tokens.css)

4. Write the `<style>` block with:
   - `.doc-tab` — inline-flex, align-items center, gap 4px, padding 3px 10px, font-size 12px, border-radius 6px 6px 0 0, cursor pointer
   - `.doc-tab.active` — background var(--surface), color var(--text-primary), border 1px solid var(--border), border-bottom-color transparent
   - `.doc-tab:not(.active)` — color var(--text-muted)
   - `.toolbar-btn` — 26px square, background none, border none, border-radius 4px, cursor pointer, display flex, align/justify center, color var(--text-muted)
   - `.toolbar-btn:hover` — background var(--surface-raised)
   - `.toolbar-btn.active` — color var(--accent)
   - `.tree-row` — flex, align-items center, gap 6px, padding 3px 8px, height 28px, cursor pointer, font-size 12px
   - `.tree-row:hover` — background hover tint
   - `.tree-row.active` — border-left 2px solid var(--accent), padding-left 6px, color var(--text-primary)
   - `.line-num` — monospace, font-size 10px, color var(--text-muted), text-align right, padding-right 8px, user-select none
   - `.outline-row` — flex, align-items center, gap 6px, padding 2px 8px, height 26px, cursor pointer, font-size 12px
   - `.outline-row.active` — border-left 2px solid var(--accent), color var(--text-primary)
   - `.h-badge` — 8px monospace, background var(--surface), border 1px solid var(--border), border-radius 2px, padding 1px 3px, color var(--text-muted), flex-shrink 0
   - `@keyframes blink` — from opacity 1, to opacity 0, step-end

5. Run `cd /Users/eldo/Downloads/Github/Libre-Apps/gallery && npm run dev` — verify all 6 cards render with real content.

6. Run `cd /Users/eldo/Downloads/Github/Libre-Apps && npm run lint` — fix any ESLint errors.

## Success signal
- `npm run dev` starts without errors
- Applications → Stack section: 6 cards, none with a dashed placeholder
- STK-1 toolbar B/I buttons toggle accent color on click
- STK-2 tree section chevrons expand/collapse on click
- STK-5 outline rows update `activeHeading` on click
- `npm run lint` exits 0

## Notes
- For STK-3, do NOT render a real `<textarea>` — it would be editable and break the gallery demo feel. Use static `<div>` rows simulating the editor. The mock is a visual demo, not a functional editor.
- The `fade-check` class is defined globally in `common-js/src/tokens.css` (already imported in the gallery). Use `<input type="checkbox" class="fade-check">` directly.
- For document tabs in STK-1, do NOT use the `<Tabs>` component — that component is for panel switching. Build the document tab strip manually with `.doc-tab` divs to get the exact file-tab aesthetic.
- The `SegmentedControl` for edit/split/preview: use `variant="sliding"` size="sm". Read `common-js/src/components/SegmentedControl.svelte` if you need to check the exact props API.
- `Tabs` component (used in STK if you want tabbed layout for the editor+preview in split mode) — see `gallery/src/sections/NavigationSection.svelte` lines 136–161 for the exact usage pattern with snippets. Only use it if it improves the demo; a flex row is fine too.
