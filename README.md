# The Freedom Apps

A minimal set of reliable, lightweight apps for [LibreWin OS](https://github.com/AncientKing37/Librewin-OS). These aren't trying to compete with the software you'll install later — they're a dependable foundation that works offline, on any hardware, from day one, with a wide range of compatibility.

Built with **Tauri 2 + Rust + Svelte 5**. No Electron. No Node runtime. ~3MB per binary.

---

### Shelf — File Manager
Your filesystem, your way. Tags backed by extended attributes, dual-pane layout, multi-tab, keyboard-driven navigation, instant search. Global Super+E shortcut summons it from anywhere. Archive browsing/extraction (ZIP, 7Z, TAR) coming in v0.2. Quick media conversion via Fade presets.

### Stack — Markdown Editor
The note-taking app that gets out of your way. Opens instantly, saves automatically, speaks Markdown natively. CodeMirror 6 editor with live outline, multi-tab, export to HTML/TXT/PDF. The modern Notepad you always wanted.

### Prism — Media Viewer
Open anything. Images, video, audio, PDFs, 3D models, and hundreds of other formats — all in one app, no plugins, no codecs to hunt down. Think macOS Quick Look, but for everything.

### Fade — Media Converter
Convert, resize, and process media without leaving your desktop. Convert a 200MB video to MP4. Batch-resize 50 photos. Extract audio. Right-click in Shelf and pick a preset — it converts in the background while you work. Supports everything FFmpeg does, which is everything.

### Ghost — The browser that leaves no imprint
Every session starts as a new, unrecognizable browser. Randomized fingerprint, spoofed identity, WebRTC disabled — websites don't know who you are and can't connect one visit to another. Not incognito (that just clears local history). Ghost changes what the web sees about you. Stateless by default, always there, starts instantly.

---

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Framework | Tauri 2 |
| Frontend | Svelte 5 + Tailwind CSS 4 |
| Backend | Rust (edition 2021) |
| Build | Vite 8 |
| Font | Geist |

## Development

```bash
cd apps/{shelf,stack,prism,fade,ghost}
npm install
npm run tauri dev
```

Requires: Rust 1.77+, Node 22+.

## License

GPL-3.0 — see [LICENSE](LICENSE).
