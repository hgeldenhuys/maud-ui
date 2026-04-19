# Drawer

Sliding panel that anchors to a screen edge using native `<dialog>` element. Supports all sides, background scaling (Vaul-style), and grab handles.

## Import

```rust
use maud_ui::primitives::drawer::{self, Props, Side};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::drawer;

html! {
    (drawer::trigger("demo-drawer", "Open Drawer"))
    (drawer::render(drawer::Props {
        id: "demo-drawer".to_string(),
        title: "Settings".to_string(),
        description: Some("Adjust your preferences.".to_string()),
        children: html! { p { "Settings content here" } },
        side: drawer::Side::Right,
        show_close_button: true,
        should_scale_background: false,
        ..Default::default()
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | `"drawer"` | Unique identifier (used by trigger to open) |
| `title` | `String` | `"Drawer"` | Drawer title/heading |
| `description` | `Option<String>` | `None` | Optional description below title |
| `children` | `Markup` | — | Body content |
| `footer` | `Option<Markup>` | `None` | Optional footer markup pinned at bottom |
| `side` | `Side` | `Side::Right` | Which side the drawer slides from |
| `should_scale_background` | `bool` | `false` | Whether page background scales down when open (Vaul pattern). Emits `data-scale-background="true"` |
| `show_close_button` | `bool` | `true` | Whether to render × close button in header |

## Side Enum

| Variant | CSS Class | Description |
|---------|-----------|-------------|
| `Left` | `mui-drawer--left` | Slides from left edge |
| `Right` | `mui-drawer--right` | Slides from right edge (default) |
| `Top` | `mui-drawer--top` | Slides from top; renders grab handle |
| `Bottom` | `mui-drawer--bottom` | Slides from bottom; renders grab handle |

## Helper Functions

### `trigger(target_id, label) → Markup`
Renders a button with `data-mui="drawer-trigger"` and `data-target` attributes.

### `close_button(label) → Markup`
Renders a × close button with `data-mui-close` attribute. Positioned absolute in header via CSS.

## Features

- **Side anchoring**: Slides from any edge; top/bottom variants include a grab handle
- **Background scaling**: When `should_scale_background: true`, emits `data-scale-background="true"` on root. Pair with CSS/JS that toggles `body[data-drawer-scaling]` for Vaul-style scale effect
- **Native `<dialog>`**: Uses browser modal behavior; ESC and backdrop click close
- **ARIA attributes**: `aria-modal="true"`, `aria-labelledby`, `aria-describedby`

## Accessibility

- Drawer has `aria-modal="true"`
- Title and description linked via ARIA
- Close button has `aria-label="Close"`
- Grab handle (top/bottom) is decorative; no interactive role needed

## Related

- Dialog
- Collapsible
- Sheet

## Shadcn reference
<https://ui.shadcn.com/docs/components/drawer>
