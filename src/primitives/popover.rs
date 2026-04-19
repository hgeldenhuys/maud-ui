//! Popover component — non-modal floating panel anchored to a trigger.
//! Click trigger opens/closes. Click outside or ESC closes. Focus is not trapped.

use maud::{html, Markup};

/// Vertical placement of the popover relative to the trigger.
///
/// Deprecated in favor of [`Side`], which supports all four directions.
/// Retained for backward compatibility with existing callers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Placement {
    Top,
    Bottom,
}

impl Placement {
    #[allow(dead_code)] // retained for symmetry with Side::class_part and external inspection
    fn class_part(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
        }
    }

    /// Convert to the equivalent `Side` for the new 4-way side matrix.
    fn to_side(self) -> Side {
        match self {
            Self::Top => Side::Top,
            Self::Bottom => Side::Bottom,
        }
    }
}

impl Default for Placement {
    fn default() -> Self {
        Self::Bottom
    }
}

/// Side of the trigger the popover should render on (shadcn-compatible).
///
/// Mirrors Radix/shadcn `side` prop: top | right | bottom | left.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

impl Side {
    fn class_part(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Right => "right",
            Self::Bottom => "bottom",
            Self::Left => "left",
        }
    }
}

impl Default for Side {
    fn default() -> Self {
        Self::Bottom
    }
}

/// Horizontal alignment of the popover
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    Start,
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

impl Default for Align {
    fn default() -> Self {
        Self::Center
    }
}

/// Popover rendering properties
#[derive(Clone, Debug)]
pub struct Props {
    /// Unique identifier for the popover content
    pub id: String,
    /// The element that triggers the popover open/close (typically a button)
    pub trigger: Markup,
    /// Markup content displayed inside the popover
    pub content: Markup,
    /// Side of the trigger the popover renders on. When set, takes precedence
    /// over `placement` (shadcn-compatible 4-way side selector).
    pub side: Option<Side>,
    /// Vertical placement relative to trigger (legacy 2-way API, default: Bottom).
    /// Kept for backward compatibility; prefer `side` for new code.
    pub placement: Placement,
    /// Horizontal alignment (default: Center)
    pub align: Align,
    /// Offset in pixels between the trigger and the popover along the side axis.
    /// Emitted as `data-side-offset` for JS-driven positioning engines.
    pub side_offset: Option<u32>,
    /// Controlled open state. When `Some(true)`, the popover is rendered visible
    /// (no `hidden` attribute, `data-state="open"`). When `Some(false)`, rendered
    /// hidden. When `None`, left for client JS to toggle (legacy default).
    pub open: Option<bool>,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            id: "popover".to_string(),
            trigger: html! {},
            content: html! {},
            side: None,
            placement: Placement::default(),
            align: Align::default(),
            side_offset: None,
            open: None,
        }
    }
}

/// Render a popover with the given properties
pub fn render(props: Props) -> Markup {
    // New `side` takes precedence over legacy `placement`.
    let side = props.side.unwrap_or_else(|| props.placement.to_side());
    let content_class = format!(
        "mui-popover__content mui-popover__content--{}-{}",
        side.class_part(),
        props.align.class_part()
    );
    let content_id = format!("{}-content", props.id);

    let is_open = props.open.unwrap_or(false);
    let data_state = if is_open { "open" } else { "closed" };
    let data_visible = if is_open { "true" } else { "false" };
    // Only emit `hidden` when NOT explicitly open. `open: Some(true)` must render visible.
    let render_hidden = !is_open;

    html! {
        span.mui-popover data-mui="popover" {
            span.mui-popover__trigger {
                (props.trigger)
            }
            div class=(content_class)
                id=(content_id)
                role="dialog"
                data-state=(data_state)
                data-visible=(data_visible)
                data-side=(side.class_part())
                data-align=(props.align.class_part())
                data-side-offset=[props.side_offset.map(|v| v.to_string())]
                hidden[render_hidden]
                tabindex="-1"
            {
                (props.content)
            }
        }
    }
}

/// Render a popover header block (shadcn: `PopoverHeader`).
/// Groups title + description; adds spacing and bottom margin.
pub fn header(children: Markup) -> Markup {
    html! {
        div.mui-popover__header {
            (children)
        }
    }
}

/// Render a popover title (shadcn: `PopoverTitle`). Semantic heading (h3)
/// styled as the panel's primary label.
pub fn title(children: Markup) -> Markup {
    html! {
        h3.mui-popover__title {
            (children)
        }
    }
}

/// Render a popover description (shadcn: `PopoverDescription`). Muted
/// supporting copy under the title.
pub fn description(children: Markup) -> Markup {
    html! {
        p.mui-popover__description {
            (children)
        }
    }
}

/// Showcase all popover placements and alignments
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            div {
                p.mui-showcase__caption { "Popover with Form (bottom-start)" }
                {
                    (render(Props {
                        id: "demo-pop-1".to_string(),
                        trigger: html! { button.mui-btn.mui-btn--primary.mui-btn--md { "Open popover" } },
                        content: html! {
                            div {
                                (header(html! {
                                    (title(html! { "Dimensions" }))
                                    (description(html! { "Set width and height for the element." }))
                                }))
                                div.mui-field {
                                    label.mui-label { "Width" }
                                    input.mui-input type="text" placeholder="e.g., 100px" {}
                                }
                                div.mui-field {
                                    label.mui-label { "Height" }
                                    input.mui-input type="text" placeholder="e.g., 100px" {}
                                }
                                button.mui-btn.mui-btn--primary style="margin-top: 1rem;" {
                                    "Apply"
                                }
                            }
                        },
                        side: Some(Side::Bottom),
                        align: Align::Start,
                        side_offset: Some(8),
                        ..Props::default()
                    }))
                }
            }
            div {
                p.mui-showcase__caption { "Right-Center with header/title/description" }
                {
                    (render(Props {
                        id: "demo-pop-2".to_string(),
                        trigger: html! { button.mui-btn.mui-btn--md { "Show info" } },
                        content: html! {
                            (header(html! {
                                (title(html! { "Shortcut" }))
                                (description(html! { "Press ⌘K to open the command palette from anywhere." }))
                            }))
                        },
                        side: Some(Side::Right),
                        align: Align::Center,
                        side_offset: Some(12),
                        ..Props::default()
                    }))
                }
            }
            div {
                p.mui-showcase__caption { "Left-End, controlled open state" }
                {
                    (render(Props {
                        id: "demo-pop-3".to_string(),
                        trigger: html! { button.mui-btn.mui-btn--md { "Always open" } },
                        content: html! {
                            (header(html! {
                                (title(html! { "Controlled" }))
                                (description(html! { "This popover is rendered open via the `open` prop." }))
                            }))
                        },
                        side: Some(Side::Left),
                        align: Align::End,
                        open: Some(true),
                        ..Props::default()
                    }))
                }
            }
        }
    }
}
