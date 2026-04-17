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
        div.mui-showcase__grid {
            // Email verification — 6-digit
            section {
                h2 { "Verify your email" }
                p.mui-showcase__caption { "We sent a code to sofia@example.com" }
                div style="display:flex;flex-direction:column;gap:0.75rem;max-width:22rem;" {
                    (render(Props {
                        length: 6,
                        group_size: 3,
                        name: "verify-email-code".to_string(),
                        id: "verify-email-code".to_string(),
                        disabled: false,
                    }))
                    div style="display:flex;justify-content:space-between;align-items:center;font-size:0.8125rem;" {
                        span style="color:var(--mui-text-muted);" { "Code expires in 9:42" }
                        button type="button" style="background:transparent;border:0;padding:0;color:var(--mui-text);font-weight:500;cursor:pointer;text-decoration:underline;font-size:0.8125rem;" {
                            "Resend"
                        }
                    }
                }
            }

            // Transaction PIN — 4-digit
            section {
                h2 { "Enter PIN" }
                p.mui-showcase__caption { "Your transaction PIN" }
                div style="display:flex;flex-direction:column;gap:0.5rem;max-width:18rem;" {
                    (render(Props {
                        length: 4,
                        group_size: 0,
                        name: "transaction-pin".to_string(),
                        id: "transaction-pin".to_string(),
                        disabled: false,
                    }))
                    p style="font-size:0.75rem;color:var(--mui-text-muted);margin:0;" {
                        "Confirms payments over $100. Never share this PIN."
                    }
                }
            }

            // Locked state
            section {
                h2 { "2FA locked" }
                p.mui-showcase__caption { "Too many failed attempts. Try again in 15 minutes." }
                div style="max-width:22rem;" {
                    (render(Props {
                        length: 6,
                        group_size: 3,
                        name: "two-fa-locked".to_string(),
                        id: "two-fa-locked".to_string(),
                        disabled: true,
                    }))
                }
            }
        }
    }
}
