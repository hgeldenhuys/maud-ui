//! Attention pill — a small accent-tinted pill that says "this wants a look".
//!
//! Two states, one component: the accent form (accent-12% fill, accent dot,
//! accent text) draws the eye; the muted zero-state form spends no accent at
//! all, so an empty count does not sit there glowing.

use maud::{html, Markup};

/// Attention-pill rendering properties.
#[derive(Debug, Clone, Default)]
pub struct Props {
    /// Caller-supplied label, e.g. `2 waiting on you`.
    pub label: String,
    /// Render the muted zero-state (no accent) instead of the accent form.
    pub muted: bool,
}

/// Render the attention pill.
pub fn render(props: Props) -> Markup {
    let class = if props.muted {
        "mui-attention-pill mui-attention-pill--muted"
    } else {
        "mui-attention-pill"
    };
    html! {
        span class=(class) {
            span class="mui-attention-pill__dot" aria-hidden="true" {}
            span class="mui-attention-pill__label" { (props.label) }
        }
    }
}

/// Showcase both states.
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            section {
                h2 { "Wants attention" }
                p.mui-showcase__caption { "Accent-12% fill, accent dot, accent text — the one thing on the row the eye should land on." }
                div style="display:flex;gap:0.75rem;flex-wrap:wrap;" {
                    (render(Props { label: "2 waiting on you".into(), ..Default::default() }))
                    (render(Props { label: "review requested".into(), ..Default::default() }))
                }
            }
            section {
                h2 { "Zero state — muted" }
                p.mui-showcase__caption { "Nothing waiting: the same pill spends no accent, so it recedes instead of glowing at an empty count." }
                div style="display:flex;gap:0.75rem;flex-wrap:wrap;" {
                    (render(Props { label: "all caught up".into(), muted: true }))
                    (render(Props { label: "0 waiting".into(), muted: true }))
                }
            }
        }
    }
}
