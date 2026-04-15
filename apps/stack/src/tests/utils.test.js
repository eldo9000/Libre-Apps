import { describe, it, expect } from 'vitest';
import { applyLineEnding } from '../lib/utils.js';

describe('applyLineEnding', () => {
  it('returns LF unchanged when mode is LF', () => {
    expect(applyLineEnding('a\nb\nc', 'LF')).toBe('a\nb\nc');
  });

  it('converts LF to CRLF', () => {
    expect(applyLineEnding('a\nb\nc', 'CRLF')).toBe('a\r\nb\r\nc');
  });

  it('converts LF to CR', () => {
    expect(applyLineEnding('a\nb\nc', 'CR')).toBe('a\rb\rc');
  });

  it('normalizes mixed CRLF+LF input before applying mode', () => {
    expect(applyLineEnding('a\r\nb\nc', 'LF')).toBe('a\nb\nc');
    expect(applyLineEnding('a\r\nb\nc', 'CRLF')).toBe('a\r\nb\r\nc');
  });

  it('handles empty string', () => {
    expect(applyLineEnding('', 'CRLF')).toBe('');
  });

  it('handles content with no line breaks', () => {
    expect(applyLineEnding('hello', 'CRLF')).toBe('hello');
  });
});
