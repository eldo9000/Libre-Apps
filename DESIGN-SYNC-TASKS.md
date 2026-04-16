# Design Sync Tasks — Libre-Apps

## Context

The shared design system for all Libre apps lives in `common-js/src/tokens.css` and is
published as the `@libre/ui` package. Flicker-App and CaseBook-App both import from this
source (or mirror it). This repo is the **canonical source** — changes here ripple to
all other products.

**Two problems exist right now:**

1. The accent blue in `tokens.css` is wrong.
2. None of the five apps (Shelf, Stack, Prism, Fade, Ghost) have frontend source yet.

---

## Canonical Design Values

These are the correct values, confirmed across Flicker-App and CaseBook-App docs:

### Accent Blue
```
--accent:       #004e9c   ← the Libre highlight blue (sourced from Fade-App)
--accent-hover: #004288
```

### Light Mode Tokens (`:root`)
```css
--accent:          #004e9c;
--accent-hover:    #004288;
--titlebar-bg:     #f5f5f5;
--surface:         #ffffff;
--surface-raised:  #f8f8f8;
--surface-panel:   #fafafa;
--border:          #e0e0e0;
--border-subtle:   #eeeeee;
--text-primary:    #1a1a1a;
--text-secondary:  #666666;
--text-muted:      #999999;
--tab-active-bg:   #ffffff;
--tab-bar-bg:      #f0f0f0;
```

### Dark Mode Tokens (`html.dark`)
```css
--titlebar-bg:     #1a1a1a;
--surface:         #111111;
--surface-raised:  #1a1a1a;
--surface-panel:   #1e1e1e;
--border:          #2a2a2a;
--border-subtle:   #222222;
--text-primary:    #f0f0f0;
--text-secondary:  #888888;
--text-muted:      #555555;
--tab-active-bg:   #111111;
--tab-bar-bg:      #0d0d0d;
```

### Typography
- **Font:** Geist (via `@fontsource/geist/latin.css`)
- **Base size:** 14px
- **Line height:** 1.5
- **Smoothing:** `-webkit-font-smoothing: antialiased`

### Button Rounding
- **Buttons:** `rounded-md` (Tailwind) = 6px
- **Dialogs/modals:** `rounded-lg` = 8px
- **Window chrome:** `border-radius: 10px` (set in tokens.css `#app > div:first-child`)
- **Focus rings:** `border-radius: 2px`
- **Scrollbar thumb:** `border-radius: 3px`

---

## Tasks

### Task 1 — Fix accent color in tokens.css

**File:** `common-js/src/tokens.css`

The canonical blue is already correct in `tokens.css` (`#004e9c`). No change needed here — this was the right value all along, sourced from Fade-App.

Verify the values match exactly:
```css
--accent:       #004e9c;
--accent-hover: #004288;
```

Both `:root` and `html.dark` blocks should have these values.

---

### Task 2 — Verify Button component uses correct rounded-md

**File:** `common-js/src/components/Button.svelte`

Confirm buttons use `rounded-md` class. This should already be correct but verify after
the accent color fix doesn't introduce any regressions.

---

### Task 3 — Commit and push

After Tasks 1–4:
1. Commit the tokens.css fix with message: `fix(tokens): correct accent blue to #0066cc`
2. Commit scaffolded app sources separately for each app
3. Push to main

---

## Notes for the Agent

- The `common-js/` directory is the **upstream source** for all Libre apps. Any token
  change here needs to be pulled downstream by Flicker-App and CaseBook-App.
- The scaffold script at `common-js/scripts/create-libre-app.js` handles the full
  Tauri + Svelte 5 + Tailwind 4 + Geist + @libre/ui wiring.
- After fixing the accent, ping the Flicker-App and CaseBook-App repos to pull the
  updated package (or manually sync tokens if they maintain their own copies).
- Dark mode is class-based: `html.dark`. The `initTheme(invoke)` function from
  `common-js/src/theme.js` handles applying this class at runtime via Tauri backend.
