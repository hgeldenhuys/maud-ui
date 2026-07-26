//! Badge component — minimal status indicators for tagging and labeling.

use maud::{html, Markup};

/// Badge color variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Variant {
    #[default]
    Default,
    Secondary,
    Success,
    Warning,
    Danger,
    /// Informational blue — the neutral-but-noteworthy tier.
    Info,
    /// Brand accent solid — same hue as `Default` but semantic, so a
    /// consumer can retint `Default` without losing the accent slot.
    Accent,
    /// Violet — for taxonomy hues beyond the four semantic states.
    Violet,
    /// Rose — for taxonomy hues beyond the four semantic states.
    Rose,
    Outline,
    Ghost,
    Link,
}

impl Variant {
    fn class(&self) -> &'static str {
        match self {
            Self::Default => "mui-badge--default",
            Self::Secondary => "mui-badge--secondary",
            Self::Success => "mui-badge--success",
            Self::Warning => "mui-badge--warning",
            Self::Danger => "mui-badge--danger",
            Self::Info => "mui-badge--info",
            Self::Accent => "mui-badge--accent",
            Self::Violet => "mui-badge--violet",
            Self::Rose => "mui-badge--rose",
            Self::Outline => "mui-badge--outline",
            Self::Ghost => "mui-badge--ghost",
            Self::Link => "mui-badge--link",
        }
    }
}

/// Badge rendering properties
#[derive(Debug, Clone, Default)]
pub struct Props {
    /// Text content displayed in the badge
    pub label: String,
    /// Visual variant (color scheme)
    pub variant: Variant,
    /// Optional href — when `Some`, the badge renders as an `<a>` element
    pub href: Option<String>,
    /// Optional leading icon — rendered before the label with `data-icon="inline-start"`
    pub leading_icon: Option<Markup>,
    /// Render the label in the monospace face — for shas, ids, counts, and
    /// other machine-register text that should not reflow with the sans stack.
    pub mono: bool,
}

/// Render a single badge with the given properties
pub fn render(props: Props) -> Markup {
    let mut class = format!("mui-badge {}", props.variant.class());
    if props.mono {
        class.push_str(" mui-badge--mono");
    }
    let data_icon = if props.leading_icon.is_some() {
        Some("inline-start")
    } else {
        None
    };

    html! {
        @if let Some(href) = props.href.as_ref() {
            a class=(class) href=(href) data-icon=[data_icon] {
                @if let Some(icon) = props.leading_icon.as_ref() {
                    (icon)
                }
                (props.label)
            }
        } @else {
            span class=(class) data-icon=[data_icon] {
                @if let Some(icon) = props.leading_icon.as_ref() {
                    (icon)
                }
                (props.label)
            }
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
                    (render(Props { label: "Default".into(), variant: Variant::Default, ..Default::default() }))
                    (render(Props { label: "Secondary".into(), variant: Variant::Secondary, ..Default::default() }))
                    (render(Props { label: "Success".into(), variant: Variant::Success, ..Default::default() }))
                    (render(Props { label: "Warning".into(), variant: Variant::Warning, ..Default::default() }))
                    (render(Props { label: "Danger".into(), variant: Variant::Danger, ..Default::default() }))
                    (render(Props { label: "Info".into(), variant: Variant::Info, ..Default::default() }))
                    (render(Props { label: "Accent".into(), variant: Variant::Accent, ..Default::default() }))
                    (render(Props { label: "Violet".into(), variant: Variant::Violet, ..Default::default() }))
                    (render(Props { label: "Rose".into(), variant: Variant::Rose, ..Default::default() }))
                    (render(Props { label: "Outline".into(), variant: Variant::Outline, ..Default::default() }))
                    (render(Props { label: "Ghost".into(), variant: Variant::Ghost, ..Default::default() }))
                    (render(Props {
                        label: "Link".into(),
                        variant: Variant::Link,
                        href: Some("#".into()),
                        ..Default::default()
                    }))
                }
            }

            // Leading icon
            section {
                h2 { "Leading icon" }
                div.mui-showcase__row {
                    (render(Props {
                        label: "Live".into(),
                        variant: Variant::Default,
                        leading_icon: Some(html! { span style="font-size:0.625rem;line-height:1;" { "●" } }),
                        ..Default::default()
                    }))
                    (render(Props {
                        label: "Paused".into(),
                        variant: Variant::Secondary,
                        leading_icon: Some(html! { span style="font-size:0.625rem;line-height:1;" { "◼" } }),
                        ..Default::default()
                    }))
                    (render(Props {
                        label: "Docs".into(),
                        variant: Variant::Link,
                        href: Some("#".into()),
                        leading_icon: Some(html! { span style="font-size:0.625rem;line-height:1;" { "→" } }),
                        ..Default::default()
                    }))
                }
            }

            // Monospace face — shas, ids, counts
            section {
                h2 { "Monospace" }
                div.mui-showcase__row {
                    (render(Props { label: "b6e38d1e".into(), variant: Variant::Outline, mono: true, ..Default::default() }))
                    (render(Props { label: "IMP-2491".into(), variant: Variant::Secondary, mono: true, ..Default::default() }))
                    (render(Props { label: "v0.3.0".into(), variant: Variant::Info, mono: true, ..Default::default() }))
                }
            }

            // Realistic inline context
            section {
                h2 { "In Context" }
                // Navigation-style items with counter badges
                div style="display:flex;flex-direction:column;gap:0.75rem;max-width:20rem;" {
                    div style="display:flex;align-items:center;justify-content:space-between;" {
                        span style="font-size:0.875rem;color:var(--mui-text);" { "Inbox" }
                        (render(Props { label: "3".into(), variant: Variant::Default, ..Default::default() }))
                    }
                    div style="display:flex;align-items:center;justify-content:space-between;" {
                        span style="font-size:0.875rem;color:var(--mui-text);" { "Drafts" }
                        (render(Props { label: "12".into(), variant: Variant::Secondary, ..Default::default() }))
                    }
                    div style="display:flex;align-items:center;justify-content:space-between;" {
                        span style="font-size:0.875rem;color:var(--mui-text);" { "Errors" }
                        (render(Props { label: "2".into(), variant: Variant::Danger, ..Default::default() }))
                    }
                }
            }

            // Labels in a list
            section {
                h2 { "Labels" }
                div style="display:flex;flex-direction:column;gap:0.5rem;" {
                    div style="display:flex;align-items:center;gap:0.5rem;" {
                        span style="font-size:0.875rem;color:var(--mui-text);min-width:8rem;" { "Authentication API" }
                        (render(Props { label: "Stable".into(), variant: Variant::Success, ..Default::default() }))
                        (render(Props { label: "v2.1".into(), variant: Variant::Outline, ..Default::default() }))
                    }
                    div style="display:flex;align-items:center;gap:0.5rem;" {
                        span style="font-size:0.875rem;color:var(--mui-text);min-width:8rem;" { "Streaming SDK" }
                        (render(Props { label: "Beta".into(), variant: Variant::Warning, ..Default::default() }))
                        (render(Props { label: "v0.9".into(), variant: Variant::Outline, ..Default::default() }))
                    }
                    div style="display:flex;align-items:center;gap:0.5rem;" {
                        span style="font-size:0.875rem;color:var(--mui-text);min-width:8rem;" { "Legacy Client" }
                        (render(Props { label: "Deprecated".into(), variant: Variant::Danger, ..Default::default() }))
                        (render(Props { label: "v1.0".into(), variant: Variant::Outline, ..Default::default() }))
                    }
                }
            }
        }
    }
}
