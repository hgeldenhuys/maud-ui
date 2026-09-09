//! Planned and actual milestones, supplied by the caller in chronological order.
use crate::blocks::action::Heading;
use maud::{html, Markup};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum State {
    Done,
    Current,
    #[default]
    Scheduled,
}
impl State {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Done => "done",
            Self::Current => "current",
            Self::Scheduled => "scheduled",
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Done => "Done",
            Self::Current => "Current",
            Self::Scheduled => "Scheduled",
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct Milestone {
    pub date: String,
    /// ISO date/datetime for the time element; visible date remains caller formatted.
    pub datetime: Option<String>,
    pub label: String,
    pub subline: Option<String>,
    pub state: State,
    /// Optional localized state label. State is always conveyed in text as well as color.
    pub state_label: Option<String>,
}
#[derive(Clone, Debug)]
pub struct Props {
    /// Explicit presentation state; Ready preserves the ordinary content.
    pub state: crate::blocks::state::State,
    pub title: String,
    pub heading: Heading,
    pub milestones: Vec<Milestone>,
}
impl Default for Props {
    fn default() -> Self {
        Self {
            state: Default::default(),
            title: "Timeline".into(),
            heading: Heading::H2,
            milestones: vec![],
        }
    }
}
/// Designed for 3–5 milestones. Empty input renders nothing; multiple current steps are invalid.
pub fn render(mut props: Props) -> Markup {
    let state = std::mem::take(&mut props.state);
    crate::blocks::state::render(state, || render_ready(props))
}

fn render_ready(props: Props) -> Markup {
    if props.milestones.is_empty() {
        return html! {};
    }
    assert!(
        props
            .milestones
            .iter()
            .filter(|m| m.state == State::Current)
            .count()
            <= 1,
        "timeline accepts at most one current milestone"
    );
    html! {
        section class="mui-record-timeline mui-stack" aria-label=(&props.title) {
            (props.heading.render(&props.title, "mui-kit-title"))
            ol class="mui-record-timeline__items" {
                @for milestone in props.milestones {
                    li class="mui-record-timeline__item" data-state=(milestone.state.as_str()) aria-current=[(milestone.state == State::Current).then_some("step")] {
                        span class="mui-record-timeline__marker" aria-hidden="true" { @if milestone.state == State::Done { "✓" } }
                        @if let Some(datetime) = milestone.datetime {
                            time class="mui-record-timeline__date" datetime=(datetime) { (milestone.date) }
                        } @else {
                            span class="mui-record-timeline__date" { (milestone.date) }
                        }
                        strong class="mui-record-timeline__label" { (milestone.label) }
                        @if let Some(line) = milestone.subline { p class="mui-record-timeline__subline" { (line) } }
                        span class="mui-record-timeline__state" { (milestone.state_label.as_deref().unwrap_or(milestone.state.label())) }
                    }
                }
            }
        }
    }
}
pub fn preview() -> Markup {
    crate::blocks::kit_preview(|_| {
        render(Props {
            title: "The stay".into(),
            milestones: vec![
                Milestone {
                    date: "24 Aug".into(),
                    datetime: Some("2026-08-24".into()),
                    label: "Booked & deposit paid".into(),
                    subline: Some("CAD $280 received".into()),
                    state: State::Done,
                    ..Default::default()
                },
                Milestone {
                    date: "Today · 11:30".into(),
                    datetime: Some("2026-09-08T11:30:00-04:00".into()),
                    label: "Arrival review".into(),
                    subline: Some("Juniper 04 ready · CAD $560 due".into()),
                    state: State::Current,
                    ..Default::default()
                },
                Milestone {
                    date: "Fri 11 Sep".into(),
                    datetime: Some("2026-09-11".into()),
                    label: "Check out by 11:00".into(),
                    subline: Some("3 nights at the lodge".into()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        })
    })
}
