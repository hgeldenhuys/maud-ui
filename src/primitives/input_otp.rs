//! InputOTP component — one-time password input with grouped slots.
use maud::{html, Markup};

/// Accepted character set for each OTP slot. Emitted as each `<input>`'s
/// `pattern` attribute and — when meaningful — also tunes `inputmode`.
#[derive(Debug, Clone, Default)]
pub enum OtpPattern {
    /// Digits only (`[0-9]`). Default. Uses `inputmode="numeric"`.
    #[default]
    Digits,
    /// Digits plus ASCII letters, case-insensitive. Uses `inputmode="text"`.
    DigitsAndChars,
    /// Caller-supplied regex snippet — inserted verbatim into `pattern=".."`.
    Custom(String),
}

impl OtpPattern {
    fn pattern_attr(&self) -> String {
        match self {
            OtpPattern::Digits => "[0-9]".to_string(),
            OtpPattern::DigitsAndChars => "[0-9A-Za-z]".to_string(),
            OtpPattern::Custom(s) => s.clone(),
        }
    }

    fn inputmode(&self) -> &'static str {
        match self {
            OtpPattern::Digits => "numeric",
            OtpPattern::DigitsAndChars | OtpPattern::Custom(_) => "text",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Props {
    /// Total number of digits
    pub length: usize,
    /// Group size for visual separator (e.g., 3 for a 3-3 layout on 6 digits)
    pub group_size: usize,
    pub name: String,
    pub id: String,
    pub disabled: bool,
    /// Accepted character pattern per slot. Defaults to [`OtpPattern::Digits`].
    pub pattern: OtpPattern,
    /// When true, emits `aria-invalid="true"` on each slot — for announcing
    /// incorrect-code validation without triggering native form validation.
    pub aria_invalid: bool,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            length: 6,
            group_size: 3,
            name: "otp".to_string(),
            id: "otp".to_string(),
            disabled: false,
            pattern: OtpPattern::Digits,
            aria_invalid: false,
        }
    }
}

pub fn render(props: Props) -> Markup {
    let pattern_attr = props.pattern.pattern_attr();
    let inputmode_attr = props.pattern.inputmode();
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
                    inputmode=(inputmode_attr)
                    pattern=(pattern_attr)
                    autocomplete="one-time-code"
                    aria-label=(format!("Digit {}", i + 1))
                    aria-invalid=[if props.aria_invalid { Some("true") } else { None }]
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
                        ..Default::default()
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
                        ..Default::default()
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
                        ..Default::default()
                    }))
                }
            }

            // Alphanumeric coupon code
            section {
                h2 { "Redeem coupon" }
                p.mui-showcase__caption { "Alphanumeric code — pattern: DigitsAndChars." }
                div style="max-width:22rem;" {
                    (render(Props {
                        length: 8,
                        group_size: 4,
                        name: "coupon".to_string(),
                        id: "coupon".to_string(),
                        pattern: OtpPattern::DigitsAndChars,
                        ..Default::default()
                    }))
                }
            }

            // Invalid (wrong code)
            section {
                h2 { "Wrong code" }
                p.mui-showcase__caption { "aria_invalid: true — announces failure to screen readers." }
                div style="max-width:22rem;" {
                    (render(Props {
                        length: 6,
                        group_size: 3,
                        name: "wrong-code".to_string(),
                        id: "wrong-code".to_string(),
                        aria_invalid: true,
                        ..Default::default()
                    }))
                }
            }
        }
    }
}
