# Linear demo-issue replica — report

Replicating linear.app's homepage demo frame (1320x720, dark) in maud-ui, from
`linear-demo-dump.json` (301 nodes, computed styles + verbatim SVGs) against
`linear-ref.png`.

**Final: 98.03% of pixels within tol 24** (round 5). Element-rect probe: **19 of
the 20 largest elements within 1px** of the reference dump; card2 is +1.5px
tall. Font verified loaded: `document.fonts` reports `Inter Variable loaded`,
`fonts.check('13px "Inter Variable"')` = true.

## 1. Match % per round

| Round | Match | Change | What changed |
|---|---|---|---|
| 1 | 94.93% | — | first full build from the dump |
| 2 | 96.04% | +1.11 | card gutter breakout (`margin: 20px -16px 0`), chat-user tint neutralised, `Label` sprite icon redrawn |
| 3 | 96.88% | +0.84 | `min-width` clamp from `defaults.css` (`--mui-control-height` 32px) overridden on all icon buttons; `card::render`'s `__body` wrapper padding zeroed; code chip margin fix |
| 4 | 97.70% | +0.82 | mono inline-code chip, reply head 17.8px, card2 padding 13px, composer transparent border |
| 5 | 98.03% | +0.33 | **`font:` shorthand on `.lr-frame` had reset the library's `cv01, ss03` Inter features** (digit/cap advances) — re-asserted; Skills chip icons 14px/8px; desc line 42px |

Round 5 is the stopping point: ≥ 97% reached. Mean abs diff 8.02/255.

### Final worst cells (round-5 diff) and what is still off

| Cell | Off | Why |
|---|---|---|
| col3 row2 (x495–660, y240–360) | 5.8% | description second line + Activity heading + first activity rows: sub-pixel glyph-advance differences between the bundled InterVariable and Linear's served copy (most visible on digits — `2min ago`, `8852`), plus the capture's colour-profile lift (+5–8/channel on every dark surface) |
| col2 row4 (x330–495, y480–600) | 5.6% | card2 rows: same glyph-advance drift on 12px text ("Draft PR awaiting your review" starts ~2px earlier), and card2 is 1.5px taller than the dump's 91px |
| col2 row1 (x330–495, y120–240) | 4.5% | title + description ghosting: ±0.5px baseline phase, same tracking drift |
| col2 row0 (x330–495, y0–120) | 4.4% | breadcrumb `DRV-8852` renders 59.5px wide vs the dump's 63px (digit/cap advances again) — shifts the star/⃞ buttons 2.4px right |
| col2 row3 (x330–495, y360–480) | 4.3% | card1 text at ±1px vertical; body copy 1px high |
| col6 row2 (x990–1155, y240–360) | 4.2% | agent panel prompt bubble + context row: bubble text at ±1px, `DRV-364` −3px wide |

The two systematic residues: (a) the reference PNG was captured with a display
colour profile that lifts near-blacks by ~+5–8 per channel (dump computed
`rgb(16,17,18)` reads back ~`rgb(22,23,24)`); uniform and inside tol 24, but it
eats margin on every surface edge. **(Superseded by Round 3: the lift is real
overlay layers, not a capture artifact — see below.)** (b) Inter glyph advances differ slightly on
digits/caps between maud-ui's bundled `InterVariable.woff2` and Linear's
hosted font — I could not swap the font (task boundaries: only the example +
`linear-replica/` css/images may be created).

## 2. GAP list — every override in replica.css

### (a) A token value maud-ui should change

1. **`--mui-nav-row-height` (1.5rem = 24px)** → Linear's sidebar rows are 28px.
2. **`--mui-icon-size` (16px)** → sidebar/menu icons are 14px in this UI.
3. **`--mui-radius-md` (6px)** → nav rows use 8px.
4. **`--mui-control-height` (2rem)** → Linear's icon controls are 28px, and it
   also leaks a `min-width: 32px` clamp onto every `--icon*` button that no
   override can beat without `min-width: 0`.
