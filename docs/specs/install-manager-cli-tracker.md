# Install Manager — CLI Update Tracker

## What it is

An advanced tab inside Install Manager for users who manage tools via the
command line. Lets them register check/update command pairs, track update
status across all their CLI ecosystems in one place, and optionally auto-run
updates on schedule.

Target user: scientists, developers — people already comfortable in the
terminal, annoyed at juggling `pip`, `conda`, `tlmgr`, `julia Pkg.update()`
separately with no unified view of what's stale.

---

## UI

### Advanced tab

- Collapsed by default. Once the user opens it, it stays open permanently.
- Badge on the tab (and on the Install Manager dock icon) reflects pending
  updates across all registered entries.

### Entry list

Horizontal rows, three columns each:

| Name | Check command | Update command |
|------|--------------|----------------|
| Python (pip) | `pip list --outdated` | `pip install --upgrade pip` |
| Julia packages | `julia -e 'using Pkg; Pkg.update()'` | *(same)* |
| TeX Live | `tlmgr update --list` | `tlmgr update --all` |

Each row also has:
- **Auto-update checkbox** — if checked, Update All runs it automatically.
  If unchecked, Update All skips it and it must be run manually. Useful for
  ecosystems that can break things (Julia, TeX Live) vs. safe ones (pip).
- **Run button** — runs the update command for that row immediately.
- **Last run timestamp** — shown inline, small, beneath the row.

### Controls

- **+ button** — adds a new empty row. User fills in the three columns.
- **Groups** — rows can be assigned to a named group (e.g. "Research",
  "System", "Work"). Groups collapse/expand. Badge per group shows how many
  entries are due. Drag rows into groups. Default group is "General."
- **Update All button** — runs update commands for all auto-update rows.
  Prompts before running any unchecked rows.

---

## Badge logic

On a configurable schedule (default: daily), Install Manager runs each
registered check command. If the command produces any output, that entry is
counted as "has updates." Badge number = count of entries with output.

No output parsing. If it prints anything, something's available. This is
intentional — matches how CLI users already read these commands.

---

## Auto-update behavior

Rows with auto-update checked run their update command on schedule without
prompting. Rows without it only run when the user clicks Run or Update All.

Suggested defaults:
- Auto-update off by default on all new entries — user opts in per row.

---

## v1 scope

- Add/remove/edit rows
- Three columns: name, check command, update command
- Auto-update checkbox per row
- Groups with names
- Update All button
- Badge on tab + dock icon
- Last run timestamp per row
- Daily check schedule (fixed, not configurable in v1)

Out of scope for v1: dependency ordering between rows, output log viewer,
notification per-entry, custom schedule per row.
