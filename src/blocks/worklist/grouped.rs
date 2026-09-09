//! Grouped, caller-ordered rows with a single responsive DOM.
use crate::{
    blocks::action::{Action, Heading},
    primitives::{
        badge,
        facts_list::{self, Fact},
    },
};
use maud::{html, Markup};

#[derive(Clone, Debug, Default)]
pub struct Row {
    pub identity: String,
    pub href: Option<String>,
    pub identity_markup: Option<Markup>,
    pub subline: String,
    pub facts: [Fact; 2],
    /// Overrides the facts region with canonical fields; retain a .mui-facts definition list.
    pub facts_markup: Option<Markup>,
    pub status: badge::Props,
    pub status_markup: Option<Markup>,
    pub action: Option<Action>,
    pub action_markup: Option<Markup>,
    pub note: Option<String>,
}
#[derive(Clone, Debug, Default)]
pub struct Group {
    /// Supply the time/state and correctly scoped count; this block never infers totals.
    pub label: String,
    pub subline: Option<String>,
    pub rows: Vec<Row>,
}
#[derive(Clone, Debug)]
pub struct Props {
    /// Explicit presentation state; Ready preserves the ordinary content.
    pub state: crate::blocks::state::State,
    pub aria_label: String,
    pub group_heading: Heading,
    pub groups: Vec<Group>,
    pub empty_message: String,
    /// Optional partial-page/freshness explanation, supplied by the caller.
    pub footer: Option<String>,
}
impl Default for Props {
    fn default() -> Self {
        Self {
            state: Default::default(),
            aria_label: "Grouped worklist".into(),
            group_heading: Heading::H3,
            groups: vec![],
            empty_message: "No items to show.".into(),
            footer: None,
        }
    }
}
pub fn render(mut props: Props) -> Markup {
    let state = std::mem::take(&mut props.state);
    crate::blocks::state::render(state, || render_ready(props))
}

fn render_ready(props: Props) -> Markup {
    let empty = props.groups.iter().all(|g| g.rows.is_empty());
    html! {
        div class="mui-grouped-worklist" role="region" aria-label=(props.aria_label) {
            @if empty { p class="mui-grouped-worklist__empty" { (props.empty_message) } }
            @else {
                @for group in props.groups.into_iter().filter(|g| !g.rows.is_empty()) {
                    section class="mui-grouped-worklist__group" {
                        header class="mui-grouped-worklist__header" {
                            (props.group_heading.render(&group.label, "mui-grouped-worklist__heading"))
                            @if let Some(subline) = group.subline { p { (subline) } }
                        }
                        ul class="mui-grouped-worklist__rows" {
                            @for row in group.rows {
                                li class="mui-grouped-worklist__row" {
                                    div class="mui-grouped-worklist__identity" {
                                        @if let Some(identity) = row.identity_markup { (identity) }
                                        @else if let Some(href) = row.href { a href=(href) { (row.identity) } }
                                        @else { strong { (row.identity) } }
                                        p { (row.subline) }
                                    }
                                    @if let Some(facts) = row.facts_markup { (facts) }
                                    @else { (facts_list::render(facts_list::Props { facts: row.facts.into() })) }
                                    div class="mui-grouped-worklist__status" {
                                        @if let Some(status) = row.status_markup { (status) } @else { (badge::render(row.status)) }
                                    }
                                    div class="mui-grouped-worklist__action" {
                                        @if let Some(action) = row.action_markup { (action) } @else if let Some(action) = row.action { (action.render(false)) }
                                    }
                                    @if let Some(note) = row.note { p class="mui-grouped-worklist__note" { (note) } }
                                }
                            }
                        }
                    }
                }
            }
            @if let Some(footer) = props.footer { p class="mui-grouped-worklist__footer" { (footer) } }
        }
    }
}
pub fn preview() -> Markup {
    crate::blocks::kit_preview(|_| {
        render(Props {
            aria_label: "Today's arrivals".into(),
            groups: vec![
                Group {
                    label: "Before 14:00 · 2 arriving".into(),
                    rows: vec![
                        Row {
                            identity: "Maya Chen".into(),
                            href: Some("/blocks/record-timeline".into()),
                            subline: "CL-260908-014 · Juniper 04 · 2 adults".into(),
                            facts: [
                                Fact::new("Arrival", "11:30"),
                                Fact::new("Payment", "CAD $560 due"),
                            ],
                            status: badge::Props {
                                label: "Room ready".into(),
                                variant: badge::Variant::Success,
                                ..Default::default()
                            },
                            action: Some(Action::link(
                                "Review check-in",
                                "/blocks/attention-banner",
                            )),
                            note: Some(
                                "Early arrival agreed. Dinner for two requested at 19:00.".into(),
                            ),
                            ..Default::default()
                        },
                        Row {
                            identity: "Oliver Reed".into(),
                            subline: "CL-260908-018 · Birch 08 · 2 adults".into(),
                            facts: [
                                Fact::new("Arrival", "12:15"),
                                Fact::new("Payment", "Paid in full"),
                            ],
                            status: badge::Props {
                                label: "Cleaning".into(),
                                variant: badge::Variant::Warning,
                                ..Default::default()
                            },
                            action: Some(Action::link(
                                "Review arrival",
                                "/blocks/record-related-card",
                            )),
                            note: Some(
                                "Room expected ready at 11:45. Offer tea in the lounge if needed."
                                    .into(),
                            ),
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                },
                Group {
                    label: "Time to confirm · 1 arriving".into(),
                    rows: vec![Row {
                        identity: "Amina Patel".into(),
                        subline: "CL-260908-021 · Cedar 12 · 2 adults".into(),
                        facts: [
                            Fact::new("Arrival", "Not confirmed"),
                            Fact::new("Payment", "Unavailable"),
                        ],
                        status: badge::Props {
                            label: "Confirm arrival".into(),
                            variant: badge::Variant::Info,
                            ..Default::default()
                        },
                        action: Some(Action::link("View booking", "/blocks/record-header")),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            ],
            footer: Some("Illustrative arrivals · Lodge local time (EDT) · Balances in CAD".into()),
            ..Default::default()
        })
    })
}
