//! Accordion component — multiple collapsibles with optional single-open mode.

use maud::{html, Markup};

/// Individual accordion item
#[derive(Clone, Debug)]
pub struct Item {
    /// Unique identifier for aria-controls and content linking
    pub id: String,
    /// Label text displayed in the trigger button
    pub trigger: String,
    /// Markup content displayed when expanded
    pub content: Markup,
    /// Initial open state (default false)
    pub open: bool,
}

/// Accordion rendering properties
#[derive(Clone, Debug)]
pub struct Props {
    /// Array of accordion items
    pub items: Vec<Item>,
    /// If true, multiple items can be open; if false, only one item at a time
    pub multiple: bool,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            items: vec![],
            multiple: false,
        }
    }
}

/// Render an accordion with the given properties
pub fn render(props: Props) -> Markup {
    let multiple_attr = if props.multiple { "true" } else { "false" };

    html! {
        div class="mui-accordion" data-mui="accordion" data-multiple=(multiple_attr) {
            @for item in props.items {
                div class="mui-accordion__item" {
                    button type="button"
                        class="mui-accordion__trigger"
                        id=(format!("{}-trigger", item.id))
                        aria-expanded=(if item.open { "true" } else { "false" })
                        aria-controls=(format!("{}-content", item.id))
                    {
                        span class="mui-accordion__label" { (item.trigger) }
                        span class="mui-accordion__chevron" aria-hidden="true" {
                            // Down-pointing chevron SVG — rotates 180deg when expanded
                            (maud::PreEscaped(r#"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6"/></svg>"#))
                        }
                    }
                    div class="mui-accordion__content"
                        id=(format!("{}-content", item.id))
                        aria-labelledby=(format!("{}-trigger", item.id))
                        role="region"
                        hidden[!item.open]
                    {
                        (item.content)
                    }
                }
            }
        }
    }
}

/// Showcase all accordion use cases
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            // Single (default) — only one item can be open at a time
            div {
                h3 class="mui-showcase__caption" { "Single (default)" }
                (render(Props {
                    items: vec![
                        Item {
                            id: "demo-acc-1-q1".to_string(),
                            trigger: "Q1".to_string(),
                            content: html! { p { "This is the first question answer with some detailed content." } },
                            open: true,
                        },
                        Item {
                            id: "demo-acc-1-q2".to_string(),
                            trigger: "Q2".to_string(),
                            content: html! { p { "This is the second question answer with more details." } },
                            open: false,
                        },
                        Item {
                            id: "demo-acc-1-q3".to_string(),
                            trigger: "Q3".to_string(),
                            content: html! { p { "This is the third question answer explaining further." } },
                            open: false,
                        },
                    ],
                    multiple: false,
                }))
            }

            // Multiple — multiple items can be open simultaneously
            div {
                h3 class="mui-showcase__caption" { "Multiple" }
                (render(Props {
                    items: vec![
                        Item {
                            id: "demo-acc-2-a".to_string(),
                            trigger: "Section A".to_string(),
                            content: html! { p { "Content for Section A with relevant information." } },
                            open: true,
                        },
                        Item {
                            id: "demo-acc-2-b".to_string(),
                            trigger: "Section B".to_string(),
                            content: html! { p { "Content for Section B with additional details." } },
                            open: false,
                        },
                        Item {
                            id: "demo-acc-2-c".to_string(),
                            trigger: "Section C".to_string(),
                            content: html! { p { "Content for Section C with more information." } },
                            open: true,
                        },
                    ],
                    multiple: true,
                }))
            }
        }
    }
}
