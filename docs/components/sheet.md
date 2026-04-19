# Sheet

Slide-out panel modal extending the Dialog primitive. Slides from a configurable edge (top, right, bottom, left) and uses the native `<dialog>` element for accessibility. Brand-new v0.2.1 primitive distinct from Dialog.

## Import

```rust
use maud_ui::primitives::sheet::{self, Side, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::sheet;

html! {
    (sheet::trigger("edit-profile", "Open sheet"))
    (sheet::render(sheet::Props {
        id: "edit-profile".to_string(),
        title: "Edit Profile".to_string(),
        description: Some("Update your personal information.".to_string()),
        children: html! {
            div class="mui-field" {
                label class="mui-label" { "Name" }
                input class="mui-input" type="text" {}
            }
        },
        footer: Some(html! {
            button class="mui-btn mui-btn--primary" { "Save" }
        }),
        side: sheet::Side::Right,
        show_close_button: true,
        open: false,
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| id | String | `"sheet"` | Unique identifier used by trigger to target this sheet. |
| title | String | `"Sheet"` | Sheet title displayed in the header. |
| description | Option<String> | `None` | Optional description text below the title. |
| children | Markup | `html! {}` | Body content markup. |
| footer | Option<Markup> | `None` | Optional footer markup pinned to the bottom (e.g., action buttons). |
| side | Side | `Right` | Which edge the sheet slides from: Top, Right, Bottom, Left. |
| show_close_button | bool | `true` | Whether to render a close button in the header. |
| open | bool | `false` | Initial open state (for SSR; client behaviour via `data-mui="sheet"`). |

## Side Enum

| Variant | Data Attr | Behavior |
|---------|-----------|----------|
| Top | `top` | Slides down from the top. |
| Right | `right` | Slides in from the right (default). |
| Bottom | `bottom` | Slides up from the bottom. |
| Left | `left` | Slides in from the left. |

## Helpers

### trigger

```rust
sheet::trigger(target_id: &str, label: &str) -> Markup
```

Renders a button that opens the sheet with the given `target_id`. Emits `data-mui="dialog-trigger"` (reuses dialog behaviour).

### close_button

```rust
sheet::close_button(label: &str) -> Markup
```

Renders a close button for use inside the sheet body. Typically rendered by `render()` automatically if `show_close_button: true`, but available for custom layouts.

### render

```rust
sheet::render(props: Props) -> Markup
```

Renders the sheet dialog element with all accessibility attributes wired up.

## Accessibility

- Sheet uses native `<dialog>` element with `role="dialog"`.
- `aria-labelledby` points to the sheet title; `aria-describedby` points to the description (if present).
- Close button labeled "Close" by default.
- Focus is managed by the browser's native `<dialog>` behaviour; CSS animates the slide transition.
- Tab order is trapped within the open sheet.

## CSS Classes

| Class | Purpose |
|-------|---------|
| `mui-sheet` | Root sheet container. |
| `mui-sheet--top` | Side variant class (also `--right`, `--bottom`, `--left`). |
| `mui-sheet__header` | Header region holding title and close button. |
| `mui-sheet__title` | Sheet title (`<h2>`). |
| `mui-sheet__description` | Optional description paragraph. |
| `mui-sheet__body` | Main content area (scrollable). |
| `mui-sheet__footer` | Footer region (sticky to bottom). |
| `mui-sheet__close` | Close button styling. |

## Related

Dialog, Drawer, AlertDialog.

## Shadcn reference

https://ui.shadcn.com/docs/components/base/sheet
