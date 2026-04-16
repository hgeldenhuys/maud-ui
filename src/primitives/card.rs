//! Card component — container with optional header, body, and footer.
use maud::{html, Markup};

/// Card rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// Optional card title displayed in header
    pub title: Option<String>,
    /// Optional card description displayed below title
    pub description: Option<String>,
    /// Main content markup
    pub children: Markup,
    /// Optional footer markup
    pub footer: Option<Markup>,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            title: None,
            description: None,
            children: html! {},
            footer: None,
        }
    }
}

/// Render a card with the given properties
pub fn render(props: Props) -> Markup {
    html! {
        div class="mui-card" {
            @if props.title.is_some() || props.description.is_some() {
                div class="mui-card__header" {
                    @if let Some(title) = props.title {
                        h3 class="mui-card__title" {
                            (title)
                        }
                    }
                    @if let Some(desc) = props.description {
                        p class="mui-card__description" {
                            (desc)
                        }
                    }
                }
            }
            div class="mui-card__body" {
                (props.children)
            }
            @if let Some(footer) = props.footer {
                div class="mui-card__footer" {
                    (footer)
                }
            }
        }
    }
}

/// Showcase all card use cases
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            div {
                p.mui-showcase__caption { "With header and body" }
                (render(Props {
                    title: Some("Card Title".into()),
                    description: Some("Card description with supporting text.".into()),
                    children: html! {
                        p style="font-size:0.875rem;color:var(--mui-text-muted);margin:0;" {
                            "Card content goes here. This area can contain any markup."
                        }
                    },
                    footer: None,
                }))
            }

            div {
                p.mui-showcase__caption { "With footer" }
                (render(Props {
                    title: Some("Notifications".into()),
                    description: Some("Manage your notification preferences.".into()),
                    children: html! {
                        div style="display:flex;flex-direction:column;gap:0.75rem;font-size:0.875rem;" {
                            div style="display:flex;justify-content:space-between;align-items:center;" {
                                span { "Push notifications" }
                                span.mui-text-muted { "Enabled" }
                            }
                            div style="display:flex;justify-content:space-between;align-items:center;" {
                                span { "Email digest" }
                                span.mui-text-muted { "Weekly" }
                            }
                        }
                    },
                    footer: Some(html! {
                        div style="display:flex;gap:0.5rem;margin-left:auto;" {
                            button class="mui-btn mui-btn--md mui-btn--outline" { "Cancel" }
                            button class="mui-btn mui-btn--md mui-btn--primary" { "Save" }
                        }
                    }),
                }))
            }

            div {
                p.mui-showcase__caption { "Body only" }
                (render(Props {
                    title: None,
                    description: None,
                    children: html! {
                        p style="font-size:0.875rem;margin:0;" {
                            "A simple card with just body content."
                        }
                    },
                    footer: None,
                }))
            }
        }
    }
}
