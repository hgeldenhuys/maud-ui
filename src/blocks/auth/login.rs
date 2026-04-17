//! `auth::login` — email + password login card with optional OAuth.
//!
//! Composes: `card`, `input`, `button`, `label`, `alert`, `separator`.
//!
//! Production-ready. Wire `action` to your POST handler and the form
//! submits `email` + `password`. OAuth providers post to their own
//! `href` individually (one form per provider so CSRF tokens can differ).
//!
//! Visual reference: shadcn/ui's login-01 block. Centered card on a
//! subtle backdrop, ~24rem wide, logo + heading + optional error +
//! OAuth stack + divider + email-password form + signup link.
//!
//! ## Example
//!
//! ```no_run
//! use maud::{html, Markup};
//! use maud_ui::blocks::auth::login;
//!
//! fn login_page() -> Markup {
//!     login::render(login::Props {
//!         action: "/auth/login".into(),
//!         heading: "Welcome back".into(),
//!         subheading: "Sign in to continue to Acme".into(),
//!         oauth_providers: vec![
//!             login::OAuthProvider {
//!                 id: "google".into(),
//!                 label: "Continue with Google".into(),
//!                 href: "/auth/oauth/google".into(),
//!                 icon: None,
//!             },
//!         ],
//!         signup_url: Some("/auth/signup".into()),
//!         forgot_password_url: Some("/auth/forgot".into()),
//!         ..Default::default()
//!     })
//! }
//! ```

use maud::{html, Markup, PreEscaped};

use crate::primitives::{alert, button, input};

/// Props for the login block.
#[derive(Clone, Debug)]
pub struct Props {
    /// Form action URL — where the email + password form POSTs.
    /// The form sends `email` and `password` fields.
    pub action: String,

    /// Optional logo shown above the heading. Typically an inline SVG
    /// or `<img>`. Use `stroke="currentColor"` on SVGs so they inherit
    /// `--mui-text`.
    pub logo: Option<Markup>,

    /// Primary heading (default: "Welcome back").
    pub heading: String,

    /// Secondary prompt below the heading (default: "Sign in to your
    /// account").
    pub subheading: String,

    /// Optional error message to show above the form — rendered with
    /// the `alert` primitive in Danger variant. Typical use: render
    /// the login page with `error: Some("Invalid email or password")`
    /// after a failed POST.
    pub error: Option<String>,

    /// OAuth providers to show above the email/password form. Empty
    /// vec = no OAuth row, no divider.
    pub oauth_providers: Vec<OAuthProvider>,

    /// URL for the "Forgot password?" link rendered next to the
    /// password label. `None` hides the link.
    pub forgot_password_url: Option<String>,

    /// URL for the "Don't have an account? Sign up" prompt at the
    /// bottom of the card. `None` hides the prompt entirely — useful
    /// for invite-only products.
    pub signup_url: Option<String>,

    /// Email value to preserve across error re-renders (e.g., after a
    /// failed POST, the email stays filled in).
    pub email_value: String,

    /// Submit button label (default: "Sign in").
    pub submit_label: String,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            action: "/login".into(),
            logo: None,
            heading: "Welcome back".into(),
            subheading: "Sign in to your account".into(),
            error: None,
            oauth_providers: Vec::new(),
            forgot_password_url: None,
            signup_url: None,
            email_value: String::new(),
            submit_label: "Sign in".into(),
        }
    }
}

/// A single OAuth button above the email/password form.
#[derive(Clone, Debug)]
pub struct OAuthProvider {
    /// Short identifier — "google", "github", etc. Used for the
    /// button id and nothing else.
    pub id: String,
    /// Visible button text — "Continue with Google".
    pub label: String,
    /// Where the OAuth button posts/links. The block wraps each
    /// provider in its own `<form method="post">` so CSRF tokens or
    /// provider-specific query args can be added per-provider.
    pub href: String,
    /// Optional inline SVG icon rendered before the label.
    pub icon: Option<Markup>,
}

