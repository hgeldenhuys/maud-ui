//! Data Table component — composes Table + sorting + filtering + pagination.
use maud::{html, Markup, PreEscaped};

#[derive(Clone, Debug)]
pub struct Column {
    pub key: String,
    pub label: String,
    pub sortable: bool,
}

#[derive(Clone, Debug)]
pub struct Props {
    pub id: String,
    pub columns: Vec<Column>,
    pub rows: Vec<Vec<String>>,
    pub page_size: usize,
    pub searchable: bool,
    pub search_placeholder: String,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            id: "data-table".to_string(),
            columns: vec![],
            rows: vec![],
            page_size: 5,
            searchable: false,
            search_placeholder: "Filter...".to_string(),
        }
    }
}

/// Escape a string for safe embedding in a JSON array stored as an HTML attribute.
fn escape_for_attr(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('&', "&amp;")
}

pub fn render(props: Props) -> Markup {
    let total = props.rows.len();
    let page_size = if props.page_size == 0 {
        5
    } else {
        props.page_size
    };
    let end = std::cmp::min(page_size, total);
    let start = if total == 0 { 0 } else { 1 };
    let has_next = end < total;

    html! {
        div.mui-data-table data-mui="data-table" id=(props.id) data-page-size=(page_size) {
            @if props.searchable {
                div.mui-data-table__toolbar {
                    input type="text" class="mui-input mui-data-table__search"
                        placeholder=(props.search_placeholder);
                }
            }
            div.mui-data-table__wrapper {
                table.mui-table.mui-table--hoverable {
                    thead {
                        tr {
                            @for col in &props.columns {
                                th.mui-table__th.mui-data-table__th
                                    data-key=(col.key)
                                    data-sortable=(col.sortable) {
                                    (col.label)
                                    @if col.sortable {
                                        span.mui-data-table__sort-icon {
                                            (PreEscaped(r#"<svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m7 15 5 5 5-5"/><path d="m7 9 5-5 5 5"/></svg>"#))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    tbody.mui-data-table__body {
                        @for (i, row) in props.rows.iter().enumerate() {
                            @let json_row = format!(
                                "[{}]",
                                row.iter()
                                    .map(|c| format!("\"{}\"", escape_for_attr(c)))
                                    .collect::<Vec<_>>()
                                    .join(",")
                            );
                            tr.mui-table__row
                                data-row-data=(json_row)
                                hidden[i >= page_size] {
                                @for cell in row {
                                    td.mui-table__td { (cell) }
                                }
                            }
                        }
                    }
                }
            }
            div.mui-data-table__footer {
                span.mui-data-table__info {
                    (format!("Showing {}-{} of {}", start, end, total))
                }
                div.mui-data-table__pagination {
                    button type="button" class="mui-data-table__page-btn" data-action="prev" disabled {
                        "Previous"
                    }
                    button type="button" class="mui-data-table__page-btn" data-action="next" disabled[!has_next] {
                        "Next"
                    }
                }
            }
        }
    }
}

pub fn showcase() -> Markup {
    let columns = vec![
        Column {
            key: "invoice".to_string(),
            label: "Invoice".to_string(),
            sortable: true,
        },
        Column {
            key: "status".to_string(),
            label: "Status".to_string(),
            sortable: true,
        },
        Column {
            key: "method".to_string(),
            label: "Method".to_string(),
            sortable: true,
        },
        Column {
            key: "amount".to_string(),
            label: "Amount".to_string(),
            sortable: true,
        },
    ];

    let rows = vec![
        vec![
            "INV001".to_string(),
            "Paid".to_string(),
            "Credit Card".to_string(),
            "$250.00".to_string(),
        ],
        vec![
            "INV002".to_string(),
            "Pending".to_string(),
            "PayPal".to_string(),
            "$150.00".to_string(),
        ],
        vec![
            "INV003".to_string(),
            "Unpaid".to_string(),
            "Bank Transfer".to_string(),
            "$350.00".to_string(),
        ],
        vec![
            "INV004".to_string(),
            "Paid".to_string(),
            "Credit Card".to_string(),
            "$450.00".to_string(),
        ],
        vec![
            "INV005".to_string(),
            "Paid".to_string(),
            "PayPal".to_string(),
            "$550.00".to_string(),
        ],
        vec![
            "INV006".to_string(),
            "Pending".to_string(),
            "Bank Transfer".to_string(),
            "$200.00".to_string(),
        ],
        vec![
            "INV007".to_string(),
            "Paid".to_string(),
            "Credit Card".to_string(),
            "$300.00".to_string(),
        ],
        vec![
            "INV008".to_string(),
            "Unpaid".to_string(),
            "PayPal".to_string(),
            "$400.00".to_string(),
        ],
        vec![
            "INV009".to_string(),
            "Paid".to_string(),
            "Bank Transfer".to_string(),
            "$500.00".to_string(),
        ],
        vec![
            "INV010".to_string(),
            "Pending".to_string(),
            "Credit Card".to_string(),
            "$275.00".to_string(),
        ],
    ];

    html! {
        div.mui-showcase__grid {
            div {
                p.mui-showcase__caption { "Searchable, sortable, paginated (5 per page)" }
                (render(Props {
                    id: "invoice-table".to_string(),
                    columns,
                    rows,
                    page_size: 5,
                    searchable: true,
                    search_placeholder: "Filter invoices...".to_string(),
                }))
            }
        }
    }
}
