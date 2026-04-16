//! Progress component — determinate and indeterminate progress indicators.

use maud::{html, Markup};

/// Progress bar rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// Current progress value (0..=100, or use any value with max)
    pub value: u32,
    /// Maximum value (default 100)
    pub max: u32,
    /// ARIA label for the progress bar
    pub label: String,
    /// If true, renders as indeterminate (animated) progress
    pub indeterminate: bool,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            value: 0,
            max: 100,
            label: String::new(),
            indeterminate: false,
        }
    }
}

/// Render a progress bar with the given properties
pub fn render(props: Props) -> Markup {
    let pct = if props.max == 0 { 0 } else { (props.value * 100) / props.max };

    html! {
        @if props.indeterminate {
            div.mui-progress.mui-progress--indeterminate role="progressbar" aria-valuemin="0" aria-valuemax=(props.max) aria-label=(props.label) {
                div.mui-progress__bar {}
            }
        } @else {
            div.mui-progress role="progressbar" aria-valuenow=(props.value) aria-valuemin="0" aria-valuemax=(props.max) aria-label=(props.label) {
                div.mui-progress__bar style=(format!("width: {}%", pct)) {}
            }
        }
    }
}

/// Showcase all progress variants and use cases
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            // 0% progress
            div {
                p.mui-showcase__label { "0%" }
                (render(Props {
                    value: 0,
                    max: 100,
                    label: "Zero progress".into(),
                    indeterminate: false,
                }))
            }

            // 25% progress
            div {
                p.mui-showcase__label { "25%" }
                (render(Props {
                    value: 25,
                    max: 100,
                    label: "Quarter progress".into(),
                    indeterminate: false,
                }))
            }

            // 50% progress
            div {
                p.mui-showcase__label { "50%" }
                (render(Props {
                    value: 50,
                    max: 100,
                    label: "Half progress".into(),
                    indeterminate: false,
                }))
            }

            // 75% progress
            div {
                p.mui-showcase__label { "75%" }
                (render(Props {
                    value: 75,
                    max: 100,
                    label: "Three-quarter progress".into(),
                    indeterminate: false,
                }))
            }

            // 100% progress
            div {
                p.mui-showcase__label { "100%" }
                (render(Props {
                    value: 100,
                    max: 100,
                    label: "Complete progress".into(),
                    indeterminate: false,
                }))
            }

            // Indeterminate
            div {
                p.mui-showcase__label { "Indeterminate" }
                (render(Props {
                    value: 0,
                    max: 100,
                    label: "Loading...".into(),
                    indeterminate: true,
                }))
            }
        }
    }
}
