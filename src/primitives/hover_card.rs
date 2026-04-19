//! HoverCard component — richer hover overlay with structured content

use maud::{html, Markup};

/// Side of the trigger to place the card on
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Placement {
    Top,
    Right,
    #[default]
    Bottom,
    Left,
}

impl Placement {
    fn class_part(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Right => "right",
            Self::Bottom => "bottom",
            Self::Left => "left",
        }
    }
}

/// Alignment of the card along the trigger's edge
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Align {
    Start,
    #[default]
    Center,
    End,
}

impl Align {
    fn class_part(&self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
        }
    }
}

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
    /// Which side of the trigger to anchor the card to (default: Bottom)
    pub side: Placement,
    /// Alignment along the trigger edge (default: Center)
    pub align: Align,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            trigger: html! {},
            content: html! {},
            id: "hover-card".to_string(),
            open_delay_ms: 300,
            close_delay_ms: 200,
            side: Placement::default(),
            align: Align::default(),
        }
    }
}

/// Render a single hover card with the given properties
pub fn render(props: Props) -> Markup {
    let card_id = format!("{}-card", props.id);
    let content_class = format!(
        "mui-hover-card__content mui-hover-card__content--{} mui-hover-card__content--align-{}",
        props.side.class_part(),
        props.align.class_part()
    );

    html! {
        span.mui-hover-card
            data-mui="hover-card"
            data-open-delay=(props.open_delay_ms.to_string())
            data-close-delay=(props.close_delay_ms.to_string())
        {
            span.mui-hover-card__trigger {
                (props.trigger)
            }
            div class=(content_class)
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
    let user_card = || {
        html! {
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
        }
    };

    html! {
        div.mui-showcase__grid {
            div {
                p.mui-showcase__caption { "HoverCard — Bottom / Center (default)" }
                div.mui-showcase__row {
                    (render(Props {
                        trigger: html! { a href="#" style="color:var(--mui-accent-text);" { "@shadcn" } },
                        content: user_card(),
                        id: "demo-hover-1".into(),
                        open_delay_ms: 300,
                        close_delay_ms: 200,
                        ..Default::default()
                    }))
                }
            }
            div {
                p.mui-showcase__caption { "HoverCard — Top / End" }
                div.mui-showcase__row style="margin-top: 6rem;" {
                    (render(Props {
                        trigger: html! { a href="#" style="color:var(--mui-accent-text);" { "@shadcn" } },
                        content: user_card(),
                        id: "demo-hover-2".into(),
                        open_delay_ms: 300,
                        close_delay_ms: 200,
                        side: Placement::Top,
                        align: Align::End,
                    }))
                }
            }
            div {
                p.mui-showcase__caption { "HoverCard — Right / Start" }
                div.mui-showcase__row {
                    (render(Props {
                        trigger: html! { a href="#" style="color:var(--mui-accent-text);" { "@shadcn" } },
                        content: user_card(),
                        id: "demo-hover-3".into(),
                        open_delay_ms: 300,
                        close_delay_ms: 200,
                        side: Placement::Right,
                        align: Align::Start,
                    }))
                }
            }
        }
    }
}
