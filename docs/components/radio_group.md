# RadioGroup

Mutually exclusive set of radio buttons grouped under a semantic `<fieldset>` with a `<legend>`. Supports vertical/horizontal layout, density variants, and native form validation.

## Import

```rust
use maud_ui::primitives::radio_group::{self, Props, RadioOption, Orientation, Variant};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::radio_group;

html! {
    (radio_group::render(radio_group::Props {
        name: "plan".to_string(),
        label: "Select a plan".to_string(),
        options: vec![
            radio_group::RadioOption { value: "free".to_string(), label: "Free".to_string(), description: Some("Up to 3 projects".to_string()) },
            radio_group::RadioOption { value: "pro".to_string(), label: "Pro".to_string(), description: Some("Unlimited projects".to_string()) },
        ],
        selected: Some("pro".to_string()),
        orientation: radio_group::Orientation::Vertical,
        required: true,
        variant: radio_group::Variant::Default,
        ..Default::default()
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| name | String | `""` | Input name attribute (all radios share this name). |
| label | String | `""` | Fieldset legend text (semantic heading for the group). |
| options | Vec<RadioOption> | `[]` | Array of options; each becomes a radio button. |
| selected | Option<String> | `None` | Value of the pre-selected option (matches `RadioOption.value`). |
| orientation | Orientation | `Vertical` | Layout direction: Vertical (stacked) or Horizontal (inline). |
| disabled | bool | `false` | If true, all radios in the group are disabled. |
| required | bool | `false` | If true, sets `required` on each input so form validation enforces selection. |
| variant | Variant | `Default` | Density variant: Default, Comfortable (more spacing), Compact (tighter). |

## RadioOption Struct

| Field | Type | Description |
|-------|------|-------------|
| value | String | Input value (sent on form submit). |
| label | String | Visible radio label. |
| description | Option<String> | Optional supporting text below the label. |

## Orientation Enum

| Value | Description |
|-------|-------------|
| Vertical | Radios stacked vertically (default). |
| Horizontal | Radios laid out inline horizontally. |

## Variant Enum

Density axis—mirrors shadcn Base UI "size":

| Value | Class | Description |
|-------|-------|-------------|
| Default | — | Default spacing between options. |
| Comfortable | `mui-radio-group--comfortable` | More breathing room; bigger hit targets. |
| Compact | `mui-radio-group--compact` | Tighter rows for dense admin UIs. |

## Usage Patterns

### Vertical (default)

```rust
html! {
    (radio_group::render(radio_group::Props {
        name: "notify".to_string(),
        label: "Notifications".to_string(),
        options: vec![
            radio_group::RadioOption { value: "all".to_string(), label: "All messages".to_string(), description: None },
            radio_group::RadioOption { value: "mentions".to_string(), label: "Mentions only".to_string(), description: None },
        ],
        selected: None,
        orientation: radio_group::Orientation::Vertical,
        ..Default::default()
    }))
}
```

### Horizontal with Comfortable spacing

```rust
html! {
    (radio_group::render(radio_group::Props {
        name: "size".to_string(),
        label: "Size".to_string(),
        options: vec![
            radio_group::RadioOption { value: "s".to_string(), label: "S".to_string(), description: None },
            radio_group::RadioOption { value: "m".to_string(), label: "M".to_string(), description: None },
            radio_group::RadioOption { value: "l".to_string(), label: "L".to_string(), description: None },
        ],
        selected: None,
        orientation: radio_group::Orientation::Horizontal,
        variant: radio_group::Variant::Comfortable,
        ..Default::default()
    }))
}
```

### Required + Compact

```rust
html! {
    (radio_group::render(radio_group::Props {
        name: "priority".to_string(),
        label: "Priority".to_string(),
        options: vec![
            radio_group::RadioOption { value: "low".to_string(), label: "Low".to_string(), description: None },
            radio_group::RadioOption { value: "normal".to_string(), label: "Normal".to_string(), description: None },
            radio_group::RadioOption { value: "high".to_string(), label: "High".to_string(), description: None },
        ],
        selected: Some("normal".to_string()),
        required: true,
        variant: radio_group::Variant::Compact,
        ..Default::default()
    }))
}
```

## Accessibility

- Rendered as `<fieldset>` with `<legend>` for semantic grouping.
- All radio buttons share the same `name` attribute (mutually exclusive at form level).
- Each radio has unique `id="{name}-{value}"` and is labeled by `<label for="{id}">`.
- `required` attribute enables native form validation (one must be selected before submit).
- Descriptions are wrapped in `<span class="mui-radio__description">` for visual distinction.
- `aria-disabled` not set (native `disabled` attribute sufficient; fieldset-level disable affects all).

## Related

Radio (single button), Checkbox (independent toggles), Select (long option lists).

## Shadcn reference

https://ui.shadcn.com/docs/components/base/radio-group
