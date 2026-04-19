//! Label component — standalone form label element.

use maud::{html, Markup};

/// Label rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// Label text
    pub text: String,
    /// ID of associated form control
    pub html_for: Option<String>,
    /// Whether the field is required
    pub required: bool,
    /// Whether the label should appear disabled
    pub disabled: bool,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            text: String::new(),
            html_for: None,
            required: false,
            disabled: false,
        }
    }
}

/// Render a label element
pub fn render(props: Props) -> Markup {
    let mut class = "mui-label".to_string();
    if props.disabled {
        class.push_str(" mui-label--disabled");
    }

    if let Some(html_for) = props.html_for {
        html! {
            label class=(class) for=(html_for) {
                (props.text)
                @if props.required {
                    span.mui-label__required aria-label="required" { " *" }
                }
            }
        }
    } else {
        html! {
            label class=(class) {
                (props.text)
                @if props.required {
                    span.mui-label__required aria-label="required" { " *" }
                }
            }
        }
    }
}

/// Showcase label variants
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            section {
                h2 { "Variants" }
                div.mui-showcase__column {
                    div {
                        p.mui-showcase__caption { "Default" }
                        (render(Props {
                            text: "Email address".into(),
                            html_for: Some("email-input".into()),
                            ..Props::default()
                        }))
                    }
                    div {
                        p.mui-showcase__caption { "Required" }
                        (render(Props {
                            text: "Password".into(),
                            html_for: Some("password-input".into()),
                            required: true,
                            ..Props::default()
                        }))
                    }
                    div {
                        p.mui-showcase__caption { "Disabled" }
                        (render(Props {
                            text: "Username".into(),
                            html_for: Some("username-input".into()),
                            disabled: true,
                            ..Props::default()
                        }))
                    }
                }
            }
            section {
                h2 { "With input" }
                div.mui-showcase__column {
                    div class="mui-field" {
                        (render(Props {
                            text: "Email".into(),
                            html_for: Some("demo-email".into()),
                            required: true,
                            ..Props::default()
                        }))
                        input class="mui-input" id="demo-email" type="email" placeholder="you@example.com" {}
                    }
                }
            }
        }
    }
}
