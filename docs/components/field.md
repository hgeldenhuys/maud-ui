# Field

Form control wrapper with label, description, error handling, and multiple layout orientations.

## Import

```rust
use maud_ui::primitives::field::{self, Props, Orientation};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::field;

html! {
    (field::render(field::Props {
        label: "Email Address".to_string(),
        id: "email".to_string(),
        description: Some("We'll never share your email.".to_string()),
        error: None,
        required: true,
        orientation: field::Orientation::Vertical,
        children: html! {
            input.mui-input type="email" id="email" name="email" placeholder="you@example.com";
        },
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `label` | `String` | `"Label"` | Label text displayed above/beside the control. |
| `id` | `String` | `"field-1"` | HTML id attribute. Links label and associates errors. |
| `description` | `Option<String>` | `None` | Helper text shown below the control. |
| `error` | `Option<String>` | `None` | Legacy single-error field (for backward compat). |
| `errors` | `Vec<String>` | `vec![]` | Multiple validation errors, rendered after `error`. |
| `required` | `bool` | `false` | Appends a red asterisk to the label. |
| `orientation` | `Orientation` | `Vertical` | Layout direction (Vertical, Horizontal, Responsive). |
| `children` | `Markup` | `<input type="text">` | The control itself (input, textarea, select, etc.). |

## Orientation

```rust
pub enum Orientation {
    Vertical,      // Stack label, control, description, error vertically
    Horizontal,    // Two-column: label left, control + meta right
    Responsive,    // Vertical on narrow, horizontal at ≥640px
}
```

## Helpers

Composable field builders for structural flexibility:

- **`label(for_id, text)`** — `<label>` bound via `for=`.
- **`description(text)`** — Helper text `<p>`.
- **`error(text)`** — Error message `<p role="alert">`.
- **`group(children)`** — Vertical stack of fields without border.
- **`legend(text)`** — `<legend>` for use in fieldset.
- **`fieldset(legend_text, children)`** — `<fieldset>` + `<legend>` wrapper.
- **`content(children)`** — Control slot separator for composed layouts.
- **`separator()`** — Visual `<hr>` divider.
- **`title(text)`** — Section heading for field groups.

## Accessibility

- Label always linked via `for` → control `id`.
- Errors render with `role="alert"` for screen reader announcement.
- Description and error ids included in `aria-describedby` when used with input.
- Required indicator marked with `aria-label="required"`.
- `data-invalid` attribute for CSS and form-framework styling hooks.

## Related

- [Input](input.md) — text/email/password/file inputs.
- [Fieldset](fieldset.md) — group of related form controls.
- [Input Group](input-group.md) — input with addons.

## Shadcn Reference

Shadows shadcn's Field pattern with orientation variants and helper-function composition.
