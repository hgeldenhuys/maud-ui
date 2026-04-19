# Table

A flexible HTML table primitive with support for striped rows, hover states, compact sizing, and rich cell content.

## Description

Table renders semantic HTML tables with built-in support for headers, body rows, and footer rows. Cells can contain plain text or rich Markup content (including nested components like badges). Tables support visual modifiers for better readability and UX patterns.

## Import

```rust
use maud_ui::primitives::table;
```

## Example

```rust
use maud::{html, Markup};
use maud_ui::primitives::table::{Props, CellMarkup, render};

let headers = vec![
    "Invoice".to_string(),
    "Status".to_string(),
    "Amount".to_string(),
];

let rows = vec![
    vec!["INV001".to_string(), "Paid".to_string(), "$250.00".to_string()],
    vec!["INV002".to_string(), "Pending".to_string(), "$150.00".to_string()],
];

let table = render(Props {
    headers,
    rows,
    striped: true,
    hoverable: true,
    right_align_cols: vec![2],
    ..Default::default()
});
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `headers` | `Vec<String>` | `vec![]` | Column header labels |
| `rows` | `Vec<Vec<String>>` | `vec![]` | Table body rows (plain text cells) |
| `rich_rows` | `Vec<Vec<CellMarkup>>` | `vec![]` | Table body rows (rich Markup cells) |
| `footer_row` | `Vec<CellMarkup>` | `vec![]` | Optional footer row (totals, summaries) |
| `striped` | `bool` | `false` | Alternating row background colors |
| `hoverable` | `bool` | `false` | Row highlight on hover |
| `compact` | `bool` | `false` | Reduced padding (compact row heights) |
| `caption` | `Option<String>` | `None` | Table caption (semantic, screen-reader visible) |
| `right_align_cols` | `Vec<usize>` | `vec![]` | Column indices to right-align (e.g., numbers) |

## CellMarkup

Helper struct for rich (Markup-based) table cells.

| Method | Signature | Description |
|--------|-----------|-------------|
| `text(s: &str)` | `Self` | Create a left-aligned text cell |
| `right(s: &str)` | `Self` | Create a right-aligned text cell |
| `markup(m: Markup, align_right: bool)` | `Self` | Create a cell with custom Markup (e.g., badges, buttons) |

## Variants

### Striped
Alternating row background colors improve readability for wide tables.

```rust
render(Props {
    striped: true,
    ..Default::default()
})
```

### Hoverable
Row background highlights on hover, improving interaction feedback.

```rust
render(Props {
    hoverable: true,
    ..Default::default()
})
```

### Compact
Reduced padding for dense data displays.

```rust
render(Props {
    compact: true,
    ..Default::default()
})
```

### Rich Cells (with Markup)
Embed components like badges, buttons, or custom HTML inside cells.

```rust
use maud_ui::primitives::{table, badge};

let rich_rows = vec![
    vec![
        CellMarkup::text("INV001"),
        CellMarkup::markup(
            badge::render(badge::Props {
                label: "Paid".into(),
                variant: badge::Variant::Success,
                ..Default::default()
            }),
            false,
        ),
        CellMarkup::right("$250.00"),
    ],
];

render(Props {
    headers: vec!["Invoice".into(), "Status".into(), "Amount".into()],
    rich_rows,
    ..Default::default()
})
```

### With Footer
Totals or summary rows with optional right alignment for numbers.

```rust
let footer_row = vec![
    CellMarkup::text("Total"),
    CellMarkup::text(""),
    CellMarkup::right("$1,750.00"),
];

render(Props {
    footer_row,
    ..Default::default()
})
```

### With Caption
Semantic table captions improve accessibility and provide context.

```rust
render(Props {
    caption: Some("A list of your recent invoices.".into()),
    ..Default::default()
})
```

## Accessibility

- **Semantic HTML:** Uses `<thead>`, `<tbody>`, `<tfoot>`, and proper table structure.
- **Caption:** Use the `caption` field for a semantic table title, visible to screen readers.
- **Right Alignment:** Declare numeric columns in `right_align_cols` for proper visual alignment and screen-reader interpretation.
- **Hover/Striping:** Visual indicators (hover, striped) reinforce row boundaries but are not required for understanding.

## Related

- [Data Table](/docs/components/data-table.md) — Complex tables with sorting, filtering, and pagination
- [Badge](/docs/components/badge.md) — Status indicators often embedded in table cells
- [Typography](/docs/components/typography.md) — Text styling inside cells

## Shadcn Reference

Maud-ui `table` is a HTML-semantic wrapper for shadcn's `<Table>` component pattern.
