# Libre-Apps — Porting Guide for App Agents

> **Read this first.** You are an agent in an app repo. You've been directed here to grab UI components or reference the design system. This document tells you exactly what to take and how to use it correctly.

---

## What this repo provides

Two things your app consumes:

- **`common-js/`** — the `@libre/ui` Svelte component library (components, design tokens, API helpers)
- **`gallery/src/lib/panels/`** — prototype settings panels from real Libre apps, usable as visual specs

---

## Step 1 — Vendor `@libre/ui` into your app

`@libre/ui` is consumed as a vendored snapshot, not a published npm package.

1. Copy the entire `common-js/` directory to your app repo root.
2. In your app's `package.json` dependencies:
   ```json
   "@libre/ui": "file:./common-js"
   ```
3. In your app's `src/app.css` (before any local overrides):
   ```css
   @import "@libre/ui/src/tokens.css";
   ```
4. In your root `App.svelte`, call `initTheme` on mount:
   ```svelte
   <script>
     import { onMount } from 'svelte';
     import { invoke } from '@tauri-apps/api/core';
     import { initTheme } from '@libre/ui';

     onMount(() => initTheme(invoke));
   </script>
   ```

---

## Step 2 — Import components

All components export from `@libre/ui`:

```svelte
<script>
  import { Select, TrafficLight, SegmentedControl, Button, Checkbox, ScrollArea } from '@libre/ui';
</script>
```

Full API reference: [`common-js/COMPONENTS.md`](common-js/COMPONENTS.md)

---

## Step 3 — Reading a gallery panel (prototype → real app)

The panels in `gallery/src/lib/panels/` are **visual prototypes, not copy-paste sources.** They contain gallery-specific machinery that must not go into a real app. Use them as a design spec; implement clean against `@libre/ui`.

### Keep — these are already correct design system usage

| Pattern | Source |
|---|---|
| `<Select>`, `<TrafficLight>`, `<Button>`, etc. | `@libre/ui` components — import and use directly |
| `.subsection-hd`, `.subsection-hd-title` | `tokens.css` — available after the `@import` |
| `.section-ruled`, `.cb-native`, `.seg-active`, `.seg-inactive`, `.sel-trigger` | `tokens.css` — same |
| `var(--surface)`, `var(--border)`, `var(--text-*)`, `var(--accent)`, etc. | Design tokens |

### Replace — gallery panels use ad-hoc versions of real components

| Panel pattern | Use instead |
|---|---|
| `<button class="tt-update-btn">` | `<Button variant="secondary">` |

### Copy the CSS — intentional custom patterns with no component equivalent

| Pattern | Why it's custom |
|---|---|
| `.tt-seg-*` segmented buttons | More compact density than `<SegmentedControl>` — deliberate for narrow panel context |
| `.tt-multi-btn` / `.tt-multi-on` | Independent toggle buttons — no MultiToggle component exists in `@libre/ui` |
| `<input type="range" class="tt-range">` | Volume/range slider — no Slider component in `@libre/ui`; copy the CSS |

### Strip — gallery-only, remove entirely

| Pattern | Why it exists | Action |
|---|---|---|
| `data-card="..."` attributes | Powers the gallery's CSS token inspector tooltip | **Remove completely** |

---

## Design tokens — canonical values

Source of truth: [`common-js/src/tokens.css`](common-js/src/tokens.css)

| Token | Light | Dark |
|---|---|---|
| `--accent` | `#2884c9` | `#2884c9` |
| `--accent-hover` | `#2373b0` | `#2373b0` |
| `--surface` | `#fff` | `#111` |
| `--surface-raised` | `#f8f8f8` | `#1a1a1a` |
| `--surface-panel` | `#fafafa` | `#1e1e1e` |
| `--border` | `#e0e0e0` | `#2a2a2a` |
| `--border-subtle` | `#eee` | `#222` |
| `--text-primary` | `#1a1a1a` | `#e6e6e6` |
| `--text-secondary` | `#666` | `#a6a6a6` |
| `--text-muted` | `#999` | `#555` |
| `--titlebar-bg` | `#f5f5f5` | `#1a1a1a` |

**Never hardcode these values.** Always use `var(--token-name)`. The accent is user-configurable at runtime via `~/.config/librewin/accent` — hardcoding it defeats the theming system.

---

## Svelte 5 rules (mandatory)

- Only `$state`, `$derived`, `$effect`, `$props`, `$bindable`
- Never `$:`, `createEventDispatcher`, or `export let`
- The compiler silently misbehaves on mixed syntax — no exceptions

---

## Checklist before shipping

- [ ] `common-js/` vendored at repo root, `npm install` resolves `@libre/ui` correctly
- [ ] `@import "@libre/ui/src/tokens.css"` present in `app.css`
- [ ] `initTheme(invoke)` called on mount in the root component
- [ ] No `data-card="..."` attributes anywhere
- [ ] No hardcoded hex values — all colors via `var(--token-name)`
- [ ] All reactive state uses Svelte 5 runes only
