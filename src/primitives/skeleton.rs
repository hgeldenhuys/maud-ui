//! Skeleton component — loading placeholder with shimmer animation.

use maud::{html, Markup};

/// Skeleton variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Text,
    Circle,
    Rect,
}

impl Variant {
    fn class(&self) -> &'static str {
        match self {
            Self::Text => "mui-skeleton--text",
            Self::Circle => "mui-skeleton--circle",
            Self::Rect => "mui-skeleton--rect",
        }
    }
}

/// Skeleton rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// Visual variant
    pub variant: Variant,
    /// Optional width override
    pub width: Option<String>,
    /// Optional height override
    pub height: Option<String>,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            variant: Variant::Rect,
            width: None,
            height: None,
        }
    }
}

/// Render a single skeleton with the given properties
pub fn render(props: Props) -> Markup {
    let width_style = props.width.as_ref().map(|w| format!("width:{};", w));
    let height_style = props.height.as_ref().map(|h| format!("height:{};", h));
    let style = format!(
        "{}{}",
        width_style.unwrap_or_default(),
        height_style.unwrap_or_default()
    );

    if style.is_empty() {
        html! {
            div class={"mui-skeleton " (props.variant.class())} aria-hidden="true" {}
        }
    } else {
        html! {
            div class={"mui-skeleton " (props.variant.class())} style=(style) aria-hidden="true" {}
        }
    }
}

/// Showcase all skeleton variants and use cases
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            // Text skeleton row
            div {
                p.mui-showcase__caption { "Text lines" }
                div.mui-showcase__column {
                    (render(Props {
                        variant: Variant::Text,
                        width: Some("100%".into()),
                        height: None,
                    }))
                    (render(Props {
                        variant: Variant::Text,
                        width: Some("80%".into()),
                        height: None,
                    }))
                    (render(Props {
                        variant: Variant::Text,
                        width: Some("60%".into()),
                        height: None,
                    }))
                }
            }

            // Circle skeleton
            div {
                p.mui-showcase__caption { "Avatar" }
                div.mui-showcase__row {
                    (render(Props {
                        variant: Variant::Circle,
                        width: Some("3rem".into()),
                        height: Some("3rem".into()),
                    }))
                }
            }

            // Rect skeleton
            div {
                p.mui-showcase__caption { "Card" }
                div.mui-showcase__row {
                    (render(Props {
                        variant: Variant::Rect,
                        width: Some("200px".into()),
                        height: Some("120px".into()),
                    }))
                }
            }
        }
    }
}
