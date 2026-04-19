# Checkbox

A labeled checkbox input with optional description, indeterminate state, and validation styling. Wraps a standard HTML checkbox with custom indicator and text layout.

## Import

```rust
use maud_ui::primitives::checkbox::{self, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::checkbox;

html! {
    (checkbox::render(checkbox::Props {
        name: "agree".into(),
        value: "yes".into(),
        label: "I agree to the terms".into(),
        id: "terms-checkbox".into(),
        checked: false,
        description: None,
        indeterminate: false,
        disabled: false,
        required: false,
        aria_invalid: false,
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | `"checkbox"` | Form input name attribute |
| `value` | `String` | `"on"` | Form input value attribute |
| `label` | `String` | `"Checkbox"` | Displayed label text |
| `id` | `String` | `"checkbox"` | HTML id attribute |
| `checked` | `bool` | `false` | Initial checked state |
| `description` | `Option<String>` | `None` | Optional descriptive text below label |
| `indeterminate` | `bool` | `false` | Visual indeterminate state (mixed/partial selection) |
| `disabled` | `bool` | `false` | Disables input and applies disabled styling |
| `required` | `bool` | `false` | Sets required attribute for form validation |
| `aria_invalid` | `bool` | `false` | Sets aria-invalid="true" for error state styling |

## Accessibility

Checkbox input is labeled via a `<label>` element wrapping the control and text. Indeterminate state is visual only (CSS class). The `aria-invalid` attribute signals form validation errors. Keyboard: Space/Enter toggles, Tab navigates.

## Related

Label, RadioButton (for mutually exclusive options), Form (for grouping checkboxes)

## Shadcn reference

https://ui.shadcn.com/docs/components/base/checkbox
