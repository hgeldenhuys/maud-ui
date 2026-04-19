# Toggle Group

Segmented control for single-select or multi-select options with roving tabindex keyboard navigation.

## Description

Toggle Group renders a set of toggle buttons that act like radio buttons (single-select) or checkboxes (multi-select). The component manages focus via roving tabindex: only the first pressed (or first) item receives `tabindex="0"`, others get `tabindex="-1"` for keyboard efficiency.

## Import

```rust
use maud_ui::primitives::toggle_group;
```

## Example

```rust
use maud_ui::primitives::toggle_group::{Props, GroupItem, Size, render};

let items = vec![
    GroupItem { value: "left".into(), label: "Left".into(), pressed: true },
    GroupItem { value: "center".into(), label: "Center".into(), pressed: false },
    GroupItem { value: "right".into(), label: "Right".into(), pressed: false },
];

let group = render(Props {
    items,
    multiple: false,
    aria_label: "Text alignment".into(),
    ..Default::default()
});
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `items` | `Vec<GroupItem>` | `vec![]` | List of toggle items |
| `multiple` | `bool` | `false` | Allow multiple selections (checkbox mode) or single (radio mode) |
| `disabled` | `bool` | `false` | Disable the entire group |
| `aria_label` | `String` | `"Toggle group"` | Accessible label for the group |
| `size` | `Size` | `Md` | Button size (Sm or Md) |

## GroupItem

| Field | Type | Description |
|-------|------|-------------|
| `value` | `String` | Unique identifier for the item (emitted in data-value) |
| `label` | `String` | Display text |
| `pressed` | `bool` | Whether the item is currently selected |

## Size Enum

```rust
pub enum Size {
    #[default]
    Md,    // Medium (default)
    Sm,    // Small (compact)
}
```

## Variants

### Single-Select (Radio Mode)
Only one item can be pressed at a time.

```rust
render(Props {
    items: vec![
        GroupItem { value: "left".into(), label: "Left".into(), pressed: true },
        GroupItem { value: "center".into(), label: "Center".into(), pressed: false },
        GroupItem { value: "right".into(), label: "Right".into(), pressed: false },
    ],
    multiple: false,
    aria_label: "Text alignment".into(),
    ..Default::default()
})
```

### Multi-Select (Checkbox Mode)
Multiple items can be pressed simultaneously.

```rust
render(Props {
    items: vec![
        GroupItem { value: "bold".into(), label: "B".into(), pressed: true },
        GroupItem { value: "italic".into(), label: "I".into(), pressed: false },
        GroupItem { value: "underline".into(), label: "U".into(), pressed: true },
    ],
    multiple: true,
    aria_label: "Text formatting".into(),
    ..Default::default()
})
```

### Small Size
Compact buttons for dense layouts (calendars, view switchers).

```rust
render(Props {
    items: vec![
        GroupItem { value: "day".into(), label: "Day".into(), pressed: false },
        GroupItem { value: "week".into(), label: "Week".into(), pressed: true },
        GroupItem { value: "month".into(), label: "Month".into(), pressed: false },
    ],
    size: Size::Sm,
    aria_label: "Calendar view".into(),
    ..Default::default()
})
```

### Medium Size (Default)
Standard button size.

```rust
render(Props {
    items,
    size: Size::Md,
    ..Default::default()
})
```

### Disabled Group
All items in the group are disabled.

```rust
render(Props {
    items,
    disabled: true,
    ..Default::default()
})
```

## Common Patterns

### Text Alignment Picker
Single-select group for paragraph alignment in an editor.

```rust
render(Props {
    items: vec![
        GroupItem { value: "left".into(), label: "Left".into(), pressed: true },
        GroupItem { value: "center".into(), label: "Center".into(), pressed: false },
        GroupItem { value: "right".into(), label: "Right".into(), pressed: false },
        GroupItem { value: "justify".into(), label: "Justify".into(), pressed: false },
    ],
    aria_label: "Text alignment".into(),
    ..Default::default()
})
```

### Calendar View Switcher
Single-select for Day/Week/Month layouts.

```rust
render(Props {
    items: vec![
        GroupItem { value: "day".into(), label: "Day".into(), pressed: false },
        GroupItem { value: "week".into(), label: "Week".into(), pressed: true },
        GroupItem { value: "month".into(), label: "Month".into(), pressed: false },
    ],
    aria_label: "Calendar view".into(),
    ..Default::default()
})
```

### Text Formatting Controls
Multi-select for Bold/Italic/Underline/Strikethrough.

```rust
render(Props {
    items: vec![
        GroupItem { value: "bold".into(), label: "B".into(), pressed: true },
        GroupItem { value: "italic".into(), label: "I".into(), pressed: false },
        GroupItem { value: "underline".into(), label: "U".into(), pressed: false },
        GroupItem { value: "strike".into(), label: "S".into(), pressed: false },
    ],
    multiple: true,
    aria_label: "Text formatting".into(),
    ..Default::default()
})
```

## Accessibility

- **Roving Tabindex:** Only the first pressed (or first) item is in the DOM tab order (`tabindex="0"`). Others have `tabindex="-1"` for efficient keyboard navigation.
- **Aria-Pressed:** Each button emits `aria-pressed="true"` or `"false"`.
- **Role:** The container has `role="group"` and an `aria-label`.
- **Data Attributes:** Each button has `data-value` containing its value for easy selection in JavaScript.
- **Disabled:** When the entire group is disabled, `data-disabled="true"` is set on the container.

## Related

- [Toggle](/docs/components/toggle.md) — Single toggle button
- [Button](/docs/components/button.md) — Standard button
- [Radio Group](/docs/components/radio-group.md) — Alternative single-select pattern
- [Checkbox Group](/docs/components/checkbox-group.md) — Alternative multi-select pattern

## Shadcn Reference

Maud-ui Toggle Group aligns with shadcn's `<ToggleGroup>` component, supporting both single-select and multi-select modes with size variants.
