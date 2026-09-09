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
        items: vec![NavItem { label: "Reservations".into(), short_label: Some("Stays".into()), href: "/reservations".into(), icon: None, badge: Some("48".into()), ..Default::default() }],
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
| topbar_title | Option<String> | None | Quiet context label; put a heading in your content. |
| topbar_actions | Markup | empty | Topbar controls. |
| sidebar_footer | Option<Markup> | None | Arbitrary footer markup, overriding user. |
| collapsible | bool | true | Offer a desktop icon rail at 64rem and above. |
| default_collapsed | bool | false | Initial rail state; a saved local preference takes precedence. |
| page_header | Option<page_header::Props> | None | Replaces the title/actions bar. The shell adds its own Menu and rail controls. |
| app_header | Markup | empty | Optional masthead above sidebar and content. |
| app_footer | Markup | empty | Optional footer below sidebar and content. |
| embedded | bool | false | Render a section instead of main for previews/nested compositions. |
| children | Markup | empty | Route content; the shell supplies main. |

`NavItem` fields: label/href: String, short_label: Option<String>, icon: Option<Markup>, badge: Option<String>, children: Vec<NavItem>. Use `..Default::default()` for optional fields. Nested items indent without a rail. A group's label names its accessible group. `Tabs` uses the first four flattened destinations plus More; any later current destination marks More as current. All destinations remain in the drawer. Put your frequent destinations first. The optional header can use a labelled input with `data-mui-nav-search` for local destination filtering.

## Accessibility and behavior
Native page links and `aria-current`, labelled groups, visible mobile labels, safe-area clearance and focus rings. More/Menu have an ordinary fallback href before enhancement. The drawer moves the existing sidebar node without duplicating controls or IDs; native dialog provides Escape and focus containment. Closing restores navigation and trigger focus. Returning to desktop closes the drawer. Server swaps reinitialize through MaudUI's existing lifecycle. Use a fresh unique `id` for each shell. Use `embedded: true` when nesting inside an existing main landmark. Embedded mobile tabs flow below the example; ordinary app tabs remain fixed.

Use `short_label: Some("Stays".into())` to fit a long label such as Reservations in a bottom tab. Use `None` to display the full label. Empty or whitespace-only short labels fall back to the full label. Labels stay on one line and ellipsize; five destinations use a smaller type size. The accessible name includes both the visible short label and the full label, so voice control and screen readers retain context. Sidebar labels ellipsize on desktop with their full text available in a title hint and accessible name; count badges never shrink. The drawer wraps labels.

## Navigation preferences
Labeled groups use native details/summary and remember their state under `mui-nav-group:{navigation-id}:{index}:{label}`. A current destination opens its group on route entry. Sidebar collapse uses `mui-shell-rail:{id}`; storage failures fall back to rendered defaults. Keep IDs stable and unique, and keep group ordering stable to retain preferences. The desktop icon rail appears at 64rem, with full accessible labels and title hints. It temporarily opens groups to retain every icon and restores their remembered state when expanded or moved into the phone drawer. Inputs in the header and arbitrary footer controls are hidden while collapsed; place essentials in the page bar as well.

## Header and footer composition
Pass `app_header::render(...)` and `app_footer::render(...)` into the corresponding shell slots. Their empty defaults emit no wrappers. Pass `page_header::Props` for breadcrumbs, native search or a custom command trigger, actions and switchers. The shell page renders this full composition in both themes. The landing example supports client-side guest filtering, row selection and adding a local example reservation; its data is fictional and never persisted or sent to a booking system.

At 40rem and below, a supplied page header moves its original search, actions and switchers into the drawer, leaving Menu and the current breadcrumb above the page. At wider sizes they return to the header. Without enhancement they remain visible on wrapped header rows. The shared example uses “Reservations” and one guest-search field; the header shortcut focuses that field, and the sidebar contains no duplicate search.

## Presentation states (0.10.1)

`Props::state: maud_ui::blocks::state::State` defaults to `Ready`. `Loading { message }` shows skeletons; `Empty { message, action }` and `Error { message, retry }` provide distinct recovery paths; `Disabled { reason }` retains admitted content in an inert subtree with an external reason. Loading, empty and error omit ready content. Inert disables interaction, not server authorization or submission of values by an enclosing form. Each live API page shows all four states together. Add `state: Default::default()` to exhaustive Props literals.
