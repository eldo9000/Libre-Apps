import { invoke } from '@tauri-apps/api/core';

export const focus = $state({ cards: [] });

function persist() {
  const json = focus.cards.length
    ? JSON.stringify(focus.cards.length === 1 ? focus.cards[0] : focus.cards)
    : '';
  invoke('write_focus', { json }).catch(() => {});
}

export function setSingleFocus(card) {
  focus.cards = [{ ...card, focusedAt: new Date().toISOString() }];
  persist();
}

export function toggleFocus(card) {
  const idx = focus.cards.findIndex(c => c.id === card.id);
  if (idx === -1) {
    focus.cards = [...focus.cards, { ...card, focusedAt: new Date().toISOString() }];
  } else {
    focus.cards = focus.cards.filter(c => c.id !== card.id);
  }
  persist();
}

export function clearFocus() {
  focus.cards = [];
  persist();
}
