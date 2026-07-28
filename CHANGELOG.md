# Changelog

All notable changes to **maud-ui** are documented here.
Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Newest on top.

---

## [0.6.5] — 2026-07-27 — slim floating scrollbar thumb

`scrollbar-width: thin` joins the transparent-track rule; WebKit gets a 10px gutter with a
6px thumb floated by a 2px transparent border + `background-clip: padding-box`, radius 5px,
hover to `--mui-border-hover`. The thumb should never be the brightest object on a page.

---

## [0.6.4] — 2026-07-27 — transparent scrollbar tracks everywhere

Operator standing order: no scrollbar track may paint a background. One inherited
`scrollbar-color: var(--mui-border) transparent` on `:root` (Firefox + Chromium 121+) plus
`::-webkit-scrollbar-track/-corner { background: transparent }` and a tokened thumb for engines
without `scrollbar-color`. Surfaces with bespoke scrollbar styling already complied; this closes
the default-scrollbar gap on every unstyled overflow surface.

---

## [0.6.3] — 2026-07-27 — asleep bar is content-sized

The Asleep composer bar was a fixed 46px box; a consumer auto-grow enhancement
that expands the field on typing overflowed it (reproduced at 72px). The bar is
now content-sized — `min-height: 46px` with 10px vertical padding, so one 24px
line still computes to exactly 46px and any growth carries the dashed bar with
it. No markup or API change.

---

## [0.6.2] — 2026-07-27 — the composer's no-JS contract, made literal

Two adversarial-review findings, both violations of the component's own "works with
JavaScript disabled" doc:

- **Asleep had no field.** The bar rendered a decorative hint span, so `Wake` POSTed a form
  with no `message` — a consumer's typed Form extractor rejects that before any handler runs.
  The bar now contains a real single-row `textarea` (`__input--sleep`); "type to wake" is
  literally true. The `__sleep-hint` class is retired.
- **Interrupt was JS-only.** `secondary_label` rendered `type="button"`. New
  `secondary_action` prop: when set, the root becomes a `<div>` holding the main form plus an
  empty sibling form, and Interrupt is a real submit targeting it via the HTML5 `form`
  attribute (nested forms are invalid HTML). Unset keeps the legacy button.

Structural note: the root element changed `<form>` → `<div class="mui-composer">` with the
form inside as `__form`. All styling keys off classes, so consumer CSS is unaffected unless
it matched on the `form` element itself.

---

## [0.6.1] — 2026-07-27 — composer contrast fix

Two composer text rules shipped as alpha `color-mix()` washes that composite below AA on the
library's own backgrounds: `__status` (`--mui-text-muted` at 42% ≈ 2.5:1) and `__state-tag`
(`--mui-text` at 34% ≈ 2.7:1). Both now use solid `--mui-text-subtle` (5.91:1 on `--mui-bg`,
the documented contrast floor). Found by alpha-aware compositing during the Conductor
session-view build — canvas-readback probes that drop the fg alpha report the base token and
false-pass exactly this pattern.

---

## [0.6.0] — 2026-07-27 — the agent session-view kit

Seven new primitives, a `Session` tier, and two extended components — the surface a Conductor
session view is built from. The component count moves 72 → 79. No new tokens: the warm palette in
the design sheets is a consumer theme, and every colour here maps to an existing `--mui-*` token
or a `color-mix()` over one, so `css_token_integrity` stays green.

**This is 0.6.0 rather than 0.5.1 because of one additive-but-breaking signature.** `badge::Props`
gained four public fields (`chip`, `trailing_count`, `kbd`, plus the previously-undocumented
`mono`); a consumer who constructs `Props { .. }` WITHOUT `..Default::default()` will no longer
compile. For a `0.x` crate Cargo treats the MINOR as the breaking position, so anyone on
`maud-ui = "0.5"` is not auto-upgraded into that error.

### Added

- **`composer`** — the multi-state prompt dock: Ready / Growing / Executing / Asleep. Server-rendered
  as a plain `<form>` + `<textarea rows="2">` (works JS-off); auto-grow, `⌘↵` and voice are an additive
  JS layer. Geometry knobs `--mui-composer-max` / `--mui-composer-max-h`.
- **`segmented_control`** — a boxed switch rendered as real `<a>` links (`aria-current="page"`), so it
  switches views with JavaScript disabled.
- **`status_dot`** — a 6px semantic dot (success / accent / warning / neutral / destructive) with a
  hollow "observing" variant.
- **`attention_pill`** — an accent-tinted "wants a look" pill with a muted zero-state that spends no
  accent at all.
