//! First-record empty state. Do not use it for errors, loading or filtered results.
use crate::blocks::action::{Heading, Link};
use maud::{html, Markup};

#[derive(Clone, Debug)]
pub struct Props {
    pub title: String,
    pub description: Option<String>,
    /// Only provide creation when the caller has admitted that action.
    pub create: Option<Link>,
    pub heading: Heading,
}
impl Default for Props {
    fn default() -> Self {
        Self {
            title: "No records yet".into(),
            description: None,
            create: None,
            heading: Heading::H2,
        }
    }
}
pub fn render(props: Props) -> Markup {
    html! {
        div class="mui-worklist-empty" {
            (props.heading.render(&props.title, "mui-worklist-empty__title"))
            @if let Some(description) = props.description { p { (description) } }
            @if let Some(create) = props.create { a class="mui-btn mui-btn--primary" href=(create.href) { (create.label) } }
        }
    }
}
pub fn preview() -> Markup {
    super::super::kit_preview(|_| {
        render(Props {
            title: "No notes yet".into(),
            description: Some("Keep arrival requests and handover details together.".into()),
            create: Some(Link {
                label: "Create your first note".into(),
                href: "/blocks/record-page".into(),
            }),
            ..Default::default()
        })
    })
}
