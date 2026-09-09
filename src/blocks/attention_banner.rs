//! A supplied attention message with one contextual action and optional local dismissal.
use crate::blocks::action::Action;
use maud::{html, Markup};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Tone {
    #[default]
    Warning,
    Danger,
    Info,
}
impl Tone {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Warning => "warning",
            Self::Danger => "danger",
            Self::Info => "info",
        }
    }
}
#[derive(Clone, Debug)]
pub struct Dismiss {
    pub label: String,
    /// Optional existing element ID to focus after dismissing. Otherwise focus the next control.
    pub focus_target: Option<String>,
}
impl Default for Dismiss {
    fn default() -> Self {
        Self {
            label: "Dismiss notice".into(),
            focus_target: None,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct Props {
    /// Explicit presentation state; Ready preserves the ordinary content.
    pub state: crate::blocks::state::State,
    pub title: String,
    pub subline: Option<String>,
    pub action: Option<Action>,
    pub action_markup: Option<Markup>,
    pub tone: Tone,
    pub dismiss: Option<Dismiss>,
}
pub fn render(mut props: Props) -> Markup {
    let state = std::mem::take(&mut props.state);
    crate::blocks::state::render(state, || render_ready(props))
}

fn render_ready(props: Props) -> Markup {
    if props.title.trim().is_empty() { return html! {}; }
    html! {
        aside class="mui-attention-banner" data-tone=(props.tone.as_str()) aria-label=(&props.title) data-mui="attention-banner" {
            div class="mui-attention-banner__body" {
                strong class="mui-attention-banner__title" { (props.title) }
                @if let Some(subline) = props.subline { p { (subline) } }
            }
            @if let Some(action) = props.action_markup { div class="mui-attention-banner__action" { (action) } }
            @else if let Some(action) = props.action { div class="mui-attention-banner__action" { (action.render(false)) } }
            @if let Some(dismiss) = props.dismiss {
                button class="mui-attention-banner__dismiss" type="button" aria-label=(dismiss.label) data-dismiss-focus=[dismiss.focus_target] hidden {
                    span aria-hidden="true" { "×" }
                }
            }
        }
    }
}
pub fn preview() -> Markup {
    crate::blocks::kit_preview(|theme| {
        html! {
            (render(Props { title: "Maya Chen arrives at 11:30 with CAD $560 due".into(), subline: Some("Juniper 04 is ready. Review payment before completing check-in.".into()), action: Some(Action::link("Review payment", "/blocks/record-money")), ..Default::default() }))
            (render(Props { title: "Arrival needs a room review".into(), subline: Some("Cedar 12 has no confirmed readiness update.".into()), tone: Tone::Danger, action: Some(Action::link("Review room", "/blocks/record-related-card")), ..Default::default() }))
            (render(Props { title: "Desk handover is ready".into(), subline: Some("Read the latest notes before starting your shift.".into()), tone: Tone::Info, dismiss: Some(Dismiss { focus_target: Some(format!("kit-banner-next-{theme}")), ..Default::default() }), ..Default::default() }))
            a id=(format!("kit-banner-next-{theme}")) class="mui-btn mui-btn--outline mui-btn--row" href="/blocks/worklist-grouped" { "Open arrivals" }
        }
    })
}
