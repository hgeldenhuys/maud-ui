# DropdownMenu

Dropdown menu for actions and navigation. Supports items, separators, labels, checkboxes, radio groups, submenus, and grouping.

## Import

```rust
use maud_ui::primitives::menu::{self, Props, MenuItem, MenuEntry, RadioItem};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::menu;

html! {
    (menu::render(menu::Props {
        trigger_label: "Actions".into(),
        id: "demo-menu".into(),
        items: vec![
            menu::MenuEntry::Item(menu::MenuItem {
                label: "Edit".into(),
                action: "edit".into(),
                disabled: false,
                destructive: false,
                shortcut: Some("⌘E".into()),
            }),
            menu::MenuEntry::Separator,
            menu::MenuEntry::Item(menu::MenuItem {
                label: "Delete".into(),
                action: "delete".into(),
                disabled: false,
                destructive: true,
                shortcut: None,
            }),
        ],
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| trigger_label | String | `""` | Text displayed on the dropdown trigger button. |
| id | String | `""` | Unique identifier for the menu; used to link trigger and content via ARIA attributes. |
| items | Vec<MenuEntry> | `vec![]` | Menu entries: items, separators, labels, checkboxes, radio groups, submenus, and groups. |

## MenuEntry Enum

The `MenuEntry` enum defines the structure of menu items. All variants are fully supported:

### MenuEntry::Item(MenuItem)

A simple clickable menu item.

```rust
pub struct MenuItem {
    pub label: String,              // Item text
    pub action: String,             // Data attribute for event handling
    pub disabled: bool,             // Disabled state
    pub destructive: bool,          // Red danger styling (e.g., delete)
    pub shortcut: Option<String>,   // Keyboard shortcut display (e.g., "⌘Z")
}
```

### MenuEntry::Separator

A visual divider between menu sections.

### MenuEntry::Label(String)

A non-interactive label or heading for grouping related items.

### MenuEntry::CheckboxItem

A menu item with toggle state (on/off).

```rust
MenuEntry::CheckboxItem {
    label: String,              // Item text
    checked: bool,              // Current state
    disabled: bool,             // Disabled state
    shortcut: Option<String>,   // Optional keyboard shortcut
}
```

### MenuEntry::RadioGroup

A named group of mutually exclusive radio options.

```rust
MenuEntry::RadioGroup {
    name: String,               // Group identifier
    items: Vec<RadioItem>,      // Radio options
}
```

Where `RadioItem` is:

```rust
pub struct RadioItem {
    pub label: String,              // Option text
    pub value: String,              // Option value
    pub checked: bool,              // Currently selected
    pub disabled: bool,             // Disabled state
    pub shortcut: Option<String>,   // Optional keyboard shortcut
}
```

### MenuEntry::Sub

A submenu trigger with nested menu entries.

```rust
MenuEntry::Sub {
    trigger_label: String,      // Submenu trigger text
    items: Vec<MenuEntry>,      // Nested menu entries (any MenuEntry variant)
}
```

Submenus can be nested arbitrarily deep.

### MenuEntry::Group

A generic grouping wrapper (renders with `role="group"`).

```rust
MenuEntry::Group {
    items: Vec<MenuEntry>,      // Grouped entries
}
```

## Variants

| Variant | Use Case |
|---------|----------|
| Item | Primary menu action. |
| Item (destructive) | Dangerous action requiring confirmation (delete, remove, etc.). |
| Separator | Visual divider between logical sections. |
| Label | Section header or grouping label. |
| CheckboxItem | Toggle option (show/hide, enable/disable feature). |
| RadioGroup | Single-selection option group (e.g., theme, position). |
| Sub | Nested menu with additional options. |
| Group | Container for related menu entries. |

## Helpers

**`render_entry(entry: &MenuEntry) -> Markup`**

Renders a single menu entry. Used internally and can be used in custom menu construction. Public for composability.

## Accessibility

- **Trigger button:** Sets `aria-expanded="false"` (true when open), `aria-haspopup="menu"`, and `aria-controls` pointing to the menu content.
- **Menu roles:** Menu items use `role="menuitem"`, checkboxes use `role="menuitemcheckbox"` with `aria-checked`, radio buttons use `role="menuitemradio"` with `aria-checked`, and separators use `role="separator"`.
- **Keyboard navigation:** Tabindex is set to `-1` for all menu items; keyboard navigation (arrow keys, Enter, Escape) is handled via JavaScript.
- **Shortcuts:** Displayed as decorative text; not enforced by the component (handled by application).
- **Screen readers:** Labels are announced for all entry types; separators are marked as decorative.

## Related

Context Menu, Menubar, Button.

## Shadcn reference

https://ui.shadcn.com/docs/components/base/dropdown-menu
