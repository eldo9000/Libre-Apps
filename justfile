# Libre Apps — top-level build orchestration
# Usage: just <recipe>  (requires https://just.systems)

# ── Build ─────────────────────────────────────────────────────────────────

# Build all five Tauri apps in release mode
build-all:
    @echo "→ shelf"
    cd apps/shelf && npm ci && npm run tauri build
    @echo "→ stack"
    cd apps/stack && npm ci && npm run tauri build
    @echo "→ prism"
    cd apps/prism && npm ci && npm run tauri build
    @echo "→ fade"
    cd apps/fade && npm ci && npm run tauri build
    @echo "→ ghost"
    cd apps/ghost && npm ci && npm run tauri build
    @echo "✓ All apps built."

# Run a single app in dev mode  e.g.: just dev shelf
dev APP:
    cd apps/{{APP}} && npm run tauri dev

# ── Linting ───────────────────────────────────────────────────────────────

# Run all four linters (eslint, stylelint, cargo fmt --check, cargo clippy)
lint:
    npm run lint
    npm run lint:css
    cd apps && cargo fmt --check --all
    cd apps && cargo clippy --workspace --all-targets -- -D warnings

# ── Formatting ────────────────────────────────────────────────────────────

# Auto-format JS/Svelte (prettier) and Rust (cargo fmt)
fmt:
    npm run fmt
    cd apps && cargo fmt --all

# ── Testing ───────────────────────────────────────────────────────────────

# Run tests (placeholder — per-app vitest suites, no integration tests yet)
test:
    @echo "Running per-app test suites…"
    cd apps/shelf && npm test
    cd apps/stack && npm test
    cd apps/prism && npm test
    cd apps/fade  && npm test
    cd apps/ghost && npm test
    cd apps && cargo test --workspace

# ── Cleanup ───────────────────────────────────────────────────────────────

# Remove all build artifacts (Vite dist dirs + Rust workspace target)
clean:
    rm -rf apps/*/dist
    rm -rf apps/target
    @echo "✓ Cleaned."
