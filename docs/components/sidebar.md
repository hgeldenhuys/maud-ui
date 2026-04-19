# Sidebar

Collapsible app-shell sidebar modelled on shadcn's Sidebar. Renders as an `<aside>` with state driven by `data-state` attributes (`expanded` / `collapsed`). Companion JS behaviour toggles on `Cmd/Ctrl+B` shortcut and `[data-mui="sidebar-trigger"]` click. Brand-new v0.2.1 primitive with 20 subcomponent helpers.

## Import

```rust
use maud_ui::primitives::sidebar::{self, Side, SidebarVariant, Collapsible, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::sidebar;

html! {
    (sidebar::provider(html! {
        (sidebar::render(sidebar::Props {
            id: "main-sidebar".to_string(),
            side: sidebar::Side::Left,
            variant: sidebar::SidebarVariant::Sidebar,
            collapsible: sidebar::Collapsible::Icon,
            default_open: true,
            children: html! {
                (sidebar::header(html! {
                    div class="brand" { "My App" }
                }))
                (sidebar::content(html! {
                    (sidebar::group(html! {
                        (sidebar::group_label(html! { "Navigation" }))
                        (sidebar::group_content(html! {
                            (sidebar::menu(html! {
                                (sidebar::menu_item(
                                    sidebar::menu_button(html! {
                                        span { "Dashboard" }
                                    })
                                ))
                                (sidebar::menu_item(
                                    sidebar::menu_button(html! {
                                        span { "Settings" }
                                    })
                                ))
                            }))
                        }))
                    }))
                }))
                (sidebar::footer(html! {
                    span { "User" }
                }))
                (sidebar::rail())
            },
        }))
        (sidebar::inset(html! {
            div { "Main content here" }
        }))
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| id | String | `"sidebar"` | Unique identifier used by trigger + behaviour to target the sidebar. |
| side | Side | `Left` | Which edge the sidebar anchors to: Left or Right. |
| variant | SidebarVariant | `Sidebar` | Visual variant: Sidebar, Floating, or Inset. |
| collapsible | Collapsible | `Offcanvas` | How the sidebar collapses: Offcanvas, Icon, or None. |
| default_open | bool | `true` | Initial state (SSR); client-side state persisted in JS. |
| children | Markup | `html! {}` | Sidebar content (header, content, footer, etc.). |

## Enums

### Side

| Variant | Data Attr | Behavior |
|---------|-----------|----------|
| Left | `left` | Sidebar anchors to the left edge. |
| Right | `right` | Sidebar anchors to the right edge. |

### SidebarVariant

| Variant | Data Attr | Behavior |
|---------|-----------|----------|
| Sidebar | `sidebar` | Full-height sidebar (default). |
| Floating | `floating` | Floating sidebar with shadow. |
| Inset | `inset` | Inset variant (reserved space). |

### Collapsible

| Variant | Data Attr | Behavior |
|---------|-----------|----------|
| Offcanvas | `offcanvas` | Slides off-canvas when collapsed (drawer-like). |
| Icon | `icon` | Collapses to icon-only mode. |
| None | `none` | Does not collapse; remains fixed width. |

## Subcomponent Helpers (20 total)

### Layout Containers

| Helper | Returns | Purpose |
|--------|---------|---------|
| `provider(Markup)` | Markup | Root wrapper (`data-mui="sidebar-provider"`); establishes flex layout. |
| `render(Props)` | Markup | Sidebar `<aside>` root with data-attributes. |
| `header(Markup)` | Markup | Top region (brand, workspace switcher). |
| `content(Markup)` | Markup | Scrollable main content region. |
| `footer(Markup)` | Markup | Bottom region (pinned). |
| `inset(Markup)` | Markup | Sibling main content container (used inside provider). |

### Group Helpers

| Helper | Returns | Purpose |
|--------|---------|---------|
| `group(Markup)` | Markup | Labeled cluster container. |
| `group_label(Markup)` | Markup | Small-caps heading for a group. |
| `group_action(Markup)` | Markup | Button anchored to group header (e.g., add icon). |
| `group_content(Markup)` | Markup | Wrapper for the menu inside a group. |

### Menu Helpers

| Helper | Returns | Purpose |
|--------|---------|---------|
| `menu(Markup)` | Markup | `<ul>` wrapper for menu items. |
| `menu_item(Markup)` | Markup | `<li>` wrapper for a single menu row. |
| `menu_button(Markup)` | Markup | Interactive button row (label + icon markup). |
| `menu_action(Markup)` | Markup | Trailing action button on a menu item (kebab). |
| `menu_sub(Markup)` | Markup | Nested `<ul>` indented under parent. |
| `menu_sub_item(Markup)` | Markup | `<li>` row inside a sub-menu. |
| `menu_badge(Markup)` | Markup | Small badge/count pill attached to a menu item. |
| `menu_skeleton()` | Markup | Loading placeholder for a menu row. |

### Control Helpers

| Helper | Returns | Purpose |
|--------|---------|---------|
| `trigger(target_id: &str, label: &str)` | Markup | Toggle button with `data-mui="sidebar-trigger"`. |
| `rail()` | Markup | Thin interactive strip along sidebar edge (for icon-collapsed mode). |

## Accessibility

- Sidebar uses semantic `<aside>` with `aria-label="Sidebar"`.
- `data-state` drives CSS animations and visibility; JS updates on collapse/expand.
- Groups and menu items use semantic `<ul>` / `<li>` lists.
- Keyboard shortcut: `Cmd/Ctrl+B` toggles the sidebar (via `dist/behaviors/sidebar.js`).
- Focus is managed within the sidebar when expanded; rail is `tabindex="-1"`.
- Icons inside menu buttons marked `aria-hidden="true"` to avoid double announcement.

## CSS Classes

| Class | Purpose |
|-------|---------|
| `mui-sidebar-provider` | Provider root flex layout container. |
| `mui-sidebar` | Root sidebar `<aside>`. |
| `mui-sidebar__header` | Header region. |
| `mui-sidebar__content` | Scrollable content region. |
| `mui-sidebar__footer` | Footer region. |
| `mui-sidebar__group` | Group container. |
| `mui-sidebar__group-label` | Group label heading. |
| `mui-sidebar__menu` | Menu `<ul>`. |
| `mui-sidebar__menu-item` | Menu item `<li>`. |
| `mui-sidebar__menu-button` | Interactive menu row button. |
| `mui-sidebar__menu-action` | Trailing action button. |
| `mui-sidebar__menu-sub` | Sub-menu `<ul>`. |
| `mui-sidebar__menu-sub-item` | Sub-menu item `<li>`. |
| `mui-sidebar__menu-badge` | Badge/count pill. |
| `mui-sidebar__rail` | Edge rail for collapsed icon mode. |
| `mui-sidebar-inset` | Main content sibling. |

## Related

Navigation, NavMenu, Shell.

## Shadcn reference

https://ui.shadcn.com/docs/components/base/sidebar
