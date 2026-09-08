//! Title, one count sentence, native GET search and a single primary action.
use crate::blocks::action::{Action, Heading};
use maud::{html, Markup};

#[derive(Clone, Debug)]
pub struct Search {
    pub action: String,
    pub name: String,
    pub value: String,
    pub label: String,
    pub placeholder: String,
    /// Preserve status filters or scope while searching. Do not include paging offsets.
    pub hidden_fields: Vec<(String, String)>,
}

impl Default for Search {
    fn default() -> Self {
        Self {
            action: String::new(),
            name: "q".into(),
            value: String::new(),
            label: "Search records".into(),
            placeholder: "Search by name or reference".into(),
            hidden_fields: vec![],
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Props {
    pub title: String,
    /// Caller-owned localized sentence, e.g. "48 reservations · 12 arriving today".
    pub count_sentence: String,
    pub search: Option<Search>,
    pub primary_action: Option<Action>,
    pub heading: Heading,
}

pub fn render(props: Props) -> Markup {
    html! {
        header class="mui-worklist-header" {
            div class="mui-worklist-header__identity" {
                (props.heading.render(&props.title, "mui-worklist-header__title"))
                @if !props.count_sentence.is_empty() { p class="mui-worklist-header__count" { (props.count_sentence) } }
            }
            @if let Some(search) = props.search {
                form class="mui-worklist-header__search" role="search" aria-label=(&search.label) method="get" action=(search.action) {
                    @for (name, value) in search.hidden_fields { input type="hidden" name=(name) value=(value); }
                    label class="mui-worklist-header__search-field" {
                        span class="mui-sr-only" { (search.label) }
                        input class="mui-input" type="search" name=(search.name) value=(search.value) placeholder=(search.placeholder);
                    }
                    button class="mui-btn mui-btn--outline mui-btn--md" type="submit" { "Search" }
                }
            }
            @if let Some(action) = props.primary_action { div class="mui-worklist-header__action" { (action.render(true)) } }
        }
    }
}

pub fn preview() -> Markup {
    html! {
        (render(Props {
            title: "Reservations".into(), count_sentence: "48 reservations · 12 arriving today".into(),
            search: Some(Search { action: "/blocks/worklist-header".into(), label: "Search reservations".into(), ..Default::default() }),
            primary_action: Some(Action::link("New reservation", "/blocks/record-header")), ..Default::default()
        }))
        (crate::primitives::status_chip_group::render(crate::primitives::status_chip_group::Props {
            items: [("All", 48, crate::primitives::status_chip_group::Tone::Neutral), ("Arriving", 12, crate::primitives::status_chip_group::Tone::Info), ("Checked in", 24, crate::primitives::status_chip_group::Tone::Success), ("Needs review", 9, crate::primitives::status_chip_group::Tone::Warning), ("Failed", 3, crate::primitives::status_chip_group::Tone::Danger)].into_iter().map(|(label, count, tone)| crate::primitives::status_chip_group::Chip {
                label: label.into(), href: format!("?status={label}"), count, tone,
            }).collect(), current: 1, ..Default::default()
        }))
        (crate::primitives::table::render(crate::primitives::table::Props {
            headers: vec!["Guest".into(), "Room".into(), "Next action".into()],
            rich_rows: [("Sofia Davis", "Garden suite"), ("Mateo Ortega", "Courtyard room")].into_iter().map(|(guest, room)| vec![
                crate::primitives::table::CellMarkup::text(guest), crate::primitives::table::CellMarkup::text(room),
                crate::primitives::table::CellMarkup::markup(html! { a class="mui-btn mui-btn--outline mui-btn--row" href="/blocks/record-header" aria-label=(format!("Check in {guest}")) { "Check in" } }, false),
            ]).collect(), ..Default::default()
        }))
    }
}
