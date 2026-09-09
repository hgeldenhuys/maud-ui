//! A display of caller-supplied monetary values, never a pricing or payment calculation.
use crate::blocks::{
    action::{Action, Heading},
    action_row,
};
use maud::{html, Markup};

#[derive(Clone, Debug, Default)]
pub struct Figure {
    pub label: String,
    /// Already formatted and authorized. None is unavailable, not zero.
    pub value: Option<Markup>,
}
impl Figure {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: Some(html! { (value.into()) }),
        }
    }
}
#[derive(Clone, Debug)]
pub struct Props {
    /// Explicit presentation state; Ready preserves the ordinary content.
    pub state: crate::blocks::state::State,
    pub title: String,
    pub heading: Heading,
    /// Currency label supplied from the Money value, never inferred from the theme.
    pub currency: String,
    pub total: Figure,
    pub paid: Figure,
    pub due: Figure,
    pub emphasize_due: bool,
    pub unavailable_label: String,
    pub breakdown: Option<String>,
    /// Use action_row::render, or a canonical server-owned action fragment.
    pub actions: Markup,
}
impl Default for Props {
    fn default() -> Self {
        Self {
            state: Default::default(),
            title: "Payment".into(),
            heading: Heading::H2,
            currency: String::new(),
            total: Figure {
                label: "Total".into(),
                ..Default::default()
            },
            paid: Figure {
                label: "Paid".into(),
                ..Default::default()
            },
            due: Figure {
                label: "Due".into(),
                ..Default::default()
            },
            emphasize_due: true,
            unavailable_label: "Unavailable".into(),
            breakdown: None,
            actions: html! {},
        }
    }
}
pub fn render(mut props: Props) -> Markup {
    let state = std::mem::take(&mut props.state);
    crate::blocks::state::render(state, || render_ready(props))
}

fn render_ready(props: Props) -> Markup {
    html! {
        section class="mui-record-money mui-stack" aria-label=(&props.title) {
            header class="mui-record-money__header" {
                (props.heading.render(&props.title, "mui-kit-title"))
                @if !props.currency.is_empty() { span class="mui-record-money__currency" { (props.currency) } }
            }
            dl class="mui-record-money__figures" {
                @for (index, figure) in [&props.total, &props.paid, &props.due].into_iter().enumerate() {
                    div class="mui-record-money__figure" data-emphasis=[(index == 2 && props.emphasize_due && figure.value.is_some()).then_some("warning")] {
                        dt { (&figure.label) }
                        dd { @if let Some(value) = &figure.value { (value) } @else { span class="mui-record-money__unavailable" { (&props.unavailable_label) } } }
                    }
                }
            }
            @if let Some(line) = props.breakdown { p class="mui-record-money__breakdown" { (line) } }
            @if !props.actions.0.is_empty() { div class="mui-record-money__actions" { (props.actions) } }
        }
    }
}
pub fn preview() -> Markup {
    crate::blocks::kit_preview(|_| {
        html! {
            (render(Props { currency: "CAD".into(), total: Figure::new("Total stay", "$840"), paid: Figure::new("Paid", "$280"), due: Figure::new("Due at check-in", "$560"), breakdown: Some("3 × $240 room nights + $120 taxes & fees. Breakfast included.".into()), actions: action_row::render(action_row::Props { primary: Some(Action::link("Review payment", "/blocks/attention-banner")), secondary: vec![Action::link("View record", "/blocks/record-timeline")], ..Default::default() }), ..Default::default() }))
            (render(Props { title: "Unavailable payment summary".into(), breakdown: Some("No financial values were supplied for this example.".into()), ..Default::default() }))
        }
    })
}
