use crate::blocks::action::Heading;
use maud::{html, Markup};

#[derive(Clone, Debug)]
pub struct Props {
    pub title: String,
    /// Human identity carried across every step, e.g. guest, room and dates.
    pub context: Option<String>,
    pub steps: Vec<String>,
    /// One-based current step. Clamped to the supplied step list.
    pub current: usize,
    /// Explicit terminal state, supplied by the application.
    pub complete: bool,
    pub heading: Heading,
}
impl Default for Props {
    fn default() -> Self {
        Self {
            title: "Check in guest".into(),
            context: None,
            steps: vec![],
            current: 1,
            complete: false,
            heading: Heading::H1,
        }
    }
}
pub fn render(props: Props) -> Markup {
    let current = props.current.max(1).min(props.steps.len().max(1));
    html! {
        header class="mui-wizard-header" {
            (props.heading.render(&props.title, "mui-wizard-header__title"))
            @if let Some(context) = props.context { p class="mui-wizard-header__context" { (context) } }
            @if !props.steps.is_empty() {
                p class="mui-wizard-header__progress" role="status" aria-atomic="true" {
                    @if props.complete { "All " (props.steps.len()) " steps complete" }
                    @else { "Step " (current) " of " (props.steps.len()) " · " (&props.steps[current - 1]) }
                }
                ol class="mui-wizard-header__steps" aria-label="Progress" {
                    @for (i, label) in props.steps.iter().enumerate() {
                        @let done = props.complete || i + 1 < current;
                        @let active = !props.complete && i + 1 == current;
                        li data-step=(if done { "complete" } else if active { "current" } else { "upcoming" }) aria-current=[if active { Some("step") } else { None }] {
                            span class="mui-wizard-header__number" aria-hidden="true" { @if done { "✓" } @else { (i + 1) } }
                            span { (label) @if done { span class="mui-sr-only" { " · Complete" } } }
                        }
                    }
                }
            }
        }
    }
}
pub fn preview() -> Markup {
    super::super::kit_preview(|_| {
        render(Props {
            context: Some("Leila Morgan · Garden suite · 10–11 September".into()),
            steps: vec![
                "Select booking".into(),
                "Verify guest".into(),
                "Check in".into(),
            ],
            current: 2,
            heading: Heading::H2,
            ..Default::default()
        })
    })
}