- **`turn_progress`** — a 2px hard two-stop gradient strip; fill carried by an inline `--mui-turn-pct`.
- **`facts_list`** — mono subtle labels + foreground values as a semantic `<dl>`, on the 4px rhythm.
- **`gutter_section`** — a mono uppercase section header over a content slot; the inspector-gutter unit.
- **`time` module** — `relative_time` (`41s`, `2m 08s`, `4h`, `1d`), `duration_long` (`4h 12m`), and
  `asleep_label` (`asleep 4h 12m`), unit-tested at the 59s/60s, 59m/60m, 23h/24h boundaries.
- **`separator::render_labelled`** — a mono letter-spaced label + hairline rule, for transcript turn
  dividers.

### Breaking

- **`badge::Props` gained `chip`, `trailing_count`, `kbd`, `mono`.** The hollow chip is the taller
  (26px) bordered-transparent counterpart to the filled pill, with an optional trailing count
  (`mail 2`) or kbd hint (`⌘K`). Migration is mechanical — add `..Default::default()` to any explicit
  `Props { .. }` literal.

---

## [0.5.0] — 2026-07-27 — the slider actually slides, and CSS is highlighted

Two of these are real consumer-facing bugs that a render test can never catch: both live in the
progressive-enhancement layer, where the server markup and the behaviour script are separate
implementations that can silently disagree.

**This is 0.5.0 rather than 0.4.2 because of one breaking signature.** For a `0.x` crate Cargo
treats the MINOR as the breaking position: `0.4.1` and `0.4.2` are compatible, so anyone
depending on `maud-ui = "0.4"` would have been auto-upgraded into a compile error.

### Breaking

- **`alert_dialog::trigger`'s `variant` is now `ButtonVariant`, not `&str`.** It matches
  `alert_dialog::action` in the same module, and a typo like `"dangerous"` previously rendered an
  unstyled button with no error at all. Migration is mechanical:

  ```rust
  // before
  trigger("confirm-delete", "Delete account", "danger")
  // after
  trigger("confirm-delete", "Delete account", ButtonVariant::Danger)
  ```

### Fixed

- **Vertical sliders animated the wrong axis, and range sliders had one live thumb.** The
  renderer was never at fault — it already emitted `data-orientation`, `data-values` and a
  `data-thumb-index` per thumb, so the initial paint was always correct. The behaviour script
  read none of it: drag maths was `clientX`/`rect.width` unconditionally and it wrote
  `thumb.style.left` + `fill.style.width`. On a vertical slider the CSS fills bottom-up via
  `height`, so dragging grew the track sideways instead of moving the thumb. Separately,
  `querySelector` (singular) bound only the first thumb, leaving a range slider's second thumb
  inert while the fill used single-thumb maths (`0..value`) instead of spanning `lo..hi`. The
  behaviour is now per-thumb and orientation-aware throughout.

- **A slider whose `min` was not a multiple of `step` could never reach its own minimum.**
  Snapping was `round(v / step) * step`, which places the stops on multiples of *step* rather
  than of *min* — so `min=5 step=10` snapped to 0 or 10 and 5 was unreachable. Now snaps relative
  to `min`.

- **Sliders did not work under touch at all.** The behaviour bound mouse events only. It now uses
  pointer events with pointer capture, so touch and pen work and a drag survives the pointer
  leaving the track. Thumbs also clamp against their neighbours instead of crossing, a track
  click moves the nearest thumb, and the native input now emits `input`/`change` so forms, htmx
  bindings and validation actually observe a value change.

- **Five `var(--mui-*)` references pointed at tokens that do not exist** — `--mui-mono`,
  a bare `--mui-radius`, `--mui-bg-hover`, `--mui-spacing-xxl` — across `docs`, `drawer`,
  `sheet` and `sidebar`. Each silently fell back to the property's initial value. `--mui-sidebar-w`
  is now declared in the token block rather than living only as an inline fallback, and
  `tests/css_token_integrity.rs` guards the invariant.

### Added

- **CSS syntax highlighting in `code_block`.** `css` previously fell through to the unhighlighted
  branch, so every CSS snippet rendered as flat grey text — a poor advertisement for a library
  whose entire argument is its custom properties. The tokenizer tracks two pieces of context
  because both are needed to tag an identifier: a stack of "is this a declaration block?" (the
  body of `@media` holds selectors, not properties) and an in-value flag set by `:` only inside a
  declaration block (so `a:hover` stays a selector). Custom properties get their own token kind,
  and `#` resolves as a hex colour in a value and an id selector outside one.

