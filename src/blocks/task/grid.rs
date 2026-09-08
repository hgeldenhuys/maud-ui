//! Three-column launcher with one named action per task; one column on phones.
use crate::blocks::action::{Action, Heading};
use maud::{html, Markup};

#[derive(Clone, Debug)]
pub struct Task {
    pub title: String,
    pub description: String,
    pub action: Action,
}

#[derive(Clone, Debug)]
pub struct Props {
    pub tasks: Vec<Task>,
    pub aria_label: String,
    pub heading: Heading,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            tasks: vec![],
            aria_label: "Available tasks".into(),
            heading: Heading::H3,
        }
    }
}

pub fn render(props: Props) -> Markup {
    html! {
        ul class="mui-task-grid" aria-label=(props.aria_label) {
            @for task in props.tasks {
                li class="mui-task-grid__card" {
                    (props.heading.render(&task.title, "mui-task-grid__title"))
                    p class="mui-task-grid__description" title=(&task.description) { (task.description) }
                    div class="mui-task-grid__action" { (task.action.render(false)) }
                }
            }
        }
    }
}

pub fn preview() -> Markup {
    render(Props {
        tasks: [
            (
                "Check in a guest",
                "Confirm the reservation and assign a room.",
                "Start check-in",
            ),
            (
                "Create a reservation",
                "Find availability for a new stay.",
                "New reservation",
            ),
            (
                "Review today's arrivals",
                "Resolve missing details before guests arrive.",
                "Review arrivals",
            ),
        ]
        .into_iter()
        .map(|(title, description, label)| Task {
            title: title.into(),
            description: description.into(),
            action: Action::link(label, "/blocks/worklist-header"),
        })
        .collect(),
        ..Default::default()
    })
}
