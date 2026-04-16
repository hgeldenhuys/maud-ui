//! Textarea component — multi-line text input field.

use maud::{html, Markup, PreEscaped};

/// Resize behavior for textarea
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resize {
    None,
    Vertical,
    Horizontal,
    Both,
}

impl Resize {
    fn class(&self) -> &'static str {
        match self {
            Self::None => "mui-textarea--resize-none",
            Self::Vertical => "mui-textarea--resize-vertical",
            Self::Horizontal => "mui-textarea--resize-horizontal",
            Self::Both => "mui-textarea--resize-both",
        }
    }
}

/// Textarea rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// Form field name
    pub name: String,
    /// Placeholder text
    pub placeholder: String,
    /// Text content inside textarea
    pub value: String,
    /// Number of rows (default 4)
    pub rows: u32,
    /// Unique identifier
    pub id: String,
    /// Whether field is disabled
    pub disabled: bool,
    /// Whether field is required
    pub required: bool,
    /// Whether field shows invalid state
    pub invalid: bool,
    /// Whether field is read-only
    pub readonly: bool,
    /// Resize behavior
    pub resize: Resize,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            name: String::new(),
            placeholder: String::new(),
            value: String::new(),
            rows: 4,
            id: String::new(),
            disabled: false,
            required: false,
            invalid: false,
            readonly: false,
            resize: Resize::Vertical,
        }
    }
}

/// Render a single textarea with the given properties
pub fn render(props: Props) -> Markup {
    let mut attrs = String::new();

    if props.required {
        attrs.push_str(" required");
    }
    if props.disabled {
        attrs.push_str(" disabled");
    }
    if props.readonly {
        attrs.push_str(" readonly");
    }
    if props.invalid {
        attrs.push_str(r#" aria-invalid="true""#);
    }

    let class = format!("mui-textarea {}", props.resize.class());
    let html_string = format!(
        r#"<textarea class="{}" name="{}" id="{}" placeholder="{}" rows="{}"{}>{}</textarea>"#,
        escape_html(&class),
        escape_html(&props.name),
        escape_html(&props.id),
        escape_html(&props.placeholder),
        props.rows,
        attrs,
        escape_html(&props.value)
    );

    PreEscaped(html_string)
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Showcase all textarea variants and use cases
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            // Default (4 rows)
            div {
                p.mui-showcase__caption { "Default (4 rows)" }
                (render(Props {
                    placeholder: "Type here…".into(),
                    ..Default::default()
                }))
            }

            // 6 rows
            div {
                p.mui-showcase__caption { "6 rows" }
                (render(Props {
                    rows: 6,
                    placeholder: "This textarea has 6 rows…".into(),
                    ..Default::default()
                }))
            }

            // Invalid
            div {
                p.mui-showcase__caption { "Invalid" }
                (render(Props {
                    invalid: true,
                    placeholder: "Required field".into(),
                    ..Default::default()
                }))
            }

            // Disabled
            div {
                p.mui-showcase__caption { "Disabled" }
                (render(Props {
                    disabled: true,
                    value: "This field is disabled".into(),
                    ..Default::default()
                }))
            }

            // Resize: none
            div {
                p.mui-showcase__caption { "Resize: none" }
                (render(Props {
                    resize: Resize::None,
                    placeholder: "Cannot be resized…".into(),
                    ..Default::default()
                }))
            }
        }
    }
}
