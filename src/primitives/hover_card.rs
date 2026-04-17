//! HoverCard component — richer hover overlay with structured content

use maud::{html, Markup};

/// HoverCard rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// The trigger element (link, button, text, etc.)
    pub trigger: Markup,
    /// The rich content displayed in the card
    pub content: Markup,
    /// Unique identifier for the card
    pub id: String,
    /// Delay in milliseconds before showing (default 300)
    pub open_delay_ms: u32,
    /// Delay in milliseconds before closing (default 200)
    pub close_delay_ms: u32,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            trigger: html! {},
            content: html! {},
            id: "hover-card".to_string(),
            open_delay_ms: 300,
            close_delay_ms: 200,
        }
    }
}

/// Render a single hover card with the given properties
pub fn render(props: Props) -> Markup {
    let card_id = format!("{}-card", props.id);

    html! {
        span.mui-hover-card
            data-mui="hover-card"
            data-open-delay=(props.open_delay_ms.to_string())
            data-close-delay=(props.close_delay_ms.to_string())
        {
            span.mui-hover-card__trigger {
                (props.trigger)
            }
            div.mui-hover-card__content
                id=(card_id)
                hidden
                data-visible="false"
            {
                (props.content)
            }
        }
    }
}

/// Showcase with a user profile card
pub fn showcase() -> Markup {
    let user_card = html! {
        div.mui-hover-card__user {
            div.mui-hover-card__user-header {
                span.mui-avatar.mui-avatar--md {
                    span.mui-avatar__fallback { "@" }
                }
                div.mui-hover-card__user-info {
                    p.mui-hover-card__user-name { "shadcn" }
                    p.mui-hover-card__user-handle { "@shadcn" }
                }
            }
            p.mui-hover-card__user-bio {
                "Crafting beautiful, accessible interfaces. Passionate about design systems."
            }
            button.mui-btn.mui-btn--primary.mui-btn--sm { "Follow" }
        }
    };

    html! {
        div.mui-showcase__grid {
            div {
                p.mui-showcase__caption { "HoverCard" }
                div.mui-showcase__row {
                    (render(Props {
                        trigger: html! { a href="#" style="color:var(--mui-accent-text);" { "@shadcn" } },
                        content: user_card,
                        id: "demo-hover-1".into(),
                        open_delay_ms: 300,
                        close_delay_ms: 200,
                    }))
                }
            }
        }
    }
}
