# NumberField

Composite number input with decrement (−) and increment (+) buttons flanking a numeric input. Supports min/max constraints and configurable step increments.

## Import

```rust
use maud_ui::primitives::number_field::{self, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::number_field;

html! {
    div {
        label for="cart-qty" { "Quantity" }
        (number_field::render(number_field::Props {
            name: "quantity".into(),
            id: "cart-qty".into(),
            value: 2.0,
            min: Some(1.0),
            max: Some(10.0),
            step: 1.0,
            disabled: false,
            label: "Cart quantity".into(),
        }))
    }
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| name | String | `""` | HTML `name` attribute for form submission. |
| id | String | `""` | HTML `id` attribute; should match the associated label's `for` attribute. |
| value | f64 | `0.0` | Current numeric value displayed in the input. |
| min | Option<f64> | `None` | Minimum allowed value. If set, values below this are invalid. |
| max | Option<f64> | `None` | Maximum allowed value. If set, values above this are invalid. |
| step | f64 | `1.0` | Increment/decrement amount for +/− buttons and keyboard arrow keys. |
| disabled | bool | `false` | Disables the entire field; user cannot interact with input or buttons. |
| label | String | `""` | Accessibility label for the number field group. Announced by screen readers. |

## Variants

| Variant | Configuration | Use Case |
|---------|---------------|----------|
| Simple counter | `min: None, max: None` | Unbounded counter (like a quantity setter). |
| Range-bounded | `min: Some(X), max: Some(Y)` | Constrained input (age, servings, file count). |
| Fixed maximum | `min: Some(X), max: Some(X)` | Read-only field; user cannot change. |
| Large step | `step: 5.0` or `10.0` | Coarse adjustments (e.g., volume in 5% increments). |
| Decimal step | `step: 0.1` | Precise adjustments (prices, measurements). |
| Disabled | `disabled: true` | Read-only or unavailable field. |

## Helpers

None. The component renders a composite structure with an input and two buttons.

## Accessibility

- **Group role:** The outer container uses `role="group"` and `aria-label` for semantic grouping.
- **aria-disabled:** Set on the group to communicate disabled state to screen readers.
- **Button labels:** Decrement and increment buttons have explicit `aria-label="Decrement"` and `aria-label="Increment"`.
- **Input constraints:** The `<input type="number">` respects `min`, `max`, and `step` attributes; native browser validation applies.
- **Keyboard support:** 
  - Arrow keys (Up/Down) increment/decrement the value by `step`.
  - Buttons are `tabindex="-1"` (not in tab order); focus remains on the input.
- **Screen readers:** Field label, current value, and min/max constraints are all announced.

## Design Notes

### Button Behavior

The decrement (−) and increment (+) buttons are styled and positioned to flank the input field, creating a compact composite control. Buttons have `tabindex="-1"` so keyboard users interact with the input directly via arrow keys, not the buttons.

### Step Precision

The `step` prop controls:
1. Increment/decrement size for button clicks and arrow keys.
2. HTML5 validation (via `<input step>`).
3. Does NOT enforce decimal places; use CSS or JavaScript if rounding is needed.

### Form Submission

The input's `name` attribute allows the field to participate in form submission as-is. The value sent is the numeric value in the input field.

## Related

Input, Slider, Meter, Label.

## Shadcn reference

https://ui.shadcn.com/docs/components/base/input
