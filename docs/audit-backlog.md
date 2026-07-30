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

---

# Deferred from the 2026-07-28/29 responsive + runtime session

Same rule as above: everything here was found and reproduced, not guessed. The session's own
fixes shipped; these are the tails it chose not to pull.

## P1

**Components ask the viewport, not their container.** Every responsive rule in `css/` is a
`@media` query, so a component cannot respond to the box it is actually in — a card in a 300px
sidebar on a 1440px monitor is told "desktop, go wide". This is also the general form of the
dialog-field bug fixed in `9eda7a0`: that fix scopes an escape to `dialog.mui-dialog`, which
works but does not generalise. Container queries are baseline; there are currently **zero** in
the tree. Convert the reflowing components (table, card, composer, data_table, sidebar, grid,
field) one at a time. See `docs/breakpoints.md` → "Prefer a container query for a component".

**Overlay behaviours leak a document listener per open.** `menu`, `context_menu`, `select`,
`combobox`, `popover`, `date_picker`, `menubar`, `navigation_menu`, `scroll_area` and
`resizable` add `document`-level listeners on OPEN and remove them only on CLOSE. An htmx swap
that removes an open overlay never runs `close()`, so the listener — and the detached subtree it
closes over — persists for the life of the page. `hover_card` holds three uncancelled
`setTimeout` handles the same way. There is no `destroy`/`teardown`/`AbortController` anywhere in
`dist/behaviors/`. For a library whose whole premise is htmx swaps, this is the wrong default.

## P2

**`docs/` never mentions htmx.** The tagline is "built on maud + htmx"; the word appears twice in
README and **zero** times in `docs/`. No swap/OOB/boosting guidance, and nothing tells a consumer
that OOB content needs the runtime hooks added in `c5de138`. Pair with the missing **asset-serving
recipe**: README shows `<link href="/css/maud-ui.min.css">` but never says how a crate consumer
gets `dist/` out of `~/.cargo/registry` onto a route (no `maud_ui::assets::css()`, no axum
`ServeDir` example). Together these are the biggest adoption blocker in the repo.

**`hover_card` is mouse-only and silent to assistive tech.** Zero `aria-`, zero `role`, and
`dist/behaviors/hover_card.js` binds only `mouseenter`/`mouseleave` — no `focus`/`blur`, no
Escape, no `aria-describedby`/`aria-expanded`. Keyboard and screen-reader users cannot reach the
content at all. The README claims "accessible by default".

**`chart.rs` renders raw SVG with no accessible channel.** No `role="img"`, no `<title>`/`<desc>`,
no `aria-label`, no data-table fallback (lines 118/136/209/224). The only primitive that renders
content with nothing for AT to read.

**`data_table` ARIA goes stale on first interaction.** `aria-sort="none"` is hardcoded at line 66
and the header menu hardcodes `aria-expanded="false"` at line 89, regardless of actual state. Row
checkboxes all share `aria-label="Select row"` (lines 165, 193) — indistinguishable in a
screen-reader list.

## P3

**320px still overflows on 5 pages.** `/gallery`, `/blocks`, `/badge`, `/data_table`, `/form`, by
9–25px, all min-width-bound content (badge labels, table headers, button rows). The CI sweep gates
390px; 320px is iPhone SE 1st-gen territory and was consciously left.

**`cargo fmt --check` and `clippy -D warnings` both fail** — 47 clippy findings as of 2026-07-28.
CI runs them advisory precisely so it is not red from day one. Clean the tree, then promote them
to gates in `.github/workflows/ci.yml`.

**Light theme has no automated coverage at all.** It is defined and shipped; the overflow sweep,
the layout assertions and the 2026-07-27 contrast numbers are all dark-only.

## Outside the repo

**The GitHub repo description still reads "58 headless, accessible UI components".** It is 79 —
now 21 behind, and still the first line a visitor reads. `gh repo edit hgeldenhuys/maud-ui
--description "…"`. Unchanged from the previous audit; belongs to the maintainer.

---

# From the stack bench, 2026-07-30 — findings from an outside consumer

These came from an independent builder in `~/WebstormProjects/todos-stack-bench` (arm B) who had
no stake in this library, was told only to use it as a consumer would, and could not see the
competing arms. That makes them the closest thing this repo has had to an unbiased adoption
report. The arm passed all seven acceptance items and did so in **1,029 LOC with zero
hand-written JavaScript**, against a control arm's 1,221 LOC and 203 lines of JS — so these are
complaints from a build that *succeeded*.

## P1

**No primitive accepts arbitrary attributes, so `hx-*` has nowhere to live.** Verified across all
73 primitives in the version they used. For a library whose tagline is "Built on maud + htmx",
there is no way to put `hx-post` / `hx-target` / `hx-swap` on a rendered element. The practical
consequence: **`form::render` is unusable for any form that does anything**, and the arm
hand-wrote all four of its mutation `<form>` tags. `hx-boost` on a wrapper was considered and
rejected, because each row needs its own target and `item` has no attribute hatch either.

Proposed fix, from the report: one `attrs: Vec<(String, String)>` field per `Props`. That is a
wide but mechanical change, and it converts the htmx claim in the README from aspiration into
something a consumer can actually use.

**The library owns `--mui-*` with a hardcoded palette and offers no way to adopt a host design
system.** Achieving fleet-token compliance (`--kp-accent`) needed a ~30-line hand-written bridge.
It works without forking, but only because both systems happen to scope on `[data-theme]` — the
arm wrote its bridge at `:root[data-theme=…]` specificity so a stylesheet reorder would not
silently break it. This is the same integration gap identified from the platform side on
2026-07-29; an independent builder has now hit it from the consumer side. A supported
token-aliasing layer (or documenting the bridge as the sanctioned pattern) closes it.

## P2

**crates.io is two minor versions behind the repo, and the docs describe the unpublished one.**
Verified against the registry index: published versions are 0.1.0, 0.2.0, 0.2.1, 0.4.1, **0.5.0**
— described there as *"72 headless, accessible UI components"*. The repo is **0.7.1 with 79
primitives**. So `cargo add maud-ui` gets a consumer 72 components while the README, the gallery
and `docs/components/*.md` all describe 79.

It cost the arm two compile errors before it understood why: `stack::Direction::Row` does not
exist in 0.5.0, and `empty_state::Props` has no `Default` — the latter also contradicting the
README's "every component has `Default::default()`" promise in whichever version is authoritative.

If staying unpublished is deliberate, the README and the gallery should say **which version a
consumer actually gets**, because today they advertise a version nobody can install.
