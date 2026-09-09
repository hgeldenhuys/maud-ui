# EmptyState

Placeholder for empty content (no results, empty table, first-run). Supports both simplified render path and composable subcomponents.

## Import

```rust
use maud_ui::primitives::empty_state::{self, Props, MediaVariant, Variant};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::empty_state;

// Simple render path
html! {
    (empty_state::render(
        empty_state::Props::new("No results found")
            .with_icon("🔍")
            .with_description("Try adjusting your search.")
            .with_action(html! {
                button class="mui-btn mui-btn--outline mui-btn--md" { "Clear filters" }
            })
    ))
}

// Composable subcomponent path
html! {
    (empty_state::compose(html! {
        (empty_state::header(html! {
            (empty_state::media(html! { "📦" }, empty_state::MediaVariant::Default))
            (empty_state::title("No packages installed"))
            (empty_state::description("Install a package to start."))
        }))
        (empty_state::content(html! {
            button class="mui-btn mui-btn--default mui-btn--md" { "Install" }
        }))
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `variant` | `Variant` | `Empty` | Empty, Filtered, Failed or compact Inline presentation. |
| `icon` | `Option<String>` | `None` (reason-specific glyph) | Optional icon (text or emoji) |
| `title` | `String` | `Nothing here yet` via Default | Main heading; new(title) supplies caller copy. |
| `description` | `Option<String>` | `None` | Optional description text |
| `action` | `Option<Markup>` | `None` | Optional action markup (e.g., button) |

### Props Builder Methods

- `for_variant(variant) → Self` — Distinct default title and recovery description for Empty, Filtered, Failed. Failed says Could not load records and uses an error border/icon.
- `with_variant(variant) → Self` — Change presentation while retaining caller-owned title/description.
- `new(title) → Self` — Create with title only
- `with_icon(icon) → Self` — Add icon
- `with_description(text) → Self` — Add description
- `with_action(markup) → Self` — Add action button/content

## Inline related collection

```rust
use maud::html;
use maud_ui::primitives::empty_state::{self, Props, Variant};
empty_state::render(Props::new("No payments yet")
    .with_variant(Variant::Inline)
    .with_action(html! { a href="/payments/new" { "Add payment" } }));
```

Inline renders one quiet paragraph plus its action; it omits icon, description and heading even when supplied. Put the collection heading in the parent section. `Props::for_variant(Variant::Inline)` defaults to “No related items yet”. Add an Inline arm to exhaustive Variant matches.

## MediaVariant Enum

| Variant | Size | Use Case |
|---------|------|----------|
| `Default` | Larger (~4rem box) | Illustrations, oversized emoji |
| `Icon` | Smaller (~2rem box) | lucide-style icon glyphs |

### `MediaVariant::as_class_suffix() → &'static str`
Returns `"default"` or `"icon"` — appended to `mui-empty-state__icon--{suffix}`.

## Composition API

Use when you need custom layout or non-text media (SVG/image):

### `compose(children) → Markup`
Root wrapper. Render header/content subcomponents inside.

### `header(children) → Markup`
Container for media + title + description.

### `media(children, variant) → Markup`
Icon/illustration slot with variant-specific sizing. `aria-hidden="true"`.

### `title(text) → Markup`
Renders an `<h2>` heading.

### `description(text) → Markup`
Renders a `<p>` description paragraph.

### `content(children) → Markup`
Slot for actions and additional body content.

## Features

- **Dual paths**: `render()` for simple cases; `compose()` + subcomponents for custom layouts
- **Semantic HTML**: Uses `<h3>` in render and `<h2>` in the title helper, `<p>` for description
- **Icon variants**: Two sizes to match design rhythm
- **Builder pattern**: Props use fluent API for readable chains

## Accessibility

- Media slot has `aria-hidden="true"` (decorative)
- Title is semantic `<h3>` in render, `<h2>` in the title helper. On asynchronous replacement, announce the change from a caller-owned live region; the whole state is not an assertive alert.
- Action content is interactive

## Related

- Dialog
- Card
- Placeholder patterns

## Shadcn reference
<https://ui.shadcn.com/docs/components/empty>
