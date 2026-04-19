# Separator

Visual and semantic divider for organizing content into sections. Renders as a simple horizontal or vertical line with optional accessibility semantics.

## Import

```rust
use maud_ui::primitives::separator::{self, Props, Orientation};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::separator;

html! {
    div { "Bio section" }
    (separator::render(separator::Props {
        orientation: separator::Orientation::Horizontal,
        decorative: false,
    }))
    div { "Settings section" }
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| orientation | Orientation | `Horizontal` | Direction: Horizontal (thin line) or Vertical (tall line). |
| decorative | bool | `false` | If true, `aria-hidden="true"` (pure decoration). If false, `role="separator"`. |

## Orientation Enum

| Value | Description |
|-------|-------------|
| Horizontal | Thin horizontal line (default). |
| Vertical | Tall vertical line. |

## Decorative vs. Semantic

### Semantic (default: decorative: false)

Renders with `role="separator"` and `aria-orientation` so assistive technology recognizes it as a content boundary.

```rust
(separator::render(separator::Props {
    orientation: separator::Orientation::Horizontal,
    decorative: false,
}))
```

Use this when the separator conveys meaning (e.g., "profile section ends, settings section begins").

### Decorative (decorative: true)

Renders with `aria-hidden="true"` so screen readers skip it. Pure visual divider.

```rust
(separator::render(separator::Props {
    orientation: separator::Orientation::Horizontal,
    decorative: true,
}))
```

Use this when the separator is purely visual and the content structure is clear without it.

## Usage Patterns

### Profile card — horizontal divider between sections

```rust
html! {
    section {
        h2 { "Bio" }
        p { "Staff engineer at Kapable. Building Claude Conductor." }
    }
    (separator::render(separator::Props {
        orientation: separator::Orientation::Horizontal,
        decorative: false,
    }))
    section {
        h2 { "Settings" }
        ul {
            li { "Email: invoice@geldentech.ca" }
            li { "Two-factor: Enabled" }
        }
    }
}
```

### Auth divider — "OR" between Google sign-in and email form

```rust
html! {
    button { "Sign in with Google" }
    div style="display: flex; align-items: center; gap: 0.75rem; margin: 1rem 0;" {
        div style="flex: 1;" {
            (separator::render(separator::Props {
                orientation: separator::Orientation::Horizontal,
                decorative: true,
            }))
        }
        span style="font-weight: 500; color: var(--mui-text-muted);" { "OR" }
        div style="flex: 1;" {
            (separator::render(separator::Props {
                orientation: separator::Orientation::Horizontal,
                decorative: true,
            }))
        }
    }
    input type="email" placeholder="you@company.com" {}
}
```

### Navigation bar — vertical divider between nav and user menu

```rust
html! {
    nav style="display: flex; align-items: center; gap: 1rem;" {
        a { "Dashboard" }
        a { "Projects" }
        a { "Settings" }
        div style="flex: 1;" {}  // Spacer
        (separator::render(separator::Props {
            orientation: separator::Orientation::Vertical,
            decorative: false,
        }))
        a { "Docs" }
        button { "HG" }  // User menu
    }
}
```

## Accessibility

- **Semantic** (default): `role="separator"` with `aria-orientation="horizontal"` or `aria-orientation="vertical"`.
  - Screen readers announce it as a divider and read the orientation.
  - Useful when the separator marks a logical boundary (new section, topic change).
- **Decorative**: `aria-hidden="true"` — screen readers skip it entirely.
  - Use when content structure is already clear from headings/elements.

## CSS Classes

- `mui-separator` — base class.
- `mui-separator--horizontal` — added when `orientation: Horizontal`.
- `mui-separator--vertical` — added when `orientation: Vertical`.

## Related

Card (section container), Divider (similar concept, depends on context), Menu (item separators).

## Shadcn reference

https://ui.shadcn.com/docs/components/separator
