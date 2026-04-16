//! InputGroup component.
use maud::{html, Markup};

pub struct InputGroupProps {
    pub prefix: Option<Markup>,
    pub suffix: Option<Markup>,
    pub children: Markup,
}

pub fn render(props: InputGroupProps) -> Markup {
    html! {
        div.mui-input-group {
            @if let Some(prefix) = props.prefix {
                span.mui-input-group__prefix { (prefix) }
            }
            (props.children)
            @if let Some(suffix) = props.suffix {
                span.mui-input-group__suffix { (suffix) }
            }
        }
    }
}

pub fn showcase() -> Markup {
    html! {
        div {
            h3 { "Input with $ prefix" }
            (render(InputGroupProps {
                prefix: Some(html! { "$" }),
                suffix: None,
                children: html! { input type="text" placeholder="Amount" {} },
            }))

            h3 { "Input with .com suffix" }
            (render(InputGroupProps {
                prefix: None,
                suffix: Some(html! { ".com" }),
                children: html! { input type="text" placeholder="Domain" {} },
            }))

            h3 { "Input with https:// prefix and Go button" }
            (render(InputGroupProps {
                prefix: Some(html! { "https://" }),
                suffix: Some(html! { button { "Go" } }),
                children: html! { input type="text" placeholder="URL" {} },
            }))

            h3 { "Email with @ prefix" }
            (render(InputGroupProps {
                prefix: Some(html! { "@" }),
                suffix: None,
                children: html! { input type="email" placeholder="username" {} },
            }))
        }
    }
}
