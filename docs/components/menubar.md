# Menubar

Horizontal menu bar for application menus (File, Edit, View, etc.). Each menu uses the same `MenuEntry` enum as DropdownMenu for flexible, consistent menu construction.

## Import

```rust
use maud_ui::primitives::menubar::{self, Props, MenubarMenu};
use maud_ui::primitives::menu::{MenuEntry, MenuItem, RadioItem};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::menubar;
use maud_ui::primitives::menu::{MenuEntry, MenuItem};

html! {
    (menubar::render(menubar::Props {
        id: "app-menubar".into(),
        menus: vec![
            menubar::MenubarMenu {
                label: "File".into(),
                items: vec![
                    MenuEntry::Item(MenuItem {
                        label: "New".into(),
                        action: "new".into(),
                        disabled: false,
                        destructive: false,
                        shortcut: Some("⌘N".into()),
                    }),
                    MenuEntry::Item(MenuItem {
                        label: "Open".into(),
                        action: "open".into(),
                        disabled: false,
                        destructive: false,
                        shortcut: Some("⌘O".into()),
                    }),
                    MenuEntry::Separator,
                    MenuEntry::Item(MenuItem {
                        label: "Save".into(),
                        action: "save".into(),
                        disabled: false,
                        destructive: false,
                        shortcut: Some("⌘S".into()),
                    }),
                ],
            },
        ],
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| id | String | `""` | Unique identifier for the menubar. |
| menus | Vec<MenubarMenu> | `vec![]` | Top-level menus in the menubar. |

## MenubarMenu

| Field | Type | Description |
|-------|------|-------------|
| label | String | Text displayed on the menu trigger button (e.g., "File", "Edit"). |
| items | Vec<MenuEntry> | Menu entries (items, separators, labels, checkboxes, radio groups, submenus, groups). |

## MenuEntry Variants (Shared)

Menubar uses the same `MenuEntry` enum as DropdownMenu, supporting all variants:

- **Item** — Clickable menu action with optional destructive styling and keyboard shortcut.
- **Separator** — Visual divider between menu sections.
- **Label** — Non-interactive heading or grouping label.
- **CheckboxItem** — Toggle option with on/off state.
- **RadioGroup** — Single-selection group of radio options.
- **Sub** — Nested submenu with additional entries.
- **Group** — Container for related entries.

See [DropdownMenu documentation](./dropdown-menu.md) for detailed `MenuEntry` structure.

## Variants

| Variant | Use Case |
|---------|----------|
| Classic menubar | File/Edit/View/Help structure (desktop applications). |
| Mixed items | Combine simple items, checkboxes, radio groups, and submenus in a single menu. |
| Disabled items | Show unavailable actions (e.g., "Redo" when no undo history). |
| Keyboard shortcuts | Display shortcut hints alongside menu items (e.g., "⌘S"). |

## Helpers

None. The component uses `render_entry()` from the menu module internally for consistency.

## Accessibility

- **Menubar role:** Sets `role="menubar"` and `aria-orientation="horizontal"`.
- **Menu triggers:** Each trigger button has `role="menuitem"`, `aria-expanded` (false/true), `aria-haspopup="menu"`, and `aria-controls` pointing to its dropdown content.
- **Keyboard navigation:** First menu trigger has `tabindex="0"`; subsequent triggers have `tabindex="-1"`. Horizontal arrow keys navigate between menus; vertical arrows navigate within a menu. Escape closes the menu.
- **Screen readers:** Menubar is announced as a menu bar; all items are properly labeled and roles are explicit.

## Related

DropdownMenu, Context Menu, Button.

## Shadcn reference

https://ui.shadcn.com/docs/components/base/menubar
