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
                p.mui-showcase__caption { "Placements" }
                div.mui-showcase__row {
                    (render(Props {
                        content: "Tooltip on top".into(),
                        placement: Placement::Top,
                        delay_ms: 500,
                        trigger: html! { button.mui-btn.mui-btn--outline.mui-btn--md { "Hover me (top)" } },
                        id: "demo-tip-1".into(),
                    }))
                    (render(Props {
                        content: "Tooltip on bottom".into(),
                        placement: Placement::Bottom,
                        delay_ms: 500,
                        trigger: html! { button.mui-btn.mui-btn--outline.mui-btn--md { "Hover me (bottom)" } },
                        id: "demo-tip-2".into(),
                    }))
                    (render(Props {
                        content: "Tooltip on left".into(),
                        placement: Placement::Left,
                        delay_ms: 500,
                        trigger: html! { button.mui-btn.mui-btn--outline.mui-btn--md { "Left" } },
                        id: "demo-tip-3".into(),
                    }))
                    (render(Props {
                        content: "Tooltip on right".into(),
                        placement: Placement::Right,
                        delay_ms: 500,
                        trigger: html! { button.mui-btn.mui-btn--outline.mui-btn--md { "Right" } },
                        id: "demo-tip-4".into(),
                    }))
                }
            }
        }
    }
}
