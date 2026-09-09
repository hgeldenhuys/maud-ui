//! The generic record archetype: identity, its action row, then spaced content blocks.
use super::{facts, header, related_list};
use crate::blocks::{
    action::{Action, Heading},
    state::State,
};
use maud::{html, Markup};

#[derive(Clone, Debug)]
pub struct Props {
    pub state: State,
    /// Defaults to an H1; choose a lower level when embedding a record preview.
    pub header: header::Props,
    /// Admitted facts, related lists and other record blocks. The page owns their gap.
    pub children: Markup,
}
impl Default for Props {
    fn default() -> Self {
        Self {
            state: State::Ready,
            header: header::Props {
                heading: Heading::H1,
                ..Default::default()
            },
            children: html! {},
        }
    }
}
pub fn render(mut props: Props) -> Markup {
    let state = std::mem::take(&mut props.state);
    crate::blocks::state::render(state, || {
        html! {
            div class="mui-record-page mui-stack" { (header::render(props.header)) (props.children) }
        }
    })
}
pub fn preview() -> Markup {
    crate::blocks::kit_preview(|_| {
        render(Props {
            header: header::Props {
                title: "Sofia Patel".into(),
                kind: Some("Guest".into()),
                reference: Some("UI-G5".into()),
                primary_action: Some(Action::link("Edit guest", "/blocks/record-facts")),
                secondary_actions: vec![Action::link("Graph view", "/blocks/record-related-list")],
                ..Default::default()
            },
            children: html! {
                (facts::render(facts::Props { groups: vec![
                    facts::Group { title: "Identity".into(), facts: vec![facts::Fact::new("Full name", "Sofia Patel"), facts::Fact::new("Email", "sofia.patel@example.com")] },
                    facts::Group { title: "Sensitive details".into(), facts: vec![facts::Fact::masked("Phone", "••••••0005"), facts::Fact::masked("ID number", "••••••0005")] },
                    facts::Group { title: "Loyalty".into(), facts: vec![facts::Fact::mono("Loyalty points", "0")] },
                ], heading: Heading::H3, ..Default::default() }))
                (related_list::render(related_list::Props { title: "Bookings".into(), items: vec![related_list::Item {
                    reference: "UI-B05".into(), title: "Garden suite".into(), href: Some("/blocks/record-timeline".into()),
                    date: Some(related_list::Date { label: "8 Sep 2026".into(), datetime: Some("2026-09-08".into()) }), ..Default::default()
                }], heading: Heading::H3, ..Default::default() }))
            },
            ..Default::default()
        })
    })
}
