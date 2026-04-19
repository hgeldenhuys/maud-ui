# Input

Text input component supporting multiple HTML input types with full XSS protection via maud's escaping.

## Import

```rust
use maud_ui::primitives::input::{self, Props, InputType};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::input;

html! {
    (input::render(input::Props {
        name: "email".to_string(),
        id: "email".to_string(),
        input_type: input::InputType::Email,
        placeholder: "you@example.com".to_string(),
        required: true,
        aria_describedby: Some("email-hint".to_string()),
        ..Default::default()
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | `""` | HTML name attribute for form submission. |
| `id` | `String` | `""` | HTML id for label linkage. |
| `input_type` | `InputType` | `Text` | HTML input type variant. |
| `placeholder` | `String` | `""` | Placeholder text when empty. |
| `value` | `String` | `""` | Current input value. |
| `disabled` | `bool` | `false` | Disables user interaction. |
| `required` | `bool` | `false` | HTML required attribute. |
| `invalid` | `bool` | `false` | Sets `aria-invalid="true"`. |
| `readonly` | `bool` | `false` | Read-only state. |
| `aria_describedby` | `Option<String>` | `None` | Links to description/error element. |

## InputType

```rust
pub enum InputType {
    Text,
    Email,
    Password,
    Url,
    Tel,
    Search,
    Number,
    File,
}
```

## Security

All attribute values (name, placeholder, value, id, aria_describedby) are automatically escaped by maud's `html!` macro. User-controlled strings cannot break out of attributes.

**Recent improvement:** Migrated from `format!`-based rendering to maud's `html!` macro (fully escaped). XSS tests verify the fix.

## Accessibility

- Label must link via `for=` to input `id`.
- `aria-invalid="true"` when in error state.
- `aria-describedby` links to hint/error text if provided.
- `required` attribute honored by browsers and screen readers.

## File Input

The `InputType::File` variant renders with class `mui-input--file` for CSS customization.

## Related

- [Field](field.md) — wraps Input with label/description/error.
- [Input Group](input-group.md) — input with prefix/suffix addons.
- [Textarea](textarea.md) — multi-line text input.

## Shadcn Reference

Mirrors shadcn's Input component with all HTML5 input types and proper accessibility attributes.
