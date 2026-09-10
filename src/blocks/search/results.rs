use crate::blocks::{
    action::{Heading, Link},
    state::State,
};
use maud::{html, Markup};

#[derive(Clone, Debug, Default)]
pub struct Item {
    pub title: String,
    pub href: String,
    /// A short entity kind, displayed as text rather than a status badge.
    pub kind: String,
    pub description: Option<String>,
    pub reference: Option<String>,
}
#[derive(Clone, Debug)]
pub struct Props {
    pub id: String,
    pub query: String,
    pub items: Vec<Item>,
    /// Authoritative total when results are paginated. None means the supplied list is complete.
    pub total: Option<usize>,
    pub clear_action: Option<Link>,
    pub state: State,
    pub heading: Heading,
}
impl Default for Props {
    fn default() -> Self {
        Self {
            id: "search-results".into(),
            query: String::new(),
            items: vec![],
            total: None,
            clear_action: None,
            state: State::Ready,
            heading: Heading::H2,
        }
    }
}
pub fn render(props: Props) -> Markup {
    super::super::state::render(props.state, || {
        let total = props.total.unwrap_or(props.items.len());
        let title = if props.query.trim().is_empty() {
            "Search results".into()
        } else {
            format!(
                "{total} {} for “{}”",
                if total == 1 { "result" } else { "results" },
                props.query
            )
        };
        html! {
            section class="mui-search-results" id=(props.id) aria-labelledby=(format!("{}-title", props.id)) {
                div id=(format!("{}-title", props.id)) { (props.heading.render(&title, "mui-search-results__title")) }
                @if props.items.is_empty() {
                    p class="mui-search-results__empty" { "No matching records. Try another name or reference." }
                    @if let Some(action) = props.clear_action { a class="mui-btn mui-btn--outline" href=(action.href) { (action.label) } }
                } @else {
                    @if total != props.items.len() { p class="mui-search-results__count" { "Showing " (props.items.len()) " of " (total) "." } }
                    ul class="mui-search-results__list" {
                        @for item in props.items {
                            li { a class="mui-search-results__item" href=(item.href) {
                                span class="mui-search-results__kind" { (item.kind) }
                                span class="mui-search-results__content" {
                                    span class="mui-search-results__identity" { (item.title) }
                                    @if let Some(description) = item.description.filter(|v| !v.trim().is_empty() && v.trim() != item.title.trim()) {
                                        span class="mui-search-results__description" { (description) }
                                    }
                                }
                                @if let Some(reference) = item.reference { span class="mui-search-results__reference" { (reference) } }
                            } }
                        }
                    }
                }
            }
        }
    })
}
pub fn preview() -> Markup {
    super::super::kit_preview(|prefix| {
        render(Props {
            id: format!("{prefix}-search"),
            query: "Leila".into(),
            items: vec![
                Item {
                    kind: "Guest".into(),
                    title: "Leila Morgan".into(),
                    description: Some("leila.morgan@example.com".into()),
                    reference: Some("G-204".into()),
                    href: "/blocks/record-page".into(),
                },
                Item {
                    kind: "Booking".into(),
                    title: "Leila Morgan · Garden suite".into(),
                    description: Some("10–11 September · Confirmed".into()),
                    reference: Some("B-410".into()),
                    href: "/blocks/record-timeline".into(),
                },
            ],
            ..Default::default()
        })
    })
}
