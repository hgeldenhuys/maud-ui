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

/// Alignment of the tooltip along the perpendicular axis of its placement.
/// For `Top` / `Bottom` this controls horizontal alignment; for `Left` / `Right`
/// it controls vertical alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Align {
    Start,
    #[default]
    Center,
    End,
}

impl Align {
    fn class(&self) -> &'static str {
        match self {
            Self::Start => "mui-tooltip__content--align-start",
            Self::Center => "mui-tooltip__content--align-center",
            Self::End => "mui-tooltip__content--align-end",
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
    /// Alignment along the perpendicular axis of placement (default `Center`)
    pub align: Align,
    /// Optional offset in pixels between the trigger and tooltip. Emitted as
    /// `data-side-offset` for client-side positioning code to consume. When
    /// `None` (the default), the CSS-level `0.375rem` gap is used.
    pub side_offset: Option<u32>,
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
            align: Align::Center,
            side_offset: None,
            delay_ms: 500,
            trigger: html! {},
            id: "tooltip".to_string(),
        }
    }
}

/// Render a single tooltip with the given properties
pub fn render(props: Props) -> Markup {
    let content_class = format!(
        "mui-tooltip__content {} {}",
        props.placement.class(),
        props.align.class()
    );
    let tip_id = format!("{}-tip", props.id);
    let side_offset_attr = props.side_offset.map(|v| v.to_string());

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
                data-side-offset=[side_offset_attr]
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
                        ..Default::default()
                    }))
                    // Bottom — avatar stack "+3 more"
                    (render(Props {
                        content: "View full list (12 members)".into(),
                        placement: Placement::Bottom,
                        delay_ms: 400,
                        trigger: html! {
                            button.mui-btn.mui-btn--outline.mui-btn--sm type="button" aria-label="+3 more members — view all" {
                                "+3"
                            }
                        },
                        id: "demo-tip-avatars".into(),
                        ..Default::default()
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
                        ..Default::default()
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
                        ..Default::default()
                    }))
                }
            }

            // Alignment variants on a Bottom-placed tooltip
            div {
                p.mui-showcase__caption { "Align start / center / end (bottom placement)" }
                div.mui-showcase__row {
                    (render(Props {
                        content: "Aligned to start".into(),
                        placement: Placement::Bottom,
                        align: Align::Start,
                        delay_ms: 300,
                        trigger: html! {
                            button.mui-btn.mui-btn--outline.mui-btn--md type="button" { "Start" }
                        },
                        id: "demo-tip-align-start".into(),
                        ..Default::default()
                    }))
                    (render(Props {
                        content: "Aligned to center".into(),
                        placement: Placement::Bottom,
                        align: Align::Center,
                        delay_ms: 300,
                        trigger: html! {
                            button.mui-btn.mui-btn--outline.mui-btn--md type="button" { "Center" }
                        },
                        id: "demo-tip-align-center".into(),
                        ..Default::default()
                    }))
                    (render(Props {
                        content: "Aligned to end".into(),
                        placement: Placement::Bottom,
                        align: Align::End,
                        delay_ms: 300,
                        trigger: html! {
                            button.mui-btn.mui-btn--outline.mui-btn--md type="button" { "End" }
                        },
                        id: "demo-tip-align-end".into(),
                        ..Default::default()
                    }))
                }
            }

            // Side offset variants
            div {
                p.mui-showcase__caption { "Custom side_offset (emits data-side-offset for client positioning code)" }
                div.mui-showcase__row {
                    (render(Props {
                        content: "Offset: 4px".into(),
                        placement: Placement::Top,
                        side_offset: Some(4),
                        delay_ms: 300,
                        trigger: html! {
                            button.mui-btn.mui-btn--outline.mui-btn--md type="button" { "4px" }
                        },
                        id: "demo-tip-offset-4".into(),
                        ..Default::default()
                    }))
                    (render(Props {
                        content: "Offset: 12px".into(),
                        placement: Placement::Top,
                        side_offset: Some(12),
                        delay_ms: 300,
                        trigger: html! {
                            button.mui-btn.mui-btn--outline.mui-btn--md type="button" { "12px" }
                        },
                        id: "demo-tip-offset-12".into(),
                        ..Default::default()
                    }))
                }
            }
        }
    }
}
