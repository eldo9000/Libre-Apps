import { describe, it, expect } from 'vitest';

// ── Pure functions extracted from the init scripts for testability ────────────

/**
 * Mirrors the isBlocked() logic in PRIVACY_INIT_SCRIPT.
 * Returns true if url's hostname (or any parent domain) is in the blocked set.
 */
function isBlocked(url, blocked) {
  try {
    const host = new URL(url, 'https://example.com').hostname.replace(/^www\./, '');
    return blocked.has(host) || [...blocked].some((d) => host.endsWith('.' + d));
  } catch {
    return false;
  }
}

/**
 * Mirrors the scheme guard in go() from TOOLBAR_INIT_SCRIPT.
 */
function isSchemeBanned(url) {
  return /^javascript:/i.test(url) || /^data:/i.test(url) || /^vbscript:/i.test(url);
}

/**
 * Mirrors the HTTP warning condition in TOOLBAR_INIT_SCRIPT.
 */
function isInsecureHttp(url) {
  try {
    const u = new URL(url);
    return u.protocol === 'http:' && u.hostname !== 'localhost';
  } catch {
    return false;
  }
}

/**
 * Mirrors load_blocklist() logic for the frontend (parse JSON → Set).
 */
function loadBlocklist(json) {
  try {
    const domains = JSON.parse(json);
    if (!Array.isArray(domains)) return new Set();
    return new Set(domains);
  } catch {
    return new Set();
  }
}

// ── isBlocked ─────────────────────────────────────────────────────────────────

describe('isBlocked', () => {
  const blocked = new Set(['google-analytics.com', 'doubleclick.net', 'hotjar.com']);

  it('blocks exact domain match', () => {
    expect(isBlocked('https://google-analytics.com/collect', blocked)).toBe(true);
  });

  it('blocks www-stripped domain', () => {
    expect(isBlocked('https://www.google-analytics.com/collect', blocked)).toBe(true);
  });

  it('blocks subdomain of blocked domain', () => {
    expect(isBlocked('https://stats.hotjar.com/track', blocked)).toBe(true);
    expect(isBlocked('https://sub.doubleclick.net/pcs/click', blocked)).toBe(true);
  });

  it('allows unblocked domain', () => {
    expect(isBlocked('https://example.com/api/data', blocked)).toBe(false);
    expect(isBlocked('https://github.com', blocked)).toBe(false);
  });

  it('returns false for malformed url', () => {
    expect(isBlocked('not-a-url', blocked)).toBe(false);
  });

  it('returns false for empty blocked set', () => {
    expect(isBlocked('https://google-analytics.com/collect', new Set())).toBe(false);
  });
});

// ── Scheme blocking ───────────────────────────────────────────────────────────

describe('scheme blocking', () => {
  it('blocks javascript: scheme (lowercase)', () => {
    expect(isSchemeBanned('javascript:alert(1)')).toBe(true);
  });

  it('blocks JavaScript: scheme (mixed case)', () => {
    expect(isSchemeBanned('JavaScript:alert(1)')).toBe(true);
  });

  it('blocks data: scheme', () => {
    expect(isSchemeBanned('data:text/html,<script>alert(1)</script>')).toBe(true);
  });

  it('blocks vbscript: scheme', () => {
    expect(isSchemeBanned('vbscript:MsgBox(1)')).toBe(true);
  });

  it('allows https:', () => {
    expect(isSchemeBanned('https://example.com')).toBe(false);
  });

  it('allows http:', () => {
    expect(isSchemeBanned('http://example.com')).toBe(false);
  });

  it('allows ftp:', () => {
    expect(isSchemeBanned('ftp://example.com/file.zip')).toBe(false);
  });
});

// ── HTTPS detection ───────────────────────────────────────────────────────────

describe('HTTPS detection', () => {
  it('flags http:// as insecure', () => {
    expect(isInsecureHttp('http://example.com/page')).toBe(true);
  });

  it('does not flag https:// as insecure', () => {
    expect(isInsecureHttp('https://example.com/page')).toBe(false);
  });

  it('does not flag localhost as insecure', () => {
    expect(isInsecureHttp('http://localhost:3000')).toBe(false);
  });

  it('returns false for invalid URL', () => {
    expect(isInsecureHttp('not a url')).toBe(false);
  });
});

// ── Blocklist loading ─────────────────────────────────────────────────────────

describe('loadBlocklist', () => {
  it('loads a valid JSON array into a Set', () => {
    const result = loadBlocklist('["google-analytics.com","doubleclick.net"]');
    expect(result.size).toBe(2);
    expect(result.has('google-analytics.com')).toBe(true);
    expect(result.has('doubleclick.net')).toBe(true);
  });

  it('returns empty Set for invalid JSON', () => {
    const result = loadBlocklist('not valid json {{{');
    expect(result.size).toBe(0);
  });

  it('returns empty Set for non-array JSON', () => {
    const result = loadBlocklist('{"key":"value"}');
    expect(result.size).toBe(0);
  });

  it('returns empty Set for empty array', () => {
    const result = loadBlocklist('[]');
    expect(result.size).toBe(0);
  });

  it('handles large lists without error', () => {
    const domains = Array.from({ length: 200 }, (_, i) => `tracker${i}.com`);
    const result = loadBlocklist(JSON.stringify(domains));
    expect(result.size).toBe(200);
    expect(result.has('tracker0.com')).toBe(true);
    expect(result.has('tracker199.com')).toBe(true);
  });
});