/// Render the login block.
pub fn render(props: Props) -> Markup {
    html! {
        div class="mui-block mui-block--auth" {
            div class="mui-block__frame mui-block--auth__frame" {
                div class="mui-block--auth__card" {
                    // Logo + heading
                    @if let Some(logo) = &props.logo {
                        div class="mui-block--auth__logo" { (logo) }
                    }
                    h1 class="mui-block--auth__heading" { (props.heading) }
                    p class="mui-block--auth__subheading" { (props.subheading) }

                    // Error banner (if any)
                    @if let Some(err) = &props.error {
                        div class="mui-block--auth__error" {
                            (alert::render(alert::Props {
                                title: "Sign in failed".into(),
                                description: Some(err.clone()),
                                variant: alert::Variant::Danger,
                                ..Default::default()
                            }))
                        }
                    }

                    // OAuth providers
                    @if !props.oauth_providers.is_empty() {
                        div class="mui-block--auth__oauth" {
                            @for provider in &props.oauth_providers {
                                form action=(provider.href) method="post"
                                     class="mui-block--auth__oauth-form" {
                                    (button::render(button::Props {
                                        label: provider.label.clone(),
                                        variant: button::Variant::Outline,
                                        size: button::Size::Md,
                                        button_type: "submit",
                                        leading_icon: provider.icon.clone(),
                                        ..Default::default()
                                    }))
                                }
                            }
                        }

                        // Divider between OAuth and email/password form
                        div class="mui-block--auth__divider" aria-hidden="true" {
                            span class="mui-block--auth__divider-text" {
                                "or continue with email"
                            }
                        }
                    }

                    // Email + password form
                    form action=(props.action) method="post"
                         class="mui-block--auth__form" {
                        // Email field
                        div class="mui-block--auth__field" {
                            label for="mui-block-auth-email"
                                  class="mui-block--auth__label" {
                                "Email"
                            }
                            (input::render(input::Props {
                                name: "email".into(),
                                id: "mui-block-auth-email".into(),
                                input_type: crate::primitives::input::InputType::Email,
                                placeholder: "you@example.com".into(),
                                value: props.email_value.clone(),
                                required: true,
                                ..Default::default()
                            }))
                        }

                        // Password field — label row has "Forgot password?" link
                        div class="mui-block--auth__field" {
                            div class="mui-block--auth__label-row" {
                                label for="mui-block-auth-password"
                                      class="mui-block--auth__label" {
                                    "Password"
                                }
                                @if let Some(url) = &props.forgot_password_url {
                                    a href=(url)
                                      class="mui-block--auth__forgot-link" {
                                        "Forgot password?"
                                    }
                                }
                            }
                            (input::render(input::Props {
                                name: "password".into(),
                                id: "mui-block-auth-password".into(),
                                input_type: crate::primitives::input::InputType::Password,
                                required: true,
                                ..Default::default()
                            }))
                        }

                        // Submit button
                        (button::render(button::Props {
                            label: props.submit_label.clone(),
                            variant: button::Variant::Primary,
                            size: button::Size::Md,
                            button_type: "submit",
                            ..Default::default()
                        }))
                    }

                    // Signup prompt at the bottom
                    @if let Some(url) = &props.signup_url {
                        p class="mui-block--auth__footer" {
                            "Don\u{2019}t have an account? "
                            a href=(url) class="mui-block--auth__footer-link" {
                                "Sign up"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// A realistic filled-in preview used by the block's showcase page
/// (`/blocks/auth-login`). Demonstrates typical configuration:
/// Google + GitHub OAuth, forgot-password link, signup prompt.
pub fn preview() -> Markup {
    render(Props {
        action: "/auth/login".into(),
        logo: Some(logo_acme()),
        heading: "Welcome back".into(),
        subheading: "Sign in to continue to Acme".into(),
        oauth_providers: vec![
            OAuthProvider {
                id: "google".into(),
                label: "Continue with Google".into(),
                href: "/auth/oauth/google".into(),
                icon: Some(icon_google()),
            },
            OAuthProvider {
                id: "github".into(),
                label: "Continue with GitHub".into(),
                href: "/auth/oauth/github".into(),
                icon: Some(icon_github()),
            },
        ],
        forgot_password_url: Some("/auth/forgot".into()),
        signup_url: Some("/auth/signup".into()),
        ..Default::default()
    })
}

// ── Inline SVG icons used by the preview — shipped as block-local
//    helpers so a consumer can copy-paste the entire module without
//    needing a separate icons crate. `stroke="currentColor"` makes
//    them inherit the button's text color.

fn logo_acme() -> Markup {
    PreEscaped(
        r##"<svg width="32" height="32" viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
<rect width="32" height="32" rx="8" fill="currentColor" opacity="0.1"/>
<path d="M10 20L16 8L22 20H18L16 16L14 20H10Z" fill="currentColor"/>
</svg>"##
            .to_string(),
    )
}

fn icon_google() -> Markup {
    PreEscaped(
        r##"<svg width="16" height="16" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
<path d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z" fill="#4285F4"/>
<path d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z" fill="#34A853"/>
<path d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z" fill="#FBBC05"/>
<path d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z" fill="#EA4335"/>
</svg>"##
            .to_string(),
    )
}

fn icon_github() -> Markup {
    PreEscaped(
        r##"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
<path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.4-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4"/>
<path d="M9 18c-4.51 2-5-2-7-2"/>
</svg>"##
            .to_string(),
    )
}
