//! Table component.
use maud::{html, Markup};

#[derive(Clone, Debug)]
pub struct Props {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub striped: bool,
    pub hoverable: bool,
    pub compact: bool,
    pub caption: Option<String>,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            headers: vec![],
            rows: vec![],
            striped: false,
            hoverable: false,
            compact: false,
            caption: None,
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

    html! {
        div.mui-table-wrapper {
            table class=(class) {
                @if let Some(caption_text) = props.caption {
                    caption.mui-table__caption { (caption_text) }
                }
                thead {
                    tr {
                        @for header in &props.headers {
                            th.mui-table__th { (header) }
                        }
                    }
                }
                tbody {
                    @for row in &props.rows {
                        tr.mui-table__row {
                            @for cell in row {
                                td.mui-table__td { (cell) }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn showcase() -> Markup {
    let headers = vec![
        "Invoice".to_string(),
        "Status".to_string(),
        "Method".to_string(),
        "Amount".to_string(),
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
    ];

    html! {
        div.mui-showcase__grid {
            div {
                p.mui-showcase__caption { "Default" }
                (render(Props {
                    headers: headers.clone(),
                    rows: rows.clone(),
                    striped: false,
                    hoverable: true,
                    compact: false,
                    caption: Some("A list of your recent invoices.".to_string()),
                }))
            }
            div {
                p.mui-showcase__caption { "Striped + hoverable" }
                (render(Props {
                    headers: headers.clone(),
                    rows: rows.clone(),
                    striped: true,
                    hoverable: true,
                    compact: false,
                    caption: None,
                }))
            }
            div {
                p.mui-showcase__caption { "Compact" }
                (render(Props {
                    headers,
                    rows,
                    striped: false,
                    hoverable: false,
                    compact: true,
                    caption: None,
                }))
            }
        }
    }
}
