//! Table component.
use maud::{html, Markup};

#[derive(Clone, Debug)]
pub struct CellMarkup {
    pub content: Markup,
    pub align_right: bool,
}

impl CellMarkup {
    pub fn text(s: &str) -> Self {
        Self {
            content: html! { (s) },
            align_right: false,
        }
    }
    pub fn right(s: &str) -> Self {
        Self {
            content: html! { (s) },
            align_right: true,
        }
    }
    pub fn markup(m: Markup, align_right: bool) -> Self {
        Self {
            content: m,
            align_right,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Props {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub rich_rows: Vec<Vec<CellMarkup>>,
    pub footer_row: Vec<CellMarkup>,
    pub striped: bool,
    pub hoverable: bool,
    pub compact: bool,
    pub caption: Option<String>,
    /// Column indices that should be right-aligned in headers
    pub right_align_cols: Vec<usize>,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            headers: vec![],
            rows: vec![],
            rich_rows: vec![],
            footer_row: vec![],
            striped: false,
            hoverable: false,
            compact: false,
            caption: None,
            right_align_cols: vec![],
        }
    }
}

pub fn render(props: Props) -> Markup {
    let mut modifiers = String::new();

    if props.striped {
        modifiers.push_str(" mui-table--striped");
    }
    if props.hoverable {
        modifiers.push_str(" mui-table--hoverable");
    }
    if props.compact {
        modifiers.push_str(" mui-table--compact");
    }

    let class = format!("mui-table{}", modifiers);
    let has_rich = !props.rich_rows.is_empty();
    let has_footer = !props.footer_row.is_empty();

    html! {
        div.mui-table-wrapper {
            table class=(class) {
                @if let Some(caption_text) = props.caption {
                    caption.mui-table__caption { (caption_text) }
                }
                thead {
                    tr {
                        @for (i, header) in props.headers.iter().enumerate() {
                            @if props.right_align_cols.contains(&i) {
                                th.mui-table__th style="text-align:right;" { (header) }
                            } @else {
                                th.mui-table__th { (header) }
                            }
                        }
                    }
                }
                tbody {
                    @if has_rich {
                        @for row in &props.rich_rows {
                            tr.mui-table__row {
                                @for cell in row {
                                    @if cell.align_right {
                                        td.mui-table__td style="text-align:right;" { (cell.content) }
                                    } @else {
                                        td.mui-table__td { (cell.content) }
                                    }
                                }
                            }
                        }
                    } @else {
                        @for row in &props.rows {
                            tr.mui-table__row {
                                @for cell in row {
                                    td.mui-table__td { (cell) }
                                }
                            }
                        }
                    }
                }
                @if has_footer {
                    tfoot {
                        tr.mui-table__row {
                            @for cell in &props.footer_row {
                                @if cell.align_right {
                                    td.mui-table__td style="text-align:right;font-weight:600;" { (cell.content) }
                                } @else {
                                    td.mui-table__td style="font-weight:600;" { (cell.content) }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn showcase() -> Markup {
    use crate::primitives::badge;

    let headers = vec![
        "Invoice".to_string(),
        "Status".to_string(),
        "Method".to_string(),
        "Amount".to_string(),
    ];

    // Status badge helper
    let status_badge = |label: &str| -> Markup {
        let variant = match label {
            "Paid" => badge::Variant::Success,
            "Pending" => badge::Variant::Warning,
            "Unpaid" => badge::Variant::Danger,
            _ => badge::Variant::Default,
        };
        badge::render(badge::Props {
            label: label.to_string(),
            variant,
            ..Default::default()
        })
    };

    let rich_rows = vec![
        vec![
            CellMarkup::text("INV001"),
            CellMarkup::markup(status_badge("Paid"), false),
            CellMarkup::text("Credit Card"),
            CellMarkup::right("$250.00"),
        ],
        vec![
            CellMarkup::text("INV002"),
            CellMarkup::markup(status_badge("Pending"), false),
            CellMarkup::text("PayPal"),
            CellMarkup::right("$150.00"),
        ],
        vec![
            CellMarkup::text("INV003"),
            CellMarkup::markup(status_badge("Unpaid"), false),
            CellMarkup::text("Bank Transfer"),
            CellMarkup::right("$350.00"),
        ],
        vec![
            CellMarkup::text("INV004"),
            CellMarkup::markup(status_badge("Paid"), false),
            CellMarkup::text("Credit Card"),
            CellMarkup::right("$450.00"),
        ],
        vec![
            CellMarkup::text("INV005"),
            CellMarkup::markup(status_badge("Paid"), false),
            CellMarkup::text("PayPal"),
            CellMarkup::right("$550.00"),
        ],
    ];

    let footer_row = vec![
        CellMarkup::text("Total"),
        CellMarkup::text(""),
        CellMarkup::text(""),
        CellMarkup::right("$1,750.00"),
    ];

    // Plain rows for the simpler variants
    let plain_rows = vec![
        vec![
            "INV001".into(),
            "Paid".into(),
            "Credit Card".into(),
            "$250.00".into(),
        ],
        vec![
            "INV002".into(),
            "Pending".into(),
            "PayPal".into(),
            "$150.00".into(),
        ],
        vec![
            "INV003".into(),
            "Unpaid".into(),
            "Bank Transfer".into(),
            "$350.00".into(),
        ],
        vec![
            "INV004".into(),
            "Paid".into(),
            "Credit Card".into(),
            "$450.00".into(),
        ],
        vec![
            "INV005".into(),
            "Paid".into(),
            "PayPal".into(),
            "$550.00".into(),
        ],
    ];

    html! {
        div.mui-showcase__grid {
            div {
                p.mui-showcase__caption { "With badges, right-aligned amounts, and footer total" }
                (render(Props {
                    headers: headers.clone(),
                    rich_rows,
                    footer_row,
                    hoverable: true,
                    right_align_cols: vec![3],
                    caption: Some("A list of your recent invoices.".to_string()),
                    ..Default::default()
                }))
            }
            div {
                p.mui-showcase__caption { "Striped + hoverable" }
                (render(Props {
                    headers: headers.clone(),
                    rows: plain_rows.clone(),
                    striped: true,
                    hoverable: true,
                    right_align_cols: vec![3],
                    ..Default::default()
                }))
            }
            div {
                p.mui-showcase__caption { "Compact" }
                (render(Props {
                    headers,
                    rows: plain_rows,
                    compact: true,
                    right_align_cols: vec![3],
                    ..Default::default()
                }))
            }
        }
    }
}
