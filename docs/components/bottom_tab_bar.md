# Bottom tab bar

Up to five mobile destinations, including an optional More link. These are page links, not tab panels.

## Import
```rust
use maud_ui::primitives::bottom_tab_bar::{self, Item, More, Position, Props};
```

## Example
```rust
use maud_ui::primitives::bottom_tab_bar::{self, Item, More, Props};
bottom_tab_bar::render(Props {
    items: vec![Item { label: "Reservations and guest arrivals".into(), short_label: Some("Stays".into()), href: "/reservations".into(), icon: None }],
    current_href: Some("/reservations".into()),
    more: Some(More {
        label: "More".into(), target_id: "navigation-drawer".into(),
        fallback_href: "/navigation".into(), current: false,
    }),
    ..Default::default()
});
```

## Props
| Field | Type | Default | Description |
|---|---|---|---|
| items | Vec<Item> | empty | Item fields: label: String, short_label: Option<String>, href: String, icon: Option<Markup>. |
| current_href | Option<String> | None | First exact href match gets aria-current. |
| more | Option<More> | None | More fields: label, target_id and fallback_href: String; current: bool. Current More takes precedence. |
| aria_label | String | Primary navigation | Accessible navigation name. |
| position | Position | Fixed | Fixed viewport bar or Inline contained layout. |

## Variants / Enums
`Position::{Fixed, Inline}`. More than five items, counting More, panics rather than silently discarding destinations. The primitive renders at all widths; the consumer controls its breakpoint. The sidebar block handles that automatically at 60rem.

## Helper Functions
`render(Props)` and `showcase()` only.

## Accessibility
Visible text labels, native links, `aria-current="page"`, optional decorative icons. More's `target_id` identifies a native dialog/sheet/drawer; JS opens it modally and updates aria-expanded. Escape closes it and focus returns to the trigger. `fallback_href` must lead to reachable navigation without JS. The shell supplies that fallback for you. The bar includes safe-area padding; reserve `padding-bottom: var(--mui-bottom-tab-bar-height)` in your content so it cannot cover the last control. The current state also has an underline, including forced-colors mode.

## Related
[Sidebar shell](../blocks/shell-sidebar.md), [drawer](drawer.md), [sheet](sheet.md).

## Shadcn reference
Application-specific navigation; no direct upstream equivalent.

Use `short_label: Some("Stays".into())` to fit a long label such as Reservations in a bottom tab. Use `None` to display the full label. Empty or whitespace-only short labels fall back to the full label. Labels stay on one line and ellipsize; five destinations use a smaller type size. The accessible name includes both the visible short label and the full label, so voice control and screen readers retain context. Sidebar labels stay full-length.
