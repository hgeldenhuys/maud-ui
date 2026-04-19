# Spinner

Circular loading indicator. Renders as a `<span>` with `role="status"` and an accessible label. Minimal primitive with three size variants.

## Import

```rust
use maud_ui::primitives::spinner::{self, Size, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::spinner;

// Default medium spinner
html! {
    (spinner::render(spinner::Props::default()))
}

// Small spinner
html! {
    (spinner::render(spinner::Props {
        size: spinner::Size::Sm,
        label: Some("Checking".into()),
    }))
}

// Large spinner
html! {
    (spinner::render(spinner::Props {
        size: spinner::Size::Lg,
        label: Some("Loading page".into()),
    }))
}

// Inside a button
html! {
    button class="mui-btn mui-btn--primary" disabled {
        (spinner::render(spinner::Props {
            size: spinner::Size::Sm,
            label: Some("Saving".into()),
        }))
        "Saving..."
    }
}

// Page-level loading state
html! {
    div style="display:flex;flex-direction:column;align-items:center;justify-content:center;gap:0.75rem;" {
        (spinner::render(spinner::Props {
            size: spinner::Size::Lg,
            label: Some("Loading".into()),
        }))
        span style="color:var(--mui-text-muted);" {
            "Loading your data..."
        }
    }
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| size | Size | `Md` | Spinner size: Sm, Md, Lg. |
| label | Option<String> | `None` | Accessible name. Defaults to `"Loading"` if not set. |

## Size Enum

| Variant | Use Case |
|---------|----------|
| Sm | Inside buttons, inline status indicators. |
| Md | Standard loading states (default). |
| Lg | Page-level or hero loading screens. |

## Accessibility

- Root has `role="status"` to mark it as a live status region.
- `aria-label` carries the label text (defaults to `"Loading"`).
- Does **not** use `aria-busy="true"` (that applies to elements that have delayed content; the spinner itself IS the status indicator).
- Screen readers announce the spinner label when it appears and when it's removed.

## CSS Classes

| Class | Purpose |
|-------|---------|
| `mui-spinner` | Root spinner element. |
| `mui-spinner--sm` | Small size variant. |
| `mui-spinner--md` | Medium size variant (default). |
| `mui-spinner--lg` | Large size variant. |

## Animation

The spinner's animation is defined in the CSS module (`mui-spinner`). It's a circular rotation with a repeating cycle. The animation runs indefinitely until the element is removed from the DOM.

## Related

Skeleton, Progress, StatusIndicator.

## Shadcn reference

https://ui.shadcn.com/docs/components/base/spinner
