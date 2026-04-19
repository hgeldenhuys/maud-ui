//! `auth::two_factor` — one-time-code confirmation card (email / SMS / TOTP).
//!
//! Composes: `card` layout (reused `.mui-block--auth__*` classes) +
//! `input_otp` primitive + `button`.
//!
//! The block is pure server-rendered HTML; the `input_otp` primitive's
//! JS behavior (auto-focus between cells, paste full code) works as
//! soon as the runtime loads.
//!
//! ## Example
//!
//! ```no_run
//! use maud::html;
//! use maud_ui::blocks::auth::two_factor;
//!
//! two_factor::render(two_factor::Props {
//!     action: "/auth/2fa/verify".into(),
//!     method: two_factor::Method::Email,
//!     sent_to: "s***@acme.com".into(),
//!     length: 6,
//!     resend_url: Some("/auth/2fa/resend".into()),
//!     cancel_url: Some("/auth/login".into()),
//!     ..Default::default()
//! });
//! ```

use maud::{html, Markup};

use crate::primitives::{alert, button, input_otp};

/// Props for the two-factor block.
#[derive(Clone, Debug)]
pub struct Props {
    /// POST target for the code-submit form. Submits field `code`.
    pub action: String,

    /// Delivery channel — used to pick the heading copy and icon.
    pub method: Method,

    /// Masked/partial identifier for where the code went
    /// (e.g. `"s***@acme.com"` or `"+1 (••) •••-4321"`). Shown in the
    /// subheading so users see which destination to check.
    pub sent_to: String,

    /// OTP length. 6 is the ubiquitous default; 4 is common for PINs
    /// and cheaper SMS cases.
    pub length: usize,

    /// Error banner shown above the OTP input — e.g. "That code was
    /// incorrect" or "Code expired; request a new one".
    pub error: Option<String>,

    /// URL for the resend link. `None` hides the resend row (useful
    /// while a client-side cooldown is active — render the same block
    /// but with this `None` and a hint in `footer_hint`).
    pub resend_url: Option<String>,

    /// Custom text below the OTP input, above the submit button.
    /// Typical use: cooldown hint ("Resend available in 23s") or
    /// channel-specific advice ("Check your spam folder").
    pub footer_hint: Option<String>,

    /// URL for the "Cancel / use a different method" link at the
    /// bottom. `None` hides it.
    pub cancel_url: Option<String>,

    /// Label for the submit button (default: "Verify").
    pub submit_label: String,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            action: "/auth/2fa/verify".into(),
            method: Method::Email,
            sent_to: String::new(),
            length: 6,
            error: None,
            resend_url: None,
            footer_hint: None,
            cancel_url: None,
            submit_label: "Verify".into(),
        }
    }
}

/// Delivery method the code was sent through.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Method {
    /// Code delivered via email.
    Email,
    /// Code delivered via SMS.
    Sms,
    /// User enters code from an authenticator app (Google
    /// Authenticator, Authy, 1Password, etc.). Heading reflects
    /// "from your authenticator app" instead of "we sent you".
    Authenticator,
}

/// Render the 2FA block.
pub fn render(props: Props) -> Markup {
    let (heading, subheading) = match props.method {
        Method::Email => (
            "Check your email",
            format!(
                "We sent a {}-digit code to {}.",
                props.length, props.sent_to
            ),
        ),
        Method::Sms => (
            "Check your phone",
            format!(
                "We texted a {}-digit code to {}.",
                props.length, props.sent_to
            ),
        ),
        Method::Authenticator => (
            "Two-factor verification",
            format!(
                "Enter the {}-digit code from your authenticator app.",
                props.length
            ),
        ),
    };

    html! {
        div class="mui-block mui-block--auth" {
            div class="mui-block__frame mui-block--auth__frame" {
                div class="mui-block--auth__card" {
                    h1 class="mui-block--auth__heading" { (heading) }
                    p class="mui-block--auth__subheading" { (subheading) }

                    @if let Some(err) = &props.error {
                        div class="mui-block--auth__error" {
                            (alert::render(alert::Props {
                                title: "Couldn\u{2019}t verify code".into(),
                                description: Some(err.clone()),
                                variant: alert::Variant::Danger,
                                ..Default::default()
                            }))
                        }
                    }

                    form action=(props.action) method="post"
                         class="mui-block--auth__form" {
                        div class="mui-block--auth__otp-wrap" {
                            (input_otp::render(input_otp::Props {
                                id: "mui-block-auth-otp".into(),
                                name: "code".into(),
                                length: props.length,
                                group_size: if props.length >= 6 { 3 } else { props.length },
                                disabled: false,
                            }))
                        }

                        @if let Some(hint) = &props.footer_hint {
                            p class="mui-block--auth__hint" style="text-align:center;" { (hint) }
                        }

                        (button::render(button::Props {
                            label: props.submit_label.clone(),
                            variant: button::Variant::Primary,
                            size: button::Size::Md,
                            button_type: "submit",
                            ..Default::default()
                        }))
                    }

                    @if let Some(url) = &props.resend_url {
                        p class="mui-block--auth__footer" {
                            "Didn\u{2019}t get it? "
                            a href=(url) class="mui-block--auth__footer-link" { "Resend code" }
                        }
                    }

                    @if let Some(url) = &props.cancel_url {
                        p class="mui-block--auth__footer" style="margin-top:0;" {
                            a href=(url) class="mui-block--auth__footer-link" {
                                "Use a different method"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Realistic filled-in preview for the showcase.
pub fn preview() -> Markup {
    render(Props {
        action: "/auth/2fa/verify".into(),
        method: Method::Email,
        sent_to: "s***@acme.com".into(),
        length: 6,
        resend_url: Some("/auth/2fa/resend".into()),
        cancel_url: Some("/auth/login".into()),
        footer_hint: Some("The code expires in 10 minutes.".into()),
        ..Default::default()
    })
}
