//! Explicit presentation states. These never infer authorization, totals or business state.
use super::action::Action;
use crate::primitives::skeleton;
use maud::{html, Markup};

#[derive(Clone, Debug, Default)]
pub enum State {
    #[default]
    Ready,
    Loading {
        message: String,
    },
    Empty {
        message: String,
        action: Option<Action>,
    },
    Error {
        message: String,
        retry: Option<Action>,
    },
    Disabled {
        reason: String,
    },
}
impl State {
    pub fn loading() -> Self {
        Self::Loading {
            message: "Loading…".into(),
        }
    }
    pub fn empty(message: impl Into<String>) -> Self {
        Self::Empty {
            message: message.into(),
            action: None,
        }
    }
    pub fn error(message: impl Into<String>) -> Self {
        Self::Error {
            message: message.into(),
            retry: None,
        }
    }
    pub fn disabled(reason: impl Into<String>) -> Self {
        Self::Disabled {
            reason: reason.into(),
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Loading { .. } => "loading",
            Self::Empty { .. } => "empty",
            Self::Error { .. } => "error",
            Self::Disabled { .. } => "disabled",
        }
    }
}
/// Loading, empty and error do not evaluate the data renderer. Disabled retains admitted
/// content and makes its whole interaction subtree inert, including canonical fragments.
pub(crate) fn render(state: State, ready: impl FnOnce() -> Markup) -> Markup {
    match state {
        State::Ready => ready(),
        State::Loading { message } => html! {
            div class="mui-block-state" data-state="loading" aria-busy="true" {
                p class="mui-block-state__message" role="status" { (message) }
                div class="mui-block-state__skeleton" aria-hidden="true" {
                    @for _ in 0..3 { (skeleton::render(skeleton::Props { variant: skeleton::Variant::Text, ..Default::default() })) }
                }
            }
        },
        State::Empty { message, action } => html! {
            div class="mui-block-state" data-state="empty" {
                p class="mui-block-state__message" { (message) }
                @if let Some(action) = action { (action.render(false)) }
            }
        },
        State::Error { message, retry } => html! {
            div class="mui-block-state" data-state="error" {
                p class="mui-block-state__message" role="alert" { (message) }
                @if let Some(action) = retry { (action.render(false)) }
            }
        },
        State::Disabled { reason } => html! {
            div class="mui-block-state" data-state="disabled" {
                p class="mui-block-state__message" { (reason) }
                div class="mui-block-state__disabled" inert="" aria-disabled="true" { (ready()) }
            }
        },
    }
}