- **Unsupported languages in `code_block` are no longer silent.** The source still renders —
  losing code is worse than losing colour — but wrapped in
  `<span data-mui-highlight="unsupported" data-language="…">`. Flat grey code beside coloured code
  reads as a broken highlighter rather than an uncovered language, and that ambiguity cost a real
  misdiagnosis.

- **Brand assets** — `assets/favicon.svg`, `assets/og.png`, `assets/apple-touch-icon.png`, plus
  `bun run build:og` to regenerate the raster pair. `assets/**/*` joins `Cargo.toml`'s `include`
  because `examples/showcase.rs` now `include_bytes!`s them, so a published crate without them
  could not build its own example.

- **A landing page at `/`; the component gallery moved to `/gallery`.** Gallery-only change — no
  consumer API is affected. Documented in the README's new route table, with `docs/brand.md` for
  the mark.

---

## [0.4.1] — 2026-07-27 — design corrections, and the first crates.io release

0.4.0 was tagged but never published. This is the first release on crates.io, and it carries the
design fixes made after that tag — so the published crate matches its own tag rather than
shipping 0.4.0's bytes under 0.4.0's name plus four commits of drift.

### Fixed

- **`date_picker` — the calendar icon sat glued to the value text**, drifting horizontally with
  the value's length while the right half of the field stayed empty. The trigger carries both
  `mui-date-picker__trigger` and `mui-input`; both set `display` at equal specificity (0,1,0), so
  source order decided it, and `input.css` is `@import`ed after `date_picker.css`. `display: block`
  won and silently disabled `justify-content`, `align-items` and `gap` — all inert on a block box.
  Fixed with a compound `(0,2,0)` selector that wins regardless of import order.

- **`tool_call` was carrying four visual tells at once.** A 3px per-kind coloured `border-left`
  (the most recognisable marker of generated UI, and it parked the colour on the opposite side of
  the row from what it described — the kind hue now lives on the glyph via `--mui-tool-accent`);
  emoji glyphs, which ignore `color` and so rendered in full OS colour inside a monochrome
  inspector and could not carry the hue at all (now text-presentation marks); UPPERCASE
  letter-spaced tool names (a tool name is an identifier — now monospace, sentence case); and
  bordered status pills, an outline inside a bordered row inside a bordered card. Also fixed a
  real bug: collapsed rows painted an empty ~19px strip, because `hidden` is a UA-stylesheet
  `display: none` and the author-level `display: flex` beat it.

- **Focus rings were neutral grey and chunky.** `.mui-input:focus-visible` drew two hard
  concentric bands — `0 0 0 2px var(--mui-bg), 0 0 0 4px var(--mui-border-focus)` — in an opaque
  grey that reads as an unstyled browser default. `--mui-border-focus` turned out to feed every
  focus outline in the library (35 files), so retinting it once corrects all of them: `#60a5fa`
  dark, `#2563eb` light. The input's own ring is now a single 3px 25%-alpha glow.

- **Five infinite animations had no `prefers-reduced-motion` fallback.** Fixed by kind rather than
  by blanket suppression: spinners and indeterminate progress are **slowed** (freezing a spinner
  makes it lie about state; a stopped indeterminate bar reads as a determinate bar stuck at that
  width), while the skeleton shimmer and message liveness pulse are **stopped**. Every infinite
  animation in the library now has a reduced-motion path.

### Added

- `scripts/gallery.sh` (`bun run gallery`) — starts the showcase on the first free port, naming
  whatever holds a busy one, builds in the foreground so compile errors surface as compile errors,
  and waits on a real route rather than the socket.
- Two more parity guards: the static export's component list must be derived from
  `COMPONENT_NAMES` rather than restated, and `public/css/maud-ui.css` must match
  `dist/maud-ui.css` — the two are regenerated by different commands, and shipping one without the
  other produced a repo that was simultaneously correct and stale.

## [0.4.0] — 2026-07-26 — the containers, and the five that were already here

64 → 72 primitives. Three of the eight are new layout containers; **five were finished months ago
and registered nowhere**, so no consumer could reach them. Both halves trace to the same root
cause, now fixed: adding a primitive meant updating five separate lists by hand and nothing ever
checked that you had.

### Added — registration parity is now enforced

- **`tests/registration_parity.rs`** — ten checks that fail the build when the five-point lockstep
  drifts: a module registered nowhere, a `TIERS` slug with no component behind it, a slug that
  falls through to the 404 page, a missing docs arm or doc file, a stylesheet that exists but is
  never `@import`ed, a component absent from the render-test macro, and the hardcoded component
  counts in `Cargo.toml`/`README.md` disagreeing with `COMPONENT_NAMES.len()`.

  This invariant had already drifted **three** separate ways, each silently, none catchable by
  `cargo build` — they are cross-file agreements, not type errors. The test was written first and
  watched to fail on all five orphans before they were registered.

