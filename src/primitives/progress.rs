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
            // File upload in progress
            div {
                p.mui-showcase__caption { "File upload" }
                div style="display:flex;flex-direction:column;gap:0.5rem" {
                    div style="display:flex;justify-content:space-between;align-items:center" {
                        span style="font-size:0.875rem;color:var(--mui-text)" { "Uploading file..." }
                        span style="font-size:0.875rem;font-weight:500;color:var(--mui-text)" { "65%" }
                    }
                    (render(Props {
                        value: 65,
                        max: 100,
                        label: "Uploading file, 65 percent".into(),
                        indeterminate: false,
                    }))
                    span style="font-size:0.75rem;color:var(--mui-text-muted)" { "report-2026.pdf — 3.2 MB of 4.9 MB" }
                }
            }

            // Stepped progress
            div {
                p.mui-showcase__caption { "Stepped progress" }
                div style="display:flex;flex-direction:column;gap:0.5rem" {
                    div style="display:flex;justify-content:space-between;align-items:center" {
                        span style="font-size:0.875rem;font-weight:500;color:var(--mui-text)" { "Step 2 of 4" }
                        span style="font-size:0.75rem;color:var(--mui-text-muted)" { "Account details" }
                    }
                    (render(Props {
                        value: 50,
                        max: 100,
                        label: "Step 2 of 4, account details".into(),
                        indeterminate: false,
                    }))
                }
            }

            // Indeterminate — processing
            div {
                p.mui-showcase__caption { "Indeterminate" }
                div style="display:flex;flex-direction:column;gap:0.5rem" {
                    span style="font-size:0.875rem;color:var(--mui-text-muted)" { "Processing your request..." }
                    (render(Props {
                        value: 0,
                        max: 100,
                        label: "Processing request".into(),
                        indeterminate: true,
                    }))
                }
            }

            // Download complete (100%)
            div {
                p.mui-showcase__caption { "Download complete" }
                div style="display:flex;flex-direction:column;gap:0.5rem" {
                    div style="display:flex;justify-content:space-between;align-items:center" {
                        span style="font-size:0.875rem;color:var(--mui-text)" { "Download complete" }
                        span style="font-size:0.875rem;font-weight:500;color:var(--mui-accent,var(--mui-text))" { "100%" }
                    }
                    (render(Props {
                        value: 100,
                        max: 100,
                        label: "Download complete".into(),
                        indeterminate: false,
                    }))
                    span style="font-size:0.75rem;color:var(--mui-text-muted)" { "dataset.csv — 12.7 MB" }
                }
            }
        }
    }
}
