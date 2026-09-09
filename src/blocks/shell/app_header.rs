//! Optional product masthead. Empty props emit no wrapper or landmark.
use crate::blocks::action::Link;
use maud::{html, Markup};

#[derive(Clone, Debug, Default)]
pub struct Props {
    /// Explicit presentation state; Ready preserves the ordinary content.
    pub state: crate::blocks::state::State,
    pub brand: Option<Markup>,
    pub links: Vec<Link>,
    pub current_href: Option<String>,
    pub actions: Option<Markup>,
    pub children: Markup,
}

pub fn render(mut props: Props) -> Markup {
    let state = std::mem::take(&mut props.state);
    crate::blocks::state::render(state, || render_ready(props))
}

fn render_ready(props: Props) -> Markup {
    if props.brand.as_ref().is_none_or(|m| m.0.is_empty())
        && props.links.is_empty()
        && props.actions.as_ref().is_none_or(|m| m.0.is_empty())
        && props.children.0.is_empty()
    {
        return html! {};
    }
    html! {
        header class="mui-app-header" {
            @if let Some(brand) = props.brand { div class="mui-app-header__brand" { (brand) } }
            @if !props.links.is_empty() {
                nav class="mui-app-header__links" aria-label="Product" {
                    @for link in props.links {
                        a href=(&link.href) aria-current=[(props.current_href.as_ref() == Some(&link.href)).then_some("page")] { (link.label) }
                    }
                }
            }
            @if let Some(actions) = props.actions { div class="mui-app-header__actions" { (actions) } }
            (props.children)
        }
    }
}

pub fn preview() -> Markup {
    render(Props {
        brand: Some(html! { a href="/" { "Garden House" } }),
        links: vec![
            Link {
                label: "Workspace".into(),
                href: "/blocks/shell-sidebar".into(),
            },
            Link {
                label: "Guide".into(),
                href: "/getting-started".into(),
            },
        ],
        current_href: Some("/blocks/shell-sidebar".into()),
        actions: Some(
            html! { a class="mui-btn mui-btn--ghost mui-btn--sm" href="/blocks/settings-profile" { "Sofia Davis" } },
        ),
        ..Default::default()
    })
}
