# maud-ui

**58 headless, accessible UI components for Rust web apps.**
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
- **Source:** `github.com/hgeldenhuys/maude-ui`
- **Docs:** [docs.rs/maud-ui](https://docs.rs/maud-ui)

---

## What you get

- **58 primitives** — every shadcn/ui component plus some (data-table, resizable, hover-card, OTP input, command palette, calendar, charts).
- **Accessible by default** — ARIA roles, keyboard navigation, focus management, WCAG-AA contrast in both themes.
- **Dark + light themes** — flip `data-theme` on `<html>` and the whole tree recolors via CSS variables.
- **Progressive enhancement** — every component renders correctly without JavaScript. JS adds drag, dropdowns, keyboard shortcuts on top.
- **Tailwind-compatible** — all classes prefixed `mui-`, no collisions. [Pairing guide →](docs/TAILWIND.md)
- **One Rust dependency** — just `maud`. No serde, no framework lock-in. Works with axum, actix, rocket, or whatever you use.
- **Ship pre-built** — 46 KB JS + 78 KB CSS minified (11 KB + 11 KB gzipped). No build step required for consumers.

## 30-second tour

```bash
cargo new my-app
cd my-app
cargo add maud maud-ui
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
            ([("content-type", "text/css")], include_str!("../../path/to/maud-ui/dist/maud-ui.min.css"))
        }))
        .route("/js/maud-ui.min.js", get(|| async {
            ([("content-type", "application/javascript")], include_str!("../../path/to/maud-ui/dist/maud-ui.min.js"))
        }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

Better yet, clone the repo and run the full component gallery:

```bash
git clone https://github.com/hgeldenhuys/maude-ui
cd maude-ui
cargo run --example showcase
# open http://127.0.0.1:3457
```

The gallery has every component, a live theme toggle, per-component routes with code samples, and a `/getting-started` page.

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
<summary><strong>58 components across three tiers</strong> (click to expand)</summary>

### Tier 1 — Pure HTML+CSS (works with JS disabled)

Alert • Aspect Ratio • Avatar • Badge • Breadcrumb • Button • Button Group • Card • Chart • Checkbox • Empty State • Field • Fieldset • Input • Kbd • Label • Meter • Native Select • Number Field • Pagination • Progress • Radio • Radio Group • Separator • Skeleton • Spinner • Table • Textarea • Typography

### Tier 2 — JS-enhanced (renders without JS; full interactivity with it)

Accordion • Collapsible • Hover Card • Input Group • Input OTP • Switch • Tabs • Toast • Toggle • Toggle Group • Tooltip

### Tier 3 — Requires JS for core functionality

Alert Dialog • Calendar • Carousel • Combobox • Command • Context Menu • Data Table • Date Picker • Dialog • Drawer • Menu • Menubar • Navigation Menu • Popover • Resizable • Scroll Area • Select • Slider

</details>

Each component's props and variants are documented in its module — run `cargo doc --open` after adding the crate.

## Architecture

```
src/primitives/     # 58 component modules (Props, Variant, render(), showcase())
src/tokens.rs       # Rust constants mirroring CSS custom properties
css/                # Source styles (one file per component + maud-ui.css tokens)
dist/               # Pre-built bundles — serve these to the browser
  ├─ maud-ui.min.css
  ├─ maud-ui.min.js
  └─ behaviors/*.js
js/build.mjs        # esbuild pipeline that concatenates + minifies dist/
examples/showcase.rs  # axum server that renders the full gallery
```

Components are pure functions: `(props) -> Markup`. No state, no framework. Pair with htmx for interactivity that spans requests, or with vanilla JS for in-page behavior.

## Development

```bash
cargo check                     # Type-check the crate
cargo test                      # Run render tests for all 58 components
cargo run --example showcase    # Gallery + getting-started at :3457

# Rebuild dist/ artifacts (requires Node + esbuild)
bun install
node js/build.mjs
```

## Tailwind

maud-ui and Tailwind coexist cleanly — see [docs/TAILWIND.md](docs/TAILWIND.md) for the pairing guide (Preflight, layer order, shared tokens, dark-mode coordination).

## License

MIT — see [LICENSE](LICENSE).

## Credits

Inspired by [Base UI](https://base-ui.com/) (headless primitives), [shadcn/ui](https://ui.shadcn.com/) (visual design), and the [WAI-ARIA Authoring Practices](https://www.w3.org/WAI/ARIA/apg/).
