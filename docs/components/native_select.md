# NativeSelect

Thin wrapper around the native HTML `<select>` element with consistent styling. Provides a chevron icon overlay and integrates with the Input component's visual design.

## Import

```rust
use maud_ui::primitives::native_select::{self, NativeSelectProps, NativeOption};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::native_select;

html! {
    div class="mui-field" {
        label class="mui-label" for="country" { "Country" }
        (native_select::render(native_select::NativeSelectProps {
            name: "country".to_string(),
            id: "country".to_string(),
            options: vec![
                native_select::NativeOption {
                    value: "us".to_string(),
                    label: "United States".to_string(),
                    disabled: false,
                },
                native_select::NativeOption {
                    value: "ca".to_string(),
                    label: "Canada".to_string(),
                    disabled: false,
                },
                native_select::NativeOption {
                    value: "mx".to_string(),
                    label: "Mexico".to_string(),
                    disabled: false,
                },
            ],
            selected: Some("us".to_string()),
            disabled: false,
            placeholder: None,
        }))
    }
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| name | String | `""` | HTML `name` attribute for form submission. |
| id | String | `""` | HTML `id` attribute; should match the associated label's `for` attribute. |
| options | Vec<NativeOption> | `vec![]` | List of selectable options. |
| selected | Option<String> | `None` | Value of the currently selected option. When Some, the matching option is marked selected. |
| disabled | bool | `false` | Disables the select; user cannot interact with it. |
| placeholder | Option<String> | `None` | When Some, a hidden placeholder option is prepended with empty value and disabled/selected attributes. |

## NativeOption

| Field | Type | Description |
|-------|------|-------------|
| value | String | Value submitted when this option is selected. |
| label | String | Text displayed to the user. |
| disabled | bool | When true, this option cannot be selected. |

## Variants

| Variant | Configuration | Use Case |
|---------|---------------|----------|
| Default | Single selected option, no placeholder | Pre-selected choice (currency, timezone). |
| With placeholder | `placeholder: Some("Select...")` | Force user to make an explicit choice. |
| Disabled | `disabled: true` | Read-only field; user cannot change the value. |
| Disabled options | Some options have `disabled: true` | Certain choices are unavailable. |
| Multiple options | Large option list | Extensive choices (country, state, category). |

## Helpers

None. The component is a minimal semantic wrapper around the native `<select>` element.

## Accessibility

- **Label association:** Use a `<label>` element with `for` matching the select's `id`.
- **Chevron icon:** Marked with `aria-hidden="true"` so screen readers ignore the decorative SVG.
- **Native control:** Leverages native browser `<select>` behavior; keyboard navigation (arrow keys) is handled by the browser.
- **Disabled state:** Sets `disabled` attribute; screen readers announce "disabled" state.
- **Placeholder:** When present, the placeholder option is marked `disabled` and `hidden`, ensuring it cannot be selected after the user chooses another option.

## Design Notes

NativeSelect uses the native `<select>` element intentionally, **not** a custom dropdown component. This approach ensures:

1. **Browser consistency:** Users experience familiar platform-specific select behavior.
2. **Mobile optimization:** Touch devices open native picker UI.
3. **Accessibility:** Native semantics are stronger than custom components.
4. **Keyboard support:** Built-in keyboard navigation is instant, no JavaScript required.

The chevron icon is purely decorative and does not indicate open/closed state (browser handles that).

## Related

Input, Native Textarea, Label, Dropdown Menu.

## Shadcn reference

https://ui.shadcn.com/docs/components/base/select
