# Investigation Log — The Freedom Apps

Append-only. One line per finding.

Format: `YYYY-MM-DD | CONFIRMED/RULED OUT/OPEN | finding`

**Rules:**
- Write an `OPEN` entry *before* testing a hypothesis, not after.
- Update to `CONFIRMED` or `RULED OUT` the moment testing returns a result.
- Never batch entries until session end — if interrupted, the trail must be readable.
- Findings only. Git has the actions and fixes.

**When to archive:** When the problem domain fundamentally changes, move this file to
`docs/investigations/INVESTIGATION-LOG-<slug>.md` and start a fresh empty log.
Slug should describe the problem (e.g. `shelf-xattr-macos-sandboxing`), not the date.

---

<!-- Append new entries below this line, newest at the bottom -->
