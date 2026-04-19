# Dialog

Modal dialog using native `<dialog>` element with focus trap, ESC/backdrop close, and size variants.

## Import

```rust
use maud_ui::primitives::dialog::{self, Props, Size};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::dialog;

html! {
    (dialog::trigger("demo-dlg", "Open Dialog"))
    (dialog::render(dialog::Props {
        id: "demo-dlg".to_string(),
        title: "Confirm".to_string(),
        description: Some("Are you sure?".to_string()),
        children: html! { p { "This action cannot be undone." } },
        footer: Some(html! {
            button class="mui-btn mui-btn--secondary" data-mui-close { "Cancel" }
            button class="mui-btn mui-btn--primary" { "Delete" }
        }),
        show_close_button: true,
        size: dialog::Size::Default,
        ..Default::default()
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | `"dialog"` | Unique identifier (used by trigger to open) |
| `title` | `String` | `"Dialog"` | Dialog title/heading |
| `description` | `Option<String>` | `None` | Optional description below title |
| `children` | `Markup` | — | Body content |
| `footer` | `Option<Markup>` | `None` | Optional footer markup (buttons/actions, right-aligned) |
| `open` | `bool` | `false` | Initial open state (if true, renders with `open` attribute for SSR) |
| `show_close_button` | `bool` | `true` | Whether to render × close button in header |
| `size` | `Size` | `Size::Default` | Dialog size variant |

## Size Enum

| Variant | Description |
|---------|-------------|
| `Default` | Default size (max-width 32rem, padded) |
| `Sm` | Compact size (max-width 28rem, reduced padding) |

## Helper Functions

### `trigger(target_id, label) → Markup`
Renders a button with `data-mui="dialog-trigger"` and `data-target` attributes. JS wires opening.

### `close_button(label) → Markup`
Renders a × close button with `data-mui-close` attribute. Positioned absolute in header via CSS.

## Features

- **Native `<dialog>` element**: Uses built-in browser modal behavior; ESC and backdrop click close
- **ARIA attributes**: `aria-labelledby` (title), `aria-describedby` (description), `aria-modal="true"`
- **ID linking**: Title ID auto-derived as `{id}-title`; description ID as `{id}-desc`
- **Automatic footer layout**: Footer content rendered right-aligned in footer slot

## Accessibility

- Dialog has `aria-modal="true"`
- Title and description linked via `aria-labelledby` and `aria-describedby`
- Close button has `aria-label="Close"`
- Semantic `<h2>` for title, `<p>` for description

## Related

- Drawer
- AlertDialog
- Command

## Shadcn reference
<https://ui.shadcn.com/docs/components/dialog>
