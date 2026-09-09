# DataTable

Table with sorting, filtering, pagination, and row selection.

## Import

```rust
use maud_ui::primitives::data_table::{self, Props, Column};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::data_table;

let columns = vec![
    data_table::Column {
        key: "name".to_string(),
        label: "Name".to_string(),
        sortable: true, ..Default::default()
    },
];

let rows = vec![
    vec!["Alice".to_string()],
    vec!["Bob".to_string()],
];

html! {
    (data_table::render(data_table::Props {
        id: "tbl".to_string(),
        columns,
        rows,
        page_size: 5,
        searchable: true,
        search_placeholder: "Filter...".to_string(),
        selectable: true,
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | `"data-table"` | Unique identifier for the table |
| `columns` | `Vec<Column>` | `vec![]` | Column definitions |
| `rows` | `Vec<Vec<String>>` | `vec![]` | Row data (each row is a Vec of cell strings) |
| `page_size` | `usize` | `5` | Number of rows displayed per page |
| `searchable` | `bool` | `false` | Whether a search/filter input is shown |
| `search_placeholder` | `String` | `"Filter..."` | Placeholder text for search input |
| `selectable` | `bool` | `false` | Whether to prepend row-selection checkboxes |

## Column Struct

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `key` | `String` | — | Unique column identifier |
| `label` | `String` | — | Column header text |
| `sortable` | `bool` | false | Whether column can be sorted (shows icon if true) |
| `align` | `Align` | Left | Left, Center or Right. Applied to headers and body; rich cell align_right overrides to Right. |

## Helper Functions

### `column_header(label, sortable) → Markup`
Renders a `<th>` with optional sort icon and `aria-sort` attribute. Use when constructing custom table headers.

### `view_options(columns) → Markup`
Renders a "Columns ▾" dropdown trigger stub for toggling column visibility. Placeholder implementation; wiring toggles is deferred.

## Features

- **Pagination**: Previous/Next buttons in footer; rows outside page size are `hidden`
- **Selection**: When `selectable: true`, prepends "Select all" checkbox in header and per-row checkboxes
- **Row data**: Each row stored in `data-row-data` as JSON array for client script access
- **Sorting**: Sortable columns emit `data-sortable="true"` with chevron icon and `aria-sort` attribute

## Accessibility

- Table has standard semantic `<thead>` and `<tbody>` structure
- Checkboxes have `aria-label` for accessibility
- Sort icons rendered with `aria-sort="none"` (values: none, ascending, descending, other, auto per ARIA 1.2)

## Related

- Table
- Checkbox
- Input (search)

## Shadcn reference
<https://ui.shadcn.com/docs/components/data-table>

### `column_header_aligned(label, sortable, Align::Right) → Markup`
The original two-argument helper keeps left alignment; the aligned helper supports Left, Center and Right. Use Right for numeric columns. `Column` now has `align: Align` and implements Default; add `..Default::default()` when migrating existing literals. Header cells remain muted and medium weight. Sortable headers respond to Enter/Space as well as clicks and announce their sort state. Runtime sorting/pagination moves existing rows, preserving alignment, rich content, checkbox state and attached controls. Rich rows sort/filter by their visible cell text; server-side rich filtering remains an application choice.
