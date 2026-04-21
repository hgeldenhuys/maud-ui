//! Streaming indicators — tiny animated elements that signal "work in progress".
//!
//! Three variants: a blinking block cursor (appended to streaming text), a
//! three-dot pulse ("Claude is thinking…"), and a ring pulse (status beacon).

use maud::{html, Markup};

/// Which animation to render.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Variant {
    #[default]
    Cursor,
    Dots,
    Pulse,
}

impl Variant {
    fn class(&self) -> &'static str {
        match self {
            Self::Cursor => "mui-streaming--cursor",
            Self::Dots => "mui-streaming--dots",
            Self::Pulse => "mui-streaming--pulse",
        }
    }
}

/// Streaming-indicator rendering properties.
#[derive(Debug, Clone, Default)]
pub struct Props {
    pub variant: Variant,
    /// Optional label rendered next to the animation (e.g. "thinking", "writing…").
    pub label: Option<String>,
}

/// Render a streaming indicator.
pub fn render(props: Props) -> Markup {
    html! {
        span class={"mui-streaming " (props.variant.class())} data-mui="streaming" role="status" aria-live="polite" {
            @match props.variant {
                Variant::Cursor => span class="mui-streaming__cursor" aria-hidden="true" {},
                Variant::Dots => span class="mui-streaming__dots" aria-hidden="true" {
                    span {} span {} span {}
                },
                Variant::Pulse => span class="mui-streaming__pulse" aria-hidden="true" {},
            }
            @if let Some(label) = props.label.as_ref() {
                span class="mui-streaming__label" { (label) }
            }
        }
    }
}

/// Showcase of all three variants.
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__column style="gap: 1rem;" {
            div {
                p class="mui-showcase__caption" { "Streaming cursor (inline text)" }
                p { "The answer is 42" (render(Props { variant: Variant::Cursor, ..Default::default() })) }
            }
            div {
                p class="mui-showcase__caption" { "Thinking dots" }
                (render(Props { variant: Variant::Dots, label: Some("thinking".into()), ..Default::default() }))
            }
            div {
                p class="mui-showcase__caption" { "Pulse beacon" }
                (render(Props { variant: Variant::Pulse, label: Some("connected".into()), ..Default::default() }))
            }
        }
    }
}
