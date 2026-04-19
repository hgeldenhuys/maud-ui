# HoverCard

Rich hover overlay with configurable placement and alignment for user profiles, tooltips, and detailed previews.

## Import

```rust
use maud_ui::primitives::hover_card::{self, Props, Placement, Align};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::hover_card;

html! {
    (hover_card::render(hover_card::Props {
        trigger: html! { a href="#" { "@jane" } },
        content: html! {
            div { "Jane Doe" }
            p { "Software designer" }
        },
        id: "user-card-1".to_string(),
        side: hover_card::Placement::Bottom,
        align: hover_card::Align::Center,
        ..Default::default()
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `trigger` | `Markup` | `html! {}` | The interactive element that triggers the card. |
| `content` | `Markup` | `html! {}` | Rich content displayed in the card. |
| `id` | `String` | `"hover-card"` | Unique identifier (used for card_id). |
| `open_delay_ms` | `u32` | `300` | Milliseconds before card shows on hover. |
| `close_delay_ms` | `u32` | `200` | Milliseconds before card hides when mouse leaves. |
| `side` | `Placement` | `Bottom` | Which side of trigger to anchor (Top, Right, Bottom, Left). |
| `align` | `Align` | `Center` | Alignment along the trigger edge (Start, Center, End). |

## Placement

```rust
pub enum Placement {
    Top,
    Right,
    Bottom,    // default
    Left,
}
```

## Align

```rust
pub enum Align {
    Start,
    Center,    // default
    End,
}
```

## Accessibility

- Trigger is a semantic link or button.
- Card hidden by default with `hidden` attribute.
- Uses `data-visible="false"` for CSS state management.
- No ARIA live region by default (async JavaScript handles visibility).

## Related

- [Tooltip](tooltip.md) — simpler text-only alternative.
- [Popover](popover.md) — interactive popovers with focus management.

## Shadcn Reference

Mirrors shadcn's HoverCard with Placement and Align variants. Positioning is CSS-driven via data attributes.
