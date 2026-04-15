import { describe, it, expect } from 'vitest';

// ── Inline copies of the helpers under test ────────────────────────────────
// (These match the implementations in QuickFiles.svelte exactly)

const ARCHIVE_EXTS = new Set(['zip', '7z', 'tar', 'gz', 'bz2', 'xz', 'rar', 'tgz', 'tbz2', 'txz']);

function isArchive(item) {
  if (!item || item.is_dir) return false;
  const name = (item.name ?? '').toLowerCase();
  if (name.endsWith('.tar.gz') || name.endsWith('.tar.bz2') || name.endsWith('.tar.xz')) return true;
  const ext = (item.extension ?? '').toLowerCase();
  return ARCHIVE_EXTS.has(ext);
}

function archiveEntriesForDir(entries, internalDir) {
  const prefix = internalDir ? internalDir + '/' : '';
  const seen = new Set();
  const result = [];
  for (const entry of entries) {
    if (!entry.path.startsWith(prefix)) continue;
    const rest = entry.path.slice(prefix.length);
    if (!rest) continue;
    const slashIdx = rest.indexOf('/');
    if (slashIdx === -1) {
      if (!seen.has(entry.path)) { seen.add(entry.path); result.push(entry); }
    } else {
      const dirName = rest.slice(0, slashIdx);
      const dirKey = prefix + dirName;
      if (!seen.has(dirKey)) {
        seen.add(dirKey);
        result.push({ name: dirName, path: dirKey, is_dir: true, size: null, modified: null, extension: null, tags: [] });
      }
    }
  }
  result.sort((a, b) => {
    if (a.is_dir && !b.is_dir) return -1;
    if (!a.is_dir && b.is_dir) return 1;
    return a.name.localeCompare(b.name);
  });
  return result;
}

// ── Flat entry list used across tests ─────────────────────────────────────
const FLAT_ENTRIES = [
  { name: 'hello.txt',   path: 'hello.txt',              is_dir: false, size: 5,  extension: 'txt',  tags: [] },
  { name: 'world.txt',   path: 'subdir/world.txt',       is_dir: false, size: 5,  extension: 'txt',  tags: [] },
  { name: 'deep.txt',    path: 'subdir/nested/deep.txt', is_dir: false, size: 4,  extension: 'txt',  tags: [] },
  { name: 'subdir',      path: 'subdir',                 is_dir: true,  size: null, extension: null, tags: [] },
];

// ── isArchive ──────────────────────────────────────────────────────────────
describe('isArchive', () => {
  it('returns true for .zip', () => {
    expect(isArchive({ name: 'files.zip', extension: 'zip', is_dir: false })).toBe(true);
  });
  it('returns true for .7z', () => {
    expect(isArchive({ name: 'backup.7z', extension: '7z', is_dir: false })).toBe(true);
  });
  it('returns true for .tar', () => {
    expect(isArchive({ name: 'src.tar', extension: 'tar', is_dir: false })).toBe(true);
  });
  it('returns true for .tar.gz (compound extension)', () => {
    expect(isArchive({ name: 'src.tar.gz', extension: 'gz', is_dir: false })).toBe(true);
  });
  it('returns true for .tar.bz2 (compound extension)', () => {
    expect(isArchive({ name: 'src.tar.bz2', extension: 'bz2', is_dir: false })).toBe(true);
  });
  it('returns true for .tar.xz (compound extension)', () => {
    expect(isArchive({ name: 'src.tar.xz', extension: 'xz', is_dir: false })).toBe(true);
  });
  it('returns true for .rar', () => {
    expect(isArchive({ name: 'stuff.rar', extension: 'rar', is_dir: false })).toBe(true);
  });
  it('returns false for .txt', () => {
    expect(isArchive({ name: 'readme.txt', extension: 'txt', is_dir: false })).toBe(false);
  });
  it('returns false for .mp4', () => {
    expect(isArchive({ name: 'video.mp4', extension: 'mp4', is_dir: false })).toBe(false);
  });
  it('returns false for directory entries', () => {
    expect(isArchive({ name: 'archive.zip', extension: 'zip', is_dir: true })).toBe(false);
  });
  it('returns false for null', () => {
    expect(isArchive(null)).toBe(false);
  });
});

// ── archiveEntriesForDir ───────────────────────────────────────────────────
describe('archiveEntriesForDir', () => {
  it('returns direct children of root', () => {
    const result = archiveEntriesForDir(FLAT_ENTRIES, '');
    const names = result.map(e => e.name);
    // Root should contain: subdir (synthesized or explicit) and hello.txt
    expect(names).toContain('hello.txt');
    expect(names).toContain('subdir');
    expect(names.length).toBe(2);
  });

  it('dirs come before files at root', () => {
    const result = archiveEntriesForDir(FLAT_ENTRIES, '');
    expect(result[0].is_dir).toBe(true);
    expect(result[0].name).toBe('subdir');
  });

  it('returns direct children of subdir', () => {
    const result = archiveEntriesForDir(FLAT_ENTRIES, 'subdir');
    const names = result.map(e => e.name);
    expect(names).toContain('world.txt');
    expect(names).toContain('nested');
    expect(names.length).toBe(2);
  });

  it('synthesizes nested dir entry', () => {
    const result = archiveEntriesForDir(FLAT_ENTRIES, 'subdir');
    const nested = result.find(e => e.name === 'nested');
    expect(nested).toBeDefined();
    expect(nested.is_dir).toBe(true);
    expect(nested.path).toBe('subdir/nested');
  });

  it('returns direct children of subdir/nested', () => {
    const result = archiveEntriesForDir(FLAT_ENTRIES, 'subdir/nested');
    expect(result.length).toBe(1);
    expect(result[0].name).toBe('deep.txt');
  });

  it('returns empty for non-existent dir', () => {
    const result = archiveEntriesForDir(FLAT_ENTRIES, 'nonexistent');
    expect(result.length).toBe(0);
  });

  it('does not return duplicate synthesized dirs', () => {
    const entriesWithDupes = [
      { name: 'a.txt', path: 'sub/a.txt', is_dir: false, size: 1, extension: 'txt', tags: [] },
      { name: 'b.txt', path: 'sub/b.txt', is_dir: false, size: 1, extension: 'txt', tags: [] },
    ];
    const result = archiveEntriesForDir(entriesWithDupes, '');
    const subdirs = result.filter(e => e.name === 'sub');
    expect(subdirs.length).toBe(1);
  });
});
