//! Record identity, status, one primary action and subordinate actions.
use crate::{
    blocks::action::{Action, Heading, Link},
    primitives::badge,
};
use maud::{html, Markup};

#[derive(Clone, Debug, Default)]
pub struct Props {
    pub title: String,
    pub subtitle: Option<String>,
    pub status: Option<badge::Props>,
    pub primary_action: Option<Action>,
    pub secondary_actions: Vec<Action>,
    pub back: Option<Link>,
    pub heading: Heading,
}

pub fn render(props: Props) -> Markup {
    html! {
        header class="mui-record-header" {
            @if let Some(back) = props.back {
                a class="mui-record-header__back" href=(back.href) { span aria-hidden="true" { "← " } (back.label) }
            }
            div class="mui-record-header__row" {
                div class="mui-record-header__identity" {
                    div class="mui-record-header__title-line" {
                        (props.heading.render(&props.title, "mui-record-header__title"))
                        @if let Some(status) = props.status { (badge::render(status)) }
                    }
                    @if let Some(subtitle) = props.subtitle { p class="mui-record-header__subtitle" { (subtitle) } }
                }
                @if props.primary_action.is_some() || !props.secondary_actions.is_empty() {
                    div class="mui-record-header__actions" {
                        @if let Some(action) = props.primary_action { (action.render(true)) }
                        @if !props.secondary_actions.is_empty() {
                            details class="mui-record-header__more" {
                                summary { "More actions" }
                                div class="mui-record-header__secondary" role="group" aria-label="Secondary actions" {
                                    @for action in props.secondary_actions { (action.render(false)) }
                                }
                            }
                        }
                    }
                }
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
