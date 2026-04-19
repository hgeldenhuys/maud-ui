# Pagination

Page navigation control with previous/next buttons and numbered page links. Supports anchor-based navigation (for static sites) and button-based navigation (for interactive apps). Intelligent ellipsis collapsing for many pages.

## Import

```rust
use maud_ui::primitives::pagination::{self, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::pagination;

html! {
    // Button-based navigation (interactive)
    (pagination::render(pagination::Props {
        current_page: 3,
        total_pages: 10,
        max_visible: 5,
        href_pattern: None,
        icons_only: false,
    }))
}

html! {
    // Anchor-based navigation (with href pattern)
    (pagination::render(pagination::Props {
        current_page: 2,
        total_pages: 5,
        max_visible: 5,
        href_pattern: Some("/results?page={page}".into()),
        icons_only: false,
    }))
}

html! {
    // Icons only (compact on mobile)
    (pagination::render(pagination::Props {
        current_page: 5,
        total_pages: 20,
        max_visible: 5,
        href_pattern: None,
        icons_only: true,
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| current_page | usize | `1` | Currently active page (1-indexed). Page 1 is the first page. |
| total_pages | usize | `1` | Total number of pages available. |
| max_visible | usize | `5` | Maximum number of page buttons to display before using ellipsis. Always shows first and last page. |
| href_pattern | Option<String> | `None` | URL pattern for page links. When Some, page buttons render as `<a>` tags with `{page}` substituted. When None, `<button>` is used. Example: `/results?page={page}` or `#page-{page}`. |
| icons_only | bool | `false` | When true, "Previous" and "Next" button text is hidden; only chevron icons remain visible. Visually-hidden text still announced to screen readers. |

## Page Display Logic

The component intelligently determines which page numbers to show:

- **Few pages (≤ max_visible):** All pages displayed; no ellipsis.
- **Many pages (> max_visible):** First page, current page ± 1, and last page are shown; ellipsis (`…`) fills gaps.

**Example:** `current_page=6, total_pages=20, max_visible=5`

```
1 … 5 6 7 … 20
```

The current page is always centered in the visible range (within bounds).

## Variants

| Variant | Configuration | Use Case |
|---------|---------------|----------|
| Button navigation | `href_pattern: None` | Interactive apps with JavaScript handlers. |
| Anchor navigation | `href_pattern: Some(...)` | Static sites, SEO-friendly, works without JavaScript. |
| Icons only | `icons_only: true` | Mobile-optimized, compact layout. |
| Full labels | `icons_only: false` | Desktop-friendly, explicit text. |
| Few pages | `total_pages: 3-5` | Short result sets; all pages always visible. |
| Many pages | `total_pages: 100+` | Large result sets; intelligent ellipsis. |

## Helpers

**`render_href(pattern: &str, page: usize) -> String`**

Internal helper that substitutes `{page}` placeholder in the href pattern. Can be used if building custom pagination UI.

## Accessibility

- **Navigation role:** The outer container uses `role="navigation"` and `aria-label="Pagination"`.
- **Previous/Next buttons:**
  - Anchor variant: `aria-label="Go to previous page"` / `"Go to next page"`
  - Button variant: Same aria-labels, with `disabled` attribute when at boundary pages.
- **Page links:**
  - Current page: `aria-current="page"` to announce active state.
  - Other pages: Standard link/button semantics.
- **Icons-only mode:** Chevron icons are placed in a `<span class="mui-pagination__btn-icon">`, while text is in a `<span class="mui-visually-hidden">` for screen reader announcement.
- **Ellipsis:** Marked `aria-hidden="true"` (purely visual).

## Design Notes

### Href Pattern Substitution

The `href_pattern` is a template string where `{page}` is replaced with the target page number. Examples:

```rust
// Query string
href_pattern: Some("/search?q=cats&page={page}".into())

// URL path
href_pattern: Some("/results/{page}".into())

// Fragment (anchor)
href_pattern: Some("#page-{page}".into())
```

### Button vs. Anchor Navigation

- **Buttons (`href_pattern: None`):** Use when page navigation triggers JavaScript-based rendering (e.g., fetching data, updating results). Pair with `data-page` attributes for event delegation.
- **Anchors (`href_pattern: Some(...)`):** Use for static-site pagination or when each page has a unique URL. Works without JavaScript; crawlers can follow links.

### Disabled/Active States

- First page: "Previous" button is disabled/inactive.
- Last page: "Next" button is disabled/inactive.
- Other pages: Both buttons are enabled.
- Current page button: Has `aria-current="page"` (buttons) or implicit current state (anchors).

## Related

Button, Link, Navigation Menu.

## Shadcn reference

https://ui.shadcn.com/docs/components/base/pagination
