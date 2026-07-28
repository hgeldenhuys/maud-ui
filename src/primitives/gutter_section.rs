//! Gutter section — a mono uppercase section header over a content slot.
//! The unit the inspector gutter is built from: SESSION, LINEAGE, LINKED DOCS.

use maud::{html, Markup};

/// Gutter-section rendering properties.
#[derive(Debug, Clone, Default)]
pub struct Props {
    /// Section title — rendered mono, uppercased by CSS, 10px, subtle.
    pub title: String,
    /// The section body.
    pub content: Markup,
}

/// Render the gutter section.
pub fn render(props: Props) -> Markup {
    html! {
        section class="mui-gutter-section" {
            h3 class="mui-gutter-section__title" { (props.title) }
            div class="mui-gutter-section__body" { (props.content) }
        }
    }
}

/// Showcase the gutter section.
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            section {
                h2 { "Inspector gutter sections" }
                p.mui-showcase__caption { "A mono uppercase header (10px, letter-spaced, subtle) over any content." }
                div style="max-width:20rem;display:flex;flex-direction:column;gap:1.25rem;" {
                    (render(Props {
                        title: "Session".into(),
                        content: html! {
                            dl class="mui-facts" {
                                div class="mui-facts__row" { dt class="mui-facts__label" { "model" } dd class="mui-facts__value" { "sonnet-4.6" } }
                                div class="mui-facts__row" { dt class="mui-facts__label" { "turn" } dd class="mui-facts__value mui-facts__value--mono" { "12" } }
                            }
                        },
                    }))
                    (render(Props {
                        title: "Linked docs".into(),
                        content: html! {
                            div style="display:flex;flex-direction:column;gap:0.25rem;font-size:0.8125rem;" {
                                a href="#" style="color:var(--mui-accent-text);text-decoration:none;" { "auth-middleware RFC" }
                                a href="#" style="color:var(--mui-accent-text);text-decoration:none;" { "tower Layer contract" }
                            }
                        },
                    }))
                }
            }
        }
    }
}
