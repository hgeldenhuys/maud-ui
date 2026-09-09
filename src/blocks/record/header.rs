//! Record identity, status, one primary action and subordinate actions.
use crate::{
    blocks::action::{Action, Heading, Link},
    primitives::badge,
};
use maud::{html, Markup};

#[derive(Clone, Debug, Default)]
pub struct Props {
    /// Explicit presentation state; Ready preserves the ordinary content.
    pub state: crate::blocks::state::State,
    pub title: String,
    /// Overrides `title`. May contain a field-emitter heading; no heading is wrapped around it.
    pub title_markup: Option<Markup>,
    pub subtitle: Option<String>,
    pub status: Option<badge::Props>,
    /// Overrides `status`, including when the supplied markup is empty.
    pub status_markup: Option<Markup>,
    pub primary_action: Option<Action>,
    pub secondary_actions: Vec<Action>,
    pub back: Option<Link>,
    pub heading: Heading,
}

pub fn render(mut props: Props) -> Markup {
    let state = std::mem::take(&mut props.state);
    crate::blocks::state::render(state, || render_ready(props))
}

fn render_ready(props: Props) -> Markup {
    html! {
        header class="mui-record-header" {
            @if let Some(back) = props.back {
                a class="mui-record-header__back" href=(back.href) { span aria-hidden="true" { "← " } (back.label) }
            }
            div class="mui-record-header__row" {
                div class="mui-record-header__identity" {
                    div class="mui-record-header__title-line" {
                        @if let Some(title) = props.title_markup {
                            div class="mui-record-header__title" { (title) }
                        } @else {
                            (props.heading.render(&props.title, "mui-record-header__title"))
                        }
                        @if let Some(status) = props.status_markup { (status) }
                        @else if let Some(status) = props.status { (badge::render(status)) }
                    }
                    @if let Some(subtitle) = props.subtitle { p class="mui-record-header__subtitle" { (subtitle) } }
                }
                (crate::blocks::action_row::render(crate::blocks::action_row::Props {
                    primary: props.primary_action, overflow: props.secondary_actions,
                    density: crate::blocks::action_row::Density::Compact, ..Default::default()
                }))
            }
        }
    }
}

pub fn preview() -> Markup {
    render(Props {
        title: "Sofia Davis".into(),
        subtitle: Some("Reservation RS-2048 · Garden suite · 8–12 September".into()),
        status: Some(badge::Props {
            label: "Arriving today".into(),
            variant: badge::Variant::Info,
            ..Default::default()
        }),
        primary_action: Some(Action::link(
            "Check in",
            "/blocks/record-header?step=check-in",
        )),
        secondary_actions: vec![
            Action::link("Edit reservation", "?step=edit"),
            Action::link("View invoice", "?step=invoice"),
        ],
        back: Some(Link {
            label: "Reservations".into(),
            href: "/blocks/worklist-header".into(),
        }),
        ..Default::default()
    })
}
