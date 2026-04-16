//! Badge component — minimal status indicators for tagging and labeling.

use maud::{html, Markup};

/// Badge color variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Default,
    Success,
    Warning,
    Danger,
    Outline,
}

impl Variant {
    fn class(&self) -> &'static str {
        match self {
            Self::Default => "mui-badge--default",
            Self::Success => "mui-badge--success",
            Self::Warning => "mui-badge--warning",
            Self::Danger => "mui-badge--danger",
            Self::Outline => "mui-badge--outline",
        }
    }
}

/// Badge rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// Text content displayed in the badge
    pub label: String,
    /// Visual variant (color scheme)
    pub variant: Variant,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            label: String::new(),
            variant: Variant::Default,
        }
    }
}

/// Render a single badge with the given properties
pub fn render(props: Props) -> Markup {
    html! {
        span class={"mui-badge " (props.variant.class())} {
            (props.label)
        }
    }
}

/// Showcase all badge variants and use cases
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            // Variants row
            div {
                p.mui-showcase__caption { "Variants" }
                div.mui-showcase__row {
                    (render(Props {
                        label: "Default".into(),
                        variant: Variant::Default,
                    }))
                    (render(Props {
                        label: "Success".into(),
                        variant: Variant::Success,
                    }))
                    (render(Props {
                        label: "Warning".into(),
                        variant: Variant::Warning,
                    }))
                    (render(Props {
                        label: "Danger".into(),
                        variant: Variant::Danger,
                    }))
                    (render(Props {
                        label: "Outline".into(),
                        variant: Variant::Outline,
                    }))
                }
            }

            // Use cases row
            div {
                p.mui-showcase__caption { "Use cases" }
                div.mui-showcase__row {
                    (render(Props {
                        label: "Active".into(),
                        variant: Variant::Success,
                    }))
                    (render(Props {
                        label: "Deprecated".into(),
                        variant: Variant::Danger,
                    }))
                    (render(Props {
                        label: "Beta".into(),
                        variant: Variant::Warning,
                    }))
                    (render(Props {
                        label: "5 new".into(),
                        variant: Variant::Default,
                    }))
                }
            }
        }
    }
}
