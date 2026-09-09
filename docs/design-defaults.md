# Design defaults · 0.9.0

Quiet operational surfaces, four levels of neutral ink, restrained blue, and a paired light/dark semantic palette. Body text is 15px, secondary text 14px, captions 12px; headings use weight 600. Prose links keep a quiet underline; navigation, table links, chips and buttons do not acquire one on hover.

The source is `static/styles/maud-ui.css`. Its component/block imports and `tokens.css` are editable. `node examples/build-assets.mjs` builds all four static bundles; `--check` verifies their freshness. Legacy css/js/dist/public/assets paths are unchanged. Same-named files in `static/behaviors/` replace legacy runtime behaviors in this builder.

## Complete token migration

This table lists every changed or added token (109). One value applies to both themes; two values are dark / light. A dash means the token did not exist. Snapshots: [before](design-tokens-before.json), [after](design-tokens-after.json). Unchanged tokens, including the lg/full radii, base spacing steps, focus ring and easing curve, remain available.

| Token | 0.8 dark / light | 0.9 dark / light |
|---|---|---|
| `--mui-bg` | `#0a0a0b` / `#ffffff` | `#101216` / `#f7f8fa` |
| `--mui-bg-card` | `#111113` / `#f9fafb` | `#181b20` / `#ffffff` |
| `--mui-bg-input` | `#18181b` / `#f4f4f5` | `#20242b` / `#f0f2f5` |
| `--mui-bg-overlay` | `rgba(0, 0, 0, 0.8)` / `rgba(0, 0, 0, 0.4)` | `rgba(0, 0, 0, 0.64)` / `rgba(15, 23, 42, 0.48)` |
| `--mui-text` | `#fafafa` / `#09090b` | `#edf0f4` / `#1b2330` |
| `--mui-text-body-leading` | — | `1.6` |
| `--mui-text-body-size` | — | `0.9375rem` |
| `--mui-text-body-tracking` | — | `0` |
| `--mui-text-caption-leading` | — | `1.4` |
| `--mui-text-caption-size` | — | `0.75rem` |
| `--mui-text-caption-tracking` | — | `0.01em` |
| `--mui-text-display-leading` | — | `1.1` |
| `--mui-text-display-size` | — | `2.75rem` |
| `--mui-text-display-tracking` | — | `-0.035em` |
| `--mui-text-h1-leading` | — | `1.2` |
| `--mui-text-h1-size` | — | `2rem` |
| `--mui-text-h1-tracking` | — | `-0.025em` |
| `--mui-text-h2-leading` | — | `1.25` |
| `--mui-text-h2-size` | — | `1.5rem` |
| `--mui-text-h2-tracking` | — | `-0.02em` |
| `--mui-text-h3-leading` | — | `1.35` |
| `--mui-text-h3-size` | — | `1.125rem` |
| `--mui-text-h3-tracking` | — | `-0.01em` |
| `--mui-text-muted` | `#a1a1aa` / `#61616b` | `#a1acba` / `#5e6a7a` |
| `--mui-text-secondary` | — | `#c1c8d2` / `#475364` |
| `--mui-text-small-leading` | — | `1.5` |
| `--mui-text-small-size` | — | `0.875rem` |
| `--mui-text-small-tracking` | — | `0` |
| `--mui-text-subtle` | `#8e8e93` / `#686872` | `#939eae` / `#606d80` |
| `--mui-accent` | `#2563eb` | `#84aaff` / `#285bc5` |
| `--mui-accent-fg` | `#ffffff` | `#101b32` / `#ffffff` |
| `--mui-accent-hover` | `#1d4ed8` | `#9bbaff` / `#214ba4` |
| `--mui-accent-soft` | — | `#26344e` / `#e9effb` |
| `--mui-accent-text` | `#60a5fa` / `#1d4ed8` | `#acbfff` / `#2855ae` |
| `--mui-info` | `#1d4ed8` | `#2456a6` |
| `--mui-info-bg` | — | `#1d2b43` / `#eaf1fc` |
| `--mui-info-border` | — | `#3c557c` / `#acc5ed` |
| `--mui-info-fg` | `#fff` | `#ffffff` |
| `--mui-info-text` | `#60a5fa` / `#1d4ed8` | `#aac7fb` / `#2456a6` |
| `--mui-success` | `#15803d` | `#28734f` |
| `--mui-success-bg` | — | `#192f27` / `#edf6f1` |
| `--mui-success-border` | — | `#365747` / `#b4d8c4` |
| `--mui-success-fg` | `#fff` | `#ffffff` |
| `--mui-success-text` | `#4ade80` / `#15803d` | `#a0d2b9` / `#246348` |
| `--mui-warning` | `#a16207` | `#805815` |
| `--mui-warning-bg` | — | `#332b1d` / `#fbf4e5` |
| `--mui-warning-border` | — | `#695333` / `#e4cd9d` |
| `--mui-warning-fg` | `#fff` | `#ffffff` |
| `--mui-warning-text` | `#fbbf24` / `#854d0e` | `#e5c78b` / `#805815` |
| `--mui-danger` | `#dc2626` / `#b91c1c` | `#a1363b` |
| `--mui-danger-bg` | — | `#382326` / `#fbefef` |
| `--mui-danger-border` | — | `#724449` / `#e4b9ba` |
| `--mui-danger-fg` | `#fff` | `#ffffff` |
| `--mui-danger-hover` | — | `#892e32` |
| `--mui-danger-text` | `#f87171` / `#b91c1c` | `#f0b1b5` / `#a1363b` |
| `--mui-violet` | `#6d28d9` | `#63458a` |
| `--mui-violet-bg` | — | `#2b243a` / `#f4effb` |
| `--mui-violet-border` | — | `#55466f` / `#cec0e8` |
| `--mui-violet-fg` | `#fff` | `#ffffff` |
| `--mui-violet-text` | `#c4b5fd` / `#6d28d9` | `#cdbbe7` / `#63458a` |
| `--mui-rose` | `#be123c` | `#96385c` |
| `--mui-rose-bg` | — | `#34232d` / `#fbf0f5` |
| `--mui-rose-border` | — | `#704459` / `#e2bbcb` |
| `--mui-rose-fg` | `#fff` | `#ffffff` |
| `--mui-rose-text` | `#fda4af` / `#be123c` | `#edb5cd` / `#96385c` |
| `--mui-border` | `#27272a` / `#e4e4e7` | `#303640` / `#dde2e8` |
| `--mui-border-control` | `#71717a` | `#727e90` / `#778396` |
| `--mui-border-focus` | `#60a5fa` / `#2563eb` | `#84aaff` / `#285bc5` |
| `--mui-border-hover` | `#3f3f46` / `#d4d4d8` | `var(--mui-border)` |
| `--mui-font-mono` | `"SF Mono", "JetBrains Mono", Monaco, Menlo, monospace` | `"JetBrains Mono", ui-monospace, "SF Mono", SFMono-Regular, Menlo, Consolas, monospace` |
| `--mui-font-sans` | `-apple-system, BlinkMacSystemFont, "Segoe UI", system-ui, sans-serif` | `"Avenir Next", "Nunito Sans", -apple-system, BlinkMacSystemFont, "Segoe UI", system-ui, sans-serif` |
| `--mui-radius-md` | `0.5rem` | `0.375rem` |
| `--mui-radius-sm` | `calc(0.5rem - 2px)` | `0.25rem` |
| `--mui-space-20` | — | `1.25rem` |
| `--mui-space-2xs` | — | `0.125rem` |
| `--mui-space-3xl` | — | `3rem` |
| `--mui-space-4xl` | — | `4rem` |
| `--mui-inset-cell` | — | `0.625rem` |
| `--mui-inset-control` | — | `0.875rem` |
| `--mui-inset-tight` | — | `0.375rem` |
| `--mui-weight-heading` | — | `600` |
| `--mui-weight-medium` | — | `500` |
| `--mui-weight-normal` | — | `400` |
| `--mui-leading-body` | — | `1.6` |
| `--mui-leading-small` | — | `1.5` |
| `--mui-leading-tight` | — | `1.25` |
| `--mui-tracking-heading` | — | `-0.025em` |
| `--mui-tracking-label` | — | `0.08em` |
| `--mui-motion-enter` | `160ms` | `180ms` |
| `--mui-motion-fast` | — | `120ms` |
| `--mui-motion-loading` | — | `1200ms` |
| `--mui-transition` | `150ms cubic-bezier(0.4, 0, 0.2, 1)` | `var(--mui-motion-fast) var(--mui-motion-ease)` |
| `--mui-shadow-lg` | `0 10px 15px rgba(0, 0, 0, 0.3)` / `0 10px 15px rgba(0, 0, 0, 0.1)` | `0 12px 32px rgba(0, 0, 0, 0.36)` / `0 12px 32px rgba(27, 35, 48, 0.14)` |
| `--mui-shadow-md` | `0 4px 6px rgba(0, 0, 0, 0.3)` / `0 4px 6px rgba(0, 0, 0, 0.07)` | `var(--mui-shadow-sm)` |
| `--mui-shadow-sm` | `0 1px 2px rgba(0, 0, 0, 0.3)` / `0 1px 2px rgba(0, 0, 0, 0.05)` | `0 1px 3px rgba(0, 0, 0, 0.18)` / `0 1px 3px rgba(27, 35, 48, 0.06)` |
| `--mui-sidebar-rail-w` | — | `4rem` |
| `--mui-sidebar-w` | `16rem` | `15rem` |
| `--mui-nav-row-height` | — | `2.25rem` |
| `--mui-header-height` | — | `3.5rem` |
| `--mui-bottom-tab-bar-height` | `calc(4rem + env(safe-area-inset-bottom, 0px))` | `calc(3.5rem + env(safe-area-inset-bottom, 0px))` |
| `--mui-control-height` | — | `2.25rem` |
| `--mui-touch-target` | — | `2.75rem` |
| `--mui-icon-size` | — | `1rem` |
| `--mui-page-gutter` | — | `1.5rem` |
| `--mui-layer-navigation` | — | `40` |
| `--mui-layer-overlay` | — | `50` |
| `--mui-layer-sticky` | — | `30` |
| `--mui-layer-toast` | — | `60` |
| `--mui-link` | `var(--mui-accent-text)` | `var(--mui-text-secondary)` |

