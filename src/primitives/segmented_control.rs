//! Segmented control — a boxed switch between a small set of mutually
//! exclusive views.
//!
//! Rendered as real `<a>` links so it works with JavaScript disabled: each
//! stop is a URL (`?view=chat`), and the selected stop is marked
//! `aria-current="page"`. The consumer wires client-side switching on top if
//! it wants to avoid the navigation.

use maud::{html, Markup};

/// One stop in the control.
#[derive(Debug, Clone, Default)]
pub struct Stop {
    /// Visible label.
    pub label: String,
    /// Destination URL for this stop.
    pub href: String,
}

impl Stop {
    /// Construct a stop from a label and href.
    pub fn new(label: impl Into<String>, href: impl Into<String>) -> Self {
        Self { label: label.into(), href: href.into() }
    }
}

/// Segmented-control rendering properties.
#[derive(Debug, Clone, Default)]
pub struct Props {
    /// The stops, left to right.
    pub stops: Vec<Stop>,
    /// Index of the active stop.
    pub selected: usize,
    /// Optional accessible label for the group.
    pub aria_label: Option<String>,
}

/// Render the segmented control.
pub fn render(props: Props) -> Markup {
    html! {
        div class="mui-segmented" role="tablist" aria-label=[props.aria_label.as_deref()] {
            @for (i, stop) in props.stops.iter().enumerate() {
                @let active = i == props.selected;
                @let class = if active { "mui-segmented__stop mui-segmented__stop--active" } else { "mui-segmented__stop" };
                a class=(class)
                  href=(stop.href)
                  role="tab"
                  aria-current=[active.then_some("page")]
                  aria-selected=(if active { "true" } else { "false" }) {
                    (stop.label)
                }
            }
        }
    }
}

/// Showcase the segmented control.
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            section {
                h2 { "Verbosity switch" }
                p.mui-showcase__caption { "Three tabs collapsed into one control — Clean · Chat · Raw — with the reading position kept across switches." }
                (render(Props {
                    stops: vec![
                        Stop::new("Clean", "?view=clean"),
                        Stop::new("Chat", "?view=chat"),
                        Stop::new("Raw", "?view=raw"),
                    ],
                    selected: 1,
                    aria_label: Some("Transcript verbosity".into()),
                }))
            }

            section {
                h2 { "Two stops" }
                p.mui-showcase__caption { "The minimum useful switch." }
                (render(Props {
                    stops: vec![Stop::new("Diff", "?mode=diff"), Stop::new("Blame", "?mode=blame")],
                    selected: 0,
                    ..Default::default()
                }))
            }
        }
    }
}
