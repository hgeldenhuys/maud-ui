//! Collapsible component — minimal expandable section with trigger and animated content.
//! Unlike accordion, this is a standalone toggle — no card wrapping, just trigger + content.

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
    /// Render as a native `<details>`/`<summary>` pair instead of the
    /// scripted button. Toggling then costs no JavaScript — the section still
    /// opens with scripts blocked, failed, or not yet loaded — at the price of
    /// the height animation. Prefer it for content-only disclosures.
    pub native: bool,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            trigger_label: "Toggle".to_string(),
            content: html! {},
            open: false,
            id: "collapsible".to_string(),
            native: false,
        }
    }
}

/// Render a single collapsible with the given properties
pub fn render(props: Props) -> Markup {
    let content_id = format!("{}-content", props.id);
    let aria_expanded = if props.open { "true" } else { "false" };

    if props.native {
        return html! {
            details class="mui-collapsible mui-collapsible--native"
                data-mui="collapsible"
                id=(props.id)
                open[props.open]
            {
                summary class="mui-collapsible__trigger" {
                    span class="mui-collapsible__label" { (props.trigger_label) }
                    span class="mui-collapsible__chevron" aria-hidden="true" { "\u{25BE}" }
                }
                div class="mui-collapsible__content" id=(content_id) {
                    (props.content)
                }
            }
        };
    }

    html! {
        div class="mui-collapsible" data-mui="collapsible" {
            button type="button"
                class="mui-collapsible__trigger"
                aria-expanded=(aria_expanded)
                aria-controls=(content_id)
            {
                span class="mui-collapsible__label" { (props.trigger_label) }
                span class="mui-collapsible__chevron" aria-hidden="true" { "\u{25BE}" }
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
                ..Default::default()
            }))

            // Open collapsible
            (render(Props {
                trigger_label: "Is it production-ready?".to_string(),
                content: html! { p { "Currently in active development. APIs may change." } },
                open: true,
                id: "demo-col-2".to_string(),
                ..Default::default()
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
                ..Default::default()
            }))

            // Native <details> — toggles with JavaScript disabled
            (render(Props {
                trigger_label: "Works with JS disabled".to_string(),
                content: html! { p { "Rendered as <details>/<summary>; the browser owns the toggle." } },
                open: false,
                id: "demo-col-4".to_string(),
                native: true,
            }))
        }
    }
}
