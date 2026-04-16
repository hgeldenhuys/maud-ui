//! Collapsible component — expandable section with trigger button and animated content.

use maud::{html, Markup};

/// Collapsible rendering properties
#[derive(Clone, Debug)]
pub struct Props {
    /// Label text displayed in the trigger button
    pub trigger_label: String,
    /// Markup content displayed when expanded
    pub content: Markup,
    /// Initial open state (default false)
    pub open: bool,
    /// Unique identifier for aria-controls and content linking
    pub id: String,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            trigger_label: "Toggle".to_string(),
            content: html! {},
            open: false,
            id: "collapsible".to_string(),
        }
    }
}

/// Render a single collapsible with the given properties
pub fn render(props: Props) -> Markup {
    let content_id = format!("{}-content", props.id);
    let aria_expanded = if props.open { "true" } else { "false" };

    html! {
        div class="mui-collapsible" data-mui="collapsible" {
            button type="button"
                class="mui-collapsible__trigger"
                aria-expanded=(aria_expanded)
                aria-controls=(content_id)
            {
                span class="mui-collapsible__chevron" aria-hidden="true" { "▸" }
                span class="mui-collapsible__label" { (props.trigger_label) }
            }
            div class="mui-collapsible__content"
                id=(content_id)
                hidden[!props.open]
            {
                (props.content)
            }
        }
    }
}

/// Showcase all collapsible use cases
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            // Closed collapsible
            (render(Props {
                trigger_label: "What is maud-ui?".to_string(),
                content: html! { p { "Headless accessible UI components for maud + htmx." } },
                open: false,
                id: "demo-col-1".to_string(),
            }))

            // Open collapsible
            (render(Props {
                trigger_label: "Is it production-ready?".to_string(),
                content: html! { p { "Currently in active development. APIs may change." } },
                open: true,
                id: "demo-col-2".to_string(),
            }))

            // Nested content with list
            (render(Props {
                trigger_label: "Show me more".to_string(),
                content: html! {
                    ul {
                        li { "Item A" }
                        li { "Item B" }
                    }
                },
                open: false,
                id: "demo-col-3".to_string(),
            }))
        }
    }
}
