# States and motion — 0.10.1

The operational kit now shares a presentation-state prop. The gallery has a Worklist and Record tab, and every component and block reference follows Anatomy, Props, Examples and Accessibility. Props tables come from public Rust fields and actual defaults.

## Operational states

`maud_ui::blocks::state::State` applies to all 13 operational blocks: worklist header/grouped, record header/timeline/money/related card, task grid, attention banner, action row, sidebar shell, page header, app header and app footer. Each API page shows loading, empty, error and disabled together; the grid becomes two columns below 64rem and one at 40rem.

```rust
use maud_ui::blocks::{action::Action, record::money, state::State};

let panel = money::render(money::Props {
    state: State::Error {
        message: "Could not load payments.".into(),
        retry: Some(Action::link("Try again", "/bookings/14/payments")),
    },
    ..Default::default()
});
```

| State | Input | Output |
|---|---|---|
| `Ready` (default) | Existing block props | Existing content and actions. |
| `Loading` | `message: String` | Skeleton lines, `aria-busy`, polite status. |
| `Empty` | `message: String`, `action: Option<Action>` | Quiet message and optional recovery action. |
| `Error` | `message: String`, `retry: Option<Action>` | Error tone, alert text, optional native retry action. |
| `Disabled` | `reason: String` | Visible reason and admitted content under `inert` and `aria-disabled`. |

`State::loading()`, `empty(message)`, `error(message)` and `disabled(reason)` are convenience constructors. Supply enum fields for translated loading copy or recovery actions. `as_str()` returns the lowercase state name.

Loading, empty and error skip the ready renderer entirely; they do not render its values or write actions. Disabled retains original field/action fragments, form owners, CSRF values and IDs. Inert disables interaction and hides that subtree from assistive technology; the reason stays outside it. It does not remove form values from an enclosing form submission or enforce authorization. Use admitted data and server authorization as before. For a readable view-only record, use Ready with authorized actions omitted.

**Source migration:** all 13 Props structs gain `state`, default Ready. Add `state: Default::default()` to exhaustive literals or use `..Default::default()`. Timeline milestones keep their separate `record::timeline::State::{Done, Current, Scheduled}`; alias the shared presentation state if importing both.

## Compact record layouts

- Timeline stays horizontal at a viewport of at least 48rem, even in a narrow desktop column. Below 48rem the same ordered list becomes vertical. Its frame uses content height instead of stretching beside taller panels. Dates and milestone status remain supplied by the caller.
- `related_card::Variant::Auto` (default) uses Inline when the effective facts are empty and Card otherwise. `Card` and `Inline` force a presentation. An explicitly empty `facts_markup` override counts as empty. Inline keeps title, subtitle and one action in a wrapping row.
- `empty_state::Variant::Inline` renders the title as a quiet paragraph followed by an optional action. It deliberately omits icon, description and heading. Use it inside an existing Payments or Notes section. Add an Inline arm to exhaustive variant matches.
- Page-header breadcrumbs retain the full root and current labels. Only middle links ellipsize, with the full label in their title attribute. Phone headers wrap rather than discard ancestors; shell search and controls still move into the existing drawer.
- Action-row overflow defaults to the small outline **More ▾** disclosure: 32px with a 44px coarse-pointer target. Record headers use that same action row. `overflow_label` remains caller controlled.

The landing Record tab is a fictional manager example with explicitly supplied CAD amounts. Its arrival-only Info banner needs no balance. In the receptionist app projection, balance is not admitted: retain that omission, or select an arrival-only rule using admitted arrival data. Actual app rules, grouped home wiring, duplicate overview removal and admission of booked timestamps belong to the consuming application.

## Dark palette source review

Record cards, timeline, money, grouped worklists, task cards, shell chrome, chips, bottom tabs and banners use shared surface/ink tokens. Dark surfaces step from page to card to input; structural edges stay quieter than controls and text. Dialog surfaces now use the same card/background/border roles. Code labels use accent text rather than the accent fill.

The default light and dark colors are unchanged. Six gallery presets had control boundaries as bright as labels; the new values preserve at least 3:1 contrast on page/card/input and the operational tints. The high-contrast preset retains stronger borders than other presets, below its text luminance.

| Preset | Control border before → after | Other changes |
|---|---|---|
| Slate | `#94a3b8` → `#7c8da5` | — |
| Zinc/violet | `#a1a1aa` → `#85858f` | — |
| Stone/amber | `#a8a29e` → `#89807b` | — |
| Emerald | `#6ee7b7` → `#40b98b` | — |
| Rose | `#fda4af` → `#d47987` | — |
| High contrast | `#ffffff` → `#a3a3a3` | Structural/hover border `#ffffff` → `#888888`. |

`node tests/curation-contrast.mjs` checks text, control boundaries, focus rings on neutral and semantic surfaces, filled tones, dark surface ordering and border luminance across all eight presets. This is token arithmetic; font rendering, layout and focus clipping still require visual review.

## Motion and type tokens

| Token | Value / change | Used for |
|---|---|---|
| `--mui-motion-fast` | 120ms, unchanged | Hover/pressed feedback and toast exit. |
| `--mui-motion-enter` | 180ms, unchanged | Shared dialog/sheet/drawer entrance, backdrop and toast entrance. |
| `--mui-motion-loading` | 1200ms → `calc(var(--mui-motion-enter) * 6)` | Compatibility alias for repeating indicators. |
| `--mui-text-input-touch-size` | New: 1rem | Existing 16px touch-input floor, now named. |

All durations derive from the two motion tokens. The surface entrance respects the sheet/drawer side and RTL. Skeletons shimmer with a six-times-enter cycle. Reduced motion removes animation and transitions, retains a static skeleton and removes dismissed toasts immediately. Normal toast dismissal follows the computed CSS exit duration and animation completion, with a timer fallback. Toast dwell time is caller-controlled content timing, separate from motion.

Seven `data-mui-type` roles (display, h1, h2, h3, body, small, caption) replace 223 local font-size declarations. Headings h5/h6 use the body role. Third-party widget numeric sizes come from a computed token bridge; no fixed font sizes remain in active source outside tokens. The touch-input size and existing compact chip/count tokens are deliberate control exceptions. The migration ledger is [night-2-typography-audit.json](night-2-typography-audit.json).

## Generated reference and progressive enhancement

Run `python3 examples/generate-props.py` after changing a public rendering Props struct or its field documentation. It reads the 105 registered APIs, follows forwarded types, and emits `src/showcase/generated_props.rs`. Rust evaluates the actual Default values when rendering. APIs without a struct Default are explicitly marked; the typography helper API is explicitly identified. `--check` detects stale generated source. Handwritten Markdown still explains variants, helpers and usage; its duplicate main Props table is omitted from live gallery pages.

Run `cargo run --example build_docs` after Markdown edits and `node examples/build-assets.mjs` after CSS or behavior edits. The Worklist/Record anchors show both panels without JavaScript. Enhancement adds tab semantics, roving focus, arrow/Home/End keys, honors the initial hash, and keeps original nodes and values when switching tabs.
