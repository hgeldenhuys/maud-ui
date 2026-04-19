# Calendar

A static HTML calendar grid with month navigation, day selection, and optional min/max date constraints. No external date library — uses Tomohiko Sakamoto's algorithm for day-of-week calculation.

## Import

```rust
use maud_ui::primitives::calendar::{self, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::calendar;

html! {
    (calendar::render(calendar::Props {
        id: "check-in".into(),
        selected: Some((2026, 4, 20)),
        month: 4,
        year: 2026,
        min_date: Some((2026, 4, 16)),
        max_date: None,
        show_outside_days: true,
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | `"calendar"` | HTML id attribute (also used as hidden input name) |
| `selected` | `Option<(u32, u32, u32)>` | `None` | Currently selected date as (year, month, day); None means no selection |
| `month` | `u32` | `1` | Initial displayed month (1-12) |
| `year` | `u32` | `2026` | Initial displayed year |
| `min_date` | `Option<(u32, u32, u32)>` | `None` | Earliest selectable date (year, month, day); earlier dates disabled |
| `max_date` | `Option<(u32, u32, u32)>` | `None` | Latest selectable date (year, month, day); later dates disabled |
| `show_outside_days` | `bool` | `true` | Show days from adjacent months to fill the grid |

## Accessibility

Emits `role="grid"` on the container with `aria-label="Calendar"`. Week rows have `role="row"`, weekday headers have `role="columnheader"`, and day buttons have `role="gridcell"` with `aria-selected` and `tabindex` states. The title area has `aria-live="polite"` so month changes are announced. Keyboard navigation: arrow keys move between days, Enter/Space selects.

## Related

Input (for form integration), Dialog (for date-picker overlay)

## Shadcn reference

https://ui.shadcn.com/docs/components/base/calendar
