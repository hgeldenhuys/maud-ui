//! Turn progress strip — a 2px full-width bar showing how far a turn has run.
//!
//! A single hard-edged two-stop gradient: accent from 0 to N%, then
//! accent-10% for the remainder. No animation, no track chrome — it is a hair
//! of colour at the top of the executing composer, not a widget.

use maud::{html, Markup};

/// Turn-progress rendering properties.
#[derive(Debug, Clone, Copy, Default)]
pub struct Props {
    /// Percent complete, 0–100. Clamped on render.
    pub percent: u8,
}

/// Render the progress strip.
pub fn render(props: Props) -> Markup {
    let pct = props.percent.min(100);
    html! {
        div class="mui-turn-progress"
            role="progressbar"
            aria-valuenow=(pct)
            aria-valuemin="0"
            aria-valuemax="100"
            style=(format!("--mui-turn-pct:{pct}%")) {}
    }
}

/// Showcase the strip at several fills.
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            section {
                h2 { "Turn progress" }
                p.mui-showcase__caption { "A 2px hard-gradient strip — accent to N%, accent-10% after. Rendered above the executing composer." }
                div style="display:flex;flex-direction:column;gap:1.25rem;max-width:32rem;" {
                    @for pct in [8u8, 34, 62, 100] {
                        div {
                            (render(Props { percent: pct }))
                            div style="margin-top:0.375rem;font-family:var(--mui-font-mono);font-size:0.6875rem;color:var(--mui-text-subtle);" { (format!("{pct}%")) }
                        }
                    }
                }
            }
        }
    }
}
