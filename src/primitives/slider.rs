//! Slider component — range input with custom styling and progressive enhancement.

use maud::{html, Markup};

/// Slider rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// HTML name attribute for form submission
    pub name: String,
    /// HTML id attribute for label linkage
    pub id: String,
    /// Current value
    pub value: f64,
    /// Minimum value (default 0.0)
    pub min: f64,
    /// Maximum value (default 100.0)
    pub max: f64,
    /// Step increment (default 1.0)
    pub step: f64,
    /// Whether the slider is disabled
    pub disabled: bool,
    /// ARIA label for the slider
    pub label: String,
    /// Whether to show the current value label
    pub show_value: bool,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            name: String::new(),
            id: String::new(),
            value: 50.0,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            disabled: false,
            label: String::new(),
            show_value: false,
        }
    }
}

/// Render a slider with the given properties
pub fn render(props: Props) -> Markup {
    let pct = if props.max != props.min {
        ((props.value - props.min) / (props.max - props.min)) * 100.0
    } else {
        0.0
    };

    html! {
        @if props.disabled {
            div.mui-slider data-mui="slider" data-min=(props.min) data-max=(props.max) data-step=(props.step) data-disabled="true" {
                div.mui-slider__track {
                    div.mui-slider__fill style=(format!("width: {}%", pct)) aria-hidden="true" {}
                    div.mui-slider__thumb
                        role="slider"
                        tabindex="0"
                        aria-valuenow=(props.value)
                        aria-valuemin=(props.min)
                        aria-valuemax=(props.max)
                        aria-label=(props.label)
                        aria-disabled="true"
                        style=(format!("left: {}%", pct)) {}
                }
                input.mui-slider__native
                    type="range"
                    name=(props.name)
                    id=(props.id)
                    value=(props.value)
                    min=(props.min)
                    max=(props.max)
                    step=(props.step)
                    disabled
                    aria-hidden="true"
                    tabindex="-1";
                @if props.show_value {
                    span.mui-slider__value { (format!("{}", props.value)) }
                }
            }
        } @else {
            div.mui-slider data-mui="slider" data-min=(props.min) data-max=(props.max) data-step=(props.step) {
                div.mui-slider__track {
                    div.mui-slider__fill style=(format!("width: {}%", pct)) aria-hidden="true" {}
                    div.mui-slider__thumb
                        role="slider"
                        tabindex="0"
                        aria-valuenow=(props.value)
                        aria-valuemin=(props.min)
                        aria-valuemax=(props.max)
                        aria-label=(props.label)
                        style=(format!("left: {}%", pct)) {}
                }
                input.mui-slider__native
                    type="range"
                    name=(props.name)
                    id=(props.id)
                    value=(props.value)
                    min=(props.min)
                    max=(props.max)
                    step=(props.step)
                    aria-hidden="true"
                    tabindex="-1";
                @if props.show_value {
                    span.mui-slider__value { (format!("{}", props.value)) }
                }
            }
        }
    }
}

/// Showcase all slider variants and use cases
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            // Default (0–100)
            div {
                p.mui-showcase__label { "Default (0–100)" }
                (render(Props {
                    name: "slider-default".into(),
                    id: "slider-default".into(),
                    value: 40.0,
                    min: 0.0,
                    max: 100.0,
                    step: 1.0,
                    label: "Default slider".into(),
                    show_value: true,
                    ..Default::default()
                }))
            }

            // Custom range (0–10, step 0.5)
            div {
                p.mui-showcase__label { "Custom range (0–10, step 0.5)" }
                (render(Props {
                    name: "slider-custom".into(),
                    id: "slider-custom".into(),
                    value: 3.5,
                    min: 0.0,
                    max: 10.0,
                    step: 0.5,
                    label: "Custom range slider".into(),
                    show_value: true,
                    ..Default::default()
                }))
            }

            // Disabled
            div {
                p.mui-showcase__label { "Disabled" }
                (render(Props {
                    name: "slider-disabled".into(),
                    id: "slider-disabled".into(),
                    value: 60.0,
                    min: 0.0,
                    max: 100.0,
                    step: 1.0,
                    disabled: true,
                    label: "Disabled slider".into(),
                    show_value: true,
                    ..Default::default()
                }))
            }
        }
    }
}
