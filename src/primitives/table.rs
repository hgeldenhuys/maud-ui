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
        "Name".to_string(),
        "Email".to_string(),
        "Role".to_string(),
        "Status".to_string(),
    ];

    let rows = vec![
        vec![
            "Alice Johnson".to_string(),
            "alice@example.com".to_string(),
            "Engineer".to_string(),
            "Active".to_string(),
        ],
        vec![
            "Bob Smith".to_string(),
            "bob@example.com".to_string(),
            "Designer".to_string(),
            "Active".to_string(),
        ],
        vec![
            "Carol Williams".to_string(),
            "carol@example.com".to_string(),
            "Manager".to_string(),
            "Active".to_string(),
        ],
        vec![
            "David Brown".to_string(),
            "david@example.com".to_string(),
            "Engineer".to_string(),
            "Inactive".to_string(),
        ],
        vec![
            "Eva Martinez".to_string(),
            "eva@example.com".to_string(),
            "Designer".to_string(),
            "Active".to_string(),
        ],
    ];

    html! {
        div.mui-showcase__grid {
            section {
                h2 { "Table with Striped & Hoverable" }
                (render(Props {
                    headers: headers.clone(),
                    rows: rows.clone(),
                    striped: true,
                    hoverable: true,
                    compact: false,
                    caption: None,
                }))
            }
            section {
                h2 { "Compact Table" }
                (render(Props {
                    headers: headers.clone(),
                    rows: rows.clone(),
                    striped: false,
                    hoverable: false,
                    compact: true,
                    caption: None,
                }))
            }
            section {
                h2 { "Table with Caption" }
                (render(Props {
                    headers,
                    rows,
                    striped: true,
                    hoverable: true,
                    compact: false,
                    caption: Some("Team members".to_string()),
                }))
            }
        }
    }
}
