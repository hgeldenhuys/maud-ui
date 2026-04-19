# DatePicker

Date input with inline calendar dropdown, range selection support, and custom formatting.

## Import

```rust
use maud_ui::primitives::date_picker::{self, Props, Mode};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::date_picker;

html! {
    (date_picker::render(date_picker::Props {
        id: "dp".to_string(),
        name: "date".to_string(),
        selected: Some((2026, 4, 20)),
        placeholder: "Pick a date".to_string(),
        mode: date_picker::Mode::Single,
        format: Some("yyyy-MM-dd".to_string()),
        ..Default::default()
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | `"date-picker"` | Unique identifier |
| `name` | `String` | `"date"` | Form field name |
| `selected` | `Option<(u32, u32, u32)>` | `None` | Selected date as (year, month, day); month is 1-based |
| `placeholder` | `String` | `"Pick a date"` | Placeholder text |
| `disabled` | `bool` | `false` | Whether the picker is disabled |
| `min_date` | `Option<(u32, u32, u32)>` | `None` | Minimum selectable date |
| `max_date` | `Option<(u32, u32, u32)>` | `None` | Maximum selectable date |
| `mode` | `Mode` | `Mode::Single` | Selection mode: `Single` or `Range` |
| `format` | `Option<String>` | `None` | Display format token (e.g., `"yyyy-MM-dd"`). Supports `yyyy`, `MM`, `dd` |

## Mode Enum

| Variant | Description |
|---------|-------------|
| `Single` | Single-date selection (default) |
| `Range` | Range selection (start + end). Interaction is a TODO; markup emits `data-mode="range"` hint |

## Features

- **Format tokens**: `yyyy` (4-digit year), `MM` (2-digit month), `dd` (2-digit day). Without format, displays long form ("April 20, 2026")
- **Hidden input**: Stores ISO-formatted date (`YYYY-MM-DD`) in a hidden `<input>` for form submission
- **Inline calendar**: Month/year navigation buttons, disabled date ranges via min/max
- **Day-of-week calculation**: Uses Tomohiko Sakamoto's algorithm; no external date library needed
- **Leap year handling**: Correct February counts for leap years

## Accessibility

- Trigger button has `aria-expanded` and `aria-haspopup="dialog"`
- Calendar buttons labeled with `aria-label` for month navigation
- Disabled dates rendered with `disabled` attribute and visually distinct

## Related

- Dialog
- Input
- Combobox

## Shadcn reference
<https://ui.shadcn.com/docs/components/date-picker>
