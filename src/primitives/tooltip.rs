//! Tooltip component — non-modal hover/focus overlay content

use maud::{html, Markup};

/// Tooltip placement relative to the trigger
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Placement {
    Top,
    Bottom,
    Left,
    Right,
}

impl Placement {
    fn class(&self) -> &'static str {
        match self {
            Self::Top => "mui-tooltip__content--top",
            Self::Bottom => "mui-tooltip__content--bottom",
            Self::Left => "mui-tooltip__content--left",
            Self::Right => "mui-tooltip__content--right",
        }
    }
}

/// Tooltip rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// The tooltip text displayed on hover/focus
    pub content: String,
    /// Position relative to trigger
    pub placement: Placement,
    /// Delay in milliseconds before showing (default 500)
    pub delay_ms: u32,
    /// The element that triggers the tooltip (button, link, icon, etc.)
    pub trigger: Markup,
    /// Unique identifier for aria-describedby linking
    pub id: String,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            content: String::new(),
            placement: Placement::Top,
            delay_ms: 500,
            trigger: html! {},
            id: "tooltip".to_string(),
        }
    }
}

/// Render a single tooltip with the given properties
pub fn render(props: Props) -> Markup {
    let content_class = format!("mui-tooltip__content {}", props.placement.class());
    let tip_id = format!("{}-tip", props.id);

    html! {
        span.mui-tooltip data-mui="tooltip" data-delay=(props.delay_ms.to_string()) {
            span.mui-tooltip__trigger aria-describedby=(tip_id.clone()) {
                (props.trigger)
            }
            span class=(content_class)
                id=(tip_id)
                role="tooltip"
                hidden
                data-visible="false"
            {
                (props.content)
            }
        }
    }
}

/// Showcase all tooltip placements
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            div {
                p.mui-showcase__caption { "Icon buttons with contextual tooltips — copy, roster, shortcut, destructive." }
                div.mui-showcase__row {
                    // Top — copy-to-clipboard on a copy icon button
                    (render(Props {
                        content: "Copy to clipboard".into(),
                        placement: Placement::Top,
                        delay_ms: 400,
                        trigger: html! {
                            button.mui-btn.mui-btn--ghost.mui-btn--icon type="button" aria-label="Copy" {
                                span aria-hidden="true" { "\u{2398}" }
                            }
                        },
                        id: "demo-tip-copy".into(),
                    }))
                    // Bottom — avatar stack "+3 more"
                    (render(Props {
                        content: "View full list (12 members)".into(),
                        placement: Placement::Bottom,
                        delay_ms: 400,
                        trigger: html! {
                            button.mui-btn.mui-btn--outline.mui-btn--sm type="button" aria-label="View all members" {
                                "+3"
                            }
                        },
                        id: "demo-tip-avatars".into(),
                    }))
                    // Left — search with keyboard shortcut hint
                    (render(Props {
                        content: "Keyboard shortcut: \u{2318}K".into(),
                        placement: Placement::Left,
                        delay_ms: 400,
                        trigger: html! {
                            button.mui-btn.mui-btn--outline.mui-btn--md type="button" aria-label="Open search" {
                                span aria-hidden="true" { "\u{1f50d}" }
                                " Search"
                            }
                        },
                        id: "demo-tip-search".into(),
                    }))
                    // Right — destructive icon with warning
                    (render(Props {
                        content: "Delete permanently".into(),
                        placement: Placement::Right,
                        delay_ms: 400,
                        trigger: html! {
                            button.mui-btn.mui-btn--danger.mui-btn--icon type="button" aria-label="Delete" {
                                span aria-hidden="true" { "\u{1f5d1}" }
                            }
                        },
                        id: "demo-tip-delete".into(),
                    }))
                }
            }
        }
    }
}
