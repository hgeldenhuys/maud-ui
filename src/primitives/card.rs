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
            // Card with title, description, and body
            div {
                p.mui-showcase__caption { "With header and body" }
                (render(Props {
                    title: Some("Card Title".into()),
                    description: Some("This is a card with a title, description, and body content.".into()),
                    children: html! {
                        p {
                            "This card contains some example content that demonstrates the basic card layout and structure."
                        }
                    },
                    footer: None,
                }))
            }

            // Card with footer and buttons
            div {
                p.mui-showcase__caption { "With footer" }
                (render(Props {
                    title: Some("Settings".into()),
                    description: None,
                    children: html! {
                        div style="display: flex; flex-direction: column; gap: 0.75rem;" {
                            div class="mui-field" {
                                label class="mui-label" { "Notifications" }
                                input class="mui-input" type="checkbox" {}
                            }
                            div class="mui-field" {
                                label class="mui-label" { "Dark mode" }
                                input class="mui-input" type="checkbox" checked {}
                            }
                        }
                    },
                    footer: Some(html! {
                        div style="display: flex; gap: 0.75rem; justify-content: flex-end;" {
                            button class="mui-btn mui-btn--secondary" { "Cancel" }
                            button class="mui-btn mui-btn--primary" { "Save" }
                        }
                    }),
                }))
            }

            // Card with body only (no header)
            div {
                p.mui-showcase__caption { "Body only" }
                (render(Props {
                    title: None,
                    description: None,
                    children: html! {
                        p {
                            "A simple card with just body content. Great for minimal layouts and focused information display."
                        }
                    },
                    footer: None,
                }))
            }
        }
    }
}
