# EmptyState

Placeholder for empty content (no results, empty table, first-run). Supports both simplified render path and composable subcomponents.

## Import

```rust
use maud_ui::primitives::empty_state::{self, Props, MediaVariant};
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
| `icon` | `Option<String>` | `Some("📭")` | Optional icon (text or emoji) |
| `title` | `String` | — | Main heading |
| `description` | `Option<String>` | `None` | Optional description text |
| `action` | `Option<Markup>` | `None` | Optional action markup (e.g., button) |

### Props Builder Methods

- `new(title) → Self` — Create with title only
- `with_icon(icon) → Self` — Add icon
- `with_description(text) → Self` — Add description
- `with_action(markup) → Self` — Add action button/content

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
- **Semantic HTML**: Uses `<h2>` for title, `<p>` for description
- **Icon variants**: Two sizes to match design rhythm
- **Builder pattern**: Props use fluent API for readable chains

## Accessibility

- Media slot has `aria-hidden="true"` (decorative)
- Title is semantic `<h2>`
- Action content is interactive

## Related

- Dialog
- Card
- Placeholder patterns

## Shadcn reference
<https://ui.shadcn.com/docs/components/empty>