pub const OPERATIONAL_BLOCKS: &[&str] = &[
    "worklist-header",
    "worklist-grouped",
    "record-header",
    "record-timeline",
    "record-money",
    "record-related-card",
    "task-grid",
    "attention-banner",
    "action-row",
    "shell-sidebar",
    "shell-page-header",
    "shell-app-header",
    "shell-app-footer",
];
fn examples(mut render: impl FnMut(State) -> Markup) -> Markup {
    html! {
        div class="mui-state-examples" {
            @for state in [State::loading(), State::Empty { message: "No items yet.".into(), action: Some(Action::link("Add an item", "/blocks/action-row")) }, State::Error { message: "Could not load this section.".into(), retry: Some(Action::link("Try again", "?retry=example")) }, State::disabled("Editing is unavailable in this example.")] {
                section class="mui-state-example" aria-label=(state.as_str()) {
                    p class="mui-eyebrow" { (state.as_str()) }
                    (render(state))
                }
            }
        }
    }
}
/// A real Props-driven example of all four states for every operational block.
pub fn preview(slug: &str) -> Option<Markup> {
    use super::{action_row, attention_banner, record, shell, task, worklist};
    use crate::primitives::{badge, breadcrumb::BreadcrumbItem, facts_list::Fact};
    Some(match slug {
        "worklist-header" => examples(|state| {
            worklist::header::render(worklist::header::Props {
                state,
                title: "Next arrivals".into(),
                primary_action: Some(Action::link("View arrivals", "/blocks/worklist-grouped")),
                ..Default::default()
            })
        }),
        "worklist-grouped" => examples(|state| {
            worklist::grouped::render(worklist::grouped::Props {
                state,
                groups: vec![worklist::grouped::Group {
                    label: "Today".into(),
                    rows: vec![worklist::grouped::Row {
                        identity: "Maya Chen".into(),
                        facts: [
                            Fact::new("Room", "Juniper 04"),
                            Fact::new("Arrival", "11:30"),
                        ],
                        status: badge::Props {
                            label: "Arriving".into(),
                            ..Default::default()
                        },
                        action: Some(Action::link("View", "/blocks/record-timeline")),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            })
        }),
        "record-header" => examples(|state| {
            record::header::render(record::header::Props {
                state,
                title: "Maya Chen".into(),
                primary_action: Some(Action::link("Review", "/blocks/record-timeline")),
                ..Default::default()
            })
        }),
        "record-timeline" => examples(|state| {
            record::timeline::render(record::timeline::Props {
                state,
                milestones: vec![
                    record::timeline::Milestone {
                        label: "Arrival".into(),
                        date: "8 Sep".into(),
                        state: record::timeline::State::Current,
                        ..Default::default()
                    },
                    record::timeline::Milestone {
                        label: "Departure".into(),
                        date: "11 Sep".into(),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            })
        }),
        "record-money" => examples(|state| {
            record::money::render(record::money::Props {
                state,
                currency: "CAD".into(),
                total: record::money::Figure::new("Total", "$840"),
                paid: record::money::Figure::new("Paid", "$280"),
                due: record::money::Figure::new("Due", "$560"),
                ..Default::default()
            })
        }),
        "record-related-card" => examples(|state| {
            record::related_card::render(record::related_card::Props {
                state,
                title: "Maya Chen".into(),
                action: Some(Action::link("View guest", "/blocks/record-header")),
                ..Default::default()
            })
        }),
        "task-grid" => examples(|state| {
            task::grid::render(task::grid::Props {
                state,
                tasks: vec![task::grid::Task {
                    title: "Review arrival".into(),
                    description: "Confirm the room and arrival time.".into(),
                    action: Action::link("Review", "/blocks/worklist-grouped"),
                }],
                ..Default::default()
            })
        }),
        "attention-banner" => examples(|state| {
            attention_banner::render(attention_banner::Props {
                state,
                title: "Arrival needs review".into(),
                action: Some(Action::link("Review", "/blocks/record-header")),
                ..Default::default()
            })
        }),
        "action-row" => examples(|state| {
            action_row::render(action_row::Props {
                state,
                primary: Some(Action::submit("Save changes")),
                secondary: vec![Action::link("Cancel", "/blocks")],
                ..Default::default()
            })
        }),
        "shell-sidebar" => examples(|state| {
            shell::sidebar::render(shell::sidebar::Props {
                state,
                id: "state-shell-example".into(),
                embedded: true,
                collapsible: false,
                children: html! { p { "Workspace content" } a href="/blocks" { "View blocks" } },
                ..Default::default()
            })
        }),
        "shell-page-header" => examples(|state| {
            shell::page_header::render(shell::page_header::Props {
                state,
                breadcrumbs: vec![
                    BreadcrumbItem {
                        label: "Workspace".into(),
                        href: Some("/blocks".into()),
                    },
                    BreadcrumbItem {
                        label: "Arrivals".into(),
                        href: None,
                    },
                ],
                ..Default::default()
            })
        }),
        "shell-app-header" => examples(|state| {
            shell::app_header::render(shell::app_header::Props {
                state,
                brand: Some(html! { a href="/" { "Cedar Lodge" } }),
                ..Default::default()
            })
        }),
        "shell-app-footer" => examples(|state| {
            shell::app_footer::render(shell::app_footer::Props {
                state,
                line: Some("Cedar Lodge · Staff workspace".into()),
                ..Default::default()
            })
        }),
        _ => return None,
    })
}
