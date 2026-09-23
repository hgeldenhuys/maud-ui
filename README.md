# maud-ui

**84 headless, accessible UI components for Rust web apps — with shadcn Base UI API parity. Plus 31 block templates, a live theme customiser, a `cmd+k` command palette, and shell hooks for 15 third-party widget integrations.**
Built on [maud](https://maud.lambda.xyz/) + [htmx](https://htmx.org/). Styled like [shadcn/ui](https://ui.shadcn.com/).

[![Crate][crate-badge]][crate]
[![Docs][docs-badge]][docs]
[![License: MIT][license-badge]][license]

[crate-badge]: https://img.shields.io/crates/v/maud-ui.svg
[crate]: https://crates.io/crates/maud-ui
[docs-badge]: https://docs.rs/maud-ui/badge.svg
[docs]: https://docs.rs/maud-ui
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license]: LICENSE

- **Live gallery:** [maudui.herman.engineer](https://maudui.herman.engineer)
- **Source:** `github.com/hgeldenhuys/maud-ui`
- **Docs:** [docs.rs/maud-ui](https://docs.rs/maud-ui)

---

## What you get

- **84 primitives** — every shadcn Base UI component plus extras (data-table, resizable, hover-card, OTP input, command palette, calendar, charts, colour swatch).
- **Layout containers** — `stack` (one axis), `grid` (two), and `form` (the submission contract). Every appearance prop is a closed enum, so a page is composed as a tree of containers instead of inline `style="display:flex"`. Added in 0.4.0.
- **A conversation tier** — `message`, `streaming_cursor`, `code_block` (with a built-in Rust/Bash/TS/JSON highlighter), `diff`, and `tool_call`: an AI-chat / agent surface kit.
- **31 pre-composed blocks** — auth (login/signup/2FA), dashboard stats, data-table-full, pricing tiers, settings (billing/profile/team), full sidebar shell, worklist and record headers, a task launcher, grouped worklists, record timeline/money/related cards, attention banners, shared action rows, and optional app masthead, page header and footer. Drop-in compositions.
- **Live brand customiser** at `/theme` — switch Lodge, Bank and Clinic, edit nine brand tokens, and export `brand.css`. Choices persist across reloads. Eight advanced theme presets remain available for preview.
- **Integration shells** for 15 third-party widgets — Monaco, xyflow, Excalidraw, TipTap, Mermaid, Cytoscape, Three.js, AG Grid, Apache ECharts, Leaflet, FullCalendar, Wavesurfer.js, PDF.js, xterm.js, SortableJS. Each ships a themed chrome around the widget so the third-party canvas adopts your design tokens automatically.
- **Global `cmd+k` command palette** — fuzzy jump to any component, block, integration, or page. Indexed from the same Rust constants the sidebar uses.
- **Accessible by default** — ARIA roles, keyboard navigation, focus management, WCAG-AA contrast in both themes.
- **Dark + light themes** — flip `data-theme` on `<html>` and the whole tree recolors via CSS variables.
- **Progressive enhancement** — every component renders correctly without JavaScript. JS adds drag, dropdowns, keyboard shortcuts on top.
- **Tailwind-compatible** — all classes prefixed `mui-`, no collisions. [Pairing guide →](docs/TAILWIND.md)
- **One Rust dependency** — just `maud`. No serde, no framework lock-in. Works with axum, actix, rocket, or whatever you use.
- **Ship pre-built** — Complete minified CSS and JavaScript bundles. No build step required for consumers.

## Operational UI

Use `short_label: Some("Stays".into())` on a bottom-tab `Item` or sidebar `NavItem` to keep a long mobile destination legible; use `None` for the full label.

Use `status_chip_group` for counted filters, `bottom_tab_bar` for mobile destinations, and `button::Size::Row` with `Variant::Outline` for compact cell actions. Blocks at `worklist::header`, `record::header`, and `task::grid` compose them into application surfaces. The sidebar shell adds `header`, `id`, and `mobile_navigation` props; `empty_state::Variant::{Empty, Filtered, Failed, Inline}` separates zero results from a failed load.

Serve `maud_ui::assets::{CSS_MIN, JS_MIN}` directly, or vendor the complete files in **static/**. The older **dist/** and **public/** trees are 0.7 snapshots and do not contain these additions. `node examples/build-assets.mjs` rebuilds the current bundles; `cargo run --example build_docs` regenerates API HTML after Markdown edits. Consumers have one direct dependency, Maud; Markdown parsing and Axum are development-only.

## The gaps a pixel replica found (0.19.0)

A pixel-level replica of linear.app's demo issue frame (docs/linear-replica/) was built on this library, and every override it needed became a default: nav rows are 28px with 14px icons and an 8px corner, badges are the 18px/1px-4px/11px chip, cards are flat `bg-card` at a 9px corner with a dedicated `--mui-border-divider` between them, breadcrumbs trail at 12px/510 in secondary ink, and icon buttons lost their 32px clamp. New props cover what a closed enum could not reach: `button::Size::IconSm28` and `Variant::Translucent` (plus a Ghost `bordered` flag), a `class:` escape hatch on button/card/badge/avatar and sidebar menu buttons, `sidebar::Surface::Transparent` and a white-4% `current` row, card `padding`/`gap`/`radius`/`bare`/`overlay`, 14px and 16px avatars, badge colour `dot`s, a `Plain` chat message, a `flat` composer with trailing action slots and an icon-only send, separator-less breadcrumbs, a `kbd::Variant::Code` inline chip, and a `Title20` text step. `node examples/build-assets.mjs` rebuilds the bundles.

## Forms and journeys (0.14.0)

Use `form::Props { feedback: true, ..Default::default() }` for inline errors, failed-field focus, accessible descriptions, pending submit/reset controls and duplicate-submit protection. Async adapters report every outcome with `MaudUI.formFeedback(form, result)`; transport, business values and authorization remain application owned. See [Form feedback](docs/components/form.md).

Five new blocks cover the surrounding flow: [result notice](docs/blocks/feedback-notice.md), [confirm an action](docs/blocks/feedback-confirm.md), [search results](docs/blocks/search-results.md), [wizard header](docs/blocks/wizard-header.md), and [create the first record](docs/blocks/worklist-empty.md). Their copy leads with the human outcome and their keyboard paths use native controls.

Generate six fixtures with `cargo run --example workflow_fixture`, then run `node tests/workflow-browser.mjs`. It launches an isolated headless Chrome (override its path with `PAGE_SHOT_CHROME`), uses a local HTTP server, and checks 24 combinations of brand, density, theme and viewport. No global packages are needed. [Live-use findings](docs/night-6-real-use.md) distinguish library fixes from the lodge application's next steps.

## Generic record pages (0.13.0)

Use `blocks::record::{page, facts, related_list}` for every generic detail page. `page` owns title → kind/reference → main action → spaced content; `facts` combines groups into one surface and renders a single fact as a row; `related_list` renders compact links/status/dates without repeated references. See [Record page](docs/blocks/record-page.md) for the complete composition and the adapter's role in labels, masking and actions.

`.mui-stack` / `stack::vertical` now use `--mui-stack-gap` (16px), with child margins reset. Explicit `Space::Md` still means 12px. Native shell search reserves its keyboard badge's width, defaults to `Search`, and breadcrumbs discard blank labels.

Generate fixtures with `cargo run --example record_page_fixture`; compare `/fixtures/record-page-comfortable-guest.html` and `/fixtures/record-page-three-cards.html` at 1280 and 390. The catalog contains 84 primitives and 31 blocks.

## 30-second tour

```bash
cargo new my-app
cd my-app
cargo add maud@0.27 --features axum
cargo add maud-ui
cargo add axum tokio --features tokio/full
```

```rust
// src/main.rs
use axum::{routing::get, Router};
use maud::{html, Markup, DOCTYPE};
use maud_ui::primitives::{button, card};

async fn index() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" data-theme="dark" {
            head {
                link rel="stylesheet" href="/css/maud-ui.min.css";
                script src="/js/maud-ui.min.js" defer {}
            }
            body style="padding: 2rem" {
                (card::render(card::Props {
                    title: Some("Welcome".into()),
                    description: Some("You're running maud-ui.".into()),
                    children: html! {
                        (button::render(button::Props {
                            label: "Ship it".into(),
                            variant: button::Variant::Primary,
                            ..Default::default()
                        }))
                    },
                    ..Default::default()
                }))
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/css/maud-ui.min.css", get(|| async {
            ([("content-type", "text/css")], maud_ui::assets::CSS_MIN)
        }))
        .route("/js/maud-ui.min.js", get(|| async {
            ([("content-type", "application/javascript")], maud_ui::assets::JS_MIN)
        }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

Better yet, clone the repo and run the site locally:

```bash
git clone https://github.com/hgeldenhuys/maud-ui
cd maud-ui
bun run gallery     # builds, picks a FREE port, waits until it really serves, prints the URL
```

The showcase defaults to port 3456. `bun run gallery` selects a free port, or set
`ADDR=127.0.0.1:$(free-port) cargo run --example showcase` explicitly.
For the compiled getting-started page, run `cargo run --example first_paint`
(default port 3000; also accepts `ADDR`). Its source is the code shown at `/getting-started`.

Routes:

| Path | What it is |
|---|---|
| `/` | Landing page — the pitch, built out of the library's own primitives |
| `/gallery` | The component index: all 84, grouped by tier, with a sidebar filter |
| `/{component}` | One component's page — variants, code samples, API docs |
| `/blocks`, `/blocks/{slug}` | The 31 pre-composed block templates |
| `/theme` | Live theme customiser (edits tokens, persists to localStorage) |
| `/getting-started` | Install, first paint, theming, runtime |
| `/integrations/{slug}` | The 15 third-party widget integrations |

Every page has a live theme toggle and a `⌘K` command palette.

## Usage

Every component is a module under `maud_ui::primitives`. Each exposes:

- a `Props` struct with sensible defaults (via `Default::default()`)
- a `render(props) -> Markup` function
- a `showcase() -> Markup` function used by the gallery (demo only; consumers don't call this)

### Example: dialog with a trigger button

```rust
use maud::html;
use maud_ui::primitives::{button, dialog};

html! {
    (dialog::trigger("settings-dialog", "Open settings"))

    (dialog::render(dialog::Props {
        id: "settings-dialog".into(),
        title: "Settings".into(),
        description: Some("Adjust your preferences".into()),
        children: html! {
            p { "Your settings go here." }
            (button::render(button::Props {
                label: "Save changes".into(),
                variant: button::Variant::Primary,
                ..Default::default()
            }))
        },
        ..Default::default()
    }))
}
```

### Example: table with data

```rust
use maud::html;
use maud_ui::primitives::table;

table::render(table::Props {
    headers: vec!["Customer".into(), "Plan".into(), "MRR".into()],
    rows: vec![
        vec!["Acme Corp".into(), "Pro".into(), "$299".into()],
        vec!["Globex".into(), "Enterprise".into(), "$1,299".into()],
    ],
    striped: true,
    ..Default::default()
})
```

## Theming

Flip the theme by setting `data-theme` on `<html>`:

```html
<html data-theme="dark">   <!-- default -->
<html data-theme="light">
```

A theme toggle is included — add `<button data-mui="theme-toggle">Toggle theme</button>` anywhere and the runtime wires it up.

### Custom palette

Override any token in your CSS. Classes are prefixed `mui-` so nothing collides with your app styles.

```css
[data-theme="dark"] {
    --mui-accent: #8b5cf6;        /* violet-500 */
    --mui-accent-hover: #a78bfa;
    --mui-bg: #0c0a1d;
    --mui-text: #ede9fe;
}
```

The full token list is in [static/styles/tokens.css](static/styles/tokens.css).

### The default theme (0.18.0)

The default theme is rebuilt on the measured Linear recipe: Inter Variable at 15px body with tight tracking, near-black five-layer surfaces (`#08090a` page, `#0f1011` cards, `#1c1c1f` inputs and overlay panels, `#232326` raised menus), translucent white borders, 32px controls, and muted 12%-tint status colours. The `@font-face` points at the **relative** url `fonts/InterVariable.woff2`, so serve the bundled font — `maud_ui::assets::INTER_WOFF2` — at `<css dir>/fonts/InterVariable.woff2` next to wherever you serve maud-ui.css; if it 404s the stack falls back to system-ui. Apps carrying a `data-brand` (or their own `--mui-brand-*` overrides) keep their accent, fonts, radius and density exactly as before.

## Component reference

<details>
<summary><strong>84 components across three progressive-enhancement tiers</strong> (click to expand)</summary>

### Tier 1 — Pure HTML+CSS (works with JS disabled)

Alert • Aspect Ratio • Avatar • Badge • Breadcrumb • Button • Button Group • Card • Chart • Checkbox • Choice Card • Date Range • Diff • Empty State • Field • Fieldset • Form • Grid • Input • Item • Kbd • Label • Message • Meter • Native Select • Number Field • Pagination • Progress • Radio • Radio Group • Separator • Skeleton • Spinner • Stack • Streaming Cursor • Table • Textarea • Typography

### Tier 2 — JS-enhanced (renders without JS; full interactivity with it)

Bottom Tab Bar • Status Chip Group • Accordion • Code Block • Collapsible • Direction • Hover Card • Input Group • Input OTP • Swatch • Switch • Tabs • Toast • Toggle • Toggle Group • Tool Call • Tooltip

### Tier 3 — Requires JS for core functionality

Alert Dialog • Calendar • Carousel • Combobox • Command • Context Menu • Data Table • Date Picker • Dialog • Drawer • Menu • Menubar • Navigation Menu • Popover • Resizable • Scroll Area • Select • Sheet • Sidebar • Slider • Sonner

</details>

The authoritative, always-current list is **[`docs/components/`](docs/components/)** — one Markdown
reference per primitive, shipped inside the crate — and the [live gallery](https://maudui.herman.engineer).
`tests/registration_parity.rs` keeps those two in lockstep with the code; the tier lists above are
hand-maintained and are the one place that can still drift.

Each component's props and variants are also documented in its module — run `cargo doc --open`.

## Architecture

```
src/primitives/     # 84 component modules (Props, Variant, render(), showcase())
src/tokens.rs       # Rust constants mirroring CSS custom properties
css/                # Legacy source snapshot; edit static/styles/ for current releases
static/             # Current bundles, editable styles/ and behavior overrides
dist/               # Legacy 0.7 bundles; retained as build inputs
  ├─ maud-ui.min.css
  ├─ maud-ui.min.js
  └─ behaviors/*.js
assets/             # Brand — favicon.svg, og.png, apple-touch-icon.png (see docs/brand.md)
examples/build-assets.mjs # Builds the complete assets in static/
examples/build-social-card.mjs # SVG → static/og.png through librsvg (no browser)
examples/showcase.rs  # axum server that renders the landing page + gallery
```

Components are pure functions: `(props) -> Markup`. No state, no framework. Pair with htmx for interactivity that spans requests, or with vanilla JS for in-page behavior.

## Development

Known gaps? **[docs/audit-backlog.md](docs/audit-backlog.md)** — deferred items from the
2026-07-27 design + a11y audit, each with numbers and a cause.

Releasing? **[docs/releasing.md](docs/releasing.md)** — publishing is irreversible; the order
and the stowaway check both matter.

Adding a component? **[docs/adding-a-primitive.md](docs/adding-a-primitive.md)** — registration
touches eight files and `tests/registration_parity.rs` will name whichever one you missed.

Changing the mark, favicon or link-preview card? **[docs/brand.md](docs/brand.md)** — two of the
three traps there are invisible outside a browser.

Writing or debugging a test? **[docs/testing.md](docs/testing.md)** — `cargo test` never executes
JavaScript and never lays anything out, so `bun run test:js` runs the built bundle and stylesheet
in real Chrome. Says what is guarded, what is not, and the two traps that produce confidently
wrong measurements.

Picking a breakpoint? **[docs/breakpoints.md](docs/breakpoints.md)** — five declared values, and
the reason a breakpoint cannot be a CSS custom property.

```bash
cargo check                     # Type-check the crate
cargo test                      # Render tests for all 84 components + registration parity
ADDR=127.0.0.1:$(free-port) cargo run --example showcase # Local live showcase

# Rebuild the current artifacts. The legacy public/ export is a separate website release.
bun install
node examples/build-assets.mjs  # → static/  the bundle the crate ships
cargo run --example build_docs # → docs/{components,blocks}/rendered/
node examples/build-social-card.mjs # → static/og.{svg,png}; requires rsvg-convert
node --test tests/curation-runtime.mjs tests/record-kit-runtime.mjs tests/states-motion-runtime.mjs tests/brand-density-runtime.mjs # Handler fixtures, no browser
node --test tests/record-kit-css.mjs # Shipped-bundle cascade and responsive contracts
node tests/curation-contrast.mjs # Theme token contrast and registration
python3 examples/generate-props.py --check # Gallery Props metadata freshness
python3 examples/generate-brands.py --check # Nine brand tokens across Rust/CSS/JS
node examples/audit-states-motion.mjs # Motion and typography source contracts
```

## Tailwind

maud-ui and Tailwind coexist cleanly — see [docs/TAILWIND.md](docs/TAILWIND.md) for the pairing guide (Preflight, layer order, shared tokens, dark-mode coordination).

## License

MIT — see [LICENSE](LICENSE).

## Credits

Inspired by [Base UI](https://base-ui.com/) (headless primitives), [shadcn/ui](https://ui.shadcn.com/) (visual design), and the [WAI-ARIA Authoring Practices](https://www.w3.org/WAI/ARIA/apg/).

### 0.9 design defaults

The defaults use a 15px body, a seven-role type scale, quiet prose links, a restrained accent, and light/dark semantic palettes. Controls, cards and navigation share spacing and radius tokens. See [the complete token migration](docs/design-defaults.md) for old → new values and deliberate exceptions.

The application shell supports persistent groups, nested destinations, a desktop icon rail and a native phone drawer. `shell::page_header` adds sticky breadcrumbs, search, actions and switchers; optional `shell::app_header` and `shell::app_footer` compose above and below the whole shell and render nothing when empty. `record::header` accepts caller-rendered title/status slots.

The editable CSS source is `static/styles/maud-ui.css` and its imports. Build with `node examples/build-assets.mjs`; this regenerates all four bundles under `static/`. Legacy `css/`, `js/` and `dist/` sources remain historical snapshots. The active asset builder reads legacy runtime behaviors unless a same-named replacement exists in `static/behaviors/`, then appends new behaviors. Regenerate API docs with `cargo run --example build_docs` and run `cargo test`, `cargo clippy --all-targets -- -D warnings`, `node tests/curation-contrast.mjs`, and `node --test tests/curation-runtime.mjs`.

### 0.9.1 compact filters
`status_chip_group::Chip::count` now accepts `Option<u64>`: migrate a known `count: n` to `count: Some(n)`. `None` renders no count bubble. Omit the group for an empty collection; retain `Some(0)` only for a zero-result filter in a populated collection. Chips default to 30px in both viewport modes, and table status badges remain 22px. At 40rem and below the sidebar shell moves page-header search and switchers into its existing drawer. See [status filters](docs/components/status_chip_group.md) and [page header](docs/blocks/shell-page-header.md).

### 0.10 record kit
The gallery now includes [record timeline](docs/blocks/record-timeline.md) (also `stay_timeline`), [money](docs/blocks/record-money.md), [related entity cards](docs/blocks/record-related-card.md), [grouped worklists](docs/blocks/worklist-grouped.md), [attention banners](docs/blocks/attention-banner.md), [choice cards](docs/components/choice_card.md), [date ranges](docs/components/date_range.md) and [action rows](docs/blocks/action-row.md). Each has paired light/dark previews and a responsive layout. Values, currency, availability, group labels and milestone states remain caller supplied.

Action rows own Save/Cancel density (row/compact/comfortable); coarse pointers keep a 44px floor. `Action::submit` uses the surrounding or named form without creating a nested form; exhaustive matches on `action::Target` need a new `Submit` arm. Existing Row/Sm button APIs remain valid, with standalone row padding corrected in the emitted bundle. Native radio cards and date inputs submit without JS; live night counts and dismissible banners are optional enhancements. The initial no-JS night count is refreshed by the server on submission.

### 0.10.1 states and motion

All 13 operational blocks accept `state: blocks::state::State`, defaulting to Ready, with Loading/Empty/Error/Disabled examples on every API page. Exhaustive Props literals need the new field. Disabled preserves admitted markup in an inert subtree; authorization and form submission policy remain with the application.

The landing workspace now has Worklist and Record tabs. All 105 component/block pages use Anatomy / Props / Examples / Accessibility, with Props tables generated by `python3 examples/generate-props.py` and defaults evaluated from Rust. Dialogs, sheets and drawers share one entrance; toasts and skeletons use the two motion durations and honor reduced motion. Six dark presets have quieter control borders.

Timeline stays horizontal at ≥48rem even in narrow columns. Related cards default to Inline without facts, empty collections can use `Variant::Inline`, breadcrumbs keep root/current whole, and overflow is a small outline More disclosure. See [States and motion](docs/states-and-motion.md) for source migrations, state semantics, tokens, palette changes and application integration notes.

### 0.11 brand and density

The theme customiser switches Lodge, Bank and Clinic live, with exactly nine tokens exported as `brand.css`. The shared `shell::brand_mark` supplies logo, wordmark and tagline through typed app-header/sidebar props. Set `data-density="compact|comfortable|spacious"` on the root to scale controls, tables, card insets and type; coarse pointers retain 44px targets.

The landing now has a Banking tab with full-precision account balances, sortable transactions and a KYC queue. It defaults to compact density. Numeric sorting compares decimals exactly, including cent differences beyond JavaScript's safe integer range.

Desktop page headers stay one 56px row with a native search disclosure. Task grids fit their item count, Compact related cards retain facts, and `State::Absent` omits undeclared inputs entirely. See [Brand and density](docs/brand-and-density.md) for all nine tokens, density values, changed defaults, source migrations and consuming-app follow-ups.

### 0.12 shell frame and radius scale

Density never moves the frame. The shell fills the viewport, stretches the sidebar column behind a sticky scrolling nav, and keeps the footer at or below the viewport bottom. Masthead, page bar, gutters and chrome type use independent `--mui-shell-*` tokens. Set `data-density` on content or the document; use `.mui-page-stack` for one shared vertical gap inside custom wrappers.

Navigation uses a flat current row and an unfilled parent label with an accent chevron. One brand radius now derives Small ×0.5, Medium ×1 and Large min(×1.5, 12px); controls cap at 8px. The theme export explains and displays resolved radii. See [Shell frame](docs/shell-frame.md) for tokens, fixture pages, browser review and migration from the old direct-child sidebar layout.

```sh
cargo run --example frame_fixture # Generate nine short/long/scoped fixture pages
node --test tests/shell-frame-css.mjs # Both bundles; frame, stack, nav and clamp contracts
```
