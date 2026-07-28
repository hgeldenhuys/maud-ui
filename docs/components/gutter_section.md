# Gutter section

A mono uppercase section header over a content slot — the unit the inspector gutter is built from (SESSION, LINEAGE, LINKED DOCS).

## Import

```rust
use maud_ui::primitives::gutter_section::{self, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::gutter_section::{self, Props};

gutter_section::render(Props {
    title: "Linked docs".into(),
    content: html! {
        div { a href="#" { "auth-middleware RFC" } }
        div { a href="#" { "tower Layer contract" } }
    },
})
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| title | String | `""` | Section title — rendered mono, uppercased by CSS, 10px, subtle. |
| content | Markup | empty | The section body (any markup). |

## CSS classes

- `mui-gutter-section` — the `<section>`.
- `mui-gutter-section__title`, `mui-gutter-section__body`.

## Related

Facts list (a common body), Sidebar, Item.
