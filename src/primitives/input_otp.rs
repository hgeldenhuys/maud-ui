//! InputOTP component — one-time password input.
use maud::{html, Markup};

#[derive(Debug, Clone)]
pub struct Props {
    pub length: usize,
    pub name: String,
    pub id: String,
    pub disabled: bool,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            length: 6,
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
            h3 { "InputOTP — 6-digit" }
            (render(Props {
                length: 6,
                name: "otp".to_string(),
                id: "otp".to_string(),
                disabled: false,
            }))
        }
        div class="mui-showcase-section" {
            h3 { "InputOTP — 4-digit PIN" }
            (render(Props {
                length: 4,
                name: "pin".to_string(),
                id: "pin".to_string(),
                disabled: false,
            }))
        }
    }
}
