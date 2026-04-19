# Alert

Prominent callout or banner for important messages, with optional icon, description, and action slot.

## Import

```rust
use maud_ui::primitives::alert::{self, Variant, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::alert;

html! {
    (alert::render(alert::Props {
        title: "System maintenance".to_string(),
        description: Some("API will be offline from 2–3 AM UTC.".to_string()),
        variant: alert::Variant::Warning,
        icon: true,
        action: None,
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| title | String | `""` | Alert title text. |
| description | Option<String> | `None` | Optional description text rendered below title. |
| variant | Variant | `Default` | Visual variant controlling color and icon: Default, Info, Success, Warning, Danger. |
| icon | bool | `true` | Whether to render the icon glyph corresponding to the variant. |
| action | Option<Markup> | `None` | Optional action slot markup (e.g., a button) rendered top-right; wrap with `alert::action()`. |

## Variants

| Variant | Icon | Meaning |
|---------|------|---------|
| Default | ○ (circle) | Neutral informational message. |
| Info | ⓘ (circled i) | Informational message. |
| Success | ✓ (checkmark) | Positive confirmation or success. |
| Warning | ⚠ (triangle) | Caution or warning requiring attention. |
| Danger | ⚠ (triangle) | Destructive action or error state. |

## Helper Functions

- `alert::action(children: Markup) -> Markup` — Wraps arbitrary markup in the action slot container (top-right) for use with `Props { action: Some(...) }`.

## Accessibility

- Emits `role="alert"` so screen readers announce the message immediately.
- Icon marked `aria-hidden="true"` to avoid redundant announcement.
- Title and description are plain text content, no special ARIA needed.

## Related

Badge, Button, Toast.

## Shadcn reference

https://ui.shadcn.com/docs/components/base/alert
