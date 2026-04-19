# ScrollArea

Custom-styled scrollbar with auto-hide behavior overlaid on native scroll. Delegates scroll control to the browser's native scrollbar to ensure full keyboard and assistive technology support.

## Import

```rust
use maud_ui::primitives::scroll_area::{self, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::scroll_area;

html! {
    (scroll_area::render(scroll_area::Props {
        max_height: "12rem".to_string(),
        id: "scroll-demo".to_string(),
        children: html! {
            div { "Scrollable content here..." }
        },
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| max_height | String | `"12rem"` | CSS value for max-height (e.g., "12rem", "200px", "50vh"). |
| id | String | `"scroll-area-default"` | Unique identifier for the viewport element. |
| children | Markup | `html! {}` | Content to scroll within the bounded area. |

## Accessibility Decision

**The viewport is the real scroll container** (has `overflow: auto`) and is exposed as a focusable `role="region"` with `aria-label="Scrollable region"`. This means:

- **Keyboard users** interact with native browser scrollbar semantics directly: PageUp/PageDown, arrow keys, Home/End.
- **Assistive technology** can read the scrollable region and navigate its content using standard scroll commands.
- **All scroll gestures** (mouse wheel, trackpad, touch scroll) work natively without custom JS event handling.
- **The custom thumb** (`.mui-scroll-area__thumb`) is purely a visual decoration that tracks native scroll position via CSS.

We keep `aria-hidden="true"` on the scrollbar track rather than promoting it to `role="scrollbar"` because:
1. The custom thumb is **not draggable/interactive**—the native scrollbar is the real control. Promoting the decoration to `role="scrollbar"` would advertise keyboard/focus semantics the decoration does not implement.
2. This matches the shadcn Base UI pattern of delegating to native scroll whenever the platform offers it, and only synthesizing ARIA scrollbar semantics when the component intercepts scroll (e.g., virtualized lists).

**If a future variant virtualizes or intercepts scroll, add a `role="scrollbar"` path** alongside `aria-valuenow` / `aria-valuemin` / `aria-valuemax` wiring.

## Usage Pattern

Wrap content that should scroll:

```rust
html! {
    div style="border: 1px solid var(--mui-border); border-radius: 0.5rem;" {
        (scroll_area::render(scroll_area::Props {
            max_height: "14rem".to_string(),
            id: "release-tags".to_string(),
            children: html! {
                div {
                    @for tag in ["v1.0.0", "v0.9.1", "v0.9.0"].iter() {
                        div style="padding: 0.5rem; border-bottom: 1px solid var(--mui-border);" {
                            (tag)
                        }
                    }
                }
            },
        }))
    }
}
```

## Structure

```html
<div class="mui-scroll-area">
  <!-- Real scroll container -->
  <div class="mui-scroll-area__viewport"
       id="scroll-area-default"
       role="region"
       aria-label="Scrollable region"
       tabindex="0">
    <!-- Content scrolls here -->
  </div>
  <!-- Visual-only custom scrollbar -->
  <div class="mui-scroll-area__scrollbar" aria-hidden="true">
    <div class="mui-scroll-area__thumb"></div>
  </div>
</div>
```

## Accessibility

- **Viewport** is `role="region"` with `aria-label="Scrollable region"` and `tabindex="0"`.
- **Scrollbar track** is `aria-hidden="true"` (decorative; native scrollbar is the control).
- **Thumb** has no role or focus handling (purely CSS decoration tracking native scroll).
- Native scroll wheel, keyboard (PageUp/PageDown, arrows), and AT scroll commands all work automatically.

## Variants

Currently, a single variant with configurable max-height. Future variants may include:
- Virtualized scrolling (for large lists)
- Horizontal scroll
- Custom scroll interception (e.g., for scroll-linked animations)

## Related

Drawer (scrollable modal), Dialog (scrollable content), Tabs (scroll container).

## Shadcn reference

https://ui.shadcn.com/docs/components/scroll-area
