# AspectRatio

Container that maintains a fixed aspect ratio for its children, useful for videos, images, and responsive layouts.

## Import

```rust
use maud_ui::primitives::aspect_ratio::{self, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::aspect_ratio;

html! {
    (aspect_ratio::render(aspect_ratio::Props::video(
        html! {
            img src="https://example.com/video-thumbnail.jpg" alt="Video" style="width: 100%; height: 100%; object-fit: cover;" {}
        }
    )))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| ratio | f64 | N/A | Aspect ratio as width/height (e.g., 16.0/9.0 for 16:9, 1.0 for square). |
| children | Markup | N/A | Child markup to be contained within the aspect ratio box. |

## Constructor Methods

- `Props::new(ratio: f64, children: Markup) -> Props` — Create with custom ratio.
- `Props::video(children: Markup) -> Props` — Create 16:9 ratio (common for video).
- `Props::four_by_three(children: Markup) -> Props` — Create 4:3 ratio.
- `Props::square(children: Markup) -> Props` — Create 1:1 square ratio.

## Accessibility

- No special ARIA attributes; the container is transparent to assistive technology.
- Images or videos placed inside should have proper `alt` text.

## Related

Image, Card, Video player components.

## Shadcn reference

https://ui.shadcn.com/docs/components/base/aspect-ratio
