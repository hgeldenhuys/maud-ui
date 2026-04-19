# Select

Dropdown select component with support for flat option lists, grouped options, size variants, and invalid state. Native HTML `<select>`-equivalent with custom styling.

## Import

```rust
use maud_ui::primitives::select::{self, Props, Size, SelectOption, SelectGroup};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::select;

html! {
    (select::render(select::Props {
        name: "theme".to_string(),
        id: "select-theme".to_string(),
        options: vec![
            select::SelectOption { value: "light".to_string(), label: "Light".to_string(), disabled: false },
            select::SelectOption { value: "dark".to_string(), label: "Dark".to_string(), disabled: false },
        ],
        selected: Some("light".to_string()),
        placeholder: "Choose theme…".to_string(),
        size: select::Size::Default,
        aria_invalid: false,
        ..Default::default()
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| name | String | `"select"` | Input name attribute (sent on form submit). |
| id | String | `"select"` | Unique identifier for the select button. |
| options | Vec<SelectOption> | `[]` | Flat list of options. Ignored if `groups` is non-empty. |
| groups | Vec<SelectGroup> | `[]` | Grouped options (overrides `options` if provided). |
| selected | Option<String> | `None` | Value of the pre-selected option (matches `SelectOption.value`). |
| placeholder | String | `"Select…"` | Placeholder text shown when no option is selected. |
| disabled | bool | `false` | If true, select button is disabled. |
| size | Size | `Default` | Size variant: Default or Sm (small). |
| aria_invalid | bool | `false` | If true, sets `aria-invalid="true"` on the trigger for error styling. |

## SelectOption Struct

| Field | Type | Description |
|-------|------|-------------|
| value | String | Option value (sent on form submit). |
| label | String | Visible option label. |
| disabled | bool | If true, option cannot be selected (grayed out). |

## SelectGroup Struct

| Field | Type | Description |
|-------|------|-------------|
| label | String | Group heading (rendered as `aria-labelledby`). |
| options | Vec<SelectOption> | Options within this group. |

## Size Enum

| Value | Class | Description |
|-------|-------|-------------|
| Default | `mui-select__trigger` | Default size. |
| Sm | `mui-select__trigger mui-select--sm` | Small, compact size. |

## Helpers

### scroll_up_button() -> Markup

Renders the up-scroll affordance for long dropdowns (` ▲ `). Decorative, `aria-hidden="true"`.

```rust
(select::scroll_up_button())
```

### scroll_down_button() -> Markup

Renders the down-scroll affordance for long dropdowns (`▼`). Decorative, `aria-hidden="true"`.

```rust
(select::scroll_down_button())
```

### separator() -> Markup

Renders a thin horizontal rule (`<div role="separator">`) between option groups.

```rust
(select::separator())
```

## Usage Patterns

### Flat option list

```rust
html! {
    (select::render(select::Props {
        name: "priority".to_string(),
        id: "select-priority".to_string(),
        options: vec![
            select::SelectOption { value: "low".to_string(), label: "Low".to_string(), disabled: false },
            select::SelectOption { value: "medium".to_string(), label: "Medium".to_string(), disabled: false },
            select::SelectOption { value: "high".to_string(), label: "High".to_string(), disabled: false },
        ],
        placeholder: "Set priority…".to_string(),
        ..Default::default()
    }))
}
```

### Grouped options

```rust
html! {
    (select::render(select::Props {
        name: "timezone".to_string(),
        id: "select-tz".to_string(),
        groups: vec![
            select::SelectGroup {
                label: "North America".to_string(),
                options: vec![
                    select::SelectOption { value: "est".to_string(), label: "Eastern".to_string(), disabled: false },
                    select::SelectOption { value: "cst".to_string(), label: "Central".to_string(), disabled: false },
                ],
            },
            select::SelectGroup {
                label: "Europe".to_string(),
                options: vec![
                    select::SelectOption { value: "gmt".to_string(), label: "London (GMT)".to_string(), disabled: false },
                ],
            },
        ],
        selected: Some("est".to_string()),
        placeholder: "Select timezone…".to_string(),
        ..Default::default()
    }))
}
```

### With error state

```rust
html! {
    (select::render(select::Props {
        name: "country".to_string(),
        id: "select-country".to_string(),
        options: vec![
            select::SelectOption { value: "us".to_string(), label: "United States".to_string(), disabled: false },
            select::SelectOption { value: "ca".to_string(), label: "Canada".to_string(), disabled: false },
        ],
        placeholder: "Select country…".to_string(),
        aria_invalid: true,  // Shows error styling
        ..Default::default()
    }))
}
```

## Accessibility

- **Trigger** is `role="combobox"` with `aria-expanded`, `aria-haspopup="listbox"`, `aria-controls`.
- **Dropdown** is `role="listbox"` with `aria-labelledby` pointing to trigger.
- **Options** are `role="option"` with `aria-selected` (true/false) and optional `aria-disabled`.
- **Option groups** are `role="group"` with `aria-labelledby` pointing to group label.
- **Selected label** is read from matched option; fallback to placeholder if none selected.
- Keyboard: Up/Down arrows move focus, Enter selects, Escape closes.

## Data Attributes

- `data-mui="select"` — root element marker.
- `data-name` — input name attribute (helps with form binding).
- `data-value` — on each option (client JS uses for selection).

## Hidden Input

A `<input type="hidden">` carries the current selection value so the form submit includes the selected value.

## Related

Combobox (searchable), Native Select (native HTML), Radio Group (mutually exclusive without dropdown).

## Shadcn reference

https://ui.shadcn.com/docs/components/base/select
