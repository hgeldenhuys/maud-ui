# Page header

Sticky route context, distinct from the optional product masthead. At a viewport of at least 64rem, the header stays one 56px row: flexible breadcrumb, a collapsed search icon and a compact actions/switcher toolbar.

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
| search | Option<Search> | None | Native GET form inside a details disclosure. Search has action/name/value/placeholder: String; name defaults to q. |
| search_markup | Option<Markup> | None | Overrides search, including an empty fragment. Use a command-palette trigger here. |
| actions | Markup | empty | Page actions on the right. |
| switchers | Markup | empty | Caller-wired role, language, theme or account controls. |

## Composition and accessibility
Pass these props as `sidebar::Props::page_header`. Use `app_header` for product-level navigation above the shell. `⌘K` / `Ctrl+K` focuses the native search when the behavior bundle is present. The native search always has a submit action; consumers supply the destination. Custom search markup owns its interaction. Root and current breadcrumb labels remain whole; only middle links ellipsize, with the full text in a title attribute. The path wraps on phones; search and other controls move into the shell drawer, or wrap below the title when used without a shell. At ≥64rem, opening search places its form below the row; custom search markup is capped at 20rem and the toolbar stays on one line. Below 64rem controls may wrap. Keep switchers short and use a menu for secondary actions. The shell's Menu opens the same sidebar node in a native dialog below 60rem.

## Phone layout (0.9.1)
Inside `shell::sidebar`, at widths of 40rem and below, the original search and action/switcher controls move into the same navigation drawer. The header retains Menu and the breadcrumb path. No controls are cloned: form values, IDs and handlers survive closing the drawer and resizing. Returning above 40rem restores their original header order; a focused control follows that move. The example search shortcut closes the modal before focusing the guest field.

Used on its own, or without JavaScript, this block wraps controls below the title instead of hiding them. Avoid duplicating the same search in a sidebar header.

## Presentation states (0.10.1)

`Props::state: maud_ui::blocks::state::State` defaults to `Ready`. `Loading { message }` shows skeletons; `Empty { message, action }` and `Error { message, retry }` provide distinct recovery paths; `Disabled { reason }` retains admitted content in an inert subtree with an external reason. Loading, empty and error omit ready content. Inert disables interaction, not server authorization or submission of values by an enclosing form. Each live API page shows all four states together. Add `state: Default::default()` to exhaustive Props literals.

## Search disclosure (0.11.0)
The native summary works without JavaScript. The enhancement opens the same input for the search shortcut, focuses it when opened from the summary, and closes on Escape while keeping the query and restoring summary focus. Caller-provided search markup owns its behavior; an icon command-palette trigger fits the compact toolbar.

`State::Absent` (0.11.0) means no input/rule was declared and emits nothing. Keep any caller-owned section heading inside the same conditional. Use Error only for a declared operation that failed; never show an unconfigured-rule message to the user.
