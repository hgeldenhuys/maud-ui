//! InputOTP component — one-time password input with grouped slots.
use maud::{html, Markup};

#[derive(Debug, Clone)]
pub struct Props {
    /// Total number of digits
    pub length: usize,
    /// Group size for visual separator (e.g., 3 for a 3-3 layout on 6 digits)
    pub group_size: usize,
    pub name: String,
    pub id: String,
    pub disabled: bool,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            length: 6,
            group_size: 3,
            name: "otp".to_string(),
            id: "otp".to_string(),
            disabled: false,
        }
    }
}

pub fn render(props: Props) -> Markup {
    html! {
        div class="mui-input-otp" data-mui="input-otp" role="group" aria-label="One-time password" {
            @for i in 0..props.length {
                // Insert group separator between groups
                @if i > 0 && props.group_size > 0 && i % props.group_size == 0 {
                    div class="mui-input-otp__separator" role="separator" {
                        div class="mui-input-otp__separator-dash" {}
                    }
                }
                input
                    type="text"
                    class="mui-input-otp__slot"
                    maxlength="1"
                    inputmode="numeric"
                    pattern="[0-9]"
                    autocomplete="one-time-code"
                    aria-label=(format!("Digit {}", i + 1))
                    name=(format!("{}-{}", props.name, i))
                    id=(format!("{}-{}", props.id, i))
                    disabled[props.disabled]
                ;
            }
            input
                type="hidden"
                name=(props.name.clone())
                class="mui-input-otp__value"
            ;
        }
    }
}

pub fn showcase() -> Markup {
    html! {
        div class="mui-showcase-section" {
            h3 { "InputOTP — 6-digit (3+3 groups)" }
            (render(Props {
                length: 6,
                group_size: 3,
                name: "otp6".to_string(),
                id: "otp6".to_string(),
                disabled: false,
            }))
        }
        div class="mui-showcase-section" {
            h3 { "InputOTP — 4-digit PIN (no groups)" }
            (render(Props {
                length: 4,
                group_size: 0,
                name: "pin".to_string(),
                id: "pin".to_string(),
                disabled: false,
            }))
        }
        div class="mui-showcase-section" {
            h3 { "InputOTP — 6-digit (2+2+2 groups)" }
            (render(Props {
                length: 6,
                group_size: 2,
                name: "otp-2s".to_string(),
                id: "otp-2s".to_string(),
                disabled: false,
            }))
        }
        div class="mui-showcase-section" {
            h3 { "InputOTP — Disabled" }
            (render(Props {
                length: 6,
                group_size: 3,
                name: "otp-disabled".to_string(),
                id: "otp-disabled".to_string(),
                disabled: true,
            }))
        }
    }
}
