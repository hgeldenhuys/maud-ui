# Command

Command palette with search, grouped items, and keyboard navigation (Cmd+K pattern).

## Import

```rust
use maud_ui::primitives::command::{self, Props, CommandItem};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::command;

let items = vec![
    command::CommandItem {
        label: "New File".to_string(),
        shortcut: Some("⌘N".to_string()),
        group: Some("Actions".to_string()),
        disabled: false,
    },
];

html! {
    (command::render(command::Props {
        id: "cmd".to_string(),
        items,
        placeholder: "Type a command...".to_string(),
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | `"command"` | Unique identifier for the dialog |
| `items` | `Vec<CommandItem>` | `vec![]` | List of command items |
| `placeholder` | `String` | `"Type a command or search…"` | Placeholder text in search input |

## CommandItem Struct

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `label` | `String` | — | Display text for the command |
| `shortcut` | `Option<String>` | `None` | Optional keyboard shortcut (e.g., "⌘N") |
| `group` | `Option<String>` | `None` | Optional group name for categorization |
| `disabled` | `bool` | `false` | Whether the item is disabled |

## Helper Functions

### `trigger(target_id, label) → Markup`
Renders a button that opens the command palette dialog. Data attributes wire JS trigger behavior.

### `shortcut(children) → Markup`
Renders a `<kbd>` element styled for keyboard shortcuts. Use when composing custom item markup.

### `separator() → Markup`
Renders a visual divider between command groups.

### `empty(text) → Markup`
Renders a no-results empty state. Use when a search yields no matches.

## Accessibility

- Dialog has `aria-label="Command palette"` and `aria-modal="true"`
- Search input has `aria-label="Search commands"`
- Items are `role="option"` with `tabindex="-1"` in a `role="listbox"` container
- Disabled items get `aria-disabled="true"`

## Related

- Dialog
- Menu
- Combobox

## Shadcn reference
<https://ui.shadcn.com/docs/components/command>
