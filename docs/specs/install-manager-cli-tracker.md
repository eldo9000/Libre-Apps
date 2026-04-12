# Install Manager — Advanced Section Spec

## Purpose

Install Manager's job is surfacing everything on the system that was installed,
is running, or needs attention — whether the user explicitly chose it or not.
The pitch: you don't have to go dig around in the terminal to remember what's
there. It's all in one place.

This is distinct from Settings, which is for configuration. Install Manager is
for things that exist (or don't yet exist) on the system.

---

## Layout

Two zones, no accordion mechanic:

**Top zone — no label.** Regular apps. The normal install/uninstall list.
Nothing special, just clean.

**Bottom zone — "Advanced" divider.** A visual spacer with the label
"Advanced" that separates it from the top. Permanently open — never collapses.
This zone is often the busiest thing on screen for power users, so folding it
would add friction for no benefit.

The fold-and-stay mechanic belongs in Settings (many categories, most users
touch one or two). Install Manager has one advanced zone — just show it.

---

## Advanced zone contents

### 1. CLI Update Tracker

For users managing tools via the command line. Register check/update command
pairs, track update status across all CLI ecosystems in one place.

**Entry list — horizontal rows, three columns:**

| Name | Check command | Update command |
|------|--------------|----------------|
| Python (pip) | `pip list --outdated` | `pip install --upgrade pip` |
| Julia packages | `julia -e 'using Pkg; Pkg.update()'` | *(same)* |
| TeX Live | `tlmgr update --list` | `tlmgr update --all` |

Each row also has:
- **Auto-update checkbox** — if checked, Update All runs it automatically.
  Per-row control matters: Julia and TeX Live can break things; pip is safe.
- **Run button** — runs the update command for that row immediately.
- **Last run timestamp** — inline, small.

Controls:
- **+ button** — adds a new empty row.
- **Groups** — named (e.g. "Research", "Work"). Badge per group. Drag to organize.
- **Update All** — runs auto-update rows automatically, prompts for the rest.

Badge logic: run each check command on schedule (daily default). If it prints
anything, count it as "has updates." No output parsing — matches how CLI users
already read these commands.

---

### 2. Flatpak Runtimes

Flatpak apps silently pull in large runtimes (GNOME Runtime, KDE Frameworks,
etc.) that accumulate invisibly. Users have no idea they're there.

Show: runtime name, version, size, which apps depend on it.
Action: remove unused runtimes with one click.

---

### 3. Firmware

`fwupdmgr` manages firmware for drives, USB controllers, Thunderbolt chips.
Completely invisible unless you know the command. Security and stability matter.

Show: device name, current firmware version, available update (if any).
Action: update per-device or update all.

---

### 4. Background Services

Friendly view of systemd services. Things installed silently by packages and
left running — not malware, just forgotten.

Show: service name, status (running / stopped), enabled at boot (yes/no).
Action: start/stop, enable/disable. No terminal commands needed.

---

### 5. Startup Items

What launches at login. Windows users expect this to exist somewhere — on Linux
it's buried. Name, source, enable/disable toggle.

---

### 6. Old Kernels

Fedora keeps 3 kernels by default. Users don't know. Disk space accumulates.

Show: list of installed kernels, which is active, size of each.
Action: "Remove old kernels" — keeps current + one previous, removes the rest.

---

### 7. Disk Usage by Source

Break down disk usage by category so users can make informed cleanup decisions:
- Flatpak apps + runtimes
- DNF package cache
- Old kernel versions
- Install Manager CLI tracker cache

One-click cleanup per category.

---

## Badge logic (unified)

The Install Manager dock badge and Advanced section header badge reflects
the total count of pending items across all advanced subsections:
- CLI tracker entries with available updates
- Flatpak runtimes with updates
- Firmware updates available
- (Startup items and services don't generate badges — status, not updates)

---

## v1 scope

- CLI Update Tracker: full (rows, groups, auto-update, badge)
- Flatpak Runtimes: list + remove unused
- Firmware: list + update
- Background Services: list + start/stop/enable/disable
- Startup Items: list + enable/disable
- Old Kernels: list + remove old
- Disk Usage: summary view + per-category cleanup

Out of scope for v1: CLI tracker output log viewer, custom check schedule
per row, kernel pinning, per-app Flatpak runtime overrides.
