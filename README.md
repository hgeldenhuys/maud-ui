# maud-ui

Headless accessible UI components for Rust web apps.

Built on [maud](https://maud.lambda.xyz/) + [htmx](https://htmx.org/). Styled like [shadcn/ui](https://ui.shadcn.com/).

[![Crate][crate-badge]][crate]
[![License: MIT][license-badge]][license]

[crate-badge]: https://img.shields.io/crates/v/maud-ui.svg
[crate]: https://crates.io/crates/maud-ui
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license]: LICENSE

## Features

- **29 UI primitives** — buttons, forms, dialogs, menus, sliders, tabs, popovers, tooltips, and more
- **Correct accessibility** — ARIA roles, keyboard navigation, focus management, semantic HTML
- **Dark + light themes** — CSS custom properties for instant customization
- **Progressive enhancement** — all components work without JavaScript; enhanced with htmx + vanilla JS
- **Single Rust dependency** — only maud; no serde, no framework lock-in
- **Minimal runtime** — 34 KB JavaScript (minified), 39 KB CSS across all components
- **Ship pre-built artifacts** — no build step required for consumers
- **MIT licensed**

## Quick start

```bash
cargo add maud-ui
```

Run the gallery:

```bash
git clone https://git.kapable.dev/kapable/maud-ui
cd maud-ui
cargo run --example showcase
# open http://127.0.0.1:3456
```

## Usage example

Render a button and dialog in an axum handler:

```rust
use maud::{html, Markup};
use maud_ui::primitives::{button, dialog};

fn my_page() -> Markup {
    html! {
        // Trigger button
        (button::render(button::Props {
            label: "Open settings".into(),
            variant: button::Variant::Primary,
            size: button::Size::Md,
            ..Default::default()
        }))

        // Dialog trigger
        (dialog::trigger("settings-dialog", "Open settings"))

        // Dialog
        (dialog::render(dialog::Props {
            id: "settings-dialog".into(),
            title: "Settings".into(),
            description: Some("Adjust your preferences".into()),
            children: html! {
                p { "Your settings go here." }
                (button::render(button::Props {
                    label: "Save".into(),
                    variant: button::Variant::Primary,
                    ..Default::default()
                }))
            },
            ..Default::default()
        }))
    }
}
```

## Component reference

All 30 primitives are organized into three tiers based on JS requirements:

### Tier 1: Pure HTML+CSS (no JavaScript required)

| Component | Module | Props | Variants |
|-----------|--------|-------|----------|
| Avatar | `avatar` | src, alt, fallback, size | Sm, Md, Lg |
| Badge | `badge` | label, variant | Default, Success, Warning, Danger, Outline |
| Button | `button` | label, variant, size, disabled, button_type | Default, Primary, Secondary, Outline, Ghost, Danger, Link; Sm, Md, Lg, Icon |
| Fieldset | `fieldset` | legend, disabled, children | — |
| Meter | `meter` | value, min, max, low, high, optimum | — |
| Progress | `progress` | value, max, label, indeterminate | — |
| Separator | `separator` | orientation, decorative | Horizontal, Vertical |
| Switch | `switch` | name, id, label, checked, disabled | — |
| Textarea | `textarea` | name, placeholder, value, rows, id, required | — |
| Toggle | `toggle` | label, pressed, disabled, id | — |

### Tier 2: JS-enhanced (works without JS, full keyboard nav requires JS)

| Component | Module | Props | Variants |
|-----------|--------|-------|----------|
| Accordion | `accordion` | items, multiple | — |
| Alert Dialog | `alert_dialog` | id, title, description, children, open | — |
| Checkbox | `checkbox` | name, value, label, id, checked, indeterminate, disabled, required | — |
| Collapsible | `collapsible` | trigger_label, content, open, id | — |
| Context Menu | `context_menu` | id, trigger, items | — |
| Dialog | `dialog` | id, title, description, children, open | — |
| Field | `field` | label, id, description, error, required, children | — |
| Input | `input` | name, input_type, placeholder, value, id, disabled, required | Text, Email, Password, Number, Tel, Url, Search |
| Menu | `menu` | trigger_label, id, items | — |
| Number Field | `number_field` | name, value, min, max, step, disabled, required | — |
| Popover | `popover` | id, trigger, content, placement, align | Top, Bottom, Left, Right; Start, Center, End |
| Radio | `radio` | name, value, label, id, checked, disabled, required | — |
| Scroll Area | `scroll_area` | max_height, id, children | — |
| Select | `select` | name, id, options, selected, placeholder, disabled | — |
| Slider | `slider` | name, id, value, min, max, step, disabled, label, show_value | — |
| Tabs | `tabs` | tabs, default_active, aria_label | — |
| Toggle Group | `toggle_group` | items, multiple, disabled, aria_label | — |
| Tooltip | `tooltip` | content, placement, delay_ms, trigger, id | Top, Bottom, Left, Right |

