//! Facts list — mono subtle labels paired with foreground values, on a 4px
//! rhythm. The key/value register used by the empty-session card and the
//! inspector's SESSION section.

use maud::{html, Markup};

/// One label/value pair.
#[derive(Debug, Clone, Default)]
pub struct Fact {
    /// Mono, subtle label — `model`, `context`, `worktree`.
    pub label: String,
    /// Foreground value.
    pub value: String,
    /// Render the value in the mono face (for shas, counts, paths).
    pub mono: bool,
}

impl Fact {
    /// A fact with a sans value.
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self { label: label.into(), value: value.into(), mono: false }
    }
    /// A fact whose value is machine-register text (mono face).
    pub fn mono(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self { label: label.into(), value: value.into(), mono: true }
    }
}

/// Facts-list rendering properties.
#[derive(Debug, Clone, Default)]
pub struct Props {
    /// The rows, top to bottom.
    pub facts: Vec<Fact>,
}

/// Render the facts list as a `<dl>`.
pub fn render(props: Props) -> Markup {
    html! {
        dl class="mui-facts" {
            @for fact in &props.facts {
                div class="mui-facts__row" {
                    dt class="mui-facts__label" { (fact.label) }
                    dd class=(if fact.mono { "mui-facts__value mui-facts__value--mono" } else { "mui-facts__value" }) {
                        (fact.value)
                    }
                }
            }
        }
    }
}

/// Showcase the facts list.
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            section {
                h2 { "Session facts" }
                p.mui-showcase__caption { "Mono subtle labels, foreground values, rows on the 4px rhythm." }
                div style="max-width:22rem;" {
                    (render(Props {
                        facts: vec![
                            Fact::new("model", "sonnet-4.6"),
                            Fact::mono("context", "18.4k / 200k"),
                            Fact::mono("spend", "$1.84"),
                            Fact::mono("worktree", "refactor-auth-middleware"),
                            Fact::new("started", "41m ago"),
                        ],
                    }))
                }
            }
        }
    }
}
