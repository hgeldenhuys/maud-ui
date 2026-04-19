# Direction

Provides directional context (LTR/RTL) for descendant elements via the HTML `dir` attribute. No JS needed — `dir` cascades natively.

## Import

```rust
use maud_ui::primitives::direction::{self, Props, Dir};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::direction;

html! {
    (direction::render(direction::Props {
        dir: direction::Dir::Rtl,
        children: html! {
            p { "مرحبا بالعالم" }
            input type="text" placeholder="اكتب شيئا ما";
        },
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `dir` | `Dir` | `Dir::Ltr` | Writing direction for the wrapped subtree |
| `children` | `Markup` | — | Child markup that inherits the direction |

## Dir Enum

| Variant | Value | Description |
|---------|-------|-------------|
| `Ltr` | `"ltr"` | Left-to-right (default; English, most European languages) |
| `Rtl` | `"rtl"` | Right-to-left (Arabic, Hebrew, Persian, Urdu) |

### `Dir::as_str() → &'static str`
Returns the HTML `dir` attribute value: `"ltr"` or `"rtl"`.

## Features

- **Native cascading**: The `dir` attribute is inherited by all descendants automatically
- **Browser-native flipping**: Scrollbars, input direction, and text alignment flip automatically without CSS
- **No JS required**: Pure HTML attribute — works on any HTML element

## Accessibility

- Assistive technologies automatically adapt text direction when `dir` is set
- Screen readers respect the `dir` attribute for proper speech direction

## Related

- Composition pattern
- Locale/i18n helpers

## Shadcn reference
<https://ui.shadcn.com/docs/components/base/direction>
