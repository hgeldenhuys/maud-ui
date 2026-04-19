# Carousel

A horizontal slide viewer with optional arrow and dot controls. Supports looping and auto-play. Each slide is independently aria-labeled.

## Import

```rust
use maud_ui::primitives::carousel::{self, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::carousel;

html! {
    (carousel::render(carousel::Props {
        id: "featured".into(),
        items: vec![
            html! { div { "Slide 1" } },
            html! { div { "Slide 2" } },
            html! { div { "Slide 3" } },
        ],
        show_dots: true,
        show_arrows: true,
        loop_slides: true,
        auto_play: false,
        aria_label: "Featured items".into(),
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | `"carousel"` | Unique element id |
| `items` | `Vec<Markup>` | `vec![]` | Each slide's content as Markup |
| `show_dots` | `bool` | `true` | Show dot indicators at bottom for slide navigation |
| `show_arrows` | `bool` | `true` | Show prev/next arrow buttons |
| `loop_slides` | `bool` | `false` | Wrap around when reaching the end (circular) |
| `auto_play` | `bool` | `false` | Auto-advance slides (interval controlled by JS) |
| `aria_label` | `String` | `"Carousel"` | Accessible label for the carousel region |

## Accessibility

Emits `role="region"` with `aria-roledescription="carousel"` and the provided `aria_label`. Each slide has `role="group"` with `aria-roledescription="slide"` and `aria-label` showing position (e.g., "Slide 1 of 3"). Dot buttons have `role="tab"` with `aria-selected`. Container uses `aria-live="polite"` when `auto_play` is true. Keyboard: arrow keys navigate, Enter activates dot.

## Related

Button (for custom slide controls), Dialog (for fullscreen carousel variants)

## Shadcn reference

https://ui.shadcn.com/docs/components/base/carousel
