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
            // Loading tweet/post card
            div {
                p.mui-showcase__caption { "Loading post" }
                div style="display:flex;gap:0.75rem;padding:1rem;max-width:24rem;border:1px solid var(--mui-border,#e5e7eb);border-radius:var(--mui-radius-lg);" {
                    // Avatar
                    (render(Props {
                        variant: Variant::Circle,
                        width: Some("2.75rem".into()),
                        height: Some("2.75rem".into()),
                    }))
                    div.mui-showcase__column style="flex:1;gap:0.5rem;min-width:0;" {
                        // Handle + timestamp row
                        div style="display:flex;gap:0.5rem;align-items:center;" {
                            (render(Props {
                                variant: Variant::Text,
                                width: Some("6rem".into()),
                                height: Some("0.875rem".into()),
                            }))
                            (render(Props {
                                variant: Variant::Text,
                                width: Some("3rem".into()),
                                height: Some("0.75rem".into()),
                            }))
                        }
                        // Body lines
                        (render(Props {
                            variant: Variant::Text,
                            width: Some("100%".into()),
                            height: None,
                        }))
                        (render(Props {
                            variant: Variant::Text,
                            width: Some("92%".into()),
                            height: None,
                        }))
                        (render(Props {
                            variant: Variant::Text,
                            width: Some("65%".into()),
                            height: None,
                        }))
                        // Action row
                        div style="display:flex;gap:1.5rem;margin-top:0.25rem;" {
                            (render(Props {
                                variant: Variant::Text,
                                width: Some("2rem".into()),
                                height: Some("0.75rem".into()),
                            }))
                            (render(Props {
                                variant: Variant::Text,
                                width: Some("2rem".into()),
                                height: Some("0.75rem".into()),
                            }))
                            (render(Props {
                                variant: Variant::Text,
                                width: Some("2rem".into()),
                                height: Some("0.75rem".into()),
                            }))
                        }
                    }
                }
            }

            // Loading table row
            div {
                p.mui-showcase__caption { "Loading table row" }
                div style="display:flex;flex-direction:column;gap:0;max-width:32rem;border:1px solid var(--mui-border);border-radius:var(--mui-radius-lg);overflow:hidden;" {
                    // Header row (real, so the skeleton has context)
                    div style="display:grid;grid-template-columns:2fr 1fr 1fr 1fr;gap:1rem;padding:0.625rem 0.875rem;background:var(--mui-bg-input);font-size:0.75rem;font-weight:600;color:var(--mui-text);text-transform:uppercase;letter-spacing:0.04em;" {
                        span { "Customer" }
                        span { "Plan" }
                        span { "Status" }
                        span { "MRR" }
                    }
                    // Skeleton rows
                    @for _ in 0..3 {
                        div style="display:grid;grid-template-columns:2fr 1fr 1fr 1fr;gap:1rem;padding:0.75rem 0.875rem;align-items:center;border-top:1px solid var(--mui-border);" {
                            div style="display:flex;align-items:center;gap:0.625rem;" {
                                (render(Props {
                                    variant: Variant::Circle,
                                    width: Some("1.75rem".into()),
                                    height: Some("1.75rem".into()),
                                }))
                                (render(Props {
                                    variant: Variant::Text,
                                    width: Some("8rem".into()),
                                    height: Some("0.875rem".into()),
                                }))
                            }
                            (render(Props {
                                variant: Variant::Text,
                                width: Some("4rem".into()),
                                height: Some("0.875rem".into()),
                            }))
                            (render(Props {
                                variant: Variant::Rect,
                                width: Some("4.5rem".into()),
                                height: Some("1.25rem".into()),
                            }))
                            (render(Props {
                                variant: Variant::Text,
                                width: Some("3rem".into()),
                                height: Some("0.875rem".into()),
                            }))
                        }
                    }
                }
            }

            // Loading product card
            div {
                p.mui-showcase__caption { "Loading product card" }
                div style="display:flex;flex-direction:column;gap:0.75rem;max-width:16rem;padding:0.75rem;border:1px solid var(--mui-border,#e5e7eb);border-radius:var(--mui-radius-lg);" {
                    // Product image
                    (render(Props {
                        variant: Variant::Rect,
                        width: Some("100%".into()),
                        height: Some("11rem".into()),
                    }))
                    // Title
                    (render(Props {
                        variant: Variant::Text,
                        width: Some("85%".into()),
                        height: Some("1rem".into()),
                    }))
                    // Subtitle / color
                    (render(Props {
                        variant: Variant::Text,
                        width: Some("55%".into()),
                        height: Some("0.75rem".into()),
                    }))
                    // Price row
                    div style="display:flex;justify-content:space-between;align-items:center;margin-top:0.25rem;" {
                        (render(Props {
                            variant: Variant::Text,
                            width: Some("4rem".into()),
                            height: Some("1.125rem".into()),
                        }))
                        (render(Props {
                            variant: Variant::Rect,
                            width: Some("2rem".into()),
                            height: Some("2rem".into()),
                        }))
                    }
                }
            }
        }
    }
}
