# TASK-4: ShelfSection — file manager gallery mockup

## Goal
`gallery/src/sections/ShelfSection.svelte` has zero dashed placeholder boxes — every card contains a fully implemented, interactive Svelte 5 UI mockup that looks like a real file manager app.

## Context
Shelf is a file manager app in the Libre family (file browsing, tags, dual-pane, tabs, search). The gallery Applications tab has a ShelfSection with 7 cards (SH-1 through SH-7), each containing a `<div class="ph">` dashed placeholder. Replace every placeholder with real UI.

**Repo:** `/Users/eldo/Downloads/Github/Libre-Apps`  
**File to edit:** `gallery/src/sections/ShelfSection.svelte`  
**Run gallery:** `cd gallery && npm run dev` (Vite-only on :1422)

**Reference files to read before starting:**
- `gallery/src/lib/Card.svelte` — Card wrapper pattern
- `gallery/src/sections/NavigationSection.svelte` — Menu component usage pattern (for context menu)
- `common-js/src/tokens.css` — token reference

**Available @libre/ui components** (import from `'@libre/ui'`):
`Button`, `Input`, `Tag`, `Menu`

**Design system rules:**
- Svelte 5 runes only: `$state`, `$derived`, `$effect` — no `$:` or `createEventDispatcher`
- Tokens only — no hardcoded colors
- All `$state` for interactive demo state — no real IPC

## In scope
- `gallery/src/sections/ShelfSection.svelte` — full replacement

## Out of scope
- `common-js/src/` — do not edit
- Any other gallery section or App.svelte

## Steps

1. Import `Button`, `Input`, `Tag`, `Menu` from `'@libre/ui'`. Import `Card` from `'../lib/Card.svelte'`.

2. Declare `$state`:
   - `currentPath` — array of strings, default `['Home', 'Projects', 'Libre-Apps']`
   - `selectedFile` — string, default `'gallery'`
   - `col1Items` — array of file/folder objects for column 1 (the "Libre-Apps" folder contents)
   - `col2Items` — array for column 2 (gallery folder contents)
   - `col3Items` — array for column 3 (preview/empty)
   - `contextOpen` — boolean, default `false`
   - `contextAnchor` — null
   - `quickFiles` — array of `{name, type, date}` for recent/pinned items
   - `fileInfo` — object for the selected file's metadata

   Seed `col1Items` with: `['src-tauri', 'src', 'node_modules', 'package.json', 'README.md', 'vite.config.js']` — some folders, some files.
   Seed `col2Items` with contents of `src/` — a few `.svelte` files.
   Seed `quickFiles` with 5 recently accessed items across the family.

