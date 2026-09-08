# Sidebar shell

Grouped navigation, a header slot, current-page indication and optional mobile tabs. Below 60rem the enhanced shell opens the same navigation inside a native modal drawer. Without JS, every navigation label remains visible above the page.

## Import and example
```rust
use maud::html;
use maud_ui::blocks::shell::sidebar::{self, MobileNavigation, NavGroup, NavItem, Props};
sidebar::render(Props {
    id: "reservation-app".into(),
    brand: html! { "Garden House" },
    header: Some(html! { label { "Workspace" select { option { "Front desk" } } } }),
    nav_groups: vec![NavGroup {
        label: Some("Operations".into()),
        items: vec![NavItem { label: "Reservations".into(), short_label: Some("Stays".into()), href: "/reservations".into(), icon: None, badge: Some("48".into()) }],
    }],
    active_path: "/reservations".into(),
    mobile_navigation: MobileNavigation::Tabs,
    children: html! { p { "Your page content" } },
    ..Default::default()
});
```

## Props
| Field | Type | Default | Description |
|---|---|---|---|
| id | String | mui-app | Unique per shell; derives navigation/drawer IDs. |
| brand | Markup | App | Branding slot. |
| header | Option<Markup> | None | Search, workspace switchers or other header controls. |
| nav_groups | Vec<NavGroup> | empty | Ordered areas; label: Option<String>, items: Vec<NavItem>. |
| active_path | String | empty | First exact href match is current. |
| mobile_navigation | MobileNavigation | Drawer | Drawer or Tabs; desktop sidebar remains grouped. |
| user | Option<UserBlock> | None | name, email, avatar_initials, menu_href: String. |
| topbar_title | Option<String> | None | Topbar H1; use section headings inside it. |
| topbar_actions | Markup | empty | Topbar controls. |
| children | Markup | empty | Route content; the shell supplies main. |

`NavItem` fields: label/href: String, short_label: Option<String>, icon: Option<Markup>, badge: Option<String>. A group's label names its accessible group. `Tabs` uses the first four flattened destinations plus More; any later current destination marks More as current. All destinations remain in the drawer. Put your frequent destinations first. The optional header can use a labelled input with `data-mui-nav-search` for local destination filtering.

## Accessibility and behavior
Native page links and `aria-current`, labelled groups, visible mobile labels, safe-area clearance and focus rings. More/Menu have an ordinary fallback href before enhancement. The drawer moves the existing sidebar node without duplicating controls or IDs; native dialog provides Escape and focus containment. Closing restores navigation and trigger focus. Returning to desktop closes the drawer. Server swaps reinitialize through MaudUI's existing lifecycle. Use a fresh unique `id` for each shell. Do not nest this block inside another main landmark.

Use `short_label: Some("Stays".into())` to fit a long label such as Reservations in a bottom tab. Use `None` to display the full label. Empty or whitespace-only short labels fall back to the full label. Labels stay on one line and ellipsize; five destinations use a smaller type size. The accessible name includes both the visible short label and the full label, so voice control and screen readers retain context. Sidebar labels stay full-length.
