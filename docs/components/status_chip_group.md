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
        Chip { label: "All".into(), href: "/reservations".into(), count: 48, tone: Tone::Neutral },
        Chip { label: "Arriving".into(), href: "/reservations?status=arriving".into(), count: 12, tone: Tone::Info },
    ],
    current: 1,
    aria_label: "Filter reservations by status".into(),
});
```

## Props
| Field | Type | Default | Description |
|---|---|---|---|
| items | Vec<Chip> | empty | Ordered filters; every Chip has label: String, href: String, count: u64, tone: Tone. |
| current | usize | 0 | Current filter index. Invalid indices fall back to the first item. Empty lists have no current item. |
| aria_label | String | Filter by status | Accessible name for the navigation region. |

## Variants / Enums
`Tone::{Neutral, Info, Success, Warning, Danger}`; Neutral is the default. Text labels, a check and an underline supplement tone. A zero count remains visible.

## Helper Functions
`Tone::as_str()` returns the stable lowercase data-tone value.

## Accessibility
Native anchors and `aria-current="page"`; all links stay in the Tab order. Enhancement adds wrapping arrow keys and Home/End to move focus, with RTL-aware horizontal arrows. Enter follows the link; moving focus does not change the current filter. The application supplies the new current index after navigation. Chips wrap; long labels can break without losing counts. Use the complete `static/maud-ui.css` and JS bundles or `maud_ui::assets` constants.

## Related
[Worklist header](../blocks/worklist-header.md), [badge](badge.md), [segmented control](segmented_control.md).

## Shadcn reference
Application-specific composition; no direct upstream equivalent.
