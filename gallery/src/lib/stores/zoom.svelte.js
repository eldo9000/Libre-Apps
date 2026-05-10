// Canonical UI zoom store — pattern lifted from Flicker-App.
//
// Uses `document.documentElement.style.zoom` so the entire render tree
// (sidebars, panels, footer, dialogs) scales proportionally in one shot.
// No transforms, no width compensation — sidebars stay the right relative
// size for free.

export const ZOOM_STEPS = [1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9, 2.0];
const KEY = 'libre-gallery-zoom-v2';

function loadInitial() {
  const raw = parseFloat(localStorage.getItem(KEY));
  return ZOOM_STEPS.includes(raw) ? raw : 1.0;
}

export function createZoom() {
  let level = $state(loadInitial());

  function apply(next) {
    level = next;
    localStorage.setItem(KEY, String(next));
    document.documentElement.style.zoom = String(next);
  }

  function stepIn()  { const i = ZOOM_STEPS.indexOf(level); if (i < ZOOM_STEPS.length - 1) apply(ZOOM_STEPS[i + 1]); }
  function stepOut() { const i = ZOOM_STEPS.indexOf(level); if (i > 0) apply(ZOOM_STEPS[i - 1]); }
  function reset()   { apply(1.0); }

  function handleKey(e) {
    if (!e.metaKey && !e.ctrlKey) return;
    if (e.key !== '=' && e.key !== '+' && e.key !== '-' && e.key !== '0') return;
    const tag = document.activeElement?.tagName;
    if (tag === 'INPUT' || tag === 'TEXTAREA') return;
    e.preventDefault();
    if (e.key === '=' || e.key === '+') stepIn();
    else if (e.key === '-') stepOut();
    else if (e.key === '0') reset();
  }

  if (typeof document !== 'undefined') document.documentElement.style.zoom = String(level);

  return {
    get level() { return level; },
    apply, stepIn, stepOut, reset, handleKey,
  };
}
