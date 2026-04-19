# Radio

Single radio button element for selection within a group. Must be paired with Radio Group for mutually exclusive choices.

## Import

```rust
use maud_ui::primitives::radio::{self, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::radio;

html! {
    (radio::render(radio::Props {
        id: "plan-pro".to_string(),
        name: "plan".to_string(),
        value: "pro".to_string(),
        label: "Pro".to_string(),
        description: Some("Unlimited projects".to_string()),
        checked: false,
        disabled: false,
        required: false,
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| id | String | `"radio-default"` | Unique identifier for the radio input. |
| name | String | `"radio"` | Input name attribute (groups mutually exclusive radios). |
| value | String | `"option"` | Input value attribute (sent on form submit). |
| label | String | `"Option"` | Visible label text rendered in `<span class="mui-radio__label">`. |
| description | Option<String> | `None` | Optional supporting text rendered in `<span class="mui-radio__description">`. |
| checked | bool | `false` | Whether the radio is initially checked. |
| disabled | bool | `false` | Whether the radio is disabled (aria-disabled set, visual greying). |
| required | bool | `false` | Sets `required` attribute on the underlying `<input type="radio">`. |

## States

| State | Checked | Disabled | Description |
|-------|---------|----------|-------------|
| Unchecked | false | false | Default state. |
| Checked | true | false | Selected option. |
| Disabled Unchecked | false | true | Disabled, not selected. |
| Disabled Checked | true | true | Disabled, selected (locked in place). |

## Usage

Use Radio as an individual component **only** when you need custom layout or a single radio outside a group. For standard mutually exclusive choices, prefer RadioGroup.

Individual radio in a custom layout:

```rust
html! {
    fieldset {
        (radio::render(radio::Props {
            id: "notify-all".to_string(),
            name: "notify".to_string(),
            value: "all".to_string(),
            label: "All new messages".to_string(),
            description: Some("Get notified for every message.".to_string()),
            checked: true,
            ..Default::default()
        }))
        (radio::render(radio::Props {
            id: "notify-mentions".to_string(),
            name: "notify".to_string(),
            value: "mentions".to_string(),
            label: "Mentions only".to_string(),
            description: Some("Only when someone mentions you.".to_string()),
            checked: false,
            ..Default::default()
        }))
    }
}
```

## Accessibility

- Rendered as `<label for="{id}">` containing `<input type="radio">`.
- Custom indicator: `<span class="mui-radio__indicator" aria-hidden="true">` (visual only).
- Label and description are child text nodes (automatically associated to input via label wrap).
- `disabled` attribute disables native form submission handling.
- `required` attribute enables native form validation (one option must be chosen).

## Related

RadioGroup (preferred for mutually exclusive groups), Checkbox (for independent toggles).

## Shadcn reference

https://ui.shadcn.com/docs/components/base/radio-group
