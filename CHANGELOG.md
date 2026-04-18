# Changelog

All notable changes to **maud-ui** are documented here.
Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Newest on top.

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