5. **`--mui-badge-height` (22px) + `0 8px` padding** → model chips are 18px,
   `1px 4px`, 11px/14px type.
6. **`--mui-radius-lg` (8px)** → Linear's issue cards are 9px.
7. **Card surface wash**: `linear-gradient(surface-wash…)` over `--mui-bg-card`
   lightens the card; Linear's card is flat `rgb(15,16,17)` (= bg-card).
8. **Separator colour** `--mui-border` (white 8%) → Linear's intra-card divider
   is `rgb(46,46,50)`.
9. **`--mui-text-small-size` on breadcrumb items** → trail is 12px/510 in
   secondary ink (not 13px muted).

### (b) A component prop/variant maud-ui should add

1. **Button**: no 28px icon-only size (`Icon` 32 / `IconSm` 24); no translucent
   pill variant with a 1px inset highlight ring (`header2` controls); Ghost has
   no bordered pill (the "New issue" button). All three hand-styled.
2. **Button/Card**: no className/attr passthrough prop — per-instance overrides
   are impossible without wrapper divs (I wrapped cards; buttons are targeted by
   container selectors).
3. **Sidebar**: `menu_button` has no active/current variant (library paints
   `--mui-accent-soft` + accent ink; Linear paints white 4% with unchanged ink).
4. **Sidebar `group_label`**: small-caps mono heading vs Linear's 24px flex-row
   label with a chevron glyph.
5. **Sidebar**: renders as a 15rem flex sibling with card background — needs a
   "grid-cell, transparent" variant for demo-frame layouts.
6. **Card**: no bare-children recipe (padding lives on `__header/__body`
   sections) and `card::render` wraps children in `__body` with its own
   20/16px padding + stack gap; needs padding/`gap`/`radius` props and a
   `padding: 0` body option.
7. **Card**: no overlay/absolute slot (the Preview chip is hand-positioned).
8. **Avatar**: sizes are fixed 28/40/56 — needs 14px/16px sizes (or a free
   `size` prop).
9. **Badge**: no size override prop; label chips in the properties column carry
   a 16px colour dot that `Badge` cannot express at all.
10. **Message (chat layout)**: the user turn paints an accent-tinted card with
    a square corner and right-aligns — needs a "plain" variant; wrappers
    (grid + `__body-wrap`) need a neutralised form for non-chat-panel use.
11. **Composer**: field chrome not overridable to a flat ring box without
    fighting `bg-card` + control border + `radius-lg`; chips cannot carry
    icons/chevrons (`Chip` is text-only); no trailing icon-action slots (retry/
    attach had to be absolutely positioned over the actions row); the send
    button is a labelled text button — Linear's is a 26px icon circle (label
    hidden, arrow drawn with a CSS mask).
12. **Breadcrumb**: separator is a required string; a trail with no separator
    needs `Some("")` + `font-size: 0` on the separator li.
13. **kbd**: renders a mono keycap (bottom border + shadow); Linear's inline
    code chip is a 24px bordered pill — a `code` chip variant of kbd (or a
    proper inline-code component) is missing.
14. **Shell/layout**: no primitive renders the rounded demo frame (grid
    232px/1072px, 8px padding, radius 12) or the dump's 5-column content grid
    (55.89 / 661.77 / 55.89 / 286.47 / 9.97).
15. **Typography**: h3 token is 18px/1.35 — the issue title is 20px/26.6 at
    −0.24px in secondary ink; needs an "issue title" scale step or free size
    props on headings.

### (c) Linear-specific, does not belong in the library

1. The `/0` full-bleed overlay surface (inner app `rgb(16,17,18)` + border)
   painted under the grid children.
