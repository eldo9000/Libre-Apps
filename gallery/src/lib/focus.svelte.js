import { invoke } from '@tauri-apps/api/core';

export const focus = $state({ card: null });

export function setFocus(card) {
  focus.card = card;
  invoke('write_focus', {
    json: JSON.stringify({ ...card, focusedAt: new Date().toISOString() }),
  }).catch(() => {});
}

export function clearFocus() {
  focus.card = null;
  invoke('write_focus', { json: '' }).catch(() => {});
}
