# Libre-Apps — Shared Code & Engineering Standards

This repo holds the **shared foundation** for the Libre family of apps (Shelf, Stack, Prism, Fade, Ghost, etc.). As of April 2026, each app lives in its own standalone repo; this repo now hosts the code and standards they all depend on.

## What lives here

- **`common/`** — `librewin-common`, the shared Rust crate (Tauri helpers, config, media, OS, window, xattr). Consumed by each app as a Cargo git dependency pinned to a SHA.
- **`common-js/`** — `@libre/ui`, the shared Svelte 5 component library and design tokens. Currently **vendored** into each app repo as a snapshot (npm's git-subdir story is poor). Treat this repo as the source of truth; sync into apps when it changes.
- **`docs/`** — shared engineering and signing specs that all apps inherit.
- **Standards** — lint config, `rust-toolchain.toml`, `eslint.config.mjs`, `justfile`, etc. that every app repo should mirror.

## The apps

Each app is now its own private repo under `eldo9000`:

| App | Repo | Description |
|-----|------|-------------|
| Shelf | [Shelf-App](https://github.com/eldo9000/Shelf-App) | File manager with xattr tags, dual-pane, keyboard-first |
| Stack | [Stack-App](https://github.com/eldo9000/Stack-App) | Markdown editor — the modern Notepad |
| Prism | [Prism-App](https://github.com/eldo9000/Prism-App) | Universal media viewer (images, video, audio, PDFs, 3D) |
| Fade  | [Fade-App](https://github.com/eldo9000/Fade-App)   | Media converter powered by FFmpeg |
| Ghost | [Ghost-App](https://github.com/eldo9000/Ghost-App) | Privacy browser — stateless, fingerprint-randomized |

Why split? Each app is a standalone product with its own users, releases, and marketing surface. Bundling them in a monorepo buried each app's discovery; now each has its own README, issues, stars, and release stream.

## Using the shared crate

In an app's `src-tauri/Cargo.toml`:

```toml
[dependencies]
librewin-common = { git = "https://github.com/eldo9000/Libre-Apps.git", rev = "<pinned-sha>" }
```

Bump the `rev` deliberately when you want to pull updates.

## Using the shared UI package

Each app repo vendors `common-js/` at the root and references it via `"@libre/ui": "file:./common-js"` in `package.json`. When this source changes, re-sync into each app repo (a helper script lives in `scripts/` — TODO).

## Tech stack

Tauri 2 · Rust 2021 · Svelte 5 · Tailwind 4 · Vite 8 · Geist font.

## Verifying releases

All release binaries and `.desktop` files are signed with [minisign](https://jedisct1.github.io/minisign/).

**Production public key** (baked into LibreWin OS):
```
RWSvBX8a5MExBbOsCmBqDLxNBb8ofBef1k3eqI79Z/LSGp/DBj1YwW5S
```

See [docs/specs/SIGNING.md](docs/specs/SIGNING.md) for the full signing infrastructure and key rotation policy.

## License

GPL-3.0 — see [LICENSE](LICENSE).