2. The 5-column article grid with empty gutters (Linear's odd measured values).
3. The `-16px` card gutter breakout (cards bleed across the article padding).
4. The delegate split-button (icon | 1px divider | chevron in one pill).
5. The agent panel itself (position, chrome, window controls, Skills chip,
   Preview chip, PR card with `master ← branch` footer).
6. The `1 / 84` pager with per-part colours.
7. Linear's icon sprite vars (`--label-faint`, `--color-yellow`, `--color-indigo`,
   `--color-orange`, `--color-text-tertiary/quaternary`, `--hero-line`) — defined
   in replica.css so the verbatim captured SVGs resolve.

## 3. Components used vs hand-written

**maud-ui components used** (with overrides): `sidebar` (render/header/
group_label/menu/menu_item/menu_button), `breadcrumb`, `button` (11 icon
buttons: star, ⃞, prev/next, copy×3), `avatar` (all 8 avatars), `badge` (Opus 5),
`card` (2 comment cards + PR card), `separator` (2 dividers), `message` (both
agent-panel turns, chat layout), `composer` (Reply composer), plus the library's
tokens/font stack.

**Hand-written HTML** (no component exists, or the component cannot carry the
shape — each has a GAP note): the frame + overlay, brand switcher, search/new
issue buttons, nav labels' chevron, the active nav row, the action bar's split
delegate button, the inline code chip, activity rows + connector, the properties
column (all rows + label chips), the agent panel chrome (header/window controls/
prompt bubble/context row/worked-row/answer/PR card interior/Preview chip), and
the composer's Skills chip + retry/attach mini buttons + send arrow (CSS mask).

## 4. Honest gaps

- **8 SVGs are truncated in the dump** (`outerHTML.slice(0,1500)` cut them
  mid-path): Inbox, My issues, Projects, Star, Copy-URL, Delegate, panel
  PR-branch, Refresh. Each is replaced with a hand-drawn approximation
  (`REDRAWN` in the icons module). Also `BarChart`, `DesignTools` and `Label`
  reference Linear's sprite (`<use href="#…">`) which the dump does not carry —
  redrawn. Icon shapes are close but not byte-identical; this is visible in the
  diff as faint icon ghosts.
- **2 SVG constants unused** (`S_SEND` — the send arrow is a CSS mask; one is
  `#[allow(dead_code)]`).
- **Colour-profile lift** in the reference (~+5–8/channel on darks) is not
  reproducible from computed styles; everything is built from the dump's values.
