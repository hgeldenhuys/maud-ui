//! Popover component — non-modal floating panel anchored to a trigger.
//! Click trigger opens/closes. Click outside or ESC closes. Focus is not trapped.

use maud::{html, Markup};

/// Vertical placement of the popover relative to the trigger
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Placement {
    Top,
    Bottom,
}

impl Placement {
    fn class_part(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
        }
    }
}

impl Default for Placement {
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
    /// Vertical placement relative to trigger (default: Bottom)
    pub placement: Placement,
    /// Horizontal alignment (default: Center)
    pub align: Align,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            id: "popover".to_string(),
            trigger: html! {},
            content: html! {},
            placement: Placement::default(),
            align: Align::default(),
        }
    }
}

/// Render a popover with the given properties
pub fn render(props: Props) -> Markup {
    let placement_align_class = format!(
        "mui-popover__content--{}-{}",
        props.placement.class_part(),
        props.align.class_part()
    );
    let content_id = format!("{}-content", props.id);

    html! {
        span.mui-popover data-mui="popover" {
            span.mui-popover__trigger
                aria-expanded="false"
                aria-haspopup="dialog"
                aria-controls=(content_id.clone())
            {
                (props.trigger)
            }
            div.mui-popover__content class=(placement_align_class)
                id=(content_id)
                role="dialog"
                hidden
                data-visible="false"
                tabindex="-1"
            {
                (props.content)
            }
        }
    }
}

/// Showcase all popover placements and alignments
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            div {
                p.mui-showcase__caption { "Popover with Form" }
                {
                    (render(Props {
                        id: "demo-pop-1".to_string(),
                        trigger: html! { button.mui-btn.mui-btn--primary.mui-btn--md { "Open popover" } },
                        content: html! {
                            div {
                                h3.mui-showcase__subheading { "Dimensions" }
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
                        placement: Placement::Bottom,
                        align: Align::Start,
                    }))
                }
            }
        }
    }
}
