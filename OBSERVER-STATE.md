# Libre-Apps Observer State
Last updated: 2026-04-20  ·  Run 1

---

## Active development areas
The repo just completed a structural inversion. As of commit 532a620, `apps/` is gone — the five Freedom Apps (Shelf, Stack, Prism, Fade, Ghost) have been split into separate repos, and Libre-Apps is now the shared foundation: `librewin-common` (Rust), `common-js`, `@libre/ui`, workspace lints, design tokens, and the release/CI pipelines that orchestrate the app repos. The last week of work is entirely post-split CI fallout: ESLint glob paths, cargo fmt trailing commas, stylelint hex-length, and system-dep installation in the lint job. Head commit is green.

## Fragile / high-risk areas
`release.yml` assumes `CARGO_DEPS_TOKEN` grants read access to every downstream app repo; no fallback if a token rotates or a repo is renamed. The same workflow uses shell uppercase-first-letter expansion on app names — works for the current 5 single-word names but would silently mis-capitalize any multi-word addition. These are the only explicit `fragile:` markers on record (commit 3baafbf note). The lint job's transitive dependency on system packages for clippy was just discovered and patched — similar latent coupling likely exists in other jobs that haven't been exercised since the split.

## Deferred work accumulation
Nothing notable. The single git-note commit declares `deferred: none`. No TODO backlog has been captured in notes yet.

## Pattern watch
None of the CLAUDE.md Known Patterns (accent `#0066cc`, Mutex-across-spawn, Svelte 5 runes, titlebar four-in-lockstep, theme sync via `librewin_common`, WebKit2GTK deps, FFmpeg PATH, `$effect` cleanup return) have surfaced in recent commits — consistent with the repo no longer hosting app code. Commit e4f27e2 ("correct tokens.css to canonical values") is adjacent to the `#0066cc` pattern but landed pre-split.

## CI health
Green on HEAD (3baafbf, 2m56s). Immediately preceded by four consecutive red runs, all tied to the split. Pattern is successive-fix rather than flake — each failure revealed a distinct post-split regression. No evidence of intermittent failures.

## Observer notes
Only one commit (3baafbf) carries a git note; the commit-note protocol is effectively un-instantiated across the history. INVESTIGATION-LOG.md exists but is empty — no findings have been recorded. SESSION-STATUS.md is stale: last updated 2026-04-12, still describes Group H testing work and M1 app features as if `apps/` were present; it has not been revised to reflect the split eight days later. CLAUDE.md also still documents the `apps/{shelf,stack,prism,fade,ghost}/` layout as current truth — the codebase description drifted from reality at 532a620 and has not caught up. OBSERVER-STATE.md did not previously exist; this is Run 1 with no continuity to compare against.
