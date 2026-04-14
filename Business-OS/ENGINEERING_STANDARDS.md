# LibreWin Freedom Apps — Engineering Standards

**Read this before building anything in this repo.** Shared conventions, Tauri 2 patterns, Svelte 5 patterns, design tokens, cross-platform gotchas, and known bug fixes. If you discover a new pattern or fix a non-obvious bug, document it here so all five apps benefit.

---

## Design Tokens

All apps share these CSS variables, defined in each app's `src/app.css`. The canonical values are:

```css
:root {
  --accent:          #0066cc;   /* user-configurable via ~/.config/librewin/accent */
  --accent-hover:    #1e5fa6;
  --titlebar-bg:     #f7f7f7;
  --surface:         #ffffff;
  --surface-raised:  #f5f5f5;
  --border:          #e5e7eb;
  --text-primary:    #111827;
  --text-secondary:  #6b7280;
}

html.dark {
  --accent:          #0066cc;   /* accent stays the same in dark mode */
  --titlebar-bg:     #1f2937;
  --surface:         #111827;
  --surface-raised:  #1f2937;
  --border:          #374151;
  --text-primary:    #f9fafb;
  --text-secondary:  #9ca3af;
}
```

**Note:** The old erroneous default was `#297acc`. All CSS and Rust defaults must use `#0066cc`. If you see `#297acc` anywhere, it is a bug.

---

## Window Chrome Recipe

Every app uses a frameless window with a custom titlebar, top resize strip, and Windows-style controls.

### tauri.conf.json window settings
```json
{
  "decorations": false,
  "transparent": true,
  "resizable": true
}
```

### app.css — rounded corners and shadow
```css
/* transparent: true makes the window surface transparent */
html, body { background: transparent; }

#app {
  padding: 8px;        /* space for drop shadow */
  box-sizing: border-box;
  overflow: visible;
  background: transparent;
}

#app > div:first-child {
  border-radius: 10px !important;
  overflow: hidden;
  box-shadow: 0 1px 8px rgba(0, 0, 0, 0.24);
}
```

### App.svelte — titlebar + resize strip
```svelte
<!-- Top resize strip (4px) — must be above the drag region in z-order -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="absolute top-[8px] left-0 right-0 h-[4px] z-50 cursor-n-resize"
  onmousedown={(e) => { e.preventDefault(); appWindow.startResizeDragging('North'); }}
></div>

<!-- Custom titlebar (draggable) -->
<div
  data-tauri-drag-region
  class="h-8 bg-[var(--titlebar-bg)] border-b border-gray-200 dark:border-gray-700
         flex items-center shrink-0 select-none"
>
  <!-- Left: icon + title -->
  <div class="flex items-center gap-2 flex-1 min-w-0 pl-3" data-tauri-drag-region>
    <!-- app icon SVG here -->
    <span class="text-[13px] font-medium text-gray-700 dark:text-gray-300 truncate">AppName</span>
  </div>

  <!-- Right: Windows-style controls — minimize, maximize, close -->
  <div class="flex items-center shrink-0 h-full">
    <button
      onclick={() => appWindow.minimize()}
      class="w-11 h-full flex items-center justify-center text-gray-500 dark:text-gray-400
             hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors text-[16px] leading-none"
      title="Minimize" aria-label="Minimize"
    >─</button>
    <button
      onclick={() => appWindow.toggleMaximize()}
      class="w-11 h-full flex items-center justify-center text-gray-500 dark:text-gray-400
             hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors text-[13px]"
      title="Maximize" aria-label="Maximize"
    >□</button>
    <button
      onclick={() => appWindow.close()}
      class="w-11 h-full flex items-center justify-center text-gray-500 dark:text-gray-400
             hover:bg-red-500 hover:text-white transition-colors text-[18px]"
      title="Close" aria-label="Close"
    >×</button>
  </div>
</div>
```

**Why the resize strip?** `data-tauri-drag-region` on the titlebar captures `mousedown` before the OS resize handler fires at the top edge, so dragging the top edge moves the window instead of resizing it. The 4px strip sits above the drag region in z-order and calls `startResizeDragging` explicitly.

---

## Theme and Accent Init Pattern

