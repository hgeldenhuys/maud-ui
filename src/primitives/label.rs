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
}

impl Default for Props {
    fn default() -> Self {
        Self {
            text: String::new(),
            html_for: None,
            required: false,
        }
    }
}

/// Render a label element
pub fn render(props: Props) -> Markup {
    if let Some(html_for) = props.html_for {
        html! {
            label class="mui-label" for=(html_for) {
                (props.text)
                @if props.required {
                    span.mui-label__required { "*" }
                }
            }
        }
    } else {
        html! {
            label class="mui-label" {
                (props.text)
                @if props.required {
                    span.mui-label__required { "*" }
                }
            }
        }
    }
}

/// Showcase label variants
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            div {
                p.mui-showcase__caption { "Basic label" }
                (render(Props {
                    text: "Email address".into(),
                    html_for: Some("email-input".into()),
                    required: false,
                }))
            }

            div {
                p.mui-showcase__caption { "Required label" }
                (render(Props {
                    text: "Password".into(),
                    html_for: Some("password-input".into()),
                    required: true,
                }))
            }

            div {
                p.mui-showcase__caption { "Label with for attribute" }
                (render(Props {
                    text: "Remember me".into(),
                    html_for: Some("remember-checkbox".into()),
                    required: false,
                }))
            }
        }
    }
}
