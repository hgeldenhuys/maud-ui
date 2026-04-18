# Changelog

All notable changes to **maud-ui** are documented here.
Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Newest on top.

---

## [0.2.0] — 2026-04-18

First crates.io release beyond the `0.1.0` first-cut. Summary of
everything that landed in the 0.1 → 0.2 window:

### Added
- **Component count 58 → 59**: new `swatch` primitive (colour chip
  with click-to-copy, design-token mode, Tailwind tone ramps).
- **Theme customiser** at `/theme`: live-edit every `--mui-*` token
  with a colour picker + free-text input, 8 Tailwind-based presets
  (Dark, Light, Slate, Zinc·Violet, Stone·Amber, Emerald, Rose,
  High-contrast), `localStorage` persistence, `:root { … }` export
  with Copy + Download.
- **Integrations shell pattern** — 15 third-party widgets wrapped
  in a maud-ui chrome: Monaco editor, xyflow, Excalidraw, TipTap,
  Mermaid, Cytoscape, Three.js, AG Grid, Apache ECharts, Leaflet,
  FullCalendar, Wavesurfer.js, PDF.js, xterm.js, SortableJS.
- **Sticky compact header** with brand · search · nav · icon
  toggles. `--mui-header-h` CSS var drives the sidebar offset and
  `scroll-margin-top` on every `[id]` anchor.
- **Global `cmd+k` command palette** — fuzzy jump to any of 88
  indexed destinations (pages + components + blocks + integrations).
  Index generated from the same Rust constants the sidebar uses.
- **Mobile drawer** — sidebar becomes an off-canvas drawer at
  `<=960px`, hamburger button in the header.
- **Interactive `button_group`** modes — `Mode::Exclusive`
  (segmented control) and `Mode::Multiple` (toggle bar), with
  `aria-pressed` wiring and a `mui:button-group-change` custom
  event. Ships as a bundled behaviour (`data-mui="button-group"`).
- Sidebar search with `/` hotkey, `<mark>` highlighting, empty-group
  collapse.

### Fixed
- `showcase_css()` was emitted without `PreEscaped`, so any new
  `[attr="value"]` selector was HTML-escaped to `[attr=&quot;…"]`
  and silently dropped by the CSS parser. Wrapped with
  `maud::PreEscaped` so attribute-selector-driven states work.

## [2026-04-18] — SortableJS integration (drag & drop)

### Added
- **`/integrations/sortable`** — three drag-and-drop demos backed
  by SortableJS 1.15.3 (UMD from jsDelivr, ~30 KB, vanilla, no
  framework):
  - **Sortable list** with drag handle — Shuffle / Reset / Export
    order (dumps current order as JSON).
  - **Kanban board** — 3 columns sharing `group: 'mui-kanban'`,
    cards drag within and across columns, live column counts, the
    "last move" line in the statusbar narrates the last action.
  - **Tile grid** — free-form rearrangement for dashboards.
- "**Drag & Drop**" group in the Advanced dropdown.

### Changed
- Static export now ships 88 pages (was 87).
- Gallery math: 15 integration pages (was 14).

## [2026-04-18] — Swatch primitive + theme customiser

### Added
- **Swatch primitive** (`maud_ui::primitives::swatch`) — colour chips
  with click-to-copy. Three modes: `Raw` (literal CSS colour value),
  `Token` (reads `var(--mui-*)` live), and scale ramps via
  `render_scale(name, stops)`. Ships with a Tailwind-family helper
  `tailwind_ramp("blue")` that returns all 22 named palettes 50..950.
  Gallery: `/swatch`.
- **Theme customiser** at `/theme` — two-column page: left column has
  grouped token controls (colours, radii, typography, spacing) with
  both a native colour picker and a free-text input for each; right
  column is a live preview (swatch grid + buttons + form controls +
  card + alert) that re-renders instantly as tokens mutate. Ships
  with **8 presets** grounded in Tailwind palettes (Dark, Light,
  Slate, Zinc·Violet, Stone·Amber, Emerald, Rose, High contrast).
  Persists to `localStorage` and exports a paste-ready `:root { ... }`
  CSS block (copy button + download as `.css`).
