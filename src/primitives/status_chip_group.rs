//! Counted, single-current filters. Links work before JavaScript; arrows move focus.
use maud::{html, Markup};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Tone {
    #[default]
    Neutral,
    Info,
    Success,
    Warning,
    Danger,
}

impl Tone {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Chip {
    pub label: String,
    pub href: String,
    pub count: u64,
    pub tone: Tone,
}

#[derive(Clone, Debug)]
pub struct Props {
    pub items: Vec<Chip>,
    /// Index of the current filter; out-of-range values select the first item.
    pub current: usize,
    pub aria_label: String,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            items: vec![],
            current: 0,
            aria_label: "Filter by status".into(),
        }
    }
}

pub fn render(props: Props) -> Markup {
    let current = if props.current < props.items.len() {
        props.current
    } else {
        0
    };
    html! {
        nav class="mui-status-chip-group" aria-label=(props.aria_label) data-mui="status-chip-group" {
            @for (index, item) in props.items.iter().enumerate() {
                a class="mui-status-chip-group__chip" href=(item.href) data-tone=(item.tone.as_str())
                    aria-current=[(index == current).then_some("page")] {
                    span class="mui-status-chip-group__mark" aria-hidden="true" { "✓" }
                    span class="mui-status-chip-group__label" { (item.label) }
                    span class="mui-status-chip-group__count" {
                        span class="mui-sr-only" { ": " }
                        (item.count)
                    }
                }
            }
        }
    }
}

pub fn showcase() -> Markup {
    html! {
        p class="mui-showcase__caption" { "One current filter. Tab between links; use arrows, Home or End to move focus, and Enter to follow a filter." }
        (render(Props {
            items: [
                ("All reservations", 48, Tone::Neutral), ("Arriving", 12, Tone::Info),
                ("Checked in", 24, Tone::Success), ("Needs review", 9, Tone::Warning),
                ("Failed", 3, Tone::Danger),
            ].into_iter().enumerate().map(|(i, (label, count, tone))| Chip {
                label: label.into(), href: format!("/status_chip_group?status={i}"), count, tone,
            }).collect(),
            current: 1, ..Default::default()
        }))
        p class="mui-showcase__caption" { "Long labels and large counts wrap as whole chips. Zero is a real count." }
        (render(Props {
            items: vec![
                Chip { label: "Awaiting confirmation from the guest".into(), href: "?status=waiting".into(), count: 10482, tone: Tone::Warning },
                Chip { label: "Cancelled".into(), href: "?status=cancelled".into(), count: 0, tone: Tone::Neutral },
            ], ..Default::default()
        }))
    }
}
