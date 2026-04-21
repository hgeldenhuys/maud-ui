//! Message bubble — chat-style message from a participant (user / assistant / system).
//!
//! Conductor-flavored: compact density, colored avatar, optional live pulse during
//! streaming, footer slot for inline tool chips / actions.
use maud::{html, Markup};

/// Who the message is from — drives colour and alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Role {
    #[default]
    Assistant,
    User,
    System,
}

impl Role {
    fn class(&self) -> &'static str {
        match self {
            Self::Assistant => "mui-message--assistant",
            Self::User => "mui-message--user",
            Self::System => "mui-message--system",
        }
    }
}

/// Message bubble rendering properties.
#[derive(Debug, Clone, Default)]
pub struct Props {
    /// Who authored the message.
    pub role: Role,
    /// Display name (e.g. "Claude", "Sofia").
    pub author: String,
    /// Avatar initials (e.g. "C"). Falls back to first letter of `author`.
    pub avatar_initials: Option<String>,
    /// Optional override for avatar background colour (CSS colour string).
    pub avatar_color: Option<String>,
    /// Human-readable timestamp ("2 min ago", "14:32").
    pub timestamp: Option<String>,
    /// The message body — may contain markdown-rendered HTML, code blocks, etc.
    pub body: Markup,
    /// When true, the avatar pulses to indicate an in-progress streaming response.
    pub is_live: bool,
    /// Footer slot — tool chips, action buttons, attachments.
    pub footer: Option<Markup>,
}

fn initials_of(author: &str, override_initials: Option<&str>) -> String {
    if let Some(s) = override_initials {
        return s.to_string();
    }
    author
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "?".to_string())
}

/// Render a single chat message bubble.
pub fn render(props: Props) -> Markup {
    let initials = initials_of(&props.author, props.avatar_initials.as_deref());
    let avatar_style = props
        .avatar_color
        .as_ref()
        .map(|c| format!("background: {c};"));

    html! {
        div class={"mui-message " (props.role.class())} data-mui="message" {
            div class={
                "mui-message__avatar"
                @if props.is_live { " mui-message__avatar--live" }
            } style=[avatar_style.as_deref()] aria-hidden="true" {
                (initials)
            }
            div class="mui-message__body-wrap" {
                div class="mui-message__header" {
                    span class="mui-message__author" { (props.author) }
                    @if let Some(ts) = props.timestamp.as_ref() {
                        span class="mui-message__timestamp" { (ts) }
                    }
                }
                div class="mui-message__body" {
                    (props.body)
                }
                @if let Some(footer) = props.footer {
                    div class="mui-message__footer" {
                        (footer)
                    }
                }
            }
        }
    }
}

/// Showcase of the three role variants plus a live assistant.
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__column style="max-width: 42rem;" {
            (render(Props {
                role: Role::User,
                author: "You".into(),
                avatar_initials: Some("H".into()),
                timestamp: Some("14:32".into()),
                body: html! { p { "Can you add a dark-mode toggle to the landing page?" } },
                ..Default::default()
            }))
            (render(Props {
                role: Role::Assistant,
                author: "Claude".into(),
                avatar_initials: Some("C".into()),
                timestamp: Some("14:32".into()),
                body: html! {
                    p { "I'll add the toggle to the header and wire it to a " code { "prefers-color-scheme" } " listener. Starting now." }
                },
                is_live: true,
                ..Default::default()
            }))
            (render(Props {
                role: Role::System,
                author: "System".into(),
                timestamp: Some("14:31".into()),
                body: html! { p { "Context window at 34% — 128k tokens remaining." } },
                ..Default::default()
            }))
        }
    }
}