- **The glow blob** (`/2/0`, 400x400 at the content pane's top) and the `/3`
  opacity-0.6 overlay carry no captured background in the dump; skipped.
- **Label dots**: the two label chips sit entirely behind the agent panel in the
  reference, so their fill colour is unknowable — rendered as empty 16px dot
  divs.
- **Glyph advances**: digits/caps run slightly narrower than the reference
  (see §1). Fixing this needs Linear's exact Inter build, outside the task's
  file boundaries.
- **`DRV-8852` width** (−3.5px) shifts the header1 star/⃞ buttons +2.4px.
- **card2 is 1.5px taller** than the dump's 91px.
- All four images downloaded successfully (karri's avatar needed an
  Accept-header dance + `sips` conversion: the CDN serves AVIF by negotiation).
- Fonts: Inter Variable loads and is used (verified via `document.fonts`).

## Round 2 (icons)

The re-capture `linear-svgs.json` (48 icon svgs, untruncated, each with its
frame-relative position + the sprite `<symbol>` defs) closed the round-1 icon
gap: **all 45 unique icon constants are now byte-verbatim from the live page.**
Verified programmatically — every constant equals its dump entry's outerHTML
(the three sprite icons carry the symbol's children inlined in place of
`<use href="#…">`, per the brief).

**What changed**

1. The 11 hand-drawn icons replaced with the verbatim SVG: Inbox, My issues,
   Projects, Star, Copy-URL, Delegate, panel PR-branch, Refresh, and the three
   sprite icons BarChart, DesignTools, Label. The Label sprite was the biggest
   single win (its activity-row region went 85.98% → 99.20%).
2. Attribute-level diffs replaced too (e.g. Branch-sm carried `#9c9da1` where
   the page computes `currentColor` → `#8a8f98`).
3. **Render-size corrections (replica.css):** the dump's `w/h` are measured
   bounding rects, and several captured svgs carry `width/height="16"` attrs
   yet render 14px on the source page (CSS-shrunk there). One rule pins those
   to 14px: `.lr-bot-row .lr-avcell > svg`, `.lr-pr-row2 > svg`,
   `.lr-mini > svg`. Element-rect probes on the replica now match the dump:
   PR-branch `(921, 453, 14, 14)` exact; card2 branch `(314.9, 570.9)` vs dump
   `(314.9, 570.4)`; both mini icons 14px.
4. **Send button glyph:** the captured send icon is a *rounded square*, not the
   down-arrow the round-1 mask drew. The CSS-mask path was swapped to the
   captured path (colour unchanged: computed fill `rgb(208,214,224)` = the
   mask's `background-color`). `S_SEND` remains dead code — the glyph lives as
   a mask because the library send button cannot host an icon (GAP note).

**Per-round score** (pixel-diff, tol 24, vs `linear-ref.png`)

| Round | Match | Change | What changed |
|---|---|---|---|
| 6 | 98.08% | +0.05 | all 45 icons verbatim + 14px render-size rules + send square glyph |

Stopped after round 6: gain < 0.2 (icons are ~0.1% of frame area; the round-1
redraws were already close). A round-5 vs round-6 change sweep confirms all
1116 changed pixels (>8/channel) sit at icon locations — no regression
elsewhere; mean abs diff 8.02 → 7.98.

**Icon-region scores, round 5 → round 6**

| Region | R5 | R6 |
|---|---|---|
| Label activity icon | 85.98 | 99.20 |
| Sidebar nav icons | 95.59 | 97.34 |
| Action bar icons | 93.80 | 94.87 |
| Composer row (minis + send) | 98.46 | 98.75 |
| Header1 star/dots | 93.15 | 92.85 |

**Still differs.** The six worst cells are unchanged from round 5 — they are
the two documented residues (bundled-InterVariable glyph advances on
digits/caps, and the reference capture's +5–8/channel dark lift), not icon
defects. The star/dots dip (−0.30) is the same known `DRV-8852` width shift
moving those buttons 2.4px; the verbatim glyph at a shifted position scores
marginally differently than the old approximation did.

**Honest gaps update:** the round-1 gap "8 SVGs truncated + 3 sprite icons
redrawn; icon shapes close but not byte-identical" is **closed**. Remaining
gaps are those listed in §4 minus the icon item (colour-profile lift, glow
blob, label-dot fills behind the panel, glyph advances, card2 +1.5px).

## Round 3 (surface lift)

The rounds 1–6 "capture colour-profile lift" theory is **retracted**: sampling
flat pixels per surface shows the lift differs per surface (sidebar +4, content
+4, panel +5, card +13), so it is real layers Linear paints and the replica
skipped — the dump's `/2/0` glow blob, the `/3` grain layer, and raised
backgrounds on the panel's interior surfaces. Both images are same-tool
page-shots, so the numbers are directly comparable.

### Sampled surfaces (PIL, 12 flat points each; ref = 7x7 median, grain-noise-robust)

Reference pixels carry visible grain noise (±1–2/px) — the `/3` grain png — so
flat-point detection needs medians, not single pixels.

| Surface | Reference | Replica before (round-6) | Replica after (round-8) | Δ after |
|---|---|---|---|---|
| Sidebar (flat rows) | (19.7, 21.3, 22.6) | (16, 17, 18) | (20.4, 21.4, 22.4) | ≤1.3 |
| Content pane (gutters, below panel, header1) | (23, 24, 25.1) | (19, 20, 21) | (23, 24, 25) | ≤0.2 |
| Header2 band | (25, 26.2, 27.4) | (19, 20, 21) | (24.9, 25.9, 26.9) | ≤0.5 |
| Card 1 | (27.7, 27.9, 28.9) | (15, 16, 17) | (26.7, 27.7, 28.7) | ≤1.0 |
| Card 2 | (27.7, 27.8, 28.8) | (15, 16, 17) | (26.8, 27.7, 28.7) | ≤0.9 |
| Agent panel (mid + low) | (26.8, 28, 29) | (22, 23, 24) | (26, 27, 28) | ≤1.0 |
| Composer field | (35, 36.1, 37.3) | (22, 23, 24) | (35, 35, 36) | ≤1.3 |
| Prompt bubble / PR card | (38, 39, 40) / (37.2, 38.6, 39.6) | (22, 23, 24) | (37.5–38, 38.4–39, 39.4–40) | ≤0.6 |

Every surface within ±2 per channel (worst single point: −2, one point, sidebar
glow edge). **All surfaces met.**

### What was added (two layer divs + washes, no per-element base colour edits)

1. **`.lr-glow`** — the dump's `/2/0` verbatim: 400x400 radial
   `rgba(255,255,255,0.04) → 0 at 90%`, placed **frame-absolute** (content pane
   child at −199,−199). Empirically settled: the reference lift fades by x≈460
   and y≈170 and is absent at x=481+, which fits centre (241,9) — the content
   pane's top-left corner — and refutes a content-relative reading. Trace fit
   after render: (260,12) 33/33, (300,12) 29/30, (340,12) 28/28.
2. **`.lr-grain`** — the `/3` Grain layer collapsed to its measured mean: a
   1.6% white wash over the whole frame (the uniform +4 lift), plus the
   broad corner glow off the frame's top-left (200x190 at 10,−10, 3.6%) that
   lifts the sidebar's top rows (fades by y≈150, x≈180 — fitted from traces
   after discarding a button-contaminated sample at (200,20)).
3. **Card wash** — cards keep `var(--mui-bg-card)` as the base (token, GAP list
   stays truthful) under a 3.6% white gradient; the flat `rgb(15,16,17)`
   override is gone.
4. **Raised panel-interior surfaces, discovered this round**: the composer
   field reads (35,36,37) and the prompt bubble + PR card (38,39,40) in the
   reference — the replica had them transparent/flat. Washed: composer field
   4.1%, prompt + PR card 5.4% over their rgb(22,23,24) bases.

### Per-round score (pixel-diff, tol 24, vs linear-ref.png)

| Round | Match | Mean abs diff | What changed |
|---|---|---|---|
| 6 (before) | 98.08% | 7.98 | — |
| 7 | 98.49% | 2.56 | glow + grain layers, card/composer/prompt/PR-card washes (first tuning) |
| 8 (final) | 98.47% | 2.38 | washes rebalanced to the sampled targets (grain 1.6%, corner radial narrowed) |

**Mean abs diff 7.98 → 2.38 (3.4x down)** — the surface lift was the dominant
residual. Match % moves 98.08 → 98.47; the round-7 → round-8 −0.02 is
sub-tolerance dither (tol 24 makes ±1 shifts score-neutral), kept because
round-8 is the sampled-truth state. Remaining worst cells (col2/col3, 3.3–5.8%)
are the documented Inter glyph-advance ghosting, untouched by this round.

**Honest gaps update:** the glow-blob/grain item in §4 is **closed** (as a mean
-effect reconstruction). New gaps from this round: (a) the grain's noise texture
is not reproduced — only its mean lift (a noise png + mix-blend-mode overlay is
not expressible in the replica's remit; invisible at tol 24); (b) the
sidebar-corner glow is an engineered fit to traces, not a captured node (the
dump has no node there — 301 nodes may be truncated); (c) composer/prompt/PR-card
wash alphas are fitted to samples, not captured values.
