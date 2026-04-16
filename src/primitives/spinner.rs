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
                    div style="display:flex;flex-direction:column;align-items:center;gap:0.25rem" {
                        (render(Props { size: Size::Sm, label: None }))
                        span.mui-muted style="font-size:0.75rem" { "sm" }
                    }
                    div style="display:flex;flex-direction:column;align-items:center;gap:0.25rem" {
                        (render(Props { size: Size::Md, label: None }))
                        span.mui-muted style="font-size:0.75rem" { "md" }
                    }
                    div style="display:flex;flex-direction:column;align-items:center;gap:0.25rem" {
                        (render(Props { size: Size::Lg, label: None }))
                        span.mui-muted style="font-size:0.75rem" { "lg" }
                    }
                }
            }

            // Inline with text
            div {
                p.mui-showcase__caption { "Inline with text" }
                div.mui-showcase__row {
                    span style="display:inline-flex;align-items:center;gap:0.5rem;font-size:0.875rem;color:var(--mui-text-muted)" {
                        (render(Props { size: Size::Sm, label: Some("Loading".into()) }))
                        "Loading..."
                    }
                }
            }

            // Inside a button
            div {
                p.mui-showcase__caption { "Button with spinner" }
                div.mui-showcase__row {
                    button type="button" class="mui-btn mui-btn--default mui-btn--md" disabled style="opacity:0.7;cursor:not-allowed" {
                        (render(Props { size: Size::Sm, label: Some("Submitting".into()) }))
                        "Submitting..."
                    }
                }
            }
        }
    }
}
