// Canonical hint/tooltip store — pattern lifted from Flicker-App / Fade-App.
//
// Drives the center text in the footer bar with polished timing:
//   - Fresh show: 100ms fade in
//   - Hover off:  hold 3s, then 4s fade out
//   - Interrupted by new hint: 100ms crossfade
//
// Producers set `data-tooltip="..."` on any element; a single root-level
// `onmouseover` delegate resolves it and calls `setHint(text)`. On mouseleave
// (or empty string), the hint fades out gracefully.
//
// Consumer binds to `tooltip.text`, `tooltip.opacity`, `tooltip.duration`,
// `tooltip.delay` and applies via CSS `opacity` + `transition`.

let _text     = $state('');
let _opacity  = $state(0);
let _duration = $state(100);
let _delay    = $state(0);

let clearTimer = null;
let swapTimer  = null;

function cancelTimers() {
  if (clearTimer) { clearTimeout(clearTimer); clearTimer = null; }
  if (swapTimer)  { clearTimeout(swapTimer);  swapTimer  = null; }
}

export function setHint(t) {
  const next = t ?? '';
  cancelTimers();

  if (!next) {
    _delay    = 3000;
    _duration = 4000;
    _opacity  = 0;
    clearTimer = setTimeout(() => { _text = ''; clearTimer = null; }, 7000);
    return;
  }

  if (_text === next && _opacity === 1 && !clearTimer) return;

  if (!_text || _opacity === 0) {
    _text     = next;
    _delay    = 0;
    _duration = 100;
    _opacity  = 1;
    return;
  }

  _delay    = 0;
  _duration = 100;
  _opacity  = 0;
  swapTimer = setTimeout(() => {
    _text    = next;
    _opacity = 1;
    swapTimer = null;
  }, 100);
}

export const tooltip = {
  get text()     { return _text; },
  get opacity()  { return _opacity; },
  get duration() { return _duration; },
  get delay()    { return _delay; },
};
