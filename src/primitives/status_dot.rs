//! Status dot — a 6px semantic dot for run state, presence, and health.
//!
//! Filled by default; an outline (hollow) variant reads as "observing / not
//! yet settled" without spending a second hue.

use maud::{html, Markup};

/// Semantic tone of the dot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tone {
    /// Green — healthy, passing, live.
    Success,
    /// Brand accent — active, selected, running.
    #[default]
    Accent,
    /// Amber — attention, degraded.
    Warning,
    /// Muted grey — idle, unknown, off.
    Neutral,
    /// Red — failed, blocked.
    Destructive,
}

impl Tone {
    fn class(self) -> &'static str {
        match self {
            Self::Success => "mui-status-dot--success",
            Self::Accent => "mui-status-dot--accent",
            Self::Warning => "mui-status-dot--warning",
            Self::Neutral => "mui-status-dot--neutral",
            Self::Destructive => "mui-status-dot--destructive",
        }
    }
}

/// Status-dot rendering properties.
#[derive(Debug, Clone, Default)]
pub struct Props {
    /// Semantic tone.
    pub tone: Tone,
    /// Render hollow (outline) instead of filled — the "observing" state.
    pub outline: bool,
    /// Optional accessible label; when set, the dot is exposed with
    /// `role="img"` so it is not silently invisible to a screen reader.
    pub label: Option<String>,
}

/// Render a single status dot.
pub fn render(props: Props) -> Markup {
    let mut class = format!("mui-status-dot {}", props.tone.class());
    if props.outline {
        class.push_str(" mui-status-dot--outline");
    }
    html! {
        @if let Some(label) = props.label.as_ref() {
            span class=(class) role="img" aria-label=(label) {}
        } @else {
            span class=(class) aria-hidden="true" {}
        }
    }
}

fn row(label: &str, props: Props) -> Markup {
    html! {
        div style="display:flex;align-items:center;gap:0.5rem;" {
            (render(props))
            span style="font-size:0.8125rem;color:var(--mui-text-muted);" { (label) }
        }
    }
}

/// Showcase every tone plus the outline state.
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            section {
                h2 { "Tones" }
                div style="display:flex;flex-direction:column;gap:0.625rem;" {
                    (row("Success — passing, live", Props { tone: Tone::Success, ..Default::default() }))
                    (row("Accent — running, selected", Props { tone: Tone::Accent, ..Default::default() }))
                    (row("Warning — degraded", Props { tone: Tone::Warning, ..Default::default() }))
                    (row("Neutral — idle, unknown", Props { tone: Tone::Neutral, ..Default::default() }))
                    (row("Destructive — failed, blocked", Props { tone: Tone::Destructive, ..Default::default() }))
                }
            }
            section {
                h2 { "Outline — observing" }
                p.mui-showcase__caption { "Hollow reads as \u{201c}watching, not yet settled\u{201d} without spending a second hue." }
                div style="display:flex;flex-direction:column;gap:0.625rem;" {
                    (row("Accent, observing", Props { tone: Tone::Accent, outline: true, ..Default::default() }))
                    (row("Success, observing", Props { tone: Tone::Success, outline: true, ..Default::default() }))
                }
            }
        }
    }
}
