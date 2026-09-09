//! Sticky page context: breadcrumbs, search, actions and arbitrary switcher slots.
use crate::primitives::breadcrumb::{self, BreadcrumbItem};
use maud::{html, Markup};

#[derive(Clone, Debug)]
pub struct Search {
    /// Native GET destination; search works without the behavior bundle.
    pub action: String,
    pub name: String,
    pub value: String,
    pub placeholder: String,
}
impl Default for Search {
    fn default() -> Self {
        Self {
            action: String::new(),
            name: "q".into(),
            value: String::new(),
            placeholder: "Search workspace…".into(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Props {
    /// Explicit presentation state; Ready preserves the ordinary content.
    pub state: crate::blocks::state::State,
    /// Mobile menu trigger or other leading control, before the breadcrumb.
    pub leading: Markup,
    pub breadcrumbs: Vec<BreadcrumbItem>,
    pub search: Option<Search>,
    /// Replaces the native search form, e.g. with a command palette trigger.
    pub search_markup: Option<Markup>,
    pub actions: Markup,
    /// Role, language and theme controls in caller-defined markup.
    pub switchers: Markup,
}

pub fn render(mut props: Props) -> Markup {
    let state = std::mem::take(&mut props.state);
    crate::blocks::state::render(state, || render_ready(props))
}

fn render_ready(props: Props) -> Markup {
    html! {
        header class="mui-page-header" {
            div class="mui-page-header__context" {
                (props.leading)
                (breadcrumb::render(breadcrumb::Props { items: props.breadcrumbs, ..Default::default() }))
            }
            div class="mui-page-header__search" {
                @if let Some(search) = props.search_markup { (search) }
                @else if let Some(search) = props.search {
                    details class="mui-page-header__search-disclosure" data-mui="header-search" {
                        summary class="mui-btn mui-btn--outline mui-page-header__search-toggle" aria-label="Search workspace" title="Search workspace" {
                            svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" aria-hidden="true" { circle cx="10" cy="10" r="6"; path d="m15 15 5 5"; }
                            span class="mui-page-header__search-label" { "Search" }
                        }
                        div class="mui-page-header__search-panel" {
                    form method="get" action=(search.action) role="search" {
                        label {
                            span class="mui-sr-only" { "Search workspace" }
                            input class="mui-input" type="search" name=(search.name) value=(search.value) placeholder=(search.placeholder) data-mui="page-search";
                        }
                        kbd aria-hidden="true" { "⌘ K" }
                        button class="mui-btn mui-btn--outline mui-page-header__submit" type="submit" { "Search" }
                    }
                        }
                    }
                }
            }
            div class="mui-page-header__controls" {
                (props.actions)
                @if !props.switchers.0.is_empty() { div class="mui-page-header__switchers" { (props.switchers) } }
            }
        }
    }
}

pub fn preview() -> Markup {
    render(Props {
        breadcrumbs: vec![
            BreadcrumbItem {
                label: "Workspace".into(),
                href: Some("/blocks/shell-sidebar".into()),
            },
            BreadcrumbItem {
                label: "Reservations".into(),
                href: None,
            },
        ],
        search: Some(Search {
            action: "/".into(),
            ..Default::default()
        }),
        actions: html! { a class="mui-btn mui-btn--outline mui-btn--sm" href="/blocks/worklist-header" { "New reservation" } },
        switchers: html! { label { span class="mui-sr-only" { "Role" } select class="mui-native-select" { option { "Front desk" } option { "Manager" } } } },
        ..Default::default()
    })
}
