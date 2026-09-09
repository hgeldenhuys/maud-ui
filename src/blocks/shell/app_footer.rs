//! Optional app footer, rendered below the sidebar and content row.
use crate::blocks::action::Link;
use maud::{html, Markup};

#[derive(Clone, Debug, Default)]
pub struct Column {
    pub title: String,
    pub links: Vec<Link>,
}

#[derive(Clone, Debug, Default)]
pub struct Props {
    /// Explicit presentation state; Ready preserves the ordinary content.
    pub state: crate::blocks::state::State,
    pub links: Vec<Link>,
    pub line: Option<String>,
    pub columns: Vec<Column>,
    pub children: Markup,
}

pub fn render(mut props: Props) -> Markup {
    let state = std::mem::take(&mut props.state);
    crate::blocks::state::render(state, || render_ready(props))
}

fn render_ready(props: Props) -> Markup {
    if props.links.is_empty()
        && props.line.as_ref().is_none_or(|s| s.is_empty())
        && props.columns.is_empty()
        && props.children.0.is_empty()
    {
        return html! {};
    }
    html! {
        footer class="mui-app-footer" {
            @if !props.columns.is_empty() {
                div class="mui-app-footer__columns" {
                    @for column in props.columns {
                        div {
                            p class="mui-eyebrow" { (column.title) }
                            ul { @for link in column.links { li { a href=(link.href) { (link.label) } } } }
                        }
                    }
                }
            }
            div class="mui-app-footer__row" {
                @if let Some(line) = props.line { span { (line) } }
                @if !props.links.is_empty() {
                    nav aria-label="Footer" { @for link in props.links { a href=(link.href) { (link.label) } } }
                }
            }
            (props.children)
        }
    }
}

pub fn preview() -> Markup {
    render(Props {
        line: Some("Garden House · Workspace 0.9".into()),
        links: vec![
            Link {
                label: "Help".into(),
                href: "/getting-started".into(),
            },
            Link {
                label: "Components and blocks".into(),
                href: "/blocks".into(),
            },
        ],
        ..Default::default()
    })
}