- **`showcase::tier_slugs()` / `showcase::tiers()`** — the tier groupings, public so consumers can
  build their own tiered navigation and so the parity test can check `TIERS` without parsing source.

### Added — the Conversation tier

Five primitives that shipped complete, with their own CSS, and were never registered:
**`message`**, **`streaming_cursor`**, **`code_block`** (954 lines, with a real syntax highlighter
for Rust/Bash/TS-JS/JSON), **`diff`**, and **`tool_call`**. They now have a tier of their own rather
than being scattered through Display — they are an AI-chat/agent surface kit, and grouping them
makes that capability legible.

Writing their API docs surfaced three genuine defects, all fixed here before the components became
publicly reachable for the first time:

- **`diff` was inaccessible in the one way that mattered.** Add/remove was conveyed by row tint plus
  a `+`/`-` sigil that was marked `aria-hidden` — so a screen reader got the line text and no way to
  tell an addition from a deletion. Now emits a visually-hidden `"Added: "` / `"Removed: "` inside
  the row's `role="cell"` span (outside a cell it would not be reliably announced). Context lines
  stay silent by design.
- **`streaming_cursor` ignored `prefers-reduced-motion`.** Three infinite animations ran
  unconditionally. Each now degrades to its static but still **visible** state — a solid caret,
  three solid dots, a solid status dot — so the indicator keeps indicating and stops moving.
- **`code_block::Props::show_copy` defaulted to `false` while documented as "default true".** The
  derived `Default` gave `bool::default()`, so every `..Default::default()` silently dropped the
  copy button. Now a hand-written `impl Default`.

### Added — layout primitives

- **`stack`** (64 → 65 primitives) — the general layout container, and the crate's **first** one.
  Before this there was no stack, box, flex, grid, row, or column of any kind: every block in
  `src/blocks/**` hand-wrote its own `div` with an inline `style="display:flex;…"`, and nothing
  could be composed as a tree of containers holding leaves. `src/primitives/stack.rs`,
  `css/components/stack.css`.

  Every appearance prop is a **closed enum**, not a free-form string, so tooling can enumerate the
  legal values — `Direction { Vertical, Horizontal }`, `Space { None, Xs, Sm, Md, Lg, Xl, Xxl }`
  (shared by `gap` and `padding`), `Align { Stretch, Start, Center, End, Baseline }`,
  `Justify { Start, Center, End, Between, Around, Evenly }`, and
  `Tag { Div, Section, Article, Aside, Nav, Header, Footer, Main }`. Helpers `stack::vertical` and
  `stack::horizontal` cover the two common cases without the `Props` ceremony.

  Two deliberate omissions, both accessibility calls: **no reversed directions** (`row-reverse`
  decouples visual order from DOM order, sending keyboard focus and screen-reader output somewhere
  other than what a sighted user sees), and **`Tag::Section` requires `aria_label`**, enforced by a
  `debug_assert!` — an unnamed `<section>` is stripped of its `region` role, so the grouping the tag
  was chosen for silently does not exist for screen-reader users.

- **`--mui-space-*` custom properties** (`xs`…`xxl`) in `css/maud-ui.css`. `tokens.rs` documented its
  `spacing` constants as mirroring the CSS custom properties, but no spacing properties existed —
  the scale was hardcoded per component. Now declared once, theme-independent, and covered by a test
  asserting `Space::as_length()` matches `tokens::spacing`.

- **`grid`** — the two-dimensional container. Shares `stack`'s vocabulary rather than duplicating
  it: `gap` is the same `Space` and `align` the same `Align`, re-exported, so there is one scale to
  learn and one for tooling to describe. `src/primitives/grid.rs`, `css/components/grid.css`.

  `Columns { AutoFit, One…Six, Twelve }` and `MinColumn { Sm, Md, Lg, Xl }`. The default is
  `AutoFit` — `repeat(auto-fit, minmax(…, 1fr))` — which reflows against its **container**, not the
  viewport, so a grid inside a sidebar behaves correctly without knowing it is in a sidebar.

  `collapse_narrow` defaults to **true**: fixed column counts become a single column below `40rem`.
  Four columns on a phone is four unreadable slivers, and the crate offers no class escape hatch a
  consumer could use to fix that themselves, so the responsive behaviour has to live in the
  primitive. Opt out for genuinely small cells (a colour palette, a keypad).

