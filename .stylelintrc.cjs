/** @type {import('stylelint').Config} */
module.exports = {
  extends: [
    'stylelint-config-standard',
    'stylelint-config-html/svelte', // parses <style> blocks in .svelte files
  ],

  rules: {
    // ── Token enforcement: forbid raw hex in color-bearing properties ──────
    // Only var(--token) references are allowed. Named colors (white, black,
    // transparent, inherit) are still permitted — the regex targets # only.
    'declaration-property-value-disallowed-list': [
      {
        '/^color/': ['/#[0-9a-fA-F]{3,8}/'],
        '/^background/': ['/#[0-9a-fA-F]{3,8}/'],
        '/^border(-color)?/': ['/#[0-9a-fA-F]{3,8}/'],
        'fill': ['/#[0-9a-fA-F]{3,8}/'],
        'stroke': ['/#[0-9a-fA-F]{3,8}/'],
        'outline-color': ['/#[0-9a-fA-F]{3,8}/'],
        'text-decoration-color': ['/#[0-9a-fA-F]{3,8}/'],
        'caret-color': ['/#[0-9a-fA-F]{3,8}/'],
        'box-shadow': ['/#[0-9a-fA-F]{3,8}/'],
      },
      {
        message: (prop, value) =>
          `Use a CSS variable (var(--token)) instead of raw hex "${value}" in "${prop}".`,
      },
    ],


    // Keep standard invalid-hex check too
    'color-no-invalid-hex': true,

    // Allow Tailwind v4 and Svelte at-rules
    'at-rule-no-unknown': [
      true,
      {
        ignoreAtRules: [
          'tailwind', 'apply', 'layer', 'variants', 'responsive', 'screen', 'variant',
        ],
      },
    ],

    // Tailwind v4 uses bare-string @import "tailwindcss" — not url() form
    'import-notation': 'string',

    // Svelte :global() is a valid pseudo-class
    'selector-pseudo-class-no-unknown': [
      true,
      { ignorePseudoClasses: ['global', 'local'] },
    ],
  },

  // tokens.css is the raw-value source of truth — hex is expected there
  overrides: [
    {
      files: ['**/tokens.css'],
      rules: {
        'declaration-property-value-disallowed-list': null,
      },
    },
  ],
};
