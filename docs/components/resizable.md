# Resizable

Draggable panel dividers that enable horizontal or vertical resizing of adjacent content panels. Uses flex layout with draggable handles.

## Import

```rust
use maud_ui::primitives::resizable::{self, Props, Panel, Direction};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::resizable;

html! {
    (resizable::render(resizable::Props {
        id: "demo-resize".to_string(),
        direction: resizable::Direction::Horizontal,
        panels: vec![
            resizable::Panel {
                content: html! { div { "Sidebar" } },
                default_size: 30.0,
                min_size: Some(15.0),
            },
            resizable::Panel {
                content: html! { div { "Main content" } },
                default_size: 70.0,
                min_size: Some(30.0),
            },
        ],
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| id | String | `"resizable"` | Unique identifier for the resizable group. |
| panels | Vec<Panel> | `[]` | Array of panels to render (2+). |
| direction | Direction | `Horizontal` | Layout direction: Horizontal (side-by-side) or Vertical (stacked). |

## Panel Struct

| Field | Type | Description |
|-------|------|-------------|
| content | Markup | Panel content markup. |
| default_size | f64 | Initial flex proportion (typically 0–100 for percentages). |
| min_size | Option<f64> | Minimum allowed size in same units (default: 10.0). |

## Direction Enum

| Value | Description |
|-------|-------------|
| Horizontal | Panels side-by-side; handles are vertical dividers. |
| Vertical | Panels stacked; handles are horizontal dividers. |

## Usage Patterns

### Two-panel horizontal layout (sidebar + content)

```rust
html! {
    (resizable::render(resizable::Props {
        id: "sidebar-layout".to_string(),
        direction: resizable::Direction::Horizontal,
        panels: vec![
            resizable::Panel {
                content: html! { div { h3 { "Navigation" } } },
                default_size: 30.0,
                min_size: Some(15.0),
            },
            resizable::Panel {
                content: html! { div { p { "Main dashboard" } } },
                default_size: 70.0,
                min_size: Some(30.0),
            },
        ],
    }))
}
```

### Three-panel horizontal (file explorer layout)

```rust
html! {
    (resizable::render(resizable::Props {
        id: "editor-layout".to_string(),
        direction: resizable::Direction::Horizontal,
        panels: vec![
            resizable::Panel {
                content: html! { div { "Explorer" } },
                default_size: 20.0,
                min_size: Some(10.0),
            },
            resizable::Panel {
                content: html! { div { "Editor" } },
                default_size: 55.0,
                min_size: Some(20.0),
            },
            resizable::Panel {
                content: html! { div { "Inspector" } },
                default_size: 25.0,
                min_size: Some(10.0),
            },
        ],
    }))
}
```

### Vertical split (top + bottom)

```rust
html! {
    (resizable::render(resizable::Props {
        id: "vertical-split".to_string(),
        direction: resizable::Direction::Vertical,
        panels: vec![
            resizable::Panel {
                content: html! { div { "Preview" } },
                default_size: 60.0,
                min_size: Some(30.0),
            },
            resizable::Panel {
                content: html! { div { "Console" } },
                default_size: 40.0,
                min_size: Some(20.0),
            },
        ],
    }))
}
```

## Accessibility

- Each handle is a `<div role="separator">` with `aria-orientation` (horizontal/vertical).
- Handles are keyboard-focusable (`tabindex="0"`).
- `aria-valuenow` reflects the handle's current position (read by screen readers).
- Drag interaction is client-JS controlled (Arrow keys, mouse drag).

## Data Attributes

- `data-mui="resizable"` — root element marker.
- `data-direction` — "horizontal" or "vertical" for client JS routing.
- `data-index` — handle index (separates panels N and N+1).
- `data-min-size` — minimum size constraint (CSS unit, e.g., "15.0").

## Related

Drawer, Card (fixed layouts), Sheet (modal overlay).

## Shadcn reference

https://ui.shadcn.com/docs/components/resizable