- **`form`** (65 → 67 primitives with `grid`) — the `<form>` element. Nothing emitted one before:
  every form in `src/blocks/**` hand-wrote the tag, so the submission contract was retyped per site.
  `src/primitives/form.rs`, `css/components/form.css`.

  `Method { Get, Post, Dialog }` and `Enctype { UrlEncoded, Multipart, TextPlain }`.
  **`method` defaults to `Post`, not HTML's `GET`** — the one knowing divergence from the platform.
  `GET` serialises every field into the URL, where it lands in browser history, access logs, and the
  `Referer` header sent to third parties; a login form whose author forgot to set `method` should
  not leak the password that way.

  Deliberately **semantic only** — no layout. A form is a column of fields and `stack` already is
  that column; `form::stacked(action, children)` pairs them in one call.

### Changed

- The showcase hero's component count is **derived from `COMPONENT_NAMES.len()`** instead of typed.
  It had drifted to a stale hardcoded `64` while the site header three sections away already
  computed the same number.
- `docs/components/stack.md` now warns that `Align::Stretch` (the default, matching flexbox) turns
  an intrinsically-sized child such as a badge into a full-width bar. Found by looking at the
  rendered grid showcase, not by reading the code.

## [0.3.0] — 2026-07-25 — primitives earned by a real migration

Grown to serve an actual surface port (an internal dashboard, askama → maud + maud-ui), not speculatively.
Every item below existed because a real page needed it and could not be built without it.

### Added

- **`typography::eyebrow(text)`** — uppercase, letter-spaced section heading. `src/primitives/typography.rs`.
- **`typography::prose(children)`** — wrapper for server-rendered markdown blobs. **Demotes incoming
  heading levels**, so an untrusted HTML blob cannot inject page-level `h1`s. The donor page rendered
  **189 `h1` elements** from markdown bodies; after this, 1.
- **`item::status_dot(tone)`** + `Tone { Ok, Warn, Down, Unknown }` — health indicator for list rows.

### Changed

- **`badge`** — new `mono: bool` prop (mono face for shas/ids/counts), and hue expansion with
  `Info`, `Accent`, `Violet`, `Rose` plus matching `--mui-*` tokens. **Every hue/background pair was
  recomputed to clear WCAG AA 4.5:1 and verified by browser pixel sampling** (tightest: light
  warning at 4.527:1). The donor surface's own badges failed at **4.04:1 across 73 nodes** — those
  values were deliberately *not* ported.
- **`stats::StatCard`** — new `value_id: Option<String>`, rendering a stable DOM id on the value node.
  **Required by any live-patched dashboard.** Without it an SSE/websocket client has no element to
  address: the page renders correctly, the numbers freeze permanently, and nothing errors. No build
  check catches this.
- **`table`** — new `hide_cols_sm: Vec<usize>` for responsive column drop via `data-hide-sm`.
- **`collapsible`** — new `native: bool` rendering a real `<details>/<summary>`, so disclosure works
  with JavaScript disabled.

### Verified

- **Z-index audit against a host app with sticky chrome.** maud-ui's maximum is `toast` at **90**,
  deliberately below a host header at 100 (rationale recorded inline at `css/components/toast.css:11`).
  Any overlay at ≥100 paints over a sticky app header. Re-audit when adding overlay components.
- Consuming surface reached **0 axe violations** (WCAG 2.2 A+AA) using only these primitives.

### Note for consumers

maud-ui declares its tokens under `:root, [data-theme="dark"]` / `[data-theme="light"]` and resets
`body`. If your host app already ships a palette on those same selectors, **strip maud-ui's token
block and body reset and bridge `--mui-*` onto your own tokens** — otherwise the later sheet
silently replaces the host palette. See the consuming app's `vendor-maud-ui.sh` for a working
subsetting script (it ships 27 KB of the 192 KB bundle by including only the components in use).

---

## [2026-04-21] — Conductor-flavoured primitives

### Added

Five new primitives modelled after Conductor UI's most distinctive components, shipped so any maud consumer can rebuild a Claude-Code-style agent surface without reinventing them:

