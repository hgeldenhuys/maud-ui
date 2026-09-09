//! One density decision for primary, secondary and native overflow actions.
use super::action::Action;
use maud::{html, Markup};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Density {
    /// 32px, zero vertical padding, for actions beside a row.
    Row,
    /// 32px with a little vertical inset, for compact forms.
    #[default]
    Compact,
    /// 36px, for an ordinary page or form.
    Comfortable,
}
impl Density {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Row => "row",
            Self::Compact => "compact",
            Self::Comfortable => "comfortable",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Props {
    pub primary: Option<Action>,
    pub secondary: Vec<Action>,
    pub overflow: Vec<Action>,
    /// Canonical action fragments override the corresponding typed actions, even if empty.
    pub primary_markup: Option<Markup>,
    pub secondary_markup: Option<Markup>,
    pub overflow_markup: Option<Markup>,
    pub density: Density,
    pub aria_label: String,
    pub overflow_label: String,
}
impl Default for Props {
    fn default() -> Self {
        Self {
            primary: None,
            secondary: vec![],
            overflow: vec![],
            primary_markup: None,
            secondary_markup: None,
            overflow_markup: None,
            density: Density::Compact,
            aria_label: "Actions".into(),
            overflow_label: "More actions".into(),
        }
    }
}

pub fn render(props: Props) -> Markup {
    let primary = props
        .primary_markup
        .unwrap_or_else(|| props.primary.map(|a| a.render(true)).unwrap_or_default());
    let secondary = props
        .secondary_markup
        .unwrap_or_else(|| html! { @for action in props.secondary { (action.render(false)) } });
    let overflow = props
        .overflow_markup
        .unwrap_or_else(|| html! { @for action in props.overflow { (action.render(false)) } });
    if primary.0.is_empty() && secondary.0.is_empty() && overflow.0.is_empty() {
        return html! {};
    }
    html! {
        div class="mui-action-row" data-density=(props.density.as_str()) role="group" aria-label=(props.aria_label) {
            (primary) (secondary)
            @if !overflow.0.is_empty() {
                details class="mui-action-row__more" {
                    summary class="mui-btn mui-btn--outline" { (props.overflow_label) }
                    div class="mui-action-row__overflow" { (overflow) }
                }
            }
        }
    }
}

pub fn preview() -> Markup {
    super::kit_preview(|theme| {
        html! {
            @for density in [Density::Row, Density::Compact, Density::Comfortable] {
                p class="mui-eyebrow" { (density.as_str()) }
                form id=(format!("kit-actions-{theme}-{}", density.as_str())) action="/blocks/action-row" method="get" {
                    input type="hidden" name="example" value=(density.as_str());
                    (render(Props {
                        density, primary: Some(Action::submit("Save changes")),
                        secondary: vec![Action::link("Cancel", "/blocks/record-timeline")],
                        overflow: vec![Action::link("View invoice", "/blocks/record-money")], ..Default::default()
                    }))
                }
            }
            p class="mui-showcase__caption" { "Save submits this example's GET form. No booking is changed. Touch actions are at least 44px." }
        }
    })
}
