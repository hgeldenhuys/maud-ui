# ContextMenu

Right-click menu overlay with support for items, separators, submenus, and destructive actions.

## Import

```rust
use maud_ui::primitives::context_menu::{self, Props};
use maud_ui::primitives::menu::{MenuEntry, MenuItem};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::{context_menu, menu};

html! {
    (context_menu::render(context_menu::Props {
        id: "ctx".to_string(),
        trigger: html! { div { "Right-click me" } },
        items: vec![
            menu::MenuEntry::Item(menu::MenuItem {
                label: "Cut".into(),
                action: "cut".into(),
                disabled: false,
                destructive: false,
                shortcut: Some("⌘X".into()),
            }),
            menu::MenuEntry::Separator,
        ],
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | — | Unique identifier for the menu |
| `trigger` | `Markup` | — | Content that triggers the menu (right-click target) |
| `items` | `Vec<MenuEntry>` | — | Menu entries (items, separators, labels, submenus) |

## MenuEntry Enum Variants

| Variant | Description |
|---------|-------------|
| `Item(MenuItem)` | A clickable menu item with optional shortcut and destructive flag |
| `Separator` | Visual divider between groups |
| `Label(String)` | Non-interactive group label |
| `CheckboxItem { label, checked, disabled, action }` | Toggle item with checkbox |
| `RadioGroup(Vec<RadioItem>)` | Group of mutually exclusive radio items |
| `RadioItem { label, value, disabled, action }` | Single radio item |
| `Sub { label, items }` | Submenu with nested entries |
| `Group { label, items }` | Grouped items with optional label |

## Helper Functions

### `destructive_item(action, label, disabled, shortcut) → MenuEntry`
Builds a `MenuEntry::Item` pre-set to `destructive: true`. Use for Delete, Sign out, and other irreversible actions. Applies the `mui-menu__item--danger` class.

## Accessibility

- Menu has `role="menu"` and `aria-orientation="vertical"`
- `data-side="inline-end"` emitted for RTL locale auto-flip of submenus
- Items support standard ARIA menu conventions via `menu.rs` renderer

## Related

- Menu
- Dialog
- Dropdown

## Shadcn reference
<https://ui.shadcn.com/docs/components/context-menu>
