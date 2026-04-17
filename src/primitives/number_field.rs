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
        div class="mui-number-field" role="group" aria-label=(props.label) aria-disabled[props.disabled] {
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
            // Cart quantity
            div {
                p.mui-showcase__caption { "Cart \u{2014} Wireless Headphones" }
                div style="display:flex;flex-direction:column;gap:0.5rem;max-width:20rem;" {
                    div style="display:flex;justify-content:space-between;align-items:center;" {
                        div {
                            p style="font-size:0.875rem;font-weight:500;margin:0;" { "Aurora Wireless Headphones" }
                            p style="font-size:0.75rem;color:var(--mui-text-muted);margin:0.125rem 0 0;" { "$149 each" }
                        }
                        span style="font-size:0.875rem;font-weight:600;" { "$298" }
                    }
                    label for="cart-qty" style="font-size:0.75rem;color:var(--mui-text-muted);" { "Quantity" }
                    (render(Props {
                        name: "cart_quantity".into(),
                        value: 2.0,
                        min: Some(1.0),
                        max: Some(10.0),
                        step: 1.0,
                        id: "cart-qty".into(),
                        label: "Cart quantity".into(),
                        ..Default::default()
                    }))
                    p style="font-size:0.75rem;color:var(--mui-text-muted);margin:0;" { "Max 10 per order" }
                }
            }

            // Age field
            div {
                p.mui-showcase__caption { "Account \u{2014} Age verification" }
                div style="display:flex;flex-direction:column;gap:0.5rem;max-width:20rem;" {
                    label for="signup-age" style="font-size:0.875rem;font-weight:500;" { "Age" }
                    (render(Props {
                        name: "age".into(),
                        value: 24.0,
                        min: Some(13.0),
                        max: Some(120.0),
                        step: 1.0,
                        id: "signup-age".into(),
                        label: "Age".into(),
                        ..Default::default()
                    }))
                    p style="font-size:0.75rem;color:var(--mui-text-muted);margin:0;" {
                        "You must be at least 13 to create an account."
                    }
                }
            }

            // Recipe servings
            div {
                p.mui-showcase__caption { "Recipe \u{2014} Tomato Basil Risotto" }
                div style="display:flex;flex-direction:column;gap:0.5rem;max-width:20rem;" {
                    label for="recipe-servings" style="font-size:0.875rem;font-weight:500;" { "Servings" }
                    (render(Props {
                        name: "servings".into(),
                        value: 4.0,
                        min: Some(1.0),
                        max: Some(12.0),
                        step: 1.0,
                        id: "recipe-servings".into(),
                        label: "Servings".into(),
                        ..Default::default()
                    }))
                    p style="font-size:0.75rem;color:var(--mui-text-muted);margin:0;" {
                        "Adjust to scale ingredients (1\u{2013}12 people)."
                    }
                }
            }

            // Disabled — backorder
            div {
                p.mui-showcase__caption { "Backordered \u{2014} quantity locked" }
                div style="display:flex;flex-direction:column;gap:0.5rem;max-width:20rem;" {
                    label for="backorder-qty" style="font-size:0.875rem;font-weight:500;color:var(--mui-text-muted);" {
                        "Quantity (reserved)"
                    }
                    (render(Props {
                        name: "backorder".into(),
                        value: 1.0,
                        min: Some(1.0),
                        max: Some(1.0),
                        step: 1.0,
                        disabled: true,
                        id: "backorder-qty".into(),
                        label: "Backordered quantity".into(),
                        ..Default::default()
                    }))
                    p style="font-size:0.75rem;color:var(--mui-text-muted);margin:0;" {
                        "Ships when restocked on April 28."
                    }
                }
            }
        }
    }
}
