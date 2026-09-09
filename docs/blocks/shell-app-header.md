# App masthead

An optional product header above the sidebar/content row. It identifies the product and offers primary links; use the separate page header for route context.

## Import and example
```rust
use maud::html;
use maud_ui::blocks::{action::Link, shell::{app_header, sidebar}};
sidebar::render(sidebar::Props {
    app_header: app_header::render(app_header::Props {
        brand: Some(html! { a href="/" { "Garden House" } }),
        links: vec![Link { label: "Workspace".into(), href: "/workspace".into() }],
        current_href: Some("/workspace".into()),
        actions: Some(html! { a href="/account" { "My account" } }),
        ..Default::default()
    }),
    ..Default::default()
});
```

## Props
| Field | Type | Default | Description |
|---|---|---|---|
| brand | Option<Markup> | None | Logo, wordmark or name; supply an accessible label for image-only content. |
| links | Vec<action::Link> | empty | Label and href strings, rendered in a product navigation landmark. |
| current_href | Option<String> | None | Exact current-link match. |
| actions | Option<Markup> | None | Right-side account/theme slot. |
| children | Markup | empty | Additional caller-rendered content. |

`render(Default::default())` returns empty markup. Empty brand/actions fragments also count as empty. The 56px minimum row wraps its navigation beneath the brand on phones. No JS is required. The sidebar showcase demonstrates this masthead and the footer together in both themes.

## Presentation states (0.10.1)

`Props::state: maud_ui::blocks::state::State` defaults to `Ready`. `Loading { message }` shows skeletons; `Empty { message, action }` and `Error { message, retry }` provide distinct recovery paths; `Disabled { reason }` retains admitted content in an inert subtree with an external reason. Loading, empty and error omit ready content. Inert disables interaction, not server authorization or submission of values by an enclosing form. Each live API page shows all four states together. Add `state: Default::default()` to exhaustive Props literals.
