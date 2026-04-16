//! ScrollArea component — custom-styled scrollbar with auto-hide behavior.
use maud::{html, Markup};

/// ScrollArea rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// CSS value for max-height (e.g., "12rem", "200px")
    pub max_height: String,
    /// Unique identifier for the viewport
    pub id: String,
    /// Content to scroll
    pub children: Markup,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            max_height: "12rem".to_string(),
            id: "scroll-area-default".to_string(),
            children: html! {},
        }
    }
}

/// Render a ScrollArea with custom scrollbar and auto-hide behavior
pub fn render(props: Props) -> Markup {
    html! {
        div class="mui-scroll-area" data-mui="scroll-area" style={"max-height: " (props.max_height)} {
            div class="mui-scroll-area__viewport" id=(props.id) {
                (props.children)
            }
            div class="mui-scroll-area__scrollbar" aria-hidden="true" {
                div class="mui-scroll-area__thumb" {}
            }
        }
    }
}

/// Showcase the ScrollArea component
pub fn showcase() -> Markup {
    let content = html! {
        @for i in 0..20 {
            p.mui-scroll-area__demo-item { "Item " (i + 1) " — Lorem ipsum dolor sit amet" }
        }
    };

    html! {
        div.mui-showcase__grid {
            div {
                p.mui-showcase__caption { "Scrollable list (12rem)" }
                (render(Props {
                    max_height: "12rem".to_string(),
                    id: "demo-scroll-1".to_string(),
                    children: content,
                }))
            }
        }
    }
}
