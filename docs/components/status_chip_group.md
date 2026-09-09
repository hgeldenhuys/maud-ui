# Status chip group

Counted links for one current status filter. Use a normal collection URL with query parameters so every filter remains bookmarkable and works without JavaScript.

## Import
```rust
use maud_ui::primitives::status_chip_group::{self, Chip, Props, Tone};
```

## Example
```rust
use maud_ui::primitives::status_chip_group::{self, Chip, Props, Tone};
status_chip_group::render(Props {
    items: vec![
        Chip { label: "All".into(), href: "/reservations".into(), count: Some(48), tone: Tone::Neutral },
        Chip { label: "Arriving".into(), href: "/reservations?status=arriving".into(), count: Some(12), tone: Tone::Info },
    ],
    current: 1,
    aria_label: "Filter reservations by status".into(),
});
```

## Props
| Field | Type | Default | Description |
|---|---|---|---|
| items | Vec<Chip> | empty | Ordered filters; every Chip has label: String, href: String, count: Option<u64>, tone: Tone. |
| current | usize | 0 | Current filter index. Invalid indices fall back to the first item. Empty lists have no current item. |
| aria_label | String | Filter by status | Accessible name for the navigation region. |

## Variants / Enums
`Tone::{Neutral, Info, Success, Warning, Danger}`; Neutral is the default. Text labels and a current-filter check supplement tone. `None` omits the bubble and count announcement; `Some(0)` displays a known zero filter. An empty items list renders nothing.

## Helper Functions
`Tone::as_str()` returns the stable lowercase data-tone value.

## Accessibility
Native anchors and `aria-current="page"`; all links stay in the Tab order. Enhancement adds wrapping arrow keys and Home/End to move focus, with RTL-aware horizontal arrows. Enter follows the link; moving focus does not change the current filter. The application supplies the new current index after navigation. Chips wrap; long labels can break without losing counts. Use the complete `static/maud-ui.css` and JS bundles or `maud_ui::assets` constants.

## Related
[Worklist header](../blocks/worklist-header.md), [badge](badge.md), [segmented control](segmented_control.md).

## Shadcn reference
Application-specific composition; no direct upstream equivalent.

Counts use a muted pill with tabular numerals and no separator. The visible number is hidden from assistive technology; the visually hidden text announces the number with “items” once (for example, “48 items”).

## Compact defaults and collection state
The default chip is 30px tall with a 13px label; its count bubble is 18px tall with 11px tabular numerals. These sizes apply on phones too. The group wraps with an 8px gap, and its parent supplies the surrounding rhythm. Long labels can grow vertically instead of clipping. Table status badges remain 22px with 12px labels.

In 0.9.1, `Chip::count` changes from `u64` to `Option<u64>`: wrap known counts in `Some(...)`. Use `None` for unknown counts; never manufacture a dash. A filter with no matches in an otherwise populated collection may show `Some(0)`. When the **collection itself** is empty, omit the chip group (or pass `items: vec![]`) and render the empty state. The library cannot infer collection state from a filtered count.

```rust
let unknown = Chip { label: "Pending".into(), href: "/reservations?status=pending".into(), count: None, tone: Tone::Info };
```
