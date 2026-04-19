# Popover

Non-modal floating panel anchored to a trigger element. Click trigger to toggle; click outside or press ESC to close. Focus is not trapped.

## Import

```rust
use maud_ui::primitives::popover::{self, Props, Side, Placement, Align};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::popover;

html! {
    (popover::render(popover::Props {
        id: "demo-pop".to_string(),
        trigger: html! { button.mui-btn.mui-btn--primary { "Open popover" } },
        content: html! { p { "This is popover content." } },
        side: Some(popover::Side::Bottom),
        align: popover::Align::Center,
        side_offset: Some(8),
        open: Some(false),
        ..Default::default()
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| id | String | `"popover"` | Unique identifier for the popover content element. |
| trigger | Markup | `html! {}` | The element (typically a button) that opens/closes the popover on click. |
| content | Markup | `html! {}` | Markup content displayed inside the popover panel. |
| side | Option<Side> | `None` | Which side of the trigger the popover should render: Top, Right, Bottom, Left. Overrides `placement` when set. |
| placement | Placement | `Bottom` | Legacy 2-way placement (Top, Bottom); prefer `side` for new code. |
| align | Align | `Center` | Horizontal alignment: Start, Center, End. |
| side_offset | Option<u32> | `None` | Pixel offset along the side axis, emitted as `data-side-offset`. |
| open | Option<bool> | `None` | Controlled visibility. `Some(true)` = render visible; `Some(false)` = render hidden; `None` = client JS controls. |

## Side Enum

New 4-way side matrix (shadcn-compatible):

| Value | Description |
|-------|-------------|
| Top | Popover above trigger. |
| Right | Popover to the right. |
| Bottom | Popover below trigger (default). |
| Left | Popover to the left. |

## Align Enum

Horizontal alignment perpendicular to side:

| Value | Description |
|-------|-------------|
| Start | Align to the left edge of trigger. |
| Center | Center horizontally on trigger (default). |
| End | Align to the right edge of trigger. |

## Placement Enum (Legacy)

Deprecated in favor of `Side`; retained for backward compatibility:

| Value | Equivalent Side |
|-------|-----------------|
| Top | Side::Top |
| Bottom | Side::Bottom |

## Helpers

### header(children: Markup)

Groups title + description with spacing; matches shadcn `PopoverHeader`.

```rust
(popover::header(html! {
    (popover::title(html! { "Dimensions" }))
    (popover::description(html! { "Set width and height." }))
}))
```

### title(children: Markup)

Semantic `<h3>` styled as the popover's primary label; matches shadcn `PopoverTitle`.

### description(children: Markup)

Muted supporting copy rendered as `<p>`; matches shadcn `PopoverDescription`.

## Accessibility

- Popover content is a `<div role="dialog">` with `tabindex="-1"` and unique `id`.
- Trigger interaction handled by client JS (toggle visibility on click, close on ESC/outside click).
- `data-state="open"` and `data-state="closed"` reflect current visibility state.
- Content is hidden via `hidden` attribute when not open (when `open: Some(false)`).

## Related

Dialog, Hover Card, Tooltip.

## Shadcn reference

https://ui.shadcn.com/docs/components/base/popover
