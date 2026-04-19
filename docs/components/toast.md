# Toast

Transient notification component with auto-dismiss, variants, and optional action buttons.

## Description

Toast renders a temporary notification message. It supports four variants (Info, Success, Warning, Danger) with appropriate ARIA roles and auto-dismiss timers. An optional `action()` helper allows embedding inline action buttons (e.g., "Undo", "Retry"). Toasts can be rendered statically or dispatched imperatively via JavaScript.

**Note:** shadcn deprecated the original Toast primitive in favor of Sonner (positioned viewport + richer API). For new call sites, prefer [`super::sonner`], which re-exports this module and adds a Position enum for viewport placement. This module is retained for backward compatibility.

## Import

```rust
use maud_ui::primitives::toast;
```

## Example

```rust
use maud_ui::primitives::toast::{Props, Variant, render, action, viewport};

// Static toast (inline)
let success = render(Props {
    title: "Invoice sent".into(),
    description: Some("Invoice #1042 has been emailed.".into()),
    variant: Variant::Success,
    duration_ms: 5000,
    id: "toast-1".into(),
});

// Toast with action
let undo_toast = html! {
    div.mui-toast.mui-toast--info role="status" aria-live="polite" {
        div.mui-toast__title { "Message archived" }
        div.mui-toast__description { "You can restore it from the archive view." }
        (action("Undo", "MaudUI.toastDismiss('toast-1')"))
        button type="button" class="mui-toast__close" aria-label="Dismiss" { "×" }
    }
};

// Imperative viewport (for programmatic toasts)
let vp = viewport();
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `String` | `""` | Primary message text |
| `description` | `Option<String>` | `None` | Optional secondary message |
| `variant` | `Variant` | `Info` | Visual variant (Info, Success, Warning, Danger) |
| `duration_ms` | `u32` | `5000` | Auto-dismiss timeout in milliseconds (0 = no auto-dismiss) |
| `id` | `String` | `""` | Unique identifier |

## Variant Enum

```rust
pub enum Variant {
    Info,       // Blue, role="status", aria-live="polite"
    Success,    // Green, role="status", aria-live="polite"
    Warning,    // Amber, role="status", aria-live="polite"
    Danger,     // Red, role="alert", aria-live="assertive"
}
```

Each variant has a corresponding CSS class and ARIA role:
- **Danger** uses `role="alert"` and `aria-live="assertive"` for urgent notifications.
- **Info, Success, Warning** use `role="status"` and `aria-live="polite"`.

## Helpers

### action(label, onclick)

Renders an inline action button (e.g., "Undo", "Retry") inside a toast.

```rust
pub fn action(label: &str, onclick: &str) -> Markup
```

**Parameters:**
- `label`: Button text
- `onclick`: JavaScript to execute (e.g., `"MaudUI.toastDismiss('id')"`)

**Returns:** Markup for a `<button class="mui-toast__action">` element.

```rust
(action("Retry", "MaudUI.toastRetry('upload-123')"))
```

### viewport()

Renders the container for imperative toasts (dispatched via JavaScript).

```rust
pub fn viewport() -> Markup
```

Emits a `<div id="mui-toast-viewport">` with ARIA live region attributes. Place this once in your page layout.

## Variants

### Info
Informational message (blue, polite).

```rust
render(Props {
    title: "Profile updated".into(),
    description: Some("Your display name has been saved.".into()),
    variant: Variant::Info,
    ..Default::default()
})
```

### Success
Successful action confirmation (green, polite).

```rust
render(Props {
    title: "Changes saved".into(),
    variant: Variant::Success,
    ..Default::default()
})
```

### Warning
Cautionary message (amber, polite).

```rust
render(Props {
    title: "Session expiring soon".into(),
    description: Some("Save your work now.".into()),
    variant: Variant::Warning,
    ..Default::default()
})
```

### Danger
Error or critical alert (red, assertive).

```rust
render(Props {
    title: "Payment failed".into(),
    description: Some("Please check your card details and try again.".into()),
    variant: Variant::Danger,
    ..Default::default()
})
```

### With Action Button
Embed an inline action (e.g., "Undo") next to the message.

```rust
html! {
    div.mui-toast.mui-toast--info role="status" aria-live="polite" {
        div.mui-toast__title { "Message archived" }
        div.mui-toast__description { "You can restore it from the archive view." }
        (action("Undo", "MaudUI.toast({variant:'success', title:'Restored', duration_ms:3000})"))
        button type="button" class="mui-toast__close" aria-label="Dismiss" { "×" }
    }
}
```

### No Auto-Dismiss
Set `duration_ms: 0` to prevent automatic dismissal.

```rust
render(Props {
    title: "Important notice".into(),
    duration_ms: 0,  // User must dismiss manually
    ..Default::default()
})
```

### Imperative API (JavaScript-driven)
Dispatch toasts from JavaScript using `MaudUI.toast()`:

```javascript
MaudUI.toast({
    variant: 'success',
    title: 'Changes saved',
    description: 'Your profile has been updated.',
    duration_ms: 5000
});
```

## Accessibility

- **ARIA Roles:** Danger toasts use `role="alert"` (assertive); others use `role="status"` (polite).
- **Aria-Live:** Danger toasts have `aria-live="assertive"` for urgent announcements; others have `aria-live="polite"`.
- **Dismiss Button:** Always include a close button with `aria-label="Dismiss"`.
- **Duration:** Auto-dismiss after the specified duration, but allow manual dismissal.
- **Action Buttons:** Use clear, action-oriented labels (e.g., "Undo", "Retry", "View").

## Related

- [Alert](/docs/components/alert.md) — Persistent dismissible alerts
- [Badge](/docs/components/badge.md) — Status indicators
- [Button](/docs/components/button.md) — Action triggers

## Shadcn Reference

Maud-ui Toast is deprecated in shadcn in favor of Sonner. This module is retained for backward compatibility. For new projects, use [`super::sonner`] which provides positioned viewport toasts with a richer API.
