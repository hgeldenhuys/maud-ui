# Switch

Toggle control rendering as a styled button with hidden form input. Emits `data-mui="switch"` for behaviour attachment. New in v0.2.1: `size`, `aria_invalid`, `required` props.

## Import

```rust
use maud_ui::primitives::switch::{self, Size, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::switch;

// Default switch with label
html! {
    (switch::render(switch::Props {
        name: "notifications".to_string(),
        id: "sw-notifications".to_string(),
        label: "Enable notifications".to_string(),
        checked: true,
        ..Default::default()
    }))
}

// Switch with external label (no label prop)
html! {
    div style="display:flex;align-items:center;justify-content:space-between;" {
        div {
            label for="sw-marketing" style="font-weight:500;" {
                "Marketing emails"
            }
            span style="font-size:0.875rem;color:var(--mui-text-muted);" {
                "Receive emails about new products."
            }
        }
        (switch::render(switch::Props {
            name: "marketing".to_string(),
            id: "sw-marketing".to_string(),
            label: String::new(),
            checked: false,
            aria_label: Some("Marketing emails".to_string()),
            ..Default::default()
        }))
    }
}

// Disabled switch
html! {
    (switch::render(switch::Props {
        name: "airplane".to_string(),
        id: "sw-airplane".to_string(),
        label: "Airplane mode".to_string(),
        checked: false,
        disabled: true,
        ..Default::default()
    }))
}

// Compact size variant
html! {
    (switch::render(switch::Props {
        name: "compact".to_string(),
        id: "sw-compact".to_string(),
        label: "Compact mode".to_string(),
        checked: false,
        size: switch::Size::Sm,
        ..Default::default()
    }))
}

// Form integration — required + aria-invalid
html! {
    (switch::render(switch::Props {
        name: "terms".to_string(),
        id: "sw-terms".to_string(),
        label: "I agree to the terms".to_string(),
        checked: false,
        required: true,
        ..Default::default()
    }))
    (switch::render(switch::Props {
        name: "consent".to_string(),
        id: "sw-consent".to_string(),
        label: "Data consent".to_string(),
        checked: false,
        aria_invalid: true,
        ..Default::default()
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| name | String | `"switch"` | HTML name attribute (form submission). |
| id | String | `"switch"` | HTML id attribute (label linkage). |
| label | String | `"Toggle"` | Visible label text. Leave empty for external label. |
| checked | bool | `false` | Initial checked state. |
| disabled | bool | `false` | Whether the switch is disabled. |
| aria_label | Option<String> | `None` | Explicit accessible name. Falls back to `label` if not set. **Required** when `label` is empty. |
| size | Size | `Default` | Visual size variant: Default (2.75rem × 1.5rem) or Sm (2rem × 1.125rem). |
| aria_invalid | bool | `false` | Propagates to hidden `<input>` as `aria-invalid="true"`. |
| required | bool | `false` | Propagates to hidden `<input>` as `required` attribute. |

## Size Enum

| Variant | Dimensions | Use Case |
|---------|-----------|----------|
| Default | 2.75rem × 1.5rem | Standard toggle controls. |
| Sm | 2rem × 1.125rem | Compact layouts, dense forms. |

## Structure

The switch renders as a compound element:

- **Button** (`<button role="switch">`): interactive toggle with `aria-checked` and `data-mui="switch"`.
- **Thumb** (`<span class="mui-switch__thumb">`): animated visual indicator inside the button.
- **Hidden input** (`<input type="hidden">`): reflects the checked state as `value="true"` or `value="false"`. Carries `aria-invalid` and `required` attributes for form integration.
- **Label** (`<label for="...">`): external label element (only rendered if `label` is not empty).

## Accessibility

- Button has `role="switch"` and `aria-checked="true"` or `"false"`.
- `aria-label` is the accessible name (defaults to the visible `label`; required if label is empty).
- Hidden input carries form attributes: `aria-invalid`, `required`.
- Disabled state managed via `disabled` attribute on the button (CSS can style it differently than `aria-disabled`).
- Keyboard: Space/Enter toggle the switch.

## CSS Classes

| Class | Purpose |
|-------|---------|
| `mui-switch-wrap` | Outer container wrapping button, input, and label. |
| `mui-switch` | Root button element. |
| `mui-switch--sm` | Compact size variant. |
| `mui-switch__thumb` | Animated thumb/indicator inside the button. |
| `mui-switch__value` | Hidden input class. |
| `mui-switch__label` | External label element. |

## Behaviour

The companion JS behaviour (`dist/behaviors/switch.js`) handles:

- Toggling `aria-checked` and the hidden input's value on click.
- Keyboard interaction (Space / Enter to toggle).
- Optional DOM sync (e.g., updating related UI when the switch changes).

## Form Integration

The hidden input always reflects the checked state as `value="true"` or `"false"`. When the form is submitted:

```html
<input type="hidden" name="notifications" value="true" />
```

Form libraries can read and mutate this value directly. The `required` and `aria-invalid` attributes propagate to the hidden input, so client-side form validation can treat it like any other input.

## Related

Checkbox, Radio, Toggle.

## Shadcn reference

https://ui.shadcn.com/docs/components/base/switch
