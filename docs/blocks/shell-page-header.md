# Page header

Sticky route context, distinct from the optional product masthead. The default row is 56px tall, with breadcrumb, search, actions and caller-provided switchers.

## Import and example
```rust
use maud::html;
use maud_ui::{blocks::shell::page_header::{self, Props, Search}, primitives::breadcrumb::BreadcrumbItem};
page_header::render(Props {
    breadcrumbs: vec![BreadcrumbItem { label: "Reservations".into(), href: None }],
    search: Some(Search { action: "/search".into(), ..Default::default() }),
    actions: html! { a class="mui-btn mui-btn--outline" href="/reservations/new" { "New reservation" } },
    switchers: html! { label { "Role" select class="mui-native-select" { option { "Front desk" } } } },
    ..Default::default()
});
```

## Props
| Field | Type | Default | Description |
|---|---|---|---|
| leading | Markup | empty | Leading menu or other control. The sidebar shell supplies its own trigger. |
| breadcrumbs | Vec<BreadcrumbItem> | empty | Quiet path; current item has href: None. |
| search | Option<Search> | None | Native GET form. Search has action/name/value/placeholder: String; name defaults to q. |
| search_markup | Option<Markup> | None | Overrides search, including an empty fragment. Use a command-palette trigger here. |
| actions | Markup | empty | Page actions on the right. |
| switchers | Markup | empty | Caller-wired role, language, theme or account controls. |

## Composition and accessibility
Pass these props as `sidebar::Props::page_header`. Use `app_header` for product-level navigation above the shell. `⌘K` / `Ctrl+K` focuses the native search when the behavior bundle is present. The native search always has a submit action; consumers supply the destination. Custom search markup owns its interaction. Ancestor breadcrumbs hide on phones; search and other controls move into the shell drawer, or wrap below the title when used without a shell. Additional controls can make the minimum-height row taller, so keep switchers short and use a menu for secondary actions. The shell's Menu opens the same sidebar node in a native dialog below 60rem.

## Phone layout (0.9.1)
Inside `shell::sidebar`, at widths of 40rem and below, the original search and action/switcher controls move into the same navigation drawer. The header retains Menu and the current breadcrumb. No controls are cloned: form values, IDs and handlers survive closing the drawer and resizing. Returning above 40rem restores their original header order; a focused control follows that move. The example search shortcut closes the modal before focusing the guest field.

Used on its own, or without JavaScript, this block wraps controls below the title instead of hiding them. Avoid duplicating the same search in a sidebar header.
