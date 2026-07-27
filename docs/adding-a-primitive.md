# Adding a primitive

Registering a component means agreeing with yourself in **eight** places. Nothing about that is
enforced by the type system — they are cross-file agreements, not types — so `cargo build` stays
green while a component is half-registered.

That has bitten this repo three times, each silently:

- `TIERS` listed a `toolbar` slug with no module behind it. Both render sites skip what they cannot
  resolve, so it rendered as *nothing at all* and no one noticed.
- Five finished primitives — `code_block`, `diff`, `message`, `streaming_cursor`, `tool_call`,
  ~1,470 lines with their own CSS — were registered **nowhere**. They shipped inside every crate
  download and no consumer could discover them. For two releases.
- Six primitives were missing from the render-test macro.

**`tests/registration_parity.rs` now fails the build on every one of those.** You do not need to
memorise the list below — run `cargo test` and the failure will name the slug and the missing step.
This document exists to explain *why* each step is there.

## The checklist

Say the primitive is `spinner`. In `src/primitives/spinner.rs`:

```rust
pub struct Props { /* all pub fields */ }
impl Default for Props { /* every field */ }
pub fn render(props: Props) -> Markup
pub fn showcase() -> Markup
```

Then:

| # | File | What to add | What breaks without it |
|---|------|-------------|------------------------|
| 1 | `src/primitives/mod.rs` | `pub mod spinner;` | The module does not compile into the crate at all |
| 2 | `src/showcase/mod.rs` → `COMPONENT_NAMES` | `"spinner",` **in alphabetical order** | Invisible to nav, routing, and the `cmd+k` palette |
| 3 | `src/showcase/mod.rs` → `TIERS` | the slug, in the right tier | Missing from the gallery's grouped navigation |
| 4 | `src/showcase/mod.rs` → `component_content()` | `"spinner" => primitives::spinner::showcase(),` | `/spinner` silently serves the **404 page** |
| 5 | `src/showcase/docs.rs` | `"spinner" => Some(include_str!("../../docs/components/spinner.md")),` | The component page renders with no API docs |
| 6 | `docs/components/spinner.md` | the 8-section doc — see `docs/components/README.md` | Step 5 fails to compile; and the docs **ship inside the published crate**, so a gap reaches consumers |
| 7 | `css/components/spinner.css` + an `@import` in `css/maud-ui.css` | only if the component has styles | The component renders **unstyled**, with no error anywhere |
| 8 | `tests/render_tests.rs` → `assert_showcase_renders!` | `spinner,` | `showcase()` is never exercised; this is the point nothing documents as part of the invariant |

Optional: an arm in `component_docs()` for a hand-written usage snippet. Only 29 of 72 components
have one, so it is genuinely optional — the doc file already carries Import and Example.

There used to be a ninth: `js/export-static.mjs` kept its own copy of the component list under a
"keep in lockstep" comment, so a fully-registered component could still fail to ship a page on the
public site. It now parses `COMPONENT_NAMES` from source, and a parity test keeps it that way.

## Then

```bash
cargo test                    # parity + render tests
node js/build.mjs             # ONLY if you touched css/ or dist/behaviors/
cargo run --example showcase  # http://127.0.0.1:3456/spinner
```

**Rebuild `dist/` whenever you touch CSS.** `dist/maud-ui.css` is committed, ships in the crate, and
`CSS_VER` is derived from its byte length as a cache-buster — so a stale `dist/` means browsers keep
the old stylesheet.

**Bump the component count** in `Cargo.toml`'s `description` and in `README.md` (two places). The
parity test checks these against `COMPONENT_NAMES.len()` and will tell you if you forget.

## House style

Read `src/primitives/card.rs` first — it is the cleanest example of a component with both a `Markup`
slot and a closed enum prop.

- **Closed enums over free-form strings.** `pub button_type: &'static str` in `button.rs` is a wart,
  not a pattern: nothing downstream can enumerate its legal values. An enum can be rendered as a
  dropdown by tooling and checked by the compiler.
- **`as_class()` returns `""` for the default variant**, and the base class carries that default.
  Default props should produce exactly `class="mui-spinner"` with no modifier noise.
- **Suppress props that would do nothing.** `grid` omits `min_column` on a fixed column count. A
  class in the markup that changes no rendering is a lie to whoever reads the markup to work out the
  layout.
- **Every field gets a `Default`**, and `Props` gets a hand-written `impl Default` if any field's
  correct default is not its zero value. `code_block::show_copy` was documented as "default true"
  while a derived `Default` silently made it `false`.
- **Colour is never the only channel.** Pair it with text or a shape, and put the text where
  assistive tech will actually read it — `item::status_dot` and `diff`'s visually-hidden
  `"Added: "` / `"Removed: "` are the precedents. In an ARIA table, that text must sit *inside* the
  `role="cell"` element; content in a `role="row"` but outside a cell is not reliably announced.
- **Respect `prefers-reduced-motion`** for anything that animates, and degrade to the *static but
  still visible* state rather than removing the indicator.
- **`debug_assert!` for conditionally-required props** — see `button.rs` (icon-only buttons need
  `aria_label`) and `stack.rs` (`Tag::Section` needs one, or the `<section>` is not a landmark at
  all).

## Writing the test

`assert_showcase_renders!` only proves `showcase()` returns non-empty markup. If the component has a
contract something *outside Rust* depends on — an attribute a browser acts on, a class the CSS
targets, a string a screen reader announces — assert on the rendered HTML in a dedicated module.
See `growth_0_3_0` and `conversation_0_4_0` in `tests/render_tests.rs`.

"It compiles" says nothing about whether a missing `enctype` uploads filenames instead of files.
