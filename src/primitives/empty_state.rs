//! EmptyState component.
use maud::{html, Markup};

pub fn render() -> Markup {
    html! { p.mui-placeholder { "TODO: empty_state" } }
}

pub fn showcase() -> Markup {
    render()
}
