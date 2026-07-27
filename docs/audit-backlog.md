# Audit backlog

Open items from the design + accessibility audit of 2026-07-27. Everything here was found,
verified with numbers, and consciously deferred — none of it is unknown or unreproduced.

The audit's P1s are already fixed and are NOT listed here: control-boundary contrast across 14
controls (1.27:1 → 3.90:1), the `tool_call` side-stripe/emoji/tracked-caps overhaul, the
`date_picker` cascade collision, library-wide focus-ring retint, and reduced-motion fallbacks for
every infinite animation.

## P2

**File input renders 42px tall where every other input type is 36px.**
`/input`, the "Input Types" grid. text/email/password/url/tel/search/number all render 36px; only
`type=file` differs, so the row is visibly uneven. Cause is the native
`::file-selector-button` box adding height inside the control. Fix in
`css/components/input.css` on `.mui-input--file`.

**Switch has no visual invalid state.**
`/switch`, the "Consent (invalid)" demo is pixel-identical to a plain unchecked switch and
`aria-invalid` reads null. Input and Select both show a red border for the same state, so this is
an inconsistency in the system rather than a missing feature. Fix in
`css/components/switch.css` + whatever sets `aria-invalid` in `src/primitives/switch.rs`.

## P3

**Two different reds for validation text.**
Input uses `rgb(239,68,68)` (5.01:1), Field uses `rgb(248,113,113)` (6.82:1). Both pass AA
individually, so nothing is broken — but they are the same semantic role rendered two ways.
Pick one (`--mui-danger-text` is the token that already exists) and use it in both.

**Slider shows its value twice.**
`/slider`, the "Budget range" two-thumb demo prints `120 — 360` from the component's built-in
`show_value` span directly beneath the demo's own formatted `$120 — $360` header. Same number,
two formats, one row. This is a *showcase* defect, not a component defect — the demo should not
set `show_value` when it renders its own header. `src/primitives/slider.rs`, `showcase()`.

**Field horizontal orientation is top-aligned, not centre-aligned.**
`/field`, "Horizontal Orientation": the label's vertical centre sits 8px above the input's
(label h=21, input h=36, both starting at the same y). Subtle, but a pixel pass would flag it.

**Disabled loading buttons sit near 3:1.**
`/button`, "Signing in…" / "Deploying…" use `aria-disabled` + `opacity: 0.5`. That 0.5 is the
library-wide disabled convention (26 rules use it), so changing it here alone would introduce an
inconsistency — this is a system-level call about the disabled opacity, not a button fix.

## Outside the repo

**The GitHub repo description still reads "58 headless, accessible UI components".**
14 behind, and it is the first line a visitor reads. `gh repo edit hgeldenhuys/maud-ui
--description "…"`. Left alone because it is public-facing metadata and belongs to the maintainer.