- **`message`** — chat bubble with `Role` variants (`User` / `Assistant` / `System`), avatar + initials + optional colour, timestamp, `is_live` flag that pulses the avatar ring during streaming, and a footer slot for inline tool chips. File: `src/primitives/message.rs`, styles in `css/components/message.css`.
- **`code_block`** — mono pre-formatted code with optional header (language + filename + copy button). Accepts raw `code: String` or `pre_rendered: Option<String>` for syntax-highlighted HTML. Optional `max_height` enables vertical scroll. Inline copy behaviour via `navigator.clipboard.writeText`.
- **`tool_call`** — collapsible agent-tool invocation with `Kind` (Edit / Write / Read / Bash / Grep / Glob / Task / Agent / Search / Custom) and `Status` (Success / Running / Error / Pending). Colour-coded left border per kind, animated "running" status pill, args + result panes in the expanded body.
- **`diff`** — unified diff viewer with `LineKind` (Context / Add / Remove / Hunk), optional line numbers, ± counts auto-computed from the `lines` vector. Hunk rows render without sigil/line-numbers; context/add/remove rows show full layout with colour-coded backgrounds.
- **`streaming_cursor`** — three variants of "work-in-progress" indicator: `Cursor` (blinking block cursor, inline with text), `Dots` (three-dot pulse, "thinking" indicator), `Pulse` (ring pulse, status beacon). Optional label slot.

All five follow the existing Props/Variant convention, include `Default` impls and `showcase()` functions, and are registered in `src/primitives/mod.rs`. Five matching stylesheets added under `css/components/` and `@import`ed from the master `css/maud-ui.css`. `dist/maud-ui.css` and `dist/maud-ui.min.css` rebuilt via `bun run build` — bundle grew ~14KB unminified.

### Rationale

Came out of decomposing an internal React Router 7 agent-console package into reusable primitives. A companion storybook app demonstrates all five in realistic fixtures, plus five more derived components composed from existing primitives (thinking block, working indicator, session list, composer, conductor header, full-layout).

---

## [Unreleased] — gallery polish after v0.2.1 publish

These commits land in the repo but do **not** change the published
`maud-ui` crate — they only affect the live gallery at
`https://maudui.herman.engineer/`. Consumers on crates.io are unaffected.

### Added
- Five new primitives (Sheet / Sidebar / Sonner / Item / Direction)
  registered in the showcase gallery — new `TIERS` entries, palette
  entries, match arms, and static-export slugs. Live at
  `maudui.herman.engineer/{sheet,sidebar,sonner,item,direction}`.
- `/sidebar` showcase got a meaningful demo: leading icons on every
  menu item (so icon-collapsed mode renders icons, not empty squares),
  a `.mui-sidebar-showcase` wrapper that caps height at 480px so the
  demo sits inside the gallery card instead of filling `100dvh`, and
  a 3-tile stats panel in the inset body to represent real app
  content rather than placeholder text.
- Component count bumped 58 → 64 in banner text and
  `COMPONENT_NAMES.len()` drives the nav badge dynamically.
- **API reference docs**: one Markdown doc per primitive at
  `docs/components/<name>.md` — 64 files, 1:1 with the primitive
  modules. Each doc includes Import, Example, Props table (every
  pub field with type/default/intent), Variants/Enums, Helper
  Functions, Accessibility notes, Related primitives, and a
  shadcn reference URL. Index at `docs/components/README.md`
  groups by category (Form / Display / Layout / Overlay /
  Navigation / Interaction / Visualisation).

### Fixed
- `/sheet` "Open sheet" button did nothing — Sheet's trigger() emitted
  `data-mui="sheet-trigger"` for which no behaviour was registered.
  Fixed by emitting `data-mui="dialog-trigger"` (target is a native
  `<dialog>`, so the existing dialog-trigger handler works).
- `/sidebar` Cmd/Ctrl+B and trigger-click did nothing — `dist/behaviors/sidebar.js`
  existed on disk but `dist/maud-ui.min.js` had been bundled before the
  file was added. Fixed by rebuilding the bundle (`bun run build`).
- `/sonner` position picker + "fire toast" button did nothing — the
  sonner-toast CustomEvent bridge lived in `dist/behaviors/toast.js`
  but hadn't been rebuilt into the bundle either. Same fix.
- `/sidebar` showcase had a visible 4px jog where the sidebar header's
  bottom border met the inset bar's bottom border. Root cause: both
  used `min-height: 3rem` but content-box plus `border-bottom: 1px`
  pushed each row past the min in different ways. Fixed by pinning
  both to `height: 3rem; box-sizing: border-box; flex-shrink: 0`
  so the border sits inside the 48px height box on both rows.

