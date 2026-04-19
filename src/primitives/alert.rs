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
            Self::Default => "\u{25cb}", // circle
            Self::Info => "\u{24d8}",    // circled i
            Self::Success => "\u{2713}", // checkmark
            Self::Warning => "\u{26a0}", // warning triangle
            Self::Danger => "\u{26a0}",  // warning triangle (destructive)
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
            div {
                p.mui-showcase__caption { "Subscription expiring" }
                div.mui-showcase__column {
                    (render(Props {
                        title: "Your Pro plan expires in 3 days".into(),
                        description: Some("Renew by Apr 19 to keep unlimited builds and priority support. After expiry your workspace drops to the Free tier and projects over 3 will be archived.".into()),
                        variant: Variant::Warning,
                        icon: true,
                    }))
                }
            }

            div {
                p.mui-showcase__caption { "Two-factor enabled" }
                div.mui-showcase__column {
                    (render(Props {
                        title: "Two-factor authentication is on".into(),
                        description: Some("Backup codes were emailed to invoice@geldentech.ca. Store them somewhere safe — you'll need one if you lose access to your authenticator.".into()),
                        variant: Variant::Success,
                        icon: true,
                    }))
                }
            }

            div {
                p.mui-showcase__caption { "Destructive — API key revocation" }
                div.mui-showcase__column {
                    (render(Props {
                        title: "Revoke key sk_live_\u{2026}A9f2?".into(),
                        description: Some("Any services using this key will stop working immediately. This action cannot be undone — you'll need to issue a new key and redeploy.".into()),
                        variant: Variant::Danger,
                        icon: true,
                    }))
                }
            }

            div {
                p.mui-showcase__caption { "Informational" }
                div.mui-showcase__column {
                    (render(Props {
                        title: "Scheduled maintenance Sunday 02:00 UTC".into(),
                        description: Some("The build pipeline will be paused for roughly 20 minutes. In-flight deploys will resume automatically once maintenance completes.".into()),
                        variant: Variant::Info,
                        icon: true,
                    }))
                }
            }
        }
    }
}