All apps read theme and accent from shared config files on mount. The shared function lives in `apps/common-js/theme.js`:

```js
import { initTheme } from '../../common-js/theme.js';
import { onMount } from 'svelte';

// In script block:
onMount(() => initTheme(invoke));
```

`initTheme(invoke)` returns a cleanup function that removes the media query listener. `onMount` will call it on component destroy.

For apps that need additional onMount logic (event listeners, etc.):
```js
onMount(async () => {
  const themeCleanup = await initTheme(invoke);
  // ... additional setup ...
  return () => {
    themeCleanup?.();
    // ... additional cleanup ...
  };
});
```

**Stack is an exception:** it uses an `isDark` reactive state variable for CodeMirror theme sync and cannot use `initTheme` directly. It implements the same logic inline.

---

## Svelte 5 Runes — No Legacy Syntax

All `.svelte` files must use Svelte 5 runes. No legacy `$:` reactive statements, no `export let` for props.

```svelte
<!-- Props -->
let { value = $bindable(false), label } = $props();

<!-- State -->
let count = $state(0);
let doubled = $derived(count * 2);

<!-- Effects with cleanup -->
$effect(() => {
  const unlisten = listen('event', handler);
  return () => { unlisten.then(f => f()); };
});
```

---

## Shared Rust Utilities

`get_theme()` and `get_accent()` live in `apps/common/` (`librewin-common` crate). Every app wraps them as Tauri commands:

```rust
#[tauri::command]
fn get_theme() -> String { librewin_common::get_theme() }

#[tauri::command]
fn get_accent() -> String { librewin_common::get_accent() }
```

---

## Tauri 2 Command Pattern

```rust
#[tauri::command]
fn my_command(state: State<'_, AppState>) -> Result<String, String> {
    // Never hold Mutex lock when spawning threads.
    // Extract data first, drop lock, then spawn.
    let data = state.inner.lock().unwrap().clone();
    Ok(data)
}
```

Capabilities live in `src-tauri/capabilities/default.json`. The `identifier` field must match the app name (not a generic "Simple Web" placeholder).

---

## WebKit dmabuf Workaround

All `.desktop` files must launch via the env wrapper:

```ini
Exec=env WEBKIT_DISABLE_DMABUF_RENDERER=1 {binary} %U
```

Without this, WebKitGTK may crash or render incorrectly on some Linux GPU drivers. Apply to every app.

---

## Tauri 2 Gotchas

- **`windows_subsystem = "windows"`** — required in `main.rs` on Windows to suppress the console window. Already present in the template; don't remove it.
- **Path joining** — always use `Path::join()`, never hardcode `/` separators. Windows builds will break.
- **Mutex + threads** — never hold a `MutexGuard` across an `.await` or `spawn`. Extract the data first, drop the guard, then spawn.
- **`transparent: true`** — requires the window compositor on Linux (X11 without compositing shows opaque corners; this is an accepted known limitation).
- **Capability schema path** — Tauri 2 looks for capabilities in `src-tauri/capabilities/`. If you move or rename this directory, the build will fail silently.

---

## Cross-Platform Notes

| Platform | Renderer | Notes |
|----------|----------|-------|
| Linux (primary) | WebKit2GTK 4.1 | Declare system deps in `tauri.conf.json` bundle section. FFmpeg/ImageMagick must be in PATH. |
| macOS | WebKit (WKWebView) | Test production `.app` bundles, not just `tauri dev`. FFmpeg needs explicit PATH config. |
| Windows | WebView2 (Chromium) | Use `Path::join()` everywhere. `windows_subsystem = "windows"` in `main.rs`. |

---

## Workspace Dependencies

Shared Rust deps are declared once in `apps/Cargo.toml` under `[workspace.dependencies]`:

```toml
[workspace.dependencies]
tauri        = { version = "2", features = [] }
serde        = { version = "1", features = ["derive"] }
serde_json   = "1"
librewin-common = { path = "common" }
```

Each app's `Cargo.toml` references them with `{ workspace = true }` — no version duplication.

---

## Dev Server Ports

| App | Port |
|-----|------|
| Shelf | 1420 |
| Ghost | 1421 |
| Stack | 1422 |
| Prism | 1423 |
| Fade | 1427 |