### Added (docs rendering — committed but NOT live yet)
- **Inline API reference on every component page.** Each primitive
  page at `/button`, `/sheet`, `/sidebar`, etc now renders the full
  `docs/components/<name>.md` content as styled HTML below the
  live showcase. New `src/showcase/docs.rs` uses `pulldown-cmark`
  to parse the Markdown with tables + strikethrough enabled, wraps
  the output in a `<section class="mui-docs">`, and returns `None`
  when no doc file exists. The docs ship with the crate via the
  Cargo.toml `include` list.
- New `css/components/docs.css` styles the rendered markdown —
  headings, tables, code blocks, blockquotes, lists, links — using
  `--mui-*` tokens so it respects dark/light themes.
- `pulldown-cmark 0.10` added as a dependency (default-features
  off, HTML-only; minimal transitive deps).

### Platform note — earlier deploy block (resolved)
- Between 11:30 and ~12:30 UTC on 2026-04-19 the Connect App Pipeline's
  pre-flight clone check returned empty-stderr failures for every app,
  rooted in an upstream API redeploy at 11:29:54Z. The platform self-
  recovered (or a fix shipped upstream) around ~12:45Z and a single
  retry pushed this commit live. Docs sections now render on every
  `/<primitive>` route.

## [0.2.1] — 2026-04-19

### API parity with shadcn Base UI

A broad alignment pass against the 58 shadcn Base UI components at
`https://ui.shadcn.com/docs/components/base/*`. Additive throughout —
every change preserves v0.2.0 call sites.

### Added — 5 new primitives (59 → 64)

- **`primitives::sheet`** — slide-out panel with `Side { Top, Right, Bottom, Left }`.
  Reuses native `<dialog>` semantics; `show_close_button` opt-out.
- **`primitives::sidebar`** — collapsible app-shell with 20 subcomponent helpers
  (provider, header, content, footer, group, group_label, group_action, menu,
  menu_item, menu_button, menu_action, menu_sub, menu_sub_item, menu_badge,
  menu_skeleton, trigger, rail, inset). `Side`, `SidebarVariant { Sidebar,
  Floating, Inset }`, `Collapsible { Offcanvas, Icon, None }`. Global `Cmd/Ctrl+B`
  toggle hotkey.
- **`primitives::sonner`** — positioned toast viewport with `Position` enum
  covering all 6 corners. Re-exports `toast::Props/Variant/render` for symmetry.
  Shared `buildToastNode` refactor in `dist/behaviors/toast.js` bridges both
  primitives.
- **`primitives::item`** — composable list-row with media + content + actions
  slots. `Variant { Default, Outline, Muted }`, `Size { Default, Sm, Xs }`,
  `MediaVariant { Default, Icon, Image }`. Nine helper fns.
- **`primitives::direction`** — tiny `<div dir="ltr|rtl">` provider for RTL
  cascades. No JS.

### Added — 20+ primitives enhanced

- **alert**: `action: Option<Markup>` + `action()` helper (CSS grid top-right slot).
- **avatar**: `badge()`, `group()`, `group_count(n)` helpers.
- **badge**: `Variant::Ghost` + `Variant::Link`; `href: Option<String>`;
  `leading_icon: Option<Markup>` + `data-icon="inline-start"`.
- **button**: `Size::Xs`, `Size::IconXs`, `Size::IconSm`, `Size::IconLg`;
  `trailing_icon: Option<Markup>`; `debug_assert!` catches icon-only buttons
  without `aria_label`.
- **button_group**: `separator()` + `text()` helpers.
- **card**: `size: Size { Default, Sm }`; `action: Option<Markup>` +
  `action()`/`content()`/`footer()` standalone helpers; header switched
  to 2-col grid.
- **checkbox**: `aria_invalid: bool`.
- **combobox**: `multiple`, `auto_highlight`, `show_clear`, `aria_invalid`;
  new `ComboboxGroup { label, options }` + `groups: Vec<ComboboxGroup>`.
- **command**: `shortcut()`, `separator()`, `empty(text)` helpers.
- **dialog**: `show_close_button: bool` (default true), `size: Size`,
  `aria-modal="true"`.
- **alert_dialog**: `media: Option<Markup>` + `media()` helper;
  `size: Size`; `action(label, variant)` + `cancel(label)` helpers.
- **drawer**: `should_scale_background: bool`, `show_close_button: bool`.
- **hover_card**: `side: Placement`, `align: Align`.
- **input**: `aria_describedby: Option<String>`, `InputType::File` variant.
- **popover**: `side: Side` (4-way), `side_offset`, `open: Option<bool>`,
  `header()`/`title()`/`description()` helpers.
- **menu + menubar**: `MenuEntry::CheckboxItem`, `RadioGroup`, `RadioItem`,
  `Sub`, `Group` — full dropdown-menu surface. Destructive items emit
  `data-variant="destructive"`.
