# Shelf

Shelf is LibreWin's built-in file manager. It's not trying to be Dolphin or Nautilus. It's not trying to replace your power tools. It's the thing that opens when you double-click a folder — fast, clean, and out of the way the moment you've found what you're looking for.

Built with Tauri 2 (Rust backend, Svelte 5 frontend), it talks directly to the filesystem with no middleware overhead. Open it, navigate, done.

---

## What makes it good

**It opens instantly.** No service daemon warming up, no plugin system scanning your drives. The window appears, the folder is already listed.

**The sidebar actually makes sense.** You get two things on the left: Shelf Access (Home, Desktop, Downloads, Documents, Pictures, Music, Videos) and a lazy-loaded Folders tree that only fetches children when you expand a branch. No waiting for a full directory tree to build on startup.

**It knows where you've been.** Full navigation history with back/forward, a live breadcrumb trail you can click anywhere along, and a search filter that narrows the current folder in real time as you type.

**Volumes just appear.** Plug in a drive — it shows up in the sidebar under Volumes. Eject it — it disappears. No configuration, no "scan for devices" button.

**It handles the heavy lifting from Rust.** Directory listing, file opening, home directory resolution, and volume enumeration all run in native Rust on a Tauri async thread. The UI never blocks.

---

## How it works day-to-day

Double-clicking any folder anywhere on the desktop opens Shelf directly — it's registered as the default handler for `inode/directory`. You won't accidentally end up in Dolphin.

The toolbar is minimal on purpose: back, forward, a reload button, and the search box. The breadcrumb at the top lets you jump to any parent folder in one click. The address bar is the breadcrumb — there's no separate place to type a path (yet).

Files open with their default system application. Images go to the viewer, videos to the player, PDFs to the reader. Shelf doesn't try to preview anything itself.

---

## Keyboard shortcuts

| Key | Action |
|-----|--------|
| `Alt ←` / `Alt →` | Back / Forward |
| `F5` | Reload current folder |
| Start typing | Filter files in current folder |
| `Escape` | Clear search |

---

## What's coming

- **Tags** — xattr-backed, write a tag on any file, filter by it from anywhere
- **Tabs** — open a second location without opening a second window
- **Dual-pane mode** — for the times you're moving things between two places
- **Shelf mode** — a global keyboard shortcut that summons a compact floating version, navigate to a file, hit Enter, it closes
- **System file picker** — register with xdg-desktop-portal so Save/Open dialogs in every app use Shelf instead of the GTK picker

---

## Under the hood

| Layer | Tech |
|-------|------|
| Runtime | Tauri 2 (Rust) |
| UI | Svelte 5 + Tailwind 4 |
| Renderer | WebKit2GTK 4.1 |
| Filesystem | Rust `std::fs` + `serde_json` |
| Volume detection | `/proc/mounts` parsing |
| Distribution | Compiled binary at `/usr/local/bin/shelf`, registered via `.desktop` entry |

The binary is ~3MB on aarch64 and x86_64. There's no Electron, no Node runtime, no bundled Chromium.