`--mui-page-gutter` becomes 1rem at ≤40rem. The 4px spacing rhythm includes explicit optical half-steps for 6/10/14px insets. Buttons use 8/14px, inputs 8/12px, cards 20/24px and table cells 10/14px. `--mui-shadow-md` is a compatibility alias for sm, leaving two elevation levels. The two interaction durations are 120/180ms; the separate 1200ms loading cadence is reserved for ongoing progress. Reduced motion disables animation and transitions.

Structural hairlines share one token; control boundaries and semantic borders have their own roles. Controls retain ≥3:1 boundaries, semantic labels ≥4.5:1 contrast. Semantic borders are supporting decoration, never the sole status signal. Coarse-pointer controls expand to 44px and text inputs use 16px to avoid mobile zoom. SVG geometry, percentages, viewport/container sizing, code-size ratios and functional animation geometry remain explicit rather than being forced into a spacing token.

## API migration

- Three new blocks: `shell::page_header`, `shell::app_header`, `shell::app_footer`. Empty masthead/footer props emit nothing.
- `sidebar::NavItem` gains `children`; use `..Default::default()` for optional fields. `sidebar::Props` adds chrome slots, `sidebar_footer`, `collapsible`, `default_collapsed`, `page_header`, and `embedded`. Use embedded mode inside an existing main.
- Labeled sidebar groups persist locally. The icon rail starts at 64rem; the native drawer below 60rem moves the same sidebar node. IDs must be stable and unique.
- Primitive sidebar adds `collapsible_group` and `menu_link`. Its rail now has one handler, full accessible labels, persistence and the same-node drawer.
- `record::header::Props` adds `title_markup`/`status_markup`. Some takes precedence over plain fallbacks; title markup may contain its own heading without nesting.
- `data_table::Column` gains `align: Align`; use Left/Center/Right or Default. `column_header_aligned` adds the same choice to the standalone helper; the two-argument helper remains left-aligned. Sorting and pagination retain original row nodes and rich cells.
- Typography adds `display` and `caption`; the theme customizer exposes role size, leading and tracking and the semantic triples.