### Tier 3: Full interaction (JavaScript required for core functionality)

| Component | Module | Props | Variants |
|-----------|--------|-------|----------|
| Toast | `toast` | title, description, variant, duration_ms, id | Default, Success, Warning, Danger |

## Theming

All components use CSS custom properties (CSS variables) for colors, spacing, shadows, and fonts. Override them via `data-theme` attributes or class selectors:

```css
/* Override for a theme variant */
[data-theme="custom"] {
  --mui-accent: #8b5cf6;
  --mui-accent-hover: #a78bfa;
  --mui-bg: #1e1e2e;
  --mui-text: #cdd6f4;
}
```

### CSS Custom Properties

**Colors (auto-switched dark/light):**
- `--mui-bg` — background color
- `--mui-bg-card` — card/elevated background
- `--mui-bg-overlay` — semi-transparent overlay
- `--mui-bg-input` — input field background
- `--mui-border` — border color
- `--mui-border-hover` — border on hover/focus
- `--mui-border-focus` — focus ring color
- `--mui-text` — primary text color
- `--mui-text-muted` — secondary text
- `--mui-text-subtle` — tertiary text
- `--mui-accent` — primary interactive color
- `--mui-accent-hover` — accent on hover/focus
- `--mui-accent-fg` — text color on accent background
- `--mui-success` — success state color
- `--mui-warning` — warning state color
- `--mui-danger` — danger/error state color

**Spacing:**
- `--mui-radius-sm` — small border radius (form fields)
- `--mui-radius-md` — medium border radius
- `--mui-radius-lg` — large border radius
- `--mui-radius-full` — fully rounded (pills)

**Typography:**
- `--mui-font-sans` — system font stack
- `--mui-font-mono` — monospace font stack

**Effects:**
- `--mui-shadow-sm`, `--mui-shadow-md`, `--mui-shadow-lg` — drop shadows
- `--mui-ring` — focus ring (computed from border-focus)
- `--mui-transition` — standard transition timing

## Design tokens

Rust code equivalents are available in `src/tokens.rs`:

```rust
use maud_ui::tokens;

let accent_color = tokens::colors::ACCENT;  // #3b82f6
let spacing_lg = tokens::spacing::LG;       // 1rem
let radius_md = tokens::radius::MD;         // 0.5rem
```

## JavaScript runtime

The maud-ui runtime provides a lightweight behavior system for progressive enhancement:

```javascript
// Components register behaviors via data-mui attributes
MaudUI.behaviors["dialog-trigger"] = (el) => {
  el.addEventListener("click", () => {
    const targetId = el.getAttribute("data-target");
    document.getElementById(targetId)?.showModal();
  });
};

// Auto-init on DOMContentLoaded and htmx:afterSwap
MaudUI.init();              // manual re-init after dynamic content
MaudUI.init(parentEl);      // re-init a subtree
```

The runtime is bundled in `dist/maud-ui.js` and automatically re-initializes components after htmx swaps via the `htmx:afterSwap` event listener.

## Development

```bash
cargo check              # Type-check the crate
cargo test               # Run tests
cargo run --example showcase  # Run the component gallery (http://127.0.0.1:3456)
```

Building the distribution artifacts:

```bash
# Bundle CSS and JS into dist/
./js/build.mjs          # Requires Node.js or deno; reads js/maud-ui.ts and css/
```

Components are added in waves. Check `examples/showcase.rs` for the latest coverage.

## Installation

As a library dependency:

```toml
[dependencies]
maud-ui = "0.1"
maud = { version = "0.27", features = ["axum"] }
```

Consumers receive pre-built CSS and JavaScript in the `dist/` directory; no build step is required.

## Architecture

- `src/primitives/` — 30 UI components, each with `Props`, `Variant` enums, and `render(props) -> Markup`
- `src/tokens.rs` — Rust design token constants (mirrors CSS custom properties)
- `css/` — Component styles organized by feature
- `js/` — Progressive enhancement behaviors and maud-ui runtime
- `dist/` — Pre-built CSS and JavaScript (minified)
- `examples/showcase.rs` — Interactive gallery of all components

## License

MIT. See [LICENSE](LICENSE) for details.

## Credits

Inspired by [Base UI](https://base-ui.com/) (headless primitives), [shadcn/ui](https://ui.shadcn.com/) (visual design), and the [WAI-ARIA Authoring Practices](https://www.w3.org/WAI/ARIA/apg/).
