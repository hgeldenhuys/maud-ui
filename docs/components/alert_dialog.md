# AlertDialog

Modal dialog with `role="alertdialog"` that cannot be dismissed via ESC or backdrop click; user must choose an explicit action.

## Import

```rust
use maud_ui::primitives::alert_dialog::{self, Size, Props};
use maud_ui::primitives::button::Variant as ButtonVariant;
```

## Example

```rust
use maud::html;
use maud_ui::primitives::alert_dialog;

html! {
    (alert_dialog::trigger("delete-dialog", "Delete", "danger"))
    (alert_dialog::render(alert_dialog::Props {
        id: "delete-dialog".to_string(),
        title: "Delete account?".to_string(),
        description: Some("This cannot be undone.".to_string()),
        children: html! {},
        footer: Some(html! {
            (alert_dialog::cancel("Cancel"))
            (alert_dialog::action("Delete", alert_dialog::ButtonVariant::Danger))
        }),
        ..Default::default()
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| id | String | `"alert-dialog"` | Unique identifier for the dialog element. |
| title | String | `"Confirm"` | Alert dialog title text. |
| description | Option<String> | `None` | Optional description text rendered below title. |
| media | Option<Markup> | `None` | Optional media slot (icon/image) rendered above the header; wrap with `alert_dialog::media()`. |
| children | Markup | `html! {}` | Optional body content markup displayed below the description. |
| footer | Option<Markup> | `None` | Optional footer markup for action buttons (Cancel + destructive action). |
| open | bool | `false` | Initial open state. |
| size | Size | `Default` | Size variant controlling max-width: Default (32rem) or Sm (24rem, compact). |

## Size Variants

| Size | Max-Width | Use Case |
|------|-----------|----------|
| Default | 32rem | Standard alert dialogs. |
| Sm | 24rem | Compact confirmations. |

## Helper Functions

- `alert_dialog::trigger(target_id: &str, label: &str, variant: &str) -> Markup` — Renders a button that opens the dialog with the given target_id and button variant class.
- `alert_dialog::media(children: Markup) -> Markup` — Wraps an icon or image in the media slot container.
- `alert_dialog::action(label: &str, variant: ButtonVariant) -> Markup` — Renders a primary action button that closes the dialog; use `ButtonVariant::Danger` for destructive actions.
- `alert_dialog::cancel(label: &str) -> Markup` — Renders a secondary cancel button (always Ghost variant).

## Accessibility

- Emits `role="alertdialog"` for assistive technology.
- Title and description IDs linked via `aria-labelledby` and `aria-describedby`.
- No backdrop dismiss or ESC handler — user must choose a button.
- Action buttons emit `data-mui-close` to signal dialog dismissal to JavaScript controller.

## Related

Dialog, Button, Button Group.

## Shadcn reference

https://ui.shadcn.com/docs/components/base/alert-dialog
