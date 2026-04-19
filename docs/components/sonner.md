# Sonner

Positioned toast viewport, shadcn's successor to the legacy `toast` primitive. Re-exports toast `Props` / `Variant` / `render` API for compatibility. Adds a `Position` enum (six placements) that drives viewport anchoring via `data-position` attribute. Brand-new v0.2.1 primitive.

## Import

```rust
use maud_ui::primitives::sonner::{self, Position, Props, Variant};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::sonner;

html! {
    // Viewport placed at bottom-right (default)
    (sonner::viewport(sonner::Position::BottomRight))
    
    // Or with explicit id for multiple independent viewports
    (sonner::viewport_with_id("my-viewport", sonner::Position::TopRight))
    
    // Render a static toast for SSR or demo
    (sonner::toast(sonner::Props {
        variant: sonner::Variant::Success,
        title: "Saved".to_string(),
        description: Some("Your changes have been stored.".to_string()),
        duration_ms: 4000,
    }))
}
```

## Props

Re-exported from `toast`. Refer to [Toast documentation](./toast.md) for complete Props details.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| variant | Variant | `Default` | Toast style: Default, Success, Error, Warning, Info. |
| title | String | (required) | Toast title/heading. |
| description | Option<String> | `None` | Optional body text. |
| duration_ms | u32 | `3000` | Time in milliseconds before auto-dismiss. |

## Position Enum

| Variant | Data Attr | Placement |
|---------|-----------|-----------|
| TopLeft | `top-left` | Top-left corner. |
| TopCenter | `top-center` | Top center. |
| TopRight | `top-right` | Top-right corner. |
| BottomLeft | `bottom-left` | Bottom-left corner. |
| BottomCenter | `bottom-center` | Bottom center. |
| BottomRight | `bottom-right` | Bottom-right corner (default). |

## Helpers

### viewport

```rust
sonner::viewport(position: Position) -> Markup
```

Renders the sonner viewport container at the given position. The element carries `id="mui-sonner-viewport"` so the behaviour file finds it without a selector walk.

### viewport_with_id

```rust
sonner::viewport_with_id(id: &str, position: Position) -> Markup
```

Renders a sonner viewport with an explicit DOM id. Use this for pages needing multiple independent viewports (rare).

### toast

```rust
sonner::toast(props: Props) -> Markup
```

Thin alias for `render(props)` mirroring shadcn's JS `sonner.toast(...)` spelling. Returns markup for a single toast node. For server-rendered static toasts, call this; for imperative dispatch, fire a `mui:sonner-toast` CustomEvent on `window`.

## Variant Styles

| Variant | Use Case |
|---------|----------|
| Default | Neutral toast. |
| Success | Success message (checkmark icon). |
| Error | Error or destructive action feedback. |
| Warning | Warning or caution message. |
| Info | Informational message. |

## Accessibility

- Viewport container has `aria-live="polite"` to announce new toasts to screen readers.
- Toast role is implicitly live-region-aware via the container.
- Toasts are announced in the order they appear; stale toasts fade silently.
- Title is the primary announcement; description is secondary.

## CSS Classes

| Class | Purpose |
|-------|---------|
| `mui-sonner` | Viewport root container. |
| `mui-toast` | Individual toast element. |
| `mui-toast--success` | Success variant styling. |
| `mui-toast--error` | Error variant styling. |
| `mui-toast--warning` | Warning variant styling. |
| `mui-toast--info` | Info variant styling. |

## Behaviour

The companion JS behaviour (`dist/behaviors/toast.js`) handles:

- Listening for `mui:sonner-toast` CustomEvents on `window`.
- Creating new toast elements and injecting them into the viewport.
- Auto-dismissing after `duration_ms`.
- Reacting to viewport `data-position` changes (live theme switching in galleries).

**Dispatching a toast from JS:**

```javascript
window.dispatchEvent(new CustomEvent('mui:sonner-toast', {
  detail: {
    variant: 'success',
    title: 'Saved',
    description: 'Your changes have been stored.',
    duration_ms: 4000
  }
}));
```

## Related

Toast (legacy), Alert, Notification.

## Shadcn reference

https://ui.shadcn.com/docs/components/base/sonner
