# Page header

Sticky route context, distinct from the optional product masthead. At a viewport of at least 64rem, the header stays one 56px row: flexible breadcrumb, a visible native search field capped at 20rem and a compact actions/switcher toolbar. Geometry and type use density-independent shell tokens.

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
| title | Option<String> | None | Current route title; falls back to the last breadcrumb. |
| single_crumb | bool | false | Prefer the title over a duplicate root breadcrumb; also inferred for one crumb. |
| search | Option<Search> | None | Native GET form inside a details disclosure. Search has action/name/value/placeholder: String; name defaults to q. |
| search_markup | Option<Markup> | None | Overrides search, including an empty fragment. Use a command-palette trigger here. |
| actions | Markup | empty | Page actions on the right. |
| switchers | Markup | empty | Caller-wired role, language, theme or account controls. |

## Composition and accessibility
Pass these props as `sidebar::Props::page_header`. Use `app_header` for product-level navigation above the shell. `⌘K` / `Ctrl+K` focuses the native search when the behavior bundle is present. The native search always has a submit action; consumers supply the destination. Custom search markup owns its interaction. Root and current breadcrumb labels remain whole; only middle links ellipsize, with the full text in a title attribute. Below 64rem the shell moves the original breadcrumb and switchers into its drawer; the header keeps its current title, menu and search icon. At 64–80rem Settings contains switchers; at ≥80rem they are inline. Desktop search stays visible, while phone search opens below the row and Escape closes it without losing the query. Custom search markup remains caller-owned. Keep switcher labels short.

## Frame and phone layout

`--mui-shell-header-height` is 56px and header controls/type are independent of content density. Without enhancement, native Search, Settings and navigation disclosures remain usable. The drawer moves the same controls rather than duplicating them, retaining values and handlers. The shell navigation breakpoint is 64rem. See [shell frame](../shell-frame.md).

## Presentation states (0.10.1)

`Props::state: maud_ui::blocks::state::State` defaults to `Ready`. `Loading { message }` shows skeletons; `Empty { message, action }` and `Error { message, retry }` provide distinct recovery paths; `Disabled { reason }` retains admitted content in an inert subtree with an external reason. Loading, empty and error omit ready content. Inert disables interaction, not server authorization or submission of values by an enclosing form. Each live API page shows all four states together. Add `state: Default::default()` to exhaustive Props literals.

## Search disclosure (0.11.0)
The native summary works without JavaScript. The enhancement opens the same input for the search shortcut, focuses it when opened from the summary, and closes phone search on Escape while keeping the query and restoring summary focus. Desktop search remains visible. Caller-provided search markup owns its behavior; an icon command-palette trigger fits the compact toolbar.

`State::Absent` (0.11.0) means no input/rule was declared and emits nothing. Keep any caller-owned section heading inside the same conditional. Use Error only for a declared operation that failed; never show an unconfigured-rule message to the user.

## Search and breadcrumb bounds (0.13.0)

Native Search defaults to the short placeholder `Search`. The keyboard badge now lives inside `.mui-page-header__search-field`, which is positioned relative; it is no longer placed using an assumed submit-button width. `--mui-search-shortcut-width` (36px) and `--mui-search-shortcut-gap` (8px) reserve 52px at the input's inline end. Phone search hides the badge and restores ordinary input padding. For `search_markup`, use the same field wrapper with input + kbd inside it, and keep the submit button outside the label.

The breadcrumb and its grid context use `min-width:0`; root/middle labels can shrink and ellipsize, and the last crumb wraps without ellipsis inside the context boundary. Empty labels are filtered before separators, mobile title and single-crumb state are derived. Native search, Ctrl/Command+K, GET submission and the phone disclosure retain their behavior.
