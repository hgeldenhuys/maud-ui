# NavigationMenu

Horizontal or vertical navigation menu with dropdown content panels (mega-menu style). Supports direct links and dropdown menus with optional descriptions.

## Import

```rust
use maud_ui::primitives::navigation_menu::{self, Props, Orientation, NavItem, NavLink, NavMenu};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::navigation_menu;

html! {
    (navigation_menu::render(navigation_menu::Props {
        id: "main-nav".into(),
        items: vec![
            navigation_menu::NavItem::Link(navigation_menu::NavLink {
                label: "Home".into(),
                href: "/".into(),
                description: None,
            }),
            navigation_menu::NavItem::Menu(navigation_menu::NavMenu {
                label: "Products".into(),
                links: vec![
                    navigation_menu::NavLink {
                        label: "Software".into(),
                        href: "/products/software".into(),
                        description: Some("Development tools".into()),
                    },
                    navigation_menu::NavLink {
                        label: "Hardware".into(),
                        href: "/products/hardware".into(),
                        description: Some("Devices and equipment".into()),
                    },
                ],
            }),
        ],
        orientation: navigation_menu::Orientation::Horizontal,
        viewport: true,
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| id | String | `"nav-menu"` | Unique identifier for the navigation menu. |
| items | Vec<NavItem> | `vec![]` | Top-level navigation items: direct links or dropdown menus. |
| orientation | Orientation | `Horizontal` | Direction of the top-level list: `Horizontal` or `Vertical`. |
| viewport | bool | `true` | When true, dropdown content renders in a shared viewport panel. When false, each menu has its own per-item popover. |

## Orientation

```rust
pub enum Orientation {
    Horizontal,  // Default; top-level items in a row
    Vertical,    // Top-level items in a column
}
```

## NavItem

Navigation item variant: either a direct link or a dropdown menu.

```rust
pub enum NavItem {
    Link(NavLink),    // Direct navigation link
    Menu(NavMenu),    // Dropdown menu
}
```

### NavLink

```rust
pub struct NavLink {
    pub label: String,                  // Link text displayed
    pub href: String,                   // URL or anchor
    pub description: Option<String>,    // Optional subtitle/description
}
```

### NavMenu

```rust
pub struct NavMenu {
    pub label: String,              // Menu trigger label
    pub links: Vec<NavLink>,        // Dropdown links
}
```

## Variants

| Variant | Configuration | Use Case |
|---------|---------------|----------|
| Horizontal mega-menu | `orientation: Horizontal, viewport: true` | Header navigation with shared content panel. |
| Horizontal popovers | `orientation: Horizontal, viewport: false` | Compact header with per-item dropdowns. |
| Vertical sidebar | `orientation: Vertical, viewport: false` | Sidebar navigation with nested menus. |
| Mixed links & menus | Combine `NavItem::Link` and `NavItem::Menu` | Homepage link + product categories. |
| With descriptions | Include `description` in NavLink entries | Knowledge base with category descriptions. |

## Helpers

**`indicator() -> Markup`**

Renders a small decorative chevron (❯) between active triggers. Purely visual; marked `aria-hidden="true"`.

```rust
use maud_ui::primitives::navigation_menu;

html! {
    span { (navigation_menu::indicator()) }
}
```

## Accessibility

- **Navigation role:** Outer container uses `role="navigation"` and `aria-label="Primary"`.
- **List structure:** Top-level items are rendered in a `<ul>` with `<li>` children.
- **Menu triggers:** Buttons have `aria-expanded="false"` (true when open) and `aria-haspopup="true"`.
- **Content visibility:** Dropdown content is initially `hidden`; JavaScript toggles visibility and updates `aria-expanded`.
- **Keyboard navigation:** 
  - Horizontal: Left/Right arrows navigate between triggers; Down arrow opens dropdown.
  - Vertical: Up/Down arrows navigate within the menu; Right arrow opens submenu.
  - Escape closes the dropdown.
- **Screen readers:** Links and button labels are announced; descriptions are optional and informative.
- **Indicator:** The chevron helper uses `aria-hidden="true"` and is decorative.

## Design Notes

### Viewport Mode (shared vs per-item)

- **Shared viewport (`viewport: true`):** One content panel anchors to the current trigger; visually smooth transitions as the user moves between menu triggers. Better for wide viewports.
- **Per-item popover (`viewport: false`):** Each menu trigger carries its own popover wrapper; independent positioning. Better for mobile or constrained widths.

The `data-viewport` attribute on the nav element indicates which mode is active, allowing CSS to style accordingly.

## Related

DropdownMenu, Menubar, Button.

## Shadcn reference

https://ui.shadcn.com/docs/components/base/navigation-menu
