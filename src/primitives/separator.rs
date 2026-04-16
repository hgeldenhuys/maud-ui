//! Separator component — visual and semantic divider for organizing content.

use maud::{html, Markup};

/// Separator orientation — horizontal (default) or vertical
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Orientation {
    fn class(&self) -> &'static str {
        match self {
            Self::Horizontal => "mui-separator--horizontal",
            Self::Vertical => "mui-separator--vertical",
        }
    }

    fn aria_orientation(&self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// Separator rendering properties
#[derive(Debug, Clone, Copy)]
pub struct Props {
    /// Orientation of the separator (default: Horizontal)
    pub orientation: Orientation,
    /// If true, render as purely decorative (aria-hidden). If false, semantic with role="separator"
    pub decorative: bool,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            orientation: Orientation::Horizontal,
            decorative: false,
        }
    }
}

/// Render a single separator with the given properties
pub fn render(props: Props) -> Markup {
    html! {
        @if props.decorative {
            div.mui-separator class=(props.orientation.class()) aria-hidden="true" {}
        } @else {
            div.mui-separator
                class=(props.orientation.class())
                role="separator"
                aria-orientation=(props.orientation.aria_orientation()) {}
        }
    }
}

/// Showcase all separator variants and use cases
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            // Horizontal separators
            div {
                p.mui-showcase__caption { "Horizontal separator" }
                (render(Props {
                    orientation: Orientation::Horizontal,
                    ..Default::default()
                }))
            }

            div {
                p.mui-showcase__caption { "Horizontal separator" }
                (render(Props {
                    orientation: Orientation::Horizontal,
                    ..Default::default()
                }))
            }

            // Vertical separators in a row
            div {
                p.mui-showcase__caption { "Vertical separators" }
                div.mui-showcase__row style="height: 1.5rem;" {
                    span.mui-showcase__label { "Home" }
                    (render(Props {
                        orientation: Orientation::Vertical,
                        ..Default::default()
                    }))
                    span.mui-showcase__label { "Docs" }
                    (render(Props {
                        orientation: Orientation::Vertical,
                        ..Default::default()
                    }))
                    span.mui-showcase__label { "About" }
                }
            }

            // Decorative variant
            div {
                p.mui-showcase__caption { "Decorative (aria-hidden)" }
                (render(Props {
                    orientation: Orientation::Horizontal,
                    decorative: true,
                }))
            }
        }
    }
}
