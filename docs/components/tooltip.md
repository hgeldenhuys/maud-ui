# Tooltip

Non-modal overlay that displays contextual information on hover or focus.

## Description

Tooltip renders a trigger element (button, link, icon) paired with a hidden content element that displays on hover/focus. The component supports four placements (Top, Bottom, Left, Right), alignment control (Start, Center, End), and optional side offset for fine-grained positioning. Content is shown/hidden via client-side JavaScript (not CSS-only).

## Import

```rust
use maud_ui::primitives::tooltip;
```

## Example

```rust
use maud_ui::primitives::tooltip::{Props, Placement, Align, render};
use maud::html;

let tooltip = render(Props {
    content: "Copy to clipboard".into(),
    placement: Placement::Top,
    align: Align::Center,
    trigger: html! {
        button.mui-btn.mui-btn--ghost.mui-btn--icon type="button" aria-label="Copy" {
            "📋"
        }
    },
    id: "tooltip-copy".into(),
    ..Default::default()
});
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | `""` | Tooltip text |
| `placement` | `Placement` | `Top` | Position relative to trigger (Top, Bottom, Left, Right) |
| `align` | `Align` | `Center` | Alignment along perpendicular axis (Start, Center, End) |
| `side_offset` | `Option<u32>` | `None` | Pixel offset between trigger and tooltip (emitted as data-side-offset) |
| `delay_ms` | `u32` | `500` | Milliseconds before tooltip appears |
| `trigger` | `Markup` | `html! {}` | The element that triggers the tooltip (button, link, icon) |
| `id` | `String` | `"tooltip"` | Unique identifier (used in aria-describedby) |

## Placement Enum

```rust
pub enum Placement {
    Top,
    Bottom,
    Left,
    Right,
}
```

Controls the tooltip's position relative to the trigger.

## Align Enum

```rust
pub enum Align {
    Start,              // Aligned to start (left for Top/Bottom, top for Left/Right)
    #[default]
    Center,             // Centered
    End,                // Aligned to end (right for Top/Bottom, bottom for Left/Right)
}
```

Controls alignment along the perpendicular axis of placement.

## Variants

### Top Placement
Tooltip appears above the trigger, centered by default.

```rust
render(Props {
    content: "Copy to clipboard".into(),
    placement: Placement::Top,
    align: Align::Center,
    ..Default::default()
})
```

### Bottom Placement
Tooltip appears below the trigger.

```rust
render(Props {
    content: "View full list".into(),
    placement: Placement::Bottom,
    ..Default::default()
})
```

### Left Placement
Tooltip appears to the left of the trigger.

```rust
render(Props {
    content: "Keyboard shortcut: ⌘K".into(),
    placement: Placement::Left,
    ..Default::default()
})
```

### Right Placement
Tooltip appears to the right of the trigger.

```rust
render(Props {
    content: "Delete permanently".into(),
    placement: Placement::Right,
    ..Default::default()
})
```

### Align Start
Tooltip aligned to the start of the perpendicular axis.

```rust
render(Props {
    content: "Aligned to start".into(),
    placement: Placement::Bottom,
    align: Align::Start,
    ..Default::default()
})
```

### Align Center (Default)
Tooltip centered on the perpendicular axis.

```rust
render(Props {
    content: "Centered tooltip".into(),
    align: Align::Center,
    ..Default::default()
})
```

### Align End
Tooltip aligned to the end of the perpendicular axis.

```rust
render(Props {
    content: "Aligned to end".into(),
    placement: Placement::Bottom,
    align: Align::End,
    ..Default::default()
})
```

### Custom Side Offset
Emit `data-side-offset` for client-side positioning code to use.

```rust
render(Props {
    content: "Offset 12px".into(),
    side_offset: Some(12),
    ..Default::default()
})
```

### Custom Delay
Control how long before the tooltip appears.

```rust
render(Props {
    content: "Appear quickly".into(),
    delay_ms: 200,
    ..Default::default()
})
```

### Icon Button Trigger
Common pattern for copy-to-clipboard or action hints.

```rust
render(Props {
    content: "Copy to clipboard".into(),
    placement: Placement::Top,
    trigger: html! {
        button.mui-btn.mui-btn--ghost.mui-btn--icon type="button" aria-label="Copy" {
            span aria-hidden="true" { "📋" }
        }
    },
    id: "tooltip-copy".into(),
    ..Default::default()
})
```

### Avatar Stack Tooltip
Show count of hidden members.

```rust
render(Props {
    content: "View full list (12 members)".into(),
    placement: Placement::Bottom,
    trigger: html! {
        button.mui-btn.mui-btn--outline.mui-btn--sm type="button" aria-label="+3 more members" {
            "+3"
        }
    },
    id: "tooltip-avatars".into(),
    ..Default::default()
})
```

### Keyboard Shortcut Hint
Hint users about available shortcuts.

```rust
render(Props {
    content: "Keyboard shortcut: ⌘K".into(),
    placement: Placement::Left,
    delay_ms: 400,
    trigger: html! {
        button.mui-btn.mui-btn--outline type="button" aria-label="Open search" {
            "🔍 Search"
        }
    },
    id: "tooltip-search".into(),
    ..Default::default()
})
```

## Accessibility

- **Aria-Describedby:** The trigger has `aria-describedby` pointing to the tooltip's `id`.
- **Role Tooltip:** The content element has `role="tooltip"`.
- **Hidden/Visible:** The tooltip is `hidden` and `data-visible="false"` until shown by client-side script.
- **Trigger Label:** Always provide a descriptive label on the trigger button or link.
- **Delay:** Default 500ms delay reduces tooltip spam on fast hovers.
- **Dismiss:** Tooltips hide on blur or pointer leave (not user-controlled dismiss).

## Data Attributes

- `data-side-offset`: When set, contains the offset value in pixels for positioning code to consume.
- `data-delay`: Contains the delay duration in milliseconds.
- `data-visible`: Client-side script sets to "true" when tooltip is shown, "false" when hidden.

## Related

- [Popover](/docs/components/popover.md) — Dismissible overlay with more interaction options
- [Dialog](/docs/components/dialog.md) — Modal overlay with richer content
- [Hover Card](/docs/components/hover-card.md) — Rich content on hover
- [Button](/docs/components/button.md) — Typical trigger element

## Shadcn Reference

Maud-ui Tooltip aligns with shadcn's `<Tooltip>` component, supporting Placement (Top/Bottom/Left/Right) and Align (Start/Center/End) options plus optional side offset for custom positioning.
