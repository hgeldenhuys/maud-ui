# Breadcrumb

Navigation component showing hierarchy from root to current location with separator between items.

## Import

```rust
use maud_ui::primitives::breadcrumb::{self, BreadcrumbItem, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::breadcrumb;

html! {
    (breadcrumb::render(breadcrumb::Props {
        items: vec![
            breadcrumb::BreadcrumbItem {
                label: "Home".to_string(),
                href: Some("/".to_string()),
            },
            breadcrumb::BreadcrumbItem {
                label: "Docs".to_string(),
                href: Some("/docs".to_string()),
            },
            breadcrumb::BreadcrumbItem {
                label: "Components".to_string(),
                href: None,
            },
        ],
        separator: Some(">".to_string()),
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| items | Vec<BreadcrumbItem> | `[]` | Array of breadcrumb items; last item's href should be None (represents current page). |
| separator | Option<String> | `None` | Custom separator character (default "/" when None). |

## BreadcrumbItem Struct

| Field | Type | Description |
|-------|------|-------------|
| label | String | Display text for the breadcrumb. |
| href | Option<String> | Navigation URL; None for the current page (last item). |

## Accessibility

- Emits `<nav aria-label="Breadcrumb">` to identify navigation context.
- Links use standard `<a href>` semantics.
- Current item (last in list) renders as `<span role="link" aria-disabled="true" aria-current="page">` for link-like semantics without clickability.
- Separators marked `aria-hidden="true"` to prevent double announcement.

## Related

Navigation, Link, Pagination.

## Shadcn reference

https://ui.shadcn.com/docs/components/base/breadcrumb
