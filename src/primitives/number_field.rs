//! Number field component — composite control with input flanked by ± buttons.

use maud::{html, Markup};

/// Number field rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// HTML name attribute for form submission
    pub name: String,
    /// Current numeric value
    pub value: f64,
    /// Minimum value (optional constraint)
    pub min: Option<f64>,
    /// Maximum value (optional constraint)
    pub max: Option<f64>,
    /// Increment/decrement step (default 1.0)
    pub step: f64,
    /// Whether the field is disabled
    pub disabled: bool,
    /// Accessibility label for the group
    pub label: String,
    /// HTML id attribute for the input
    pub id: String,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            name: String::new(),
            value: 0.0,
            min: None,
            max: None,
            step: 1.0,
            disabled: false,
            label: String::new(),
            id: String::new(),
        }
    }
}

/// Render a single number field with the given properties
pub fn render(props: Props) -> Markup {
    html! {
        div class="mui-number-field" role="group" aria-label=(props.label) data-disabled[props.disabled] {
            button type="button" class="mui-number-field__btn mui-number-field__btn--decrement" tabindex="-1" aria-label="Decrement" data-mui="number-field-dec" disabled[props.disabled] { "−" }
            input type="number" class="mui-number-field__input" name=(props.name) id=(props.id) value=(props.value)
                  step=(props.step) min=[props.min] max=[props.max] disabled[props.disabled];
            button type="button" class="mui-number-field__btn mui-number-field__btn--increment" tabindex="-1" aria-label="Increment" data-mui="number-field-inc" disabled[props.disabled] { "+" }
        }
    }
}

/// Showcase all number field variants
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            // Default (step 1, no bounds)
            div {
                p.mui-showcase__caption { "Default (step 1, no bounds)" }
                (render(Props {
                    name: "demo-nf-1".into(),
                    value: 10.0,
                    step: 1.0,
                    id: "demo-nf-1".into(),
                    label: "Default number field".into(),
                    ..Default::default()
                }))
            }

            // Bounded (0..100)
            div {
                p.mui-showcase__caption { "Bounded (0..100)" }
                (render(Props {
                    name: "demo-nf-2".into(),
                    value: 50.0,
                    min: Some(0.0),
                    max: Some(100.0),
                    id: "demo-nf-2".into(),
                    label: "Bounded number field".into(),
                    ..Default::default()
                }))
            }

            // Decimal (step 0.1)
            div {
                p.mui-showcase__caption { "Decimal (step 0.1)" }
                (render(Props {
                    name: "demo-nf-3".into(),
                    value: 1.5,
                    step: 0.1,
                    id: "demo-nf-3".into(),
                    label: "Decimal number field".into(),
                    ..Default::default()
                }))
            }

            // Disabled
            div {
                p.mui-showcase__caption { "Disabled" }
                (render(Props {
                    name: "demo-nf-4".into(),
                    value: 42.0,
                    disabled: true,
                    id: "demo-nf-4".into(),
                    label: "Disabled number field".into(),
                    ..Default::default()
                }))
            }
        }
    }
}
