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
                        span class="mui-accordion__chevron" aria-hidden="true" { "\u{25BE}" }
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
            // FAQ — single open (shadcn-style)
            div {
                h3 class="mui-showcase__caption" { "FAQ" }
                (render(Props {
                    items: vec![
                        Item {
                            id: "faq-accessible".to_string(),
                            trigger: "Is it accessible?".to_string(),
                            content: html! {
                                p { "Yes. It adheres to the WAI-ARIA design pattern." }
                            },
                            open: true,
                        },
                        Item {
                            id: "faq-styled".to_string(),
                            trigger: "Is it styled?".to_string(),
                            content: html! {
                                p { "Yes. It comes with a default theme that matches shadcn." }
                            },
                            open: false,
                        },
                        Item {
                            id: "faq-animated".to_string(),
                            trigger: "Is it animated?".to_string(),
                            content: html! {
                                p { "Yes. JavaScript handles expand/collapse with ARIA state." }
                            },
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
                            id: "multi-acc-a".to_string(),
                            trigger: "Section A".to_string(),
                            content: html! { p { "Content for Section A with relevant information." } },
                            open: true,
                        },
                        Item {
                            id: "multi-acc-b".to_string(),
                            trigger: "Section B".to_string(),
                            content: html! { p { "Content for Section B with additional details." } },
                            open: false,
                        },
                        Item {
                            id: "multi-acc-c".to_string(),
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
