# Grid

The two-dimensional layout container. Where [stack](stack.md) lays children along one axis, `grid` lays them into columns.

The two share one spacing scale and one alignment vocabulary — `gap` is `Space` and `align` is `Align`, both re-exported from `stack` — so there is one set of values to learn, and one for tooling to describe.

The default is `Columns::AutoFit`: `repeat(auto-fit, minmax(…, 1fr))`, which fills the available width with as many columns as fit and needs no media query. It also responds to its **container**, not the viewport, so a grid inside a sidebar behaves correctly without knowing it is in a sidebar.

## Import

```rust
use maud_ui::primitives::grid::{self, Align, Columns, MinColumn, Props, Space};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::{card, grid};

// A dashboard card grid: as many ≥18rem columns as fit.
html! {
    (grid::render(grid::Props {
        min_column: grid::MinColumn::Lg,
        gap: grid::Space::Lg,
        children: html! {
            @for title in ["Revenue", "Subscriptions", "Active now"] {
                (card::render(card::Props {
                    title: Some(title.into()),
                    ..Default::default()
                }))
            }
        },
        ..Default::default()
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `columns` | `Columns` | `AutoFit` | Column track definition |
| `min_column` | `MinColumn` | `Md` (`14rem`) | Minimum column width for `AutoFit`; no effect on fixed counts |
| `gap` | `Space` | `Md` (`0.75rem`) | Space between both rows and columns |
| `padding` | `Space` | `None` | Inset applied by the container itself |
| `align` | `Align` | `Stretch` | Block-axis alignment of items within their track (`align-items`) |
| `collapse_narrow` | `bool` | `true` | Collapse fixed column counts to one column below `40rem` |
| `id` | `Option<String>` | `None` | Optional `id`, for anchor targets and `aria-labelledby` references |
| `children` | `Markup` | `html! {}` | Grid children — each becomes one grid item |

## Variants / Enums

### Columns
- `AutoFit`: as many equal columns as fit, each at least `min_column` wide — the default, and the only option that adapts to its container rather than the viewport
- `One` · `Two` · `Three` · `Four` · `Five` · `Six`: that many equal columns
- `Twelve`: the classic layout grid, for children that span several tracks

### MinColumn
Decides how many columns `AutoFit` produces. Ignored by fixed counts.

- `Sm`: `10rem` — dense: chips, swatches, small tiles
- `Md`: `14rem` — the default: stat cards, thumbnails
- `Lg`: `18rem` — content cards
- `Xl`: `24rem` — wide panels, forms

### Space
Re-exported from [stack](stack.md) — `None` · `Xs` · `Sm` · `Md` · `Lg` · `Xl` · `Xxl`, resolving to `0` · `0.25rem` · `0.5rem` · `0.75rem` · `1rem` · `1.5rem` · `2rem` through the `--mui-space-*` custom properties.

### Align
Re-exported from [stack](stack.md) — `Stretch` · `Start` · `Center` · `End` · `Baseline`.

## Helper Functions

| Function | Signature | Purpose |
|----------|-----------|---------|
| `auto` | `fn(Markup) -> Markup` | Auto-fitting grid with the default gap and minimum column width |
| `columns` | `fn(Columns, Markup) -> Markup` | Fixed-column grid with the default gap, collapsing on narrow viewports |
| `Columns::as_class` | `fn(self) -> &'static str` | Modifier class for a track definition, empty for the default |
| `Columns::count` | `fn(self) -> Option<u8>` | The number of columns, or `None` for `AutoFit` |
| `MinColumn::as_class` | `fn(self) -> &'static str` | Modifier class, empty for the default |
| `MinColumn::as_length` | `fn(self) -> &'static str` | The CSS length a step resolves to |

## Accessibility

A grid is presentational: it changes visual position without changing DOM order, so keyboard focus and screen-reader order continue to follow source order. Keep the children in the order they should be read.

`collapse_narrow` defaults to `true` because a four-column grid on a phone is four unreadable slivers, and this crate offers no class escape hatch a consumer could use to fix that themselves — so the responsive behaviour has to live in the primitive. Set it to `false` only when the cells are genuinely small (a colour palette, a keypad) and should stay side by side at every width.

Note that `grid` does not set `role="grid"`. That ARIA role describes an interactive tabular widget with its own keyboard model, not a layout — applying it here would promise navigation behaviour that does not exist. For tabular data use [table](table.md) or [data_table](data_table.md).

## Related

Stack (one-axis container; the two compose — grid for columns, stack for each cell's rows), Card (the usual grid child), AspectRatio (fixed-ratio media cells), Table (tabular *data*, not layout)

## Shadcn reference

No direct shadcn/Base UI equivalent — shadcn leans on Tailwind utility classes (`grid grid-cols-3 gap-4`) for this role. `grid` fills the same need in a utility-free, enumerable form.