- **empty_state**: `compose()`, `header()`, `media(children, variant)`,
  `title()`, `description()`, `content()`. `MediaVariant { Default, Icon }`.
- **field**: `orientation: Orientation { Vertical, Horizontal, Responsive }`;
  `errors: Vec<String>` multi-error; 9 subcomponent helpers (label, description,
  error, group, legend, fieldset, content, separator, title).
- **input_group**: `Align { InlineStart, InlineEnd, BlockStart, BlockEnd }`;
  `addon()`, `button()`, `text()`, `input_el()`, `textarea()` helpers.
- **select**: `size: Size`, `aria_invalid: bool`; `SelectGroup` + `groups`;
  `scroll_up_button()`, `scroll_down_button()`, `separator()` helpers.
- **tabs**: `orientation`, `variant: { Default, Line }`, `activation_mode`,
  per-tab `disabled`.
- **slider**: multi-thumb via `values: Vec<f64>`; `orientation: Orientation`.
- **typography**: `list_ul()`, `list_ol()`, `large()`, `small()`, `table()`.
- **switch**: `size: Size`, `aria_invalid`, `required`.
- **tooltip**: `align: Align`, `side_offset`.
- **toggle**: `Size::Lg`.
- **pagination**: `href_pattern` anchor mode, `icons_only`, explicit
  `aria-label="Go to previous/next page"`, ellipsis `aria-hidden="true"`.
- **toast**: `action(label, onclick)` helper + shadcn-deprecated doc note.
- **textarea**: `aria_invalid: bool`.
- **label**: required indicator switched from `aria-hidden` to
  `aria-label="required"` (actually announced by SRs).
- **radio_group**: `required: bool`, `variant: { Default, Comfortable, Compact }`.
- **input_otp**: `pattern: OtpPattern { Digits, DigitsAndChars, Custom(String) }`,
  `aria_invalid: bool`.
- **navigation_menu**: `orientation`, `viewport: bool`, `indicator()` helper,
  `aria-haspopup="true"` on items with submenus.
- **progress**: `label(text)`, `value(val)` helpers.
- **data_table**: `column_header(label, sortable)`, `view_options(columns)`,
  `selectable: bool`.
- **date_picker**: `Mode { Single, Range }`, `format: Option<String>`.
- **chart**: `ChartConfigEntry` + `ChartConfig`, `accessibility_layer: bool`.
- **context_menu**: `destructive_item()` helper, `data-side="inline-end"`
  for RTL.

### Added — context_menu, command, drawer, menubar a11y

- `aria-modal="true"` on command + drawer dialogs.
- `aria-orientation` wired on button_group (vertical), context_menu (vertical),
  menubar (horizontal).
- `menu` content div back-references trigger via `aria-labelledby`.

### Fixed

- **SECURITY**: `primitives::input::render()` no longer builds HTML via
  `format!` + `PreEscaped`. All attribute values now auto-escape through
  maud's `html!` macro — a caller passing user-controlled text via
  `value`/`name`/`placeholder`/`id` could previously inject a live
  `<script>` tag. Three unit tests lock this in:
  `render_escapes_attribute_values`, `aria_describedby_emitted_only_when_some`,
  `file_variant_renders_type_file`.
- **accordion**: new optional `aria_label: Option<String>` Prop for
  top-level SR context.
- **avatar**: double-announcement bug — when `src: Some`, the outer
  `<span>` no longer carries `role="img"` + `aria-label={alt}`; the
  native `<img alt="…">` takes over as the accessible name. Previously
  SRs announced the name twice.
- **breadcrumb**: current-page item wraps in
  `<span role="link" aria-disabled="true" aria-current="page">` for
  consistent link-like semantics.
- **carousel**: `aria-live` flips to `"polite"` when `auto_play=true`
  so SR users hear slide changes.
- **radio_group**: dropped `role="radiogroup"` from `<fieldset>`
  (role-on-fieldset anti-pattern); native fieldset+legend carries
  the grouping semantics.

### Internal

- `ComboboxGroup`, `RadioItem`, `SelectGroup`, `ChartConfigEntry`,
  `ChartConfig` all newly public.
- CSS additions live under `css/components/<name>.css`, imported
  alphabetically from `css/maud-ui.css`.
- `Props` in most primitives gained `Default` derives (manual where
  a non-zero default was needed, e.g. `show_close_button: true`).
  Existing struct-literal callers updated throughout the repo and
  blocks to use `..Default::default()` spread for forward-compat.

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
