//! Alert component — callout/banner for important messages.
use maud::{html, Markup};

/// Alert color variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Default,
    Info,
    Success,
    Warning,
    Danger,
}

impl Variant {
    fn class(&self) -> &'static str {
        match self {
            Self::Default => "mui-alert--default",
            Self::Info => "mui-alert--info",
            Self::Success => "mui-alert--success",
            Self::Warning => "mui-alert--warning",
            Self::Danger => "mui-alert--danger",
        }
    }

    fn icon_char(&self) -> &'static str {
        match self {
            Self::Default | Self::Info => "ℹ",
            Self::Success => "✓",
            Self::Warning => "⚠",
            Self::Danger => "✕",
        }
    }
}

/// Alert rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// Alert title text
    pub title: String,
    /// Optional description text
    pub description: Option<String>,
    /// Visual variant (color scheme)
    pub variant: Variant,
    /// Whether to show the icon (default true)
    pub icon: bool,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: None,
            variant: Variant::Default,
            icon: true,
        }
    }
}

/// Render a single alert with the given properties
pub fn render(props: Props) -> Markup {
    html! {
        div class={"mui-alert " (props.variant.class())} role="alert" {
            @if props.icon {
                div class="mui-alert__icon" aria-hidden="true" {
                    (props.variant.icon_char())
                }
            }
            div class="mui-alert__content" {
                div class="mui-alert__title" {
                    (props.title)
                }
                @if let Some(desc) = props.description {
                    p class="mui-alert__description" {
                        (desc)
                    }
                }
            }
        }
    }
}

/// Showcase all alert variants and use cases
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            // Variants row
            div {
                p.mui-showcase__caption { "Variants" }
                div.mui-showcase__column {
                    (render(Props {
                        title: "Default".into(),
                        description: Some("This is a default alert message.".into()),
                        variant: Variant::Default,
                        icon: true,
                    }))
                    (render(Props {
                        title: "Info".into(),
                        description: Some("This is an info alert message.".into()),
                        variant: Variant::Info,
                        icon: true,
                    }))
                    (render(Props {
                        title: "Success".into(),
                        description: Some("This is a success alert message.".into()),
                        variant: Variant::Success,
                        icon: true,
                    }))
                    (render(Props {
                        title: "Warning".into(),
                        description: Some("This is a warning alert message.".into()),
                        variant: Variant::Warning,
                        icon: true,
                    }))
                    (render(Props {
                        title: "Danger".into(),
                        description: Some("This is a danger alert message.".into()),
                        variant: Variant::Danger,
                        icon: true,
                    }))
                }
            }

            // Use cases row
            div {
                p.mui-showcase__caption { "Use cases" }
                div.mui-showcase__column {
                    (render(Props {
                        title: "System maintenance".into(),
                        description: Some("The system will be down for maintenance on Sunday at 2 AM.".into()),
                        variant: Variant::Info,
                        icon: true,
                    }))
                    (render(Props {
                        title: "Changes saved".into(),
                        description: None,
                        variant: Variant::Success,
                        icon: true,
                    }))
                    (render(Props {
                        title: "Storage nearly full".into(),
                        description: Some("You have 50 MB left. Please delete some files.".into()),
                        variant: Variant::Warning,
                        icon: true,
                    }))
                }
            }
        }
    }
}
