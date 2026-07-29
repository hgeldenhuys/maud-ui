# Breakpoints

maud-ui declares five viewport breakpoints. They live in
[`src/tokens.rs`](../src/tokens.rs) under `tokens::breakpoints`, and
`tests/breakpoint_scale.rs` fails the build if a `@media` rule anywhere in
`css/` or `src/showcase/` names a width outside the scale.

| Token | Value | Pixels | Where it earns its keep |
|-------|-------|--------|--------------------------|
| `XS`  | `30rem` | 480px  | Small phones — side-by-side pairs stop fitting |
| `SM`  | `40rem` | 640px  | Phone → tablet. The library's dominant breakpoint |
| `MD`  | `48rem` | 768px  | Tablet portrait — shell layouts go single-column |
| `LG`  | `60rem` | 960px  | A 240px sidebar plus a readable content column both fit |
| `XL`  | `64rem` | 1024px | Embedded editor / map / calendar demos get full height |

```rust
use maud_ui::tokens::breakpoints;

assert_eq!(breakpoints::SM, "40rem");
```

`LG` and `XL` sit only 64px apart, which looks like drift and isn't. They
govern unrelated things — when the gallery shell swaps its hamburger for a
sidebar, and when an embedded editor is tall enough to be worth showing at full
height. Merging them was tried and reverted; see *Folding is not free* below.

## Three things that will bite you

### 1. A breakpoint cannot be a CSS custom property

```css
/* Silently never matches. Not a parse error — the block is simply dead. */
@media (max-width: var(--mui-bp-sm)) { … }
```

Custom properties are not permitted inside a media *condition*, and the
`@custom-media` at-rule from Media Queries Level 5 is still not implemented by
any browser (verified against Chrome 150, 2026-07-28). So a breakpoint is a
literal in CSS and a constant in `tokens.rs`, and the two are kept in step by
hand — the test above is what keeps them honest.

This is why the library ships no `--mui-bp-*` variable: a token you cannot use
where you would obviously reach for it is a trap, not a convenience.

### 2. Author in `rem`, not `px`

Inside a media query, `rem` resolves against the browser's *initial* font size
— not `html { font-size }`. A reader who raises their default font size in
browser settings therefore reaches the single-column layout sooner, which is
usually what they wanted. A `px` breakpoint ignores that preference outright.

### 3. `BELOW_*` is for max/min pairs — which this tree does not yet have

`max-width: 60rem` and `min-width: 60rem` **both** match at exactly 960px, so a
pair written that way leaves the boundary pixel to source order. The `BELOW_*`
constants exist for whoever writes the first such pair:

```css
@media (max-width: 59.99rem) { /* below lg */ }
@media (min-width: 60rem)    { /* lg and up */ }
```

Every responsive rule here is currently single-direction — it overrides a base
that already holds for the other side — so no rule needs a `BELOW_*` today.
Check before reaching for one.

## Folding is not free

The scale was declared on 2026-07-28, after an audit found **eight** distinct
widths live in the tree. Five were genuine drift and were folded onto the
scale. Two were not, and the difference is worth understanding, because the
guard test cannot tell them apart.

The gallery is **desktop-first**: its base styles are the sidebar layout, and
`@media (max-width: 60rem)` is the *override* that swaps in the hamburger. A
`min-width: 64rem` rule elsewhere in the file governs something unrelated
(editor demo heights). Seeing a `max-width: 960px` and a `min-width: 1024px`
in the same stylesheet, it is easy to read them as a mismatched pair leaving
961–1023px unstyled — and to "fix" it by moving 960 up to 1024.

That reading is wrong, and the fix is a regression: 961–1023px is not
unstyled, it inherits the desktop base, and folding the override upward drags
that whole band onto the mobile nav. It was caught by diffing computed styles
against the deployed build before it shipped.

**The lesson:** the test tells you a width is off-scale. It cannot tell you
whether folding it onto the scale preserves behaviour. Before moving any
breakpoint, check which side is the base and which is the override, and
measure the band between the old value and the new one.

## Prefer a container query for a component

These five are for the **page shell**. A component does not know the viewport;
it knows its slot. A card dropped into a 300px sidebar on a 1440px monitor
should stack — and a viewport query gets that exactly backwards, because it
reports "desktop, go wide".

```css
.mui-card { container-type: inline-size; }

@container (max-width: 30rem) {
  .mui-card__row { flex-direction: column; }
}
```

Container queries and `cqi` units are supported by every current browser.
Reach for `@media` only when the rule genuinely concerns the page.

## Adding a sixth breakpoint

Don't inline it. Declare it in `tokens::breakpoints`, add a row to the table
above with the reason it exists, and the guard test will accept it. The point
of the test is to make the next one a decision rather than an accident.
