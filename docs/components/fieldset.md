# Fieldset

Groups related form controls with an optional legend and disabled state.

## Import

```rust
use maud_ui::primitives::fieldset::{self, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::fieldset;

html! {
    (fieldset::render(fieldset::Props {
        legend: "Personal Information".to_string(),
        disabled: false,
        children: html! {
            input.mui-input type="text" placeholder="First Name";
            input.mui-input type="text" placeholder="Last Name";
        },
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `legend` | `String` | `"Fieldset"` | Legend text displayed above the group. |
| `disabled` | `bool` | `false` | Disables all controls within the fieldset. |
| `children` | `Markup` | `html! {}` | Form controls (typically Field components). |

## Accessibility

- Renders as semantic HTML `<fieldset>` + `<legend>`.
- Legend announces the group purpose to screen readers.
- Disabled state propagates to child inputs.
- Implicit `role="group"` from `<fieldset>` element.

## Related

- [Field](field.md) — individual form control with label/description/error.
- [Field Group Helper](field.md#helpers) — vertical stack without border.

## Shadcn Reference

Minimal fieldset wrapper for semantic HTML grouping. Use with Field components inside.
