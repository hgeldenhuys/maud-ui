# Brand mark

Logo, wordmark and optional tagline shared by app_header and the sidebar. Visual identity comes from the nine brand tokens; content and destinations stay in props. An empty wordmark emits nothing.

## Import and example
```rust
use maud::html;
use maud_ui::blocks::shell::brand_mark::{self, Props};
brand_mark::render(Props {
    wordmark: "Northline".into(), href: Some("/accounts".into()),
    tagline: Some(html! { "Business banking" }),
    ..Default::default()
});
```

## Props
| Field | Type | Default | Description |
|---|---|---|---|
| wordmark | String | empty | Escaped identity text; empty emits nothing. |
| href | Option<String> | None | Destination of the wordmark link. |
| logo | Option<Markup> | None | Overrides the CSS logo mask; supports full-color images. |
| tagline | Option<Markup> | None | Caller-owned tagline fragment, outside the link. |

## Helpers and composition
`preset("lodge" | "bank" | "clinic") -> Option<Props>` supplies fictional example identities. Pair it with a matching `data-brand` wrapper for a showcase preset. Pass typed props through `app_header::Props::brand_mark` or `sidebar::Props::brand_mark`; they override the legacy raw brand slot when supplied.

Brand tokens: `--mui-brand-accent`, `--mui-brand-accent-ink`, `--mui-brand-font-heading`, `--mui-brand-font-body`, `--mui-brand-radius`, `--mui-brand-density`, `--mui-brand-logo-mask`, `--mui-brand-logo-size`, `--mui-brand-logo-radius`. The theme customiser exports exactly these names as brand.css. Load it after the library CSS.

## Accessibility
The wordmark names the identity; the logo is decorative. Optional tagline markup stays outside the wordmark link. In the desktop icon rail the text and its link are hidden together; they return in the phone drawer. Supply fonts with local fallbacks, and verify contrast for custom accent/ink values in both themes.

## Related
[App header](shell-app-header.md), [sidebar](shell-sidebar.md), [page header](shell-page-header.md).
