# maud-ui

**81 headless, accessible UI components for Rust web apps — with shadcn Base UI API parity. Plus 13 block templates, a live theme customiser, a `cmd+k` command palette, and shell hooks for 15 third-party widget integrations.**
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

- **81 primitives** — every shadcn Base UI component plus extras (data-table, resizable, hover-card, OTP input, command palette, calendar, charts, colour swatch).
- **Layout containers** — `stack` (one axis), `grid` (two), and `form` (the submission contract). Every appearance prop is a closed enum, so a page is composed as a tree of containers instead of inline `style="display:flex"`. Added in 0.4.0.
- **A conversation tier** — `message`, `streaming_cursor`, `code_block` (with a built-in Rust/Bash/TS/JSON highlighter), `diff`, and `tool_call`: an AI-chat / agent surface kit.
- **13 pre-composed blocks** — auth (login/signup/2FA), dashboard stats, data-table-full, pricing tiers, settings (billing/profile/team), full sidebar shell, worklist and record headers, and a task launcher. Drop-in compositions.
- **Live theme customiser** at `/theme` — tweak every `--mui-*` token in the browser, persists to `localStorage`, exports a paste-ready `:root { … }` block. 8 Tailwind-based presets.
- **Integration shells** for 15 third-party widgets — Monaco, xyflow, Excalidraw, TipTap, Mermaid, Cytoscape, Three.js, AG Grid, Apache ECharts, Leaflet, FullCalendar, Wavesurfer.js, PDF.js, xterm.js, SortableJS. Each ships a themed chrome around the widget so the third-party canvas adopts your design tokens automatically.
- **Global `cmd+k` command palette** — fuzzy jump to any component, block, integration, or page. Indexed from the same Rust constants the sidebar uses.
- **Accessible by default** — ARIA roles, keyboard navigation, focus management, WCAG-AA contrast in both themes.
- **Dark + light themes** — flip `data-theme` on `<html>` and the whole tree recolors via CSS variables.
- **Progressive enhancement** — every component renders correctly without JavaScript. JS adds drag, dropdowns, keyboard shortcuts on top.
- **Tailwind-compatible** — all classes prefixed `mui-`, no collisions. [Pairing guide →](docs/TAILWIND.md)
- **One Rust dependency** — just `maud`. No serde, no framework lock-in. Works with axum, actix, rocket, or whatever you use.
- **Ship pre-built** — Complete minified CSS and JavaScript bundles. No build step required for consumers.

## 0.8 operational UI

Use `status_chip_group` for counted filters, `bottom_tab_bar` for mobile destinations, and `button::Size::Row` with `Variant::Outline` for compact cell actions. Blocks at `worklist::header`, `record::header`, and `task::grid` compose them into application surfaces. The sidebar shell adds `header`, `id`, and `mobile_navigation` props; `empty_state::Variant::{Empty, Filtered, Failed}` separates zero results from a failed load.

Serve `maud_ui::assets::{CSS_MIN, JS_MIN}` directly, or vendor the complete files in **static/**. The older **dist/** and **public/** trees are 0.7 snapshots and do not contain these additions. `node examples/build-assets.mjs` rebuilds the 0.8 bundles; `cargo run --example build_docs` regenerates API HTML after Markdown edits. Consumers have one direct dependency, Maud; Markdown parsing and Axum are development-only.

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

Use `bun run gallery` rather than `cargo run --example showcase` — the example hardcodes
:3456, and a port collision answers every route with an empty 404 that reads exactly like a
broken site. `scripts/gallery.sh` refuses a busy port and says which process holds it.

Routes:

| Path | What it is |
|---|---|
| `/` | Landing page — the pitch, built out of the library's own primitives |
| `/gallery` | The component index: all 72, grouped by tier, with a sidebar filter |
| `/{component}` | One component's page — variants, code samples, API docs |
| `/blocks`, `/blocks/{slug}` | The 13 pre-composed block templates |
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

The full token list is in [css/maud-ui.css](css/maud-ui.css).

## Component reference

<details>
<summary><strong>81 components across three progressive-enhancement tiers</strong> (click to expand)</summary>

### Tier 1 — Pure HTML+CSS (works with JS disabled)

Alert • Aspect Ratio • Avatar • Badge • Breadcrumb • Button • Button Group • Card • Chart • Checkbox • Diff • Empty State • Field • Fieldset • Form • Grid • Input • Item • Kbd • Label • Message • Meter • Native Select • Number Field • Pagination • Progress • Radio • Radio Group • Separator • Skeleton • Spinner • Stack • Streaming Cursor • Table • Textarea • Typography

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
src/primitives/     # 72 component modules (Props, Variant, render(), showcase())
src/tokens.rs       # Rust constants mirroring CSS custom properties
css/                # Source styles (one file per component + maud-ui.css tokens)
static/             # 0.8 pre-built bundles — serve these to the browser
dist/               # Legacy 0.7 bundles; retained as build inputs
  ├─ maud-ui.min.css
  ├─ maud-ui.min.js
  └─ behaviors/*.js
assets/             # Brand — favicon.svg, og.png, apple-touch-icon.png (see docs/brand.md)
examples/build-assets.mjs # Builds the complete 0.8 assets in static/
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
cargo test                      # Render tests for all 81 components + registration parity
ADDR=127.0.0.1:$(free-port) cargo run --example showcase # Local live showcase

# Rebuild the 0.8 artifacts. The legacy public/ export is a separate website release.
bun install
node examples/build-assets.mjs  # → static/  the 0.8 bundle the crate ships
cargo run --example build_docs # → docs/{components,blocks}/rendered/
node examples/build-social-card.mjs # → static/og.{svg,png}; requires rsvg-convert
node --test tests/curation-runtime.mjs # Handler fixtures, no browser
node tests/curation-contrast.mjs # Theme token contrast and registration
```

## Tailwind

maud-ui and Tailwind coexist cleanly — see [docs/TAILWIND.md](docs/TAILWIND.md) for the pairing guide (Preflight, layer order, shared tokens, dark-mode coordination).

## License

MIT — see [LICENSE](LICENSE).

## Credits

Inspired by [Base UI](https://base-ui.com/) (headless primitives), [shadcn/ui](https://ui.shadcn.com/) (visual design), and the [WAI-ARIA Authoring Practices](https://www.w3.org/WAI/ARIA/apg/).