3. Implement each card:

   **SH-1 — Path Bar** (full width, ~36px tall):
   - Flex row: back arrow (‹) + forward arrow (›) + up arrow (↑) + separator + breadcrumb segments
   - Breadcrumb: each segment = clickable text button ("Home" › "Projects" › "Libre-Apps"), separated by `/` in `var(--text-muted)`, current segment bold/primary, previous segments `var(--text-secondary)`
   - Right side: compact search input (same `.search-compact` style as NavigationSection — search icon + placeholder "Search…")
   - Right side also: view mode toggle buttons (list / column / grid icons)

   **SH-2 — File Browser — Column View** (full width, ~320px tall):
   - Three column panes side by side, each with a vertical border separator
   - Column 1 (~220px): folder "Libre-Apps" — list of `col1Items`, each row 26px: file/folder icon (SVG, 14px) + name (13px) + optional size right-aligned (11px muted)
   - Column 2 (~220px): folder "gallery" — list of `col2Items`
   - Column 3 (~220px): preview placeholder — shows selected file name centered with a large file-type icon
   - Selected item in col1 has accent-tinted background row; selected in col2 likewise
   - Right-clicking a row (or clicking a "…" button on hover) opens the context menu

   **SH-3 — Quick Files** (~200px wide, ~300px tall):
   - Header: "Quick Files" label (9px uppercase muted) + pin icon button
   - Two sections: "Pinned" (2 items) + "Recent" (3 items), each with a small section divider label
   - Each item row: icon (14px) + filename (12px primary) + date (10px muted, right-aligned)
   - Hover: background `color-mix(in srgb, var(--surface) 94%, var(--text-primary))`

   **SH-4 — Preview Panel** (~280px wide, ~300px tall):
   - Header: "Preview" (9px uppercase muted)
   - Large file icon area (~140×100px centered): SVG folder or file icon in `var(--accent)` at 40% opacity, scaled to 64px
   - Filename below: "gallery" (14px primary, bold)
   - Type/size: "Folder · 24 items" (12px secondary)
   - "Open" button (full width, Button component, variant="primary")
   - "Show in Finder" link (12px accent, cursor pointer)

   **SH-5 — File Info** (~220px wide, ~300px tall):
   - Header: "Info" (9px uppercase muted) — same collapsible style as ir-sec-hd
   - Rows (label + value, label is 10px muted, value is 11px secondary):
     - Name: gallery
     - Kind: Folder
     - Size: 2.4 MB
     - Items: 24
     - Created: Mar 12, 2026
     - Modified: May 10, 2026
     - Permissions: drwxr-xr-x (monospace)
   - Tag section below separator: "Tags" header + 3 `<Tag>` components with labels "Work", "Active", "UI" in accent/muted variants

   **SH-6 — Tag Chip** (flex wrap, natural height):
   - Show 6 `<Tag>` components with different labels: "Work", "Personal", "Archive", "Urgent", "Media", "Design"
   - Mix: some with `variant="default"`, some with `variant="accent"` (or whatever variants the Tag component supports — read the component if needed: `common-js/src/components/Tag.svelte`)
   - Add a "+ New Tag" button (small, `var(--text-muted)` color, dashed border)
   - Show them wrapping naturally in a flex-wrap row

   **SH-7 — Context Menu** (~180px wide, ~220px tall):
   - Show the Menu component open/always-visible (not as a popup — render it inline using its items prop)
   - If Menu can't render inline, manually build a static context menu list with the same visual style
   - Items: Open, Open With ▸, Get Info, separator, Copy, Cut, Paste, separator, Move to Trash (destructive — red `var(--text-primary)` with warning tint or just normal style)
   - Note: read `common-js/src/components/Menu.svelte` to see if it supports `open={true}` static display; if not, mock the styles manually

4. Context menu interaction (SH-2): when user right-clicks or clicks "…" on a file row, set `contextAnchor` to the event target and `contextOpen = true`. Wire `<Menu bind:open={contextOpen} anchor={contextAnchor} items={contextMenuItems} />` to the section.

5. Write the `<style>` block with:
   - `.col-pane` — flex column, min-width 0, flex:1, border-right 1px solid var(--border), overflow-y auto
   - `.file-row` — display flex, align-items center, gap 6px, padding 3px 8px, height 28px, cursor pointer
   - `.file-row:hover` — background hover pattern
   - `.file-row.selected` — accent-tinted background
   - `.file-icon` — width 14px, height 14px, flex-shrink 0, color var(--text-muted)
   - `.breadcrumb-seg` — button, background none, border none, cursor pointer, font-size 12px, padding 2px 4px
   - `.info-row` — display flex, justify-content space-between, padding 3px 10px, font-size 11px

6. Run `cd /Users/eldo/Downloads/Github/Libre-Apps/gallery && npm run dev` — verify all 7 cards render with real content.

7. Run `cd /Users/eldo/Downloads/Github/Libre-Apps && npm run lint` — fix any ESLint errors.

## Success signal
- `npm run dev` starts without errors
- Applications → Shelf section: 7 cards, none with a dashed placeholder
- SH-2 file rows are clickable and update `selectedFile`
- SH-6 shows wrapped Tag components
- `npm run lint` exits 0

## Notes
- Read `common-js/src/components/Tag.svelte` briefly to see what props it accepts before using it. It likely takes `label` and optionally `variant` or a color prop.
- The Menu component's `anchor` prop expects a DOM element reference. For SH-7 static display, if Menu doesn't work inline, build an identical-looking manual list using the same CSS patterns visible in `common-js/src/components/Menu.svelte`.
- SVG icons: use simple geometric SVGs inline. For folder: a small rect with a tab notch on top-left. For file: rect with folded corner. For audio/video: appropriate simple symbols. Keep all strokes/fills using `currentColor` so they inherit from parent color.
