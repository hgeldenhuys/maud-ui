# Label

Standalone form label element for associating text with form controls. Supports required field indicators with built-in accessibility.

## Import

```rust
use maud_ui::primitives::label::{self, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::label;

html! {
    div class="mui-field" {
        (label::render(label::Props {
            text: "Email address".into(),
            html_for: Some("email-input".into()),
            required: false,
            disabled: false,
        }))
        input class="mui-input" id="email-input" type="email" placeholder="you@example.com" {}
    }
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| text | String | `""` | Label text displayed to the user. |
| html_for | Option<String> | `None` | HTML ID of the associated form control. Links the label semantically to its input via `for` attribute. |
| required | bool | `false` | When true, displays a red asterisk `*` with `aria-label="required"` for accessibility. |
| disabled | bool | `false` | When true, applies `mui-label--disabled` class for visual feedback. |

## Variants

| Variant | Configuration | Use Case |
|---------|---------------|----------|
| Default | All false | Standard label for optional fields. |
| Required | `required: true` | Label for mandatory form fields; displays required indicator. |
| Disabled | `disabled: true` | Label associated with disabled input (e.g., read-only fields). |
| Required Disabled | Both true | Combination of required and disabled states. |

## Helpers

None. The component is a minimal wrapper around the HTML `<label>` element.

## Accessibility

- **`html_for` attribute:** Creates semantic association with the form control, improving usability for all users and assistive technology.
- **Required indicator:** Uses `aria-label="required"` on the `<span>` containing the asterisk, ensuring screen readers announce the required status without redundant text.
- **Disabled state:** Applies visual class but does NOT add `disabled` attribute (only the input should be disabled). Visual styling communicates disabled state.

## Related

Input, Checkbox, Radio Button, Textarea, Native Select, Number Field.

## Shadcn reference

https://ui.shadcn.com/docs/components/base/label
