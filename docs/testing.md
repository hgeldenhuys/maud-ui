# Testing

```bash
cargo test              # 194 tests — structure, rendering, tokens, breakpoints
bun run test:js         # 10 assertions — behaviour and layout, in real Chrome
bun run sweep:overflow  # every exported page, checked for sideways scroll
```

All three run in CI on every push (`.github/workflows/ci.yml`).

## What each suite guards

| Suite | Tests | Guards |
|---|---|---|
| `tests/render_tests.rs` | 132 | every `showcase()` renders non-empty, plus per-component markup assertions |
| `tests/registration_parity.rs` | 18 | a component is wired up *everywhere* — see below |
| `tests/breakpoint_scale.rs` | 3 | every `@media` width is a declared breakpoint ([breakpoints.md](breakpoints.md)) |
| `tests/css_token_integrity.rs` | 2 | every bare `var(--mui-*)` resolves to a defined token |
| unit (`#[cfg(test)]` in `src/`) | 29 | markdown table transform, code highlighter, time formatting |
| doc-tests | 10 | the examples in `///` docs actually compile |
| `js/runtime-test.mjs` | 10 | JS behaviour + rendered layout, in a browser |

`registration_parity` is the load-bearing one. Adding a primitive means touching ~8 places, and it
asserts they all agree: module ↔ `COMPONENT_NAMES` ↔ doc file ↔ CSS import ↔ render-test macro ↔
static export ↔ `Cargo.toml` include globs ↔ the component counts in README, `Cargo.toml` and the
rasterised OG card. Read it before adding a component; it will tell you what you missed.

## The browser suite

`cargo test` never executes JavaScript and never lays anything out. Everything it knows is
structural. That blind spot is not theoretical — on 2026-07-28, with every Rust test green:

- 74 of 82 gallery routes scrolled sideways on a phone
- `MaudUI.init()` silently attached nothing when an htmx swap target *was* the component
- forms inside dialogs stopped 80px short of their own column

All three were reported by a human looking at the screen. `bun run test:js` exists to close that
gap: it runs the **built bundle** and the **built stylesheet** in real headless Chrome, with no DOM
shim and nothing mocked.

It needs Chrome or Chromium — the same binary `js/build-og.mjs` looks for. Point `CHROME_PATH` at
one if it lives somewhere unusual.

### Two kinds of case

**Behaviour** — synthetic markup, asserts what the runtime does. Both htmx swap shapes (target
contains the component; target *is* the component), OOB swaps, history restore, idempotence,
unknown-behavior handling.

**Layout** — real markup, asserts where things land. These extract actual `<dialog>` elements from
`public/dialog/index.html` rather than using hand-written HTML, so a case cannot quietly drift from
the component it claims to test.

### Adding a case

Edit the `CASES` block in `js/runtime-test.mjs` and assign a boolean to `results.<name>`. The name
is what prints, so make it a claim: `dialog_form_fills_the_column`, not `test4`. Then:

```bash
bun run build && bun run test:js
```

**Rebuild first — always.** The harness tests `dist/`, not the sources. Editing
`dist/maud-ui.js.bak` or `css/` without rebuilding tests the previous bundle and passes.

**Watch your new case fail before you trust it.** Break the thing it guards, confirm it goes red,
put it back. Every assertion in the file was added that way; two of them turned out to catch more
than expected, which is how the htmx bug's real blast radius (three failing paths, not one) was
discovered.

### Two traps that produce confidently wrong numbers

**Dialogs animate.** `dialog.mui-dialog` runs `scale(0.95) → scale(1)` over 150ms, so any
measurement taken mid-flight reads every box 5% narrow — internally consistent and completely
wrong. The harness disables the animation before measuring. Do the same in any ad-hoc probe, or
wait ~400ms.

**The runtime source is not the file you would expect.** `js/build.mjs` builds from
`dist/maud-ui.js.bak`. `js/maud-ui.ts` is **not built** and has diverged; it carries a banner
saying so. Edit the `.bak`.

## The overflow sweep

```bash
bun run sweep:overflow                 # 390px
bun run sweep:overflow -- 360 390 1280 # several widths
```

Loads all 109 exported pages at each width and reports any whose document is wider than the
viewport, naming the widest offending element. Exits non-zero, so it gates a change.

It only counts elements that are **not** inside an `overflow-x` container — otherwise every code
block and scrollable table on the site reads as a failure.

Run it after any CSS change that touches layout. Rebuild first (`bun run build && bun run
build:static`) — it sweeps `public/`, not the sources.

### Do not point it at the deployed site

The measurement needs same-origin iframes, and the deployed site sends frame-denial headers **even
to itself**. A sweep aimed at the live URL gets an empty document for every route and reports the
entire site as broken — the instrument failing, not the site. That misread cost real time on
2026-07-28.

Verify the deployed site instead by byte-comparing what it serves against `public/`:

```bash
curl -s -o /tmp/live.css https://maudui.herman.engineer/css/maud-ui.min.css
cmp /tmp/live.css public/css/maud-ui.min.css && echo "live matches the build"
```

If the bytes match and the build passed the sweep, the deployed site passes it too. (Compare with
`cmp` on files, not `$(curl ...)` in a shell — command substitution strips the trailing newline and
invents a one-byte difference.)

## Ad-hoc measurement

For a one-off check, drive a real browser — do not reason about layout from CSS:

```bash
bun run build && bun run build:static
PORT=3210 bun server.ts
```

Then in the console, the check that catches the whole class:

```js
document.documentElement.scrollWidth - document.documentElement.clientWidth   // want 0
```

## Not covered

Worth knowing, so a green run is not read as more than it is:

- **Accessibility** — a handful of incidental ARIA assertions, no systematic per-component contract
- **Colour contrast** — measured by hand in the 2026-07-27 audit, nothing prevents regression
- **Keyboard interaction** — no test presses a key
- **Light theme** — defined, never asserted; the sweep and the layout cases both run dark only
- **Overflow below 360px** — the sweep gates 390px in CI. 320px still has 5 known offenders
  (`/gallery`, `/blocks`, `/badge`, `/data_table`, `/form`, 9–25px), all min-width-bound content

`cargo fmt --check` and `cargo clippy -D warnings` both fail on the tree (47 clippy findings as of
2026-07-28). CI runs them **advisory, not gating** — a CI that is red from its first run teaches
everyone to ignore it. Promote them to gates once the tree is clean.
