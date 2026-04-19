# Combobox

A searchable select component with optional grouping, multi-select mode, and auto-highlight. Extends Select with text filtering and chip rendering for multi-select.

## Import

```rust
use maud_ui::primitives::combobox::{self, ComboboxOption, ComboboxGroup, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::combobox;

html! {
    (combobox::render(combobox::Props {
        id: "framework".into(),
        name: "framework".into(),
        options: vec![
            combobox::ComboboxOption { value: "react".into(), label: "React".into() },
            combobox::ComboboxOption { value: "vue".into(), label: "Vue".into() },
        ],
        selected: Some("react".into()),
        placeholder: "Select...".into(),
        search_placeholder: "Search...".into(),
        empty_text: "No results.".into(),
        disabled: false,
        multiple: false,
        auto_highlight: true,
        show_clear: true,
        aria_invalid: false,
        groups: vec![],
        selected_values: vec![],
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | `"combobox"` | Unique element id |
| `name` | `String` | `"combobox"` | Form input name attribute |
| `options` | `Vec<ComboboxOption>` | `vec![]` | Flat list of selectable options |
| `selected` | `Option<String>` | `None` | Currently selected value (single-select) |
| `placeholder` | `String` | `"Select…"` | Trigger placeholder text when no selection |
| `search_placeholder` | `String` | `"Search…"` | Search input placeholder |
| `empty_text` | `String` | `"No results found."` | Message when filter returns no matches |
| `disabled` | `bool` | `false` | Disables the trigger and all interactions |
| `multiple` | `bool` | `false` | Multi-select mode: selected values render as chips |
| `auto_highlight` | `bool` | `false` | First filtered option gets aria-selected + data-highlighted for keyboard Enter |
| `show_clear` | `bool` | `false` | Render a small X button to clear the selection |
| `aria_invalid` | `bool` | `false` | Sets aria-invalid="true" for form validation styling |
| `groups` | `Vec<ComboboxGroup>` | `vec![]` | Grouped options; takes precedence over `options` when non-empty |
| `selected_values` | `Vec<String>` | `vec![]` | Selected values for multi-select mode (rendered as chips) |

## Accessibility

Trigger has `role="combobox"` with `aria-expanded` and `aria-haspopup="listbox"`. Dropdown list has `role="listbox"`, options have `role="option"` with `aria-selected`. Groups have `role="group"` with `aria-label`. When `auto_highlight` is true, the first matching option has `data-highlighted` and `aria-selected="true"` for keyboard activation. Search input has `aria-label="Search options"`.

## Related

Select (for non-searchable select), Dialog (for modal combobox), Label (for form integration)

## Shadcn reference

https://ui.shadcn.com/docs/components/base/combobox