- "Theme" link in the sticky header nav.

### Changed
- Component count bumped from 58 → 59 (swatch is the new one).
- Static export now ships 87 pages (was 85).

## [2026-04-18] — UX: sidebar search + "/" hotkey

### Added
- **Sidebar search** — new input in the page header filters the
  sidebar component list in real time. Matches are highlighted
  inline with `<mark>` chips, empty tier groups collapse, and an
  "No matches. Press Esc to clear." message shows when nothing hits.
  Searches across display names and slugs.
- Global **`/` hotkey** focuses the search bar (GitHub convention —
  skipped when already typing into another input / textarea /
  contenteditable). **`Esc`** clears the query and restores the
  full list.

## [2026-04-18] — UX: sticky compact header, icon toggles, scroll-anchor offsets

### Changed
- **Page header** is now **sticky** (pinned to the top on scroll) with a
  translucent `backdrop-filter: blur` chrome, so primary nav is always
  one click away as you read down a component / integration page.
- Header restructured to a **single-row compact layout**: brand +
  component count on the left, primary nav in the middle, icon-only
  Theme (◐) and Direction (⇄) toggles on the right. Overall header
  height dropped from ~160 px to ~52 px — a much bigger content viewport.
- Published `--mui-header-h` as a CSS custom property, used by the
  sidebar (now sticks below the header instead of top: 0) and by
  every `[id]` anchor (adds `scroll-margin-top` so jump links land
  *below* the sticky header instead of vanishing behind it).

### Fixed
- Dark rectangle bleed behind the nav on first paint: the dist
  `maud-ui.css` styles `<nav>` elements with a card background by
  default, which conflicted with the transparent header backdrop.
  Scoped `background: transparent` + `box-shadow: none` for
  `.mui-showcase__nav` specifically.

## [2026-04-18] — Fix: AG Grid empty render + Wavesurfer zoom UX

### Fixed
- **AG Grid** rendered an empty frame. `.ag-root-wrapper` collapsed to
  2px despite 50 rows being present in the grid API and DOM. AG Grid
  v32.3's legacy CSS path doesn't reliably size the wrapper inside
  custom hosts — added explicit `height: 100% !important` on
  `.ag-root-wrapper` and made the theme container a flex column so
  the wrapper grows to fill.
- **Wavesurfer zoom** had no visible effect on the first few clicks.
  Root cause: 4 second audio × default 50 px/s = 200 px, but the
  container is ~700 px wide → wavesurfer stretches the waveform to
  fit the host, and it takes 6 zoom-in clicks before the natural
  width exceeds the container. Fixed by lengthening the audio to 20
  seconds (with a pitch sweep + amplitude pulse for visual interest)
  and bumping the default zoom to 100 px/s with 40 px/s increments.
  Now every click scrolls the waveform visibly.

## [2026-04-18] — Advanced integrations, part 2 (11 new)

### Added
Eleven new `/integrations/*` pages, all following the same maud-ui shell
pattern and loaded on demand from CDN (esm.sh importmap / jsDelivr UMD):

- **Code & Text** — `tiptap` (ProseMirror-based rich text editor with
  formatting toolbar, active-state tracking, word / character counter,
  live HTML output panel).
- **Diagrams & Graphs** — `mermaid` (text-to-diagram renderer with
  tabbed samples for flowchart / sequence / class / Gantt, live render
  on change, split-pane source + preview), `cytoscape` (network graph
  visualisation with layout switcher and a seeded service-mesh graph).
- **Canvas** — `threejs` (WebGL 3D scene with torus knot / icosahedron
  / box / sphere / cone shapes, orbit controls, wireframe toggle,
  spin toggle, FPS and triangle count in status bar).
