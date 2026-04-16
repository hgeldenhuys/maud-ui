//! Spinner component — circular loading indicator.

use maud::{html, Markup};

/// Spinner size
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Size {
    Sm,
    Md,
    Lg,
}

impl Size {
    fn class(&self) -> &'static str {
        match self {
            Self::Sm => "mui-spinner--sm",
            Self::Md => "mui-spinner--md",
            Self::Lg => "mui-spinner--lg",
        }
    }
}

/// Spinner rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// Spinner size
    pub size: Size,
    /// Optional aria-label override
    pub label: Option<String>,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            size: Size::Md,
            label: None,
        }
    }
}

/// Render a single spinner with the given properties
pub fn render(props: Props) -> Markup {
    let label = props.label.unwrap_or_else(|| "Loading".into());
    html! {
        span class={"mui-spinner " (props.size.class())} role="status" aria-label=(label) {}
    }
}

/// Showcase all spinner sizes and use cases
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            // Sizes row
            div {
                p.mui-showcase__caption { "Sizes" }
                div.mui-showcase__row {
                    (render(Props {
                        size: Size::Sm,
                        label: None,
                    }))
                    (render(Props {
                        size: Size::Md,
                        label: None,
                    }))
                    (render(Props {
                        size: Size::Lg,
                        label: None,
                    }))
                }
            }

            // With label row
            div {
                p.mui-showcase__caption { "With label" }
                div.mui-showcase__row {
                    (render(Props {
                        size: Size::Md,
                        label: Some("Saving...".into()),
                    }))
                }
            }
        }
    }
}
