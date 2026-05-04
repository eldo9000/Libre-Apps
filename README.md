# Libre-Apps — Shared Code & Engineering Standards

This repo holds the **shared foundation** for the Libre family of apps (Shelf, Stack, Prism, Fade, Ghost, etc.). As of April 2026, each app lives in its own standalone repo; this repo now hosts the code and standards they all depend on.

## What lives here

- **`common/`** — `librewin-common`, the shared Rust crate (Tauri helpers, config, media, OS, window, xattr). Consumed by each app as a Cargo git dependency pinned to a SHA.
- **`common-js/`** — `@libre/ui`, the shared Svelte 5 component library and design tokens. Currently **vendored** into each app repo as a snapshot (npm's git-subdir story is poor). Treat this repo as the source of truth; sync into apps when it changes.
- **`docs/`** — shared engineering and signing specs that all apps inherit.
- **Standards** — lint config, `rust-toolchain.toml`, `eslint.config.mjs`, `justfile`, etc. that every app repo should mirror.


See [docs/specs/SIGNING.md](docs/specs/SIGNING.md) for the full signing infrastructure and key rotation policy.

## License

GPL-3.0 — see [LICENSE](LICENSE).