- **Data** — `ag-grid` (AG Grid Community with 50 seed rows, sortable
  / filterable columns, row selection, CSV export, theme auto-swap
  between quartz and quartz-dark), `echarts` (Apache ECharts with
  chart-type switcher, randomise button, PNG export).
- **Maps & Scheduling** — `leaflet` (OpenStreetMap tile viewer with
  marker add / clear / fit, three tile provider options), `fullcalendar`
  (FullCalendar 6 with month / week / day / list views, drag-to-move
  events, dateClick-to-create, seeded around today).
- **Media** — `wavesurfer` (Wavesurfer.js waveform with in-browser
  OfflineAudioContext synthesis + Web Audio BufferSourceNode playback,
  zoom, speed, theme-aware colours), `pdfjs` (Mozilla PDF.js rendering
  a multi-page in-browser-generated PDF, prev/next/zoom/fit toolbar).
- **Terminal** — `xterm` (xterm.js terminal with FitAddon +
  WebLinksAddon, a demo command set, toolbar-driven pipeline replay,
  theme-reactive colour palette).

### Changed
- `page_header()` nav replaced the plain Advanced dropdown with a
  **grouped** dropdown — Code & Text / Diagrams & Graphs / Canvas /
  Data / Maps & Scheduling / Media / Terminal. Each entry carries a
  one-line description. Still zero-JS (`<details>`/`<summary>`).
- Static export now ships **85 pages** (was 74).

### Fixed
- Wavesurfer.js v7's `url: blobUrl` option hangs with `readyState: 0`
  on synthesised WAV blobs. Switched to the `peaks:` + `duration:`
  pattern (pre-computed visualisation) plus an independent Web Audio
  `BufferSourceNode` driving sound, with `requestAnimationFrame`
  syncing the wavesurfer cursor to the audio clock. Decode bottleneck
  bypassed entirely.

---

## [2026-04-18] — Advanced integrations: xyflow + Excalidraw

### Added
- **xyflow node editor** integration page at `/integrations/xyflow` — embeds
  `@xyflow/react` (v12.3.6) loaded via native ESM importmap from `esm.sh`,
  seeded with an 8-node / 9-edge API-pipeline graph. Toolbar: Add Node,
  Reset, Fit, Layout (horizontal / vertical), Export JSON. Includes
  minimap, controls, animated edges, and theme sync with `data-theme`.
- **Excalidraw whiteboard** integration page at `/integrations/excalidraw`
  — embeds `@excalidraw/excalidraw` (v0.17.6) loaded via native ESM
  importmap from `esm.sh`. Toolbar wraps the imperative
  `excalidrawAPI` with + Rect / + Ellipse / + Text / Reset / PNG / SVG.
- **"Advanced" nav dropdown** in the page header — groups Monaco, xyflow,
  and Excalidraw under a single `<details>`/`<summary>` menu with a
  rotating caret. Zero JS; titles + one-line descriptions per entry.

### Changed
- Renamed `monaco_css()` → `integration_shell_css()` and now loaded on
  all three integration pages. The shared `.mui-integration__*` shell
  (header, editor, statusbar, output, dimensions) previously lived
  inside the Monaco-only CSS function and collapsed the xyflow graph
  container to 0 height.
- Static export now ships 74 pages (up from 72) — both new integration
  routes registered in `js/export-static.mjs`.

### Fixed
- xyflow editor container rendering at 0px height on first load because
  the shell CSS didn't load outside the Monaco page.
- Excalidraw imports failing with
  `SyntaxError: The requested module '@excalidraw/excalidraw' does not
  provide an export named 'Excalidraw'` — esm.sh wraps Excalidraw's CJS
  build such that named exports can land on the namespace OR on
  `.default`. Replaced named imports with a namespace-import +
  default-fallback resolution pattern.