## Design choices and review

The house's restrained typography, neutral surfaces, single accent and purposeful hierarchy apply. The artifact doctrine's rigid register layout does not suit an operational UI. The UI/UX search tool suggested organic styling, a horizontal journey and Lora/Raleway; those conflict with this brief's application navigation and fixed font stack, so the checklist informed accessibility and interaction instead. The 15px body and 36px desktop navigation deliberately depart from its blanket 16px/44px defaults; touch targets expand separately.

The landing defaults to light to emphasize clear tables and white working surfaces. Dark is independently tuned and available through the live theme toggle; saved choices apply before paint. The hero is composed from actual blocks and primitives, with local-only guest filtering, row selection and reservation creation. No external booking operation or persistence is implied.

Visual review remains with the supervisor, as requested: inspect both themes at 1280px and 390px; use keyboard navigation, a native dialog and VoiceOver; confirm overflow, focus, motion preferences and safe areas on devices. This lane ran no browser or screenshot process.

Validation: 208 unit/integration tests and 9 doctests passed (2 pre-existing ignored), strict Clippy passed, 392 contrast pairs across 8 presets and 11 browser-free Node fixtures passed. The complete [HTTP audit](design-http-audit.json) covers 117 routes with unique IDs, 3 byte-exact assets and 21 parsed inline scripts. [Source sweep](design-sweep.md) lists every primitive/block.
