//! Badge component — minimal status indicators for tagging and labeling.

use maud::{html, Markup};

/// Badge color variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Default,
    Secondary,
    Success,
    Warning,
    Danger,
    Outline,
}

impl Variant {
    fn class(&self) -> &'static str {
        match self {
            Self::Default => "mui-badge--default",
            Self::Secondary => "mui-badge--secondary",
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
            // All variants
            section {
                h2 { "Variants" }
                div.mui-showcase__row {
                    (render(Props { label: "Default".into(), variant: Variant::Default }))
                    (render(Props { label: "Secondary".into(), variant: Variant::Secondary }))
                    (render(Props { label: "Success".into(), variant: Variant::Success }))
                    (render(Props { label: "Warning".into(), variant: Variant::Warning }))
                    (render(Props { label: "Danger".into(), variant: Variant::Danger }))
                    (render(Props { label: "Outline".into(), variant: Variant::Outline }))
                }
            }

            // Realistic inline context
            section {
                h2 { "In Context" }
                // Navigation-style items with counter badges
                div style="display:flex;flex-direction:column;gap:0.75rem;max-width:20rem;" {
                    div style="display:flex;align-items:center;justify-content:space-between;" {
                        span style="font-size:0.875rem;color:var(--mui-text);" { "Inbox" }
                        (render(Props { label: "3".into(), variant: Variant::Default }))
                    }
                    div style="display:flex;align-items:center;justify-content:space-between;" {
                        span style="font-size:0.875rem;color:var(--mui-text);" { "Drafts" }
                        (render(Props { label: "12".into(), variant: Variant::Secondary }))
                    }
                    div style="display:flex;align-items:center;justify-content:space-between;" {
                        span style="font-size:0.875rem;color:var(--mui-text);" { "Errors" }
                        (render(Props { label: "2".into(), variant: Variant::Danger }))
                    }
                }
            }

            // Labels in a list
            section {
                h2 { "Labels" }
                div style="display:flex;flex-direction:column;gap:0.5rem;" {
                    div style="display:flex;align-items:center;gap:0.5rem;" {
                        span style="font-size:0.875rem;color:var(--mui-text);min-width:8rem;" { "Authentication API" }
                        (render(Props { label: "Stable".into(), variant: Variant::Success }))
                        (render(Props { label: "v2.1".into(), variant: Variant::Outline }))
                    }
                    div style="display:flex;align-items:center;gap:0.5rem;" {
                        span style="font-size:0.875rem;color:var(--mui-text);min-width:8rem;" { "Streaming SDK" }
                        (render(Props { label: "Beta".into(), variant: Variant::Warning }))
                        (render(Props { label: "v0.9".into(), variant: Variant::Outline }))
                    }
                    div style="display:flex;align-items:center;gap:0.5rem;" {
                        span style="font-size:0.875rem;color:var(--mui-text);min-width:8rem;" { "Legacy Client" }
                        (render(Props { label: "Deprecated".into(), variant: Variant::Danger }))
                        (render(Props { label: "v1.0".into(), variant: Variant::Outline }))
                    }
                }
            }
        }
    }
}
