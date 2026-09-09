//! Fictional manager record for the second landing tab. All values are supplied fixtures.
use super::{page_header, sidebar};
use crate::{
    blocks::{
        action::{Action, Heading, Link},
        action_row, attention_banner, record,
    },
    primitives::{badge, breadcrumb::BreadcrumbItem, empty_state, facts_list::Fact},
};
use maud::{html, Markup};

pub fn render(id: &str) -> Markup {
    sidebar::render(sidebar::Props {
        id: id.into(),
        embedded: true,
        brand: html! { span class="mui-block--shell__brand-name" { "Garden House" } },
        nav_groups: vec![sidebar::NavGroup {
            label: Some("Workspace".into()),
            items: vec![
                sidebar::NavItem {
                    label: "Reservations".into(),
                    href: "#lp-worklist-panel".into(),
                    ..Default::default()
                },
                sidebar::NavItem {
                    label: "Reservation record".into(),
                    href: "#lp-record-panel".into(),
                    ..Default::default()
                },
            ],
        }],
        active_path: "#lp-record-panel".into(),
        page_header: Some(page_header::Props {
            breadcrumbs: vec![
                BreadcrumbItem {
                    label: "Garden House".into(),
                    href: Some("/".into()),
                },
                BreadcrumbItem {
                    label: "Reservations and guest management".into(),
                    href: Some("#lp-worklist-panel".into()),
                },
                BreadcrumbItem {
                    label: "Amira Khan".into(),
                    href: None,
                },
            ],
            switchers: html! { span class="mui-caption" { "Manager example" } },
            ..Default::default()
        }),
        children: html! {
            div class="mui-record-example" {
                (record::header::render(record::header::Props { title: "Amira Khan".into(), subtitle: Some("RS-2048 · Garden suite · 8–12 September · 4 nights".into()), heading: Heading::H2,
                    status: Some(badge::Props { label: "Arriving today".into(), variant: badge::Variant::Info, ..Default::default() }),
                    back: Some(Link { label: "Reservations".into(), href: "#lp-worklist-panel".into() }),
                    primary_action: Some(Action::link("Review payment", format!("#{id}-payment"))),
                    secondary_actions: vec![Action::link("Explore record blocks", "/blocks/record-timeline")], ..Default::default() }))
                (attention_banner::render(attention_banner::Props { title: "Amira arrives today at 11:30".into(), subline: Some("Garden suite is ready. Confirm the arrival time before check-in.".into()), tone: attention_banner::Tone::Info, action: Some(Action::link("View arrivals", "#lp-worklist-panel")), ..Default::default() }))
                (record::timeline::render(record::timeline::Props { title: "The stay".into(), milestones: vec![
                    record::timeline::Milestone { date: "24 Aug".into(), datetime: Some("2026-08-24".into()), label: "Booked".into(), subline: Some("Deposit received".into()), state: record::timeline::State::Done, ..Default::default() },
                    record::timeline::Milestone { date: "8 Sep · 11:30".into(), datetime: Some("2026-09-08T11:30:00-04:00".into()), label: "Arrival review".into(), subline: Some("Garden suite ready".into()), state: record::timeline::State::Current, ..Default::default() },
                    record::timeline::Milestone { date: "12 Sep · 11:00".into(), datetime: Some("2026-09-12T11:00:00-04:00".into()), label: "Check out".into(), subline: Some("4 nights".into()), ..Default::default() },
                ], ..Default::default() }))
                div class="mui-record-example__columns" {
                    div id=(format!("{id}-payment")) class="mui-record-example__payment" tabindex="-1" {
                        (record::money::render(record::money::Props { currency: "CAD".into(), total: record::money::Figure::new("Total stay", "$1,080"), paid: record::money::Figure::new("Paid", "$360"), due: record::money::Figure::new("Due", "$720"), breakdown: Some("4 × $240 room nights + $120 taxes & fees".into()), actions: action_row::render(action_row::Props { primary: Some(Action::link("Explore payment actions", "/blocks/action-row")), ..Default::default() }), ..Default::default() }))
                    }
                    div class="mui-record-example__related" {
                        (record::related_card::render(record::related_card::Props { title: "Guest".into(), subtitle: Some("Amira Khan · Returning guest".into()), facts: vec![Fact::new("Contact", "Email preferred"), Fact::new("Language", "English")], action: Some(Action::link("Guest card API", "/blocks/record-related-card")), heading: Heading::H3, ..Default::default() }))
                        (record::related_card::render(record::related_card::Props { title: "Garden suite".into(), subtitle: Some("Room reference".into()), action: Some(Action::link("Room choices", "/choice_card")), heading: Heading::H3, ..Default::default() }))
                    }
                }
                (empty_state::render(empty_state::Props::new("No handover notes yet").with_variant(empty_state::Variant::Inline).with_action(html! { a class="mui-btn mui-btn--outline mui-btn--row" href="/textarea" { "Explore note fields" } })))
                p class="mui-caption" { "Example reservation · Manager view · Amounts in CAD · Times in EDT" }
            }
        },
        ..Default::default()
    })
}
