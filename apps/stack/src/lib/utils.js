/**
 * Pure utility functions for Stack.
 * Extracted here to be testable without Tauri or CodeMirror.
 */

/**
 * Normalize line endings in content before saving.
 * @param {string} content
 * @param {'LF'|'CRLF'|'CR'} lineEnding
 * @returns {string}
 */
export function applyLineEnding(content, lineEnding) {
  // Normalize to LF first (handles mixed CRLF/CR/LF input)
  const lf = content.replace(/\r\n/g, '\n').replace(/\r/g, '\n');
  if (lineEnding === 'CRLF') return lf.replace(/\n/g, '\r\n');
  if (lineEnding === 'CR') return lf.replace(/\n/g, '\r');
  return lf;
}
