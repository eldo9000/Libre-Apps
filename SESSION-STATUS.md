## Current Focus
Post-split stabilization of Libre-Apps as shared foundation. Apps have moved to separate repos (Shelf-App, Stack-App, Prism-App, Fade-App, Ghost-App) as of commit 532a620. This repo now hosts `librewin-common` (Rust), `@libre/ui` (Svelte), shared docs, and the cross-repo release pipeline.

## Last Session (2026-04-20)
- Refreshed CLAUDE.md and SESSION-STATUS.md to reflect post-split reality (prior versions still described `apps/` monorepo layout from pre-532a620).
- Created OBSERVER-STATE.md (Run 1).
- CI returned to green on commit 3baafbf after four successive red runs patching post-split fallout: ESLint globs, cargo fmt trailing commas, stylelint hex length, system-dep install in lint job.

## Next Action
Harden `release.yml` against the two fragilities flagged in commit 3baafbf's git note:
1. Graceful failure when `CARGO_DEPS_TOKEN` lacks read access to a downstream app repo (currently silent).
2. Replace the shell first-letter-uppercase app-name expansion with an explicit map — current form works for the five single-word names but will break on any multi-word app.

## Known Risks
- **Commit-note protocol barely instantiated:** only 1 of the last 40 commits (3baafbf) carries a structured note. Either adopt it consistently or retire it.
- **INVESTIGATION-LOG.md is empty:** either nothing notable has been investigated, or findings aren't being captured. Confirm which.
- **Downstream sync story for `@libre/ui` is manual:** `common-js/` changes require re-vendoring into each app repo, no automated sync script exists yet.

## Blocked
Nothing.

## Updated
2026-04-20
