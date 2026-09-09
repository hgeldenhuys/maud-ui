# Shell frame — 0.12.0

**Density never moves the frame.** Density belongs to route content. A document preference may still set it globally, but the masthead, page bar, sidebar, footer and content gutters use independent shell tokens.

The reported lodge measurements were 59 / 66px mastheads, content origins at (260, 135) / (264, 146), and 80 / 87px footers between compact and other routes. Those were measurements from the consuming app, supplied in the brief. The defaults below are library contracts verified by source/DOM fixtures; they are not new measurements of that app.

## Frame tokens

| Token | Default | Applies to |
|---|---|---|
| `--mui-shell-masthead-height` | 64px | Desktop product masthead; phone minimum |
| `--mui-shell-header-height` | 56px | Route context / fallback topbar |
| `--mui-shell-footer-min-height` | 64px | Footer; extra columns can make it taller |
| `--mui-shell-sidebar-width` | `var(--mui-sidebar-w)` = 240px | Expanded sidebar column |
| `--mui-shell-rail-width` | `var(--mui-sidebar-rail-w)` = 64px | Collapsed desktop column |
| `--mui-shell-gutter` | 24px; 16px at ≤40rem | Main content inset and shell inline padding |
| `--mui-shell-inset` | 16px | Sidebar chrome and footer block inset |
| `--mui-shell-control-height` | 36px | Chrome inputs / ordinary buttons |
| `--mui-shell-row-height` | 32px | Compact chrome controls |
| `--mui-shell-nav-row-height` | 36px | Desktop destinations |
| `--mui-shell-text-body-size` | 15px | Chrome body type |
| `--mui-shell-text-small-size` | 14px | Chrome labels / controls |
| `--mui-shell-text-caption-size` | 12px | Group labels, taglines and footer copy |
| `--mui-shell-text-brand-size` | 18px | Wordmark and h3 |
| `--mui-shell-text-h1-size` / `-h2-size` / `-display-size` | 32 / 24 / 44px | Headings supplied inside chrome |

These defaults are the same in compact, comfortable and spacious. Chrome still inherits brand fonts and colors. Coarse pointer targets keep their 44px floor. Changing a shell token is an explicit frame change; changing density is not. Document scrollbars reserve a stable gutter so short/long routes do not change the available width.

## Full-height body and footer

The shell root is a one-column grid with `min-height: 100dvh` and rows `auto 1fr auto`. The optional masthead occupies row one, `.mui-block--shell__body` row two and the optional footer row three. Missing optional chrome leaves no reserved height. The footer follows content and sits at least at the viewport bottom on a short page.

The body contains `.mui-block--shell__sidebar-column` and `.mui-block--shell__main`. The column stretches to the full body row; its background and hairline continue to the footer on long pages. Size containment stops a long menu from forcing the body taller than the route. Its inner `.mui-block--shell__sidebar` sticks at the viewport top, has a viewport-bounded scroll area, and retains every destination.

The phone drawer still moves the original sidebar into the native dialog. Closing or returning to desktop restores it into the column. The column is hidden on enhanced phones; the no-JS details fallback remains available. Embedded gallery previews intentionally keep a 32rem minimum instead of creating another viewport-sized page.

Update CSS and JS together. Existing Rust Props remain compatible, but CSS or scripts that assume `.mui-block--shell > .mui-block--shell__sidebar` must use the new column. Keep shell chrome present during route loading and apply loading states to its content.

## Content density and one vertical gap

The same `data-density="compact|comfortable|spacious"` attribute works on `<html>`, `<main>` or another content container. The action row's existing density attribute remains its own variant and does not create a document density scope.

```rust
use maud::html;
use maud_ui::blocks::shell::sidebar;
sidebar::render(sidebar::Props {
    children: html! {
        div class="mui-page-stack" data-density="compact" {
            // Banner, Today strip and table are sibling blocks here.
        }
    },
    ..Default::default()
});
```

The shell content and `.mui-page-stack` own `gap: var(--mui-stack-gap)` (16px). Their immediate children have no block margins. Use this class on caller-owned page wrappers and remove their old vertical margins; do not combine a gap with child margin spacing. Nested horizontal grids retain their own layout. Banners, cards and table frames all use the same Large radius.

## Current item and containing group

Exactly the first matching destination receives `aria-current="page"`. Its row has a flat tint, zero radius and a 2px accent bar flush with the sidebar edge. Nested labels indent inside the row rather than moving that edge. Focus outlines sit inside it and hover changes the tint.

The containing group receives `data-contains-current="true"`, including when a descendant is current. Its label uses `--mui-text-primary` (an alias of `--mui-text`) at weight 600 and its chevron uses `--mui-accent-text`. The parent has no fill or shadow. This state remains legible when a user collapses the group. Both sidebar APIs use the same treatment; group and rail preferences still persist.

## Radius scale

Only `--mui-brand-radius` is a public brand override. Its neutral default stays 0.375rem. The derivation changes from Small ×2/3 and Large ×2 to:

| Role | Formula | Default brand 6px | Brand 12px | Brand 64px |
|---|---|---:|---:|---:|
| `--mui-radius-sm` | brand ×0.5 | 3px | 6px | 32px |
| `--mui-radius-md` | brand | 6px | 12px | 64px |
| `--mui-radius-lg` | min(brand ×1.5, 12px) | 9px | 12px | 12px |
| `--mui-radius-control` | min(Small, 8px) | 3px | 6px | 8px |
| `--mui-radius-tight` | min(Small, 4px) | 3px | 4px | 4px |

Buttons, inputs, selects and status chips use Control. Its 8px maximum is at most a third of even a 24px control. Compact indicators, badges, count bubbles, checkbox indicators and small switches use Tight. Cards, table frames and banners use Large. The Full token is reserved for avatar and status-dot shapes; radio indicators and spinners keep their intrinsic circle geometry independently of the brand scale.

The theme page shows live resolved Small / Medium / Large samples. Copy and Download still emit exactly nine declarations; a comment gives the formulas and resolved radii. Advanced theme preview overrides are excluded. See [Brand and density](brand-and-density.md) for the unchanged brand contract.

## Fixtures and verification

`cargo run --example frame_fixture` generates nine pages under `docs/fixtures/`. The gallery serves them at `/fixtures/shell-frame-{compact|comfortable|spacious}-{short|long|scoped}.html`. Use 1280×900 and 390×900, in both themes. A deliberately long nav and 70-row long page expose column or footer sizing mistakes; the short page has two rows. The brand radius is 12px to reproduce the reported aggressive-corner case.

- `tests/shell_frame.rs`: real Rust composition, unique sidebar/body/column, nested current selection and the unchanged nine-token export.
- `tests/shell-frame-css.mjs`: both emitted stylesheets over those pages; frame coordinates, short/long grid arithmetic, container density, single stack gap, flat current state and radius bounds. Negative mutations reinstate density padding, the 512px minimum, missing column containment, doubled corners and a filled parent; each is rejected.
- `tests/curation-runtime.mjs`: both bundles move and restore the same sidebar to the column, retaining values, links and lifecycle cleanup.
- `tests/brand-density-runtime.mjs`: both bundles expose computed radius guidance without adding export declarations.
- `tests/shell-frame-browser.mjs`: prepared for supervisor browser execution; checks 24 viewport/theme/density/length combinations, actual footer/column bounds, spacing, radius, drawer and no-JS fallback. It was not run in the design lane, which is prohibited from launching a browser.

Source/DOM fixtures are not browser layout measurements. The report distinguishes these gates from the supervisor's required pixel review.
