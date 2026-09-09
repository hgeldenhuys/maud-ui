//! Related records in compact rows, with no schema or relationship diagnostics.
use crate::{
    blocks::{action::Heading, state::State},
    primitives::badge,
};
use maud::{html, Markup};

#[derive(Clone, Debug, Default)]
pub struct Date {
    /// Short human-facing date, e.g. 8 Sep 2026.
    pub label: String,
    /// Optional machine-readable value for the native time element.
    pub datetime: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct Item {
    /// The reference alone; an optional leading # is preserved.
    pub reference: String,
    /// Human-facing title. Repeated leading copies of the reference are removed.
    pub title: String,
    pub href: Option<String>,
    pub status: Option<badge::Props>,
    pub date: Option<Date>,
}

#[derive(Clone, Debug)]
pub struct Props {
    pub state: State,
    /// Use an audience-facing plural such as Bookings, never a schema relationship name.
    pub title: String,
    pub items: Vec<Item>,
    pub empty_message: String,
    pub heading: Heading,
}
impl Default for Props {
    fn default() -> Self {
        Self {
            state: State::Ready,
            title: "Related records".into(),
            items: vec![],
            empty_message: "No related records.".into(),
            heading: Heading::H2,
        }
    }
}

fn distinct_title<'a>(reference: &str, title: &'a str) -> &'a str {
    let reference = reference.trim().trim_start_matches('#');
    let mut title = title.trim();
    if reference.is_empty() {
        return title;
    }
    loop {
        let candidate = title.trim_start_matches('#');
        let Some(rest) = candidate.strip_prefix(reference) else {
            return title;
        };
        if !rest.is_empty()
            && !rest.starts_with(|c: char| c.is_whitespace() || matches!(c, '·' | '—' | ':'))
        {
            return title;
        }
        title =
            rest.trim_start_matches(|c: char| c.is_whitespace() || matches!(c, '·' | '—' | ':'));
    }
}

fn identity(reference: &str, title: &str) -> Markup {
    html! {
        @if !reference.is_empty() { span class="mui-related-list__reference" { (reference) } }
        @if !reference.is_empty() && !title.is_empty() { span class="mui-related-list__separator" aria-hidden="true" { "·" } }
        @if !title.is_empty() { span class="mui-related-list__title" { (title) } }
    }
}

pub fn render(mut props: Props) -> Markup {
    let state = std::mem::take(&mut props.state);
    crate::blocks::state::render(state, || {
        props
            .items
            .retain(|item| !item.reference.trim().is_empty() || !item.title.trim().is_empty());
        html! {
            section class="mui-related-list mui-stack" {
                (props.heading.render(&props.title, "mui-kit-title"))
                @if props.items.is_empty() { p class="mui-related-list__empty" { (props.empty_message) } }
                @else {
                    ul class="mui-related-list__items" {
                        @for item in props.items {
                            @let reference = item.reference.trim();
                            @let title = distinct_title(reference, &item.title);
                            @let label = [reference, title].into_iter().filter(|part| !part.is_empty()).collect::<Vec<_>>().join(" · ");
                            li class="mui-related-list__item" {
                                @if let Some(href) = item.href.filter(|href| !href.trim().is_empty()) {
                                    a class="mui-related-list__identity" href=(href) title=(&label) aria-label=(&label) { (identity(reference, title)) }
                                } @else { span class="mui-related-list__identity" title=(&label) { (identity(reference, title)) } }
                                @if let Some(status) = item.status { (badge::render(status)) }
                                @if let Some(date) = item.date.filter(|date| !date.label.trim().is_empty()) {
                                    time class="mui-related-list__date" datetime=[date.datetime.as_deref()] { (date.label) }
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}

pub fn preview() -> Markup {
    crate::blocks::kit_preview(|_| {
        html! {
            (render(Props { title: "Bookings".into(), items: vec![
                Item { reference: "UI-B05".into(), title: "UI-B05 · Garden suite".into(), href: Some("/blocks/record-page".into()),
                    status: Some(badge::Props { label: "Confirmed".into(), variant: badge::Variant::Success, ..Default::default() }),
                    date: Some(Date { label: "8 Sep 2026".into(), datetime: Some("2026-09-08".into()) }) },
                Item { reference: "#UI-B02".into(), title: "#UI-B02 #UI-B02".into(), href: Some("/blocks/record-page".into()), ..Default::default() },
            ], ..Default::default() }))
            (render(Props { title: "Notes".into(), empty_message: "No notes yet.".into(), ..Default::default() }))
        }
    })
}
