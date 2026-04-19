# Skeleton

Loading placeholder with shimmer animation. Renders as a bare `<div>` with `aria-hidden="true"` so it's skipped by screen readers. Supports three shape variants and custom width/height dimensions.

## Import

```rust
use maud_ui::primitives::skeleton::{self, Variant, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::skeleton;

// Default rect
html! {
    (skeleton::render(skeleton::Props::default()))
}

// Text line
html! {
    (skeleton::render(skeleton::Props {
        variant: skeleton::Variant::Text,
        width: Some("8rem".into()),
        height: Some("0.875rem".into()),
    }))
}

// Circle avatar
html! {
    (skeleton::render(skeleton::Props {
        variant: skeleton::Variant::Circle,
        width: Some("2.75rem".into()),
        height: Some("2.75rem".into()),
    }))
}

// Loading post card
html! {
    div style="display:flex;gap:0.75rem;" {
        // Avatar
        (skeleton::render(skeleton::Props {
            variant: skeleton::Variant::Circle,
            width: Some("2.75rem".into()),
            height: Some("2.75rem".into()),
        }))
        div style="flex:1;" {
            // Title line
            (skeleton::render(skeleton::Props {
                variant: skeleton::Variant::Text,
                width: Some("60%".into()),
                height: Some("1rem".into()),
            }))
            // Body line 1
            (skeleton::render(skeleton::Props {
                variant: skeleton::Variant::Text,
                width: Some("100%".into()),
                height: None,
            }))
            // Body line 2
            (skeleton::render(skeleton::Props {
                variant: skeleton::Variant::Text,
                width: Some("85%".into()),
                height: None,
            }))
        }
    }
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| variant | Variant | `Rect` | Shape: Text, Circle, or Rect. |
| width | Option<String> | `None` | Optional width CSS override (e.g., `"8rem"`, `"100%"`). |
| height | Option<String> | `None` | Optional height CSS override. |

## Variant Enum

| Variant | Use Case | Dimensions |
|---------|----------|------------|
| Text | Loading text lines (name, title, etc.). | Default: `1em` height, 100% width. Aspect is typically wide & shallow. |
| Circle | Loading avatars, profile pictures. | Default: square (aspect 1:1). Set both width and height equally. |
| Rect | Loading images, cards, blocks. | Default: square (aspect 1:1) or use width/height. |

## Accessibility

- Every skeleton has `aria-hidden="true"` so screen readers skip it.
- When page is fully loaded, skeletons are replaced with real content; no announcement needed.
- ARIA live regions (e.g., `aria-live="polite"`) should wrap the entire loading container so screen readers announce when content loads.

## CSS Classes

| Class | Purpose |
|-------|---------|
| `mui-skeleton` | Root skeleton element. |
| `mui-skeleton--text` | Text variant (wide, shallow shimmer). |
| `mui-skeleton--circle` | Circle variant (square aspect). |
| `mui-skeleton--rect` | Rect variant (default; square or custom dimensions). |

## Sizing

By default, skeletons are **square** (except Text, which is wider). Override with:

- `width="100%"` — stretch to container width.
- `height="1rem"` — set explicit height (common for text lines).
- Both set — define exact dimensions.

**Common patterns:**

| Use Case | Props |
|----------|-------|
| Text line (name) | `variant: Text, width: "8rem", height: "0.875rem"` |
| Body paragraph (3 lines) | Render 3x `variant: Text, width: "100%"`, last one `width: "65%"` |
| Avatar | `variant: Circle, width: "2.75rem", height: "2.75rem"` |
| Product image | `variant: Rect, width: "100%", height: "11rem"` |
| Status badge | `variant: Rect, width: "4.5rem", height: "1.25rem"` |

## Animation

The shimmer animation is defined in the CSS module (`mui-skeleton`). Customisation varies by theme. The animation plays on a loop while the skeleton is mounted.

## Related

Spinner, Card, Progress.

## Shadcn reference

https://ui.shadcn.com/docs/components/base/skeleton
