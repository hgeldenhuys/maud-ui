//! Message bubble — chat-style message from a participant (user / assistant / system).
//!
//! Conductor-flavored: compact density, colored avatar, optional live pulse during
//! streaming, footer slot for inline tool chips / actions.
//!
//! Two layouts. [`Layout::Avatar`] (the original) is a transcript row: avatar,
//! author, timestamp, body. [`Layout::Chat`] is what a person reads a
//! conversation in: no avatars, one reading column ([`thread`]), the person's
//! prompt as a right-side tinted card at most three quarters of the column,
//! the assistant's reply flat across the full column, a timestamp that shows
//! on hover, and a quiet action row ([`Props::actions`]) under a reply.
//! That is the whole difference between reading a chat and reading a log,
//! and it costs only markup and CSS.
use maud::{html, Markup};

/// How a message is laid out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Layout {
    /// Transcript row: avatar, author, timestamp, body.
    #[default]
    Avatar,
    /// Conversation: prompt right as a tinted card, reply flat and full
    /// width, no avatar, timestamp on hover, action row under a reply.
    Chat,
}

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
    /// Which layout to render. See [`Layout`].
    pub layout: Layout,
    /// The quiet action row under a reply (copy, fork from here, the model
    /// that answered, the cost). Revealed on hover on a pointer device,
    /// always visible where there is no hover. Chat layout only; the avatar
    /// layout renders it as part of the footer.
    pub actions: Option<Markup>,
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

/// One reading column for a conversation: centred, capped at a comfortable
/// measure, messages stacked with air between them. The pane is not the
/// column — a chat read across a 1400px pane is a log with better fonts.
pub fn thread(body: Markup) -> Markup {
    html! { div class="mui-thread" data-mui="thread" { (body) } }
}

/// The "jump to latest" pill a conversation shows once the reader has
/// scrolled up. Rendered hidden; the consumer's follow logic shows it and
/// handles the click. `label` is the visible text ("↓ 3 new", "Jump to
/// latest").
pub fn jump_to_latest(id: &str, label: &str) -> Markup {
    html! {
        button type="button" id=(id) class="mui-jump-pill" hidden aria-live="polite" {
            (label)
        }
    }
}

/// Render a single chat message bubble.
pub fn render(props: Props) -> Markup {
    if props.layout == Layout::Chat {
        return render_chat(props);
    }
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

/// The chat layout. No avatar and no author line: in a two-party
/// conversation the side and the shape say who spoke. The user's card is
/// right-aligned and tinted; the assistant's body is flat. A `System`
/// message keeps its muted italic treatment, centred.
fn render_chat(props: Props) -> Markup {
    html! {
        article class={"mui-message mui-message--chat " (props.role.class())} data-mui="message"
            aria-label=[(!props.author.is_empty()).then_some(props.author.as_str())] {
            div class="mui-message__body-wrap" {
                div class={
                    "mui-message__body"
                    @if props.is_live { " mui-message__body--live" }
                } {
                    (props.body)
                }
                @if let Some(ts) = props.timestamp.as_ref() {
                    span class="mui-message__timestamp" { (ts) }
                }
                @if let Some(footer) = props.footer {
                    div class="mui-message__footer" { (footer) }
                }
                @if let Some(actions) = props.actions {
                    div class="mui-message__actions" { (actions) }
                }
            }
        }
    }
}

/// Showcase of the three role variants plus a live assistant, then the
/// chat layout in a reading column.
pub fn showcase() -> Markup {
    let chat = thread(html! {
        (render(Props {
            layout: Layout::Chat,
            role: Role::User,
            author: "You".into(),
            timestamp: Some("14:32".into()),
            body: html! { "Can you add a dark-mode toggle to the landing page?" },
            ..Default::default()
        }))
        (render(Props {
            layout: Layout::Chat,
            role: Role::Assistant,
            author: "Claude".into(),
            timestamp: Some("14:32".into()),
            body: html! {
                p { "I'll add the toggle to the header and wire it to a " code { "prefers-color-scheme" } " listener." }
                p { "Two files change: the header partial and the theme script." }
            },
            actions: Some(html! {
                button type="button" class="mui-message__action" { "copy" }
                button type="button" class="mui-message__action" { "fork from here" }
                span class="mui-message__meta" { "claude-opus-5 \u{00B7} $0.04" }
            }),
            ..Default::default()
        }))
        (render(Props {
            layout: Layout::Chat,
            role: Role::Assistant,
            author: "Claude".into(),
            body: html! { p { "Running the tests now" } },
            is_live: true,
            ..Default::default()
        }))
    });
    html! {
        div.mui-showcase__column style="max-width: 42rem;" {
            p class="mui-showcase__caption" { "Chat layout — one reading column, prompt right, reply flat" }
            (chat)
            (jump_to_latest("showcase-jump", "\u{2193} 3 new"))
            p class="mui-showcase__caption" { "Avatar layout — a transcript row" }
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
