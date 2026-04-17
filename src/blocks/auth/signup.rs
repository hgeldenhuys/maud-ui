//! `auth::signup` — account creation card. Twin of `auth::login`;
//! shares the same visual language and CSS classes.
//!
//! Composes: `card` layout, `input` (name + email + password + confirm),
//! `button` (primary + OAuth), `alert` (error banner), `separator`.
//!
//! ## Example
//!
//! ```no_run
//! use maud::html;
//! use maud_ui::blocks::auth::signup;
//!
//! signup::render(signup::Props {
//!     action: "/auth/signup".into(),
//!     heading: "Create your account".into(),
//!     subheading: "Start your 14-day free trial.".into(),
//!     oauth_providers: vec![],
//!     terms_url: Some("/legal/terms".into()),
//!     privacy_url: Some("/legal/privacy".into()),
//!     signin_url: Some("/auth/login".into()),
//!     ..Default::default()
//! });
//! ```

use maud::{html, Markup, PreEscaped};

use crate::primitives::{alert, button, input};

/// Props for the signup block.
#[derive(Clone, Debug)]
pub struct Props {
    /// POST target for the email + password form. Submits fields:
    /// `name`, `email`, `password`, `password_confirm`, `accept_terms`.
    pub action: String,

    /// Optional brand logo shown above the heading.
    pub logo: Option<Markup>,

    /// Headline (default: "Create your account").
    pub heading: String,

    /// Subheading under the heading (default: "Start in under a minute.").
    pub subheading: String,

    /// Error banner (danger variant). Typical use: re-render the page
    /// with `Some("Email already in use")` after a failed POST.
    pub error: Option<String>,

    /// OAuth providers shown above the email form. Same shape as
    /// `auth::login::OAuthProvider` — consumers often share a Vec
    /// between their login + signup pages.
    pub oauth_providers: Vec<crate::blocks::auth::login::OAuthProvider>,

    /// Include the full-name field at the top of the form. Many
    /// B2C flows skip this and collect the name later. Default: true.
    pub collect_name: bool,

    /// Include the password-confirmation field. Default: true.
    pub require_password_confirm: bool,

    /// Terms of Service URL. If `terms_url` or `privacy_url` is set,
    /// a checkbox row appears above the submit button.
    pub terms_url: Option<String>,

    /// Privacy Policy URL. Rendered next to the Terms link in the
    /// checkbox label.
    pub privacy_url: Option<String>,

    /// URL for the "Already have an account? Sign in" link at the
    /// bottom of the card. `None` hides the prompt — useful for
    /// embedded signup where the parent page already has a sign-in
    /// option.
    pub signin_url: Option<String>,

    /// Submit button label (default: "Create account").
    pub submit_label: String,

    /// Email value to preserve across error re-renders.
    pub email_value: String,

    /// Name value to preserve across error re-renders.
    pub name_value: String,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            action: "/signup".into(),
            logo: None,
            heading: "Create your account".into(),
            subheading: "Start in under a minute.".into(),
            error: None,
            oauth_providers: Vec::new(),
            collect_name: true,
            require_password_confirm: true,
            terms_url: None,
            privacy_url: None,
            signin_url: None,
            submit_label: "Create account".into(),
            email_value: String::new(),
            name_value: String::new(),
        }
    }
}

/// Render the signup block.
pub fn render(props: Props) -> Markup {
    let show_terms = props.terms_url.is_some() || props.privacy_url.is_some();

    html! {
        div class="mui-block mui-block--auth" {
            div class="mui-block__frame mui-block--auth__frame" {
                div class="mui-block--auth__card" {
                    @if let Some(logo) = &props.logo {
                        div class="mui-block--auth__logo" { (logo) }
                    }
                    h1 class="mui-block--auth__heading" { (props.heading) }
                    p class="mui-block--auth__subheading" { (props.subheading) }

                    @if let Some(err) = &props.error {
                        div class="mui-block--auth__error" {
                            (alert::render(alert::Props {
                                title: "Couldn\u{2019}t create account".into(),
                                description: Some(err.clone()),
                                variant: alert::Variant::Danger,
                                ..Default::default()
                            }))
                        }
                    }

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
                        div class="mui-block--auth__divider" aria-hidden="true" {
                            span class="mui-block--auth__divider-text" {
                                "or sign up with email"
                            }
                        }
                    }

                    form action=(props.action) method="post"
                         class="mui-block--auth__form" {
                        @if props.collect_name {
                            div class="mui-block--auth__field" {
                                label for="mui-block-signup-name"
                                      class="mui-block--auth__label" {
                                    "Full name"
                                }
                                (input::render(input::Props {
                                    name: "name".into(),
                                    id: "mui-block-signup-name".into(),
                                    input_type: crate::primitives::input::InputType::Text,
                                    placeholder: "Your name".into(),
                                    value: props.name_value.clone(),
                                    required: true,
                                    ..Default::default()
                                }))
                            }
                        }

                        div class="mui-block--auth__field" {
                            label for="mui-block-signup-email"
                                  class="mui-block--auth__label" {
                                "Work email"
                            }
                            (input::render(input::Props {
                                name: "email".into(),
                                id: "mui-block-signup-email".into(),
                                input_type: crate::primitives::input::InputType::Email,
                                placeholder: "you@company.com".into(),
                                value: props.email_value.clone(),
                                required: true,
                                ..Default::default()
                            }))
                        }

                        div class="mui-block--auth__field" {
                            label for="mui-block-signup-password"
                                  class="mui-block--auth__label" {
                                "Password"
                            }
                            (input::render(input::Props {
                                name: "password".into(),
                                id: "mui-block-signup-password".into(),
                                input_type: crate::primitives::input::InputType::Password,
                                required: true,
                                ..Default::default()
                            }))
                            span class="mui-block--auth__hint" {
                                "At least 8 characters."
                            }
                        }

                        @if props.require_password_confirm {
                            div class="mui-block--auth__field" {
                                label for="mui-block-signup-confirm"
                                      class="mui-block--auth__label" {
                                    "Confirm password"
                                }
                                (input::render(input::Props {
                                    name: "password_confirm".into(),
                                    id: "mui-block-signup-confirm".into(),
                                    input_type: crate::primitives::input::InputType::Password,
                                    required: true,
                                    ..Default::default()
                                }))
                            }
                        }

                        @if show_terms {
                            div class="mui-block--auth__terms" {
                                input type="checkbox"
                                      id="mui-block-signup-terms"
                                      name="accept_terms"
                                      value="yes"
                                      required
                                      class="mui-block--auth__terms-input";
                                label for="mui-block-signup-terms"
                                      class="mui-block--auth__terms-label" {
                                    "I agree to the "
                                    @if let Some(url) = &props.terms_url {
                                        a href=(url) class="mui-block--auth__footer-link" {
                                            "Terms of Service"
                                        }
                                    } @else {
                                        "Terms of Service"
                                    }
                                    @if props.terms_url.is_some() && props.privacy_url.is_some() {
                                        " and "
                                    } @else if props.privacy_url.is_some() {
                                        " and "
                                    }
                                    @if let Some(url) = &props.privacy_url {
                                        a href=(url) class="mui-block--auth__footer-link" {
                                            "Privacy Policy"
                                        }
                                    }
                                    "."
                                }
                            }
                        }

                        (button::render(button::Props {
                            label: props.submit_label.clone(),
                            variant: button::Variant::Primary,
                            size: button::Size::Md,
                            button_type: "submit",
                            ..Default::default()
                        }))
                    }

                    @if let Some(url) = &props.signin_url {
                        p class="mui-block--auth__footer" {
                            "Already have an account? "
                            a href=(url) class="mui-block--auth__footer-link" {
                                "Sign in"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Realistic filled-in preview for the showcase page.
pub fn preview() -> Markup {
    use crate::blocks::auth::login::OAuthProvider;
    render(Props {
        action: "/auth/signup".into(),
        heading: "Create your account".into(),
        subheading: "14-day free trial. No credit card required.".into(),
        oauth_providers: vec![OAuthProvider {
            id: "google".into(),
            label: "Continue with Google".into(),
            href: "/auth/oauth/google".into(),
            icon: Some(icon_google()),
        }],
        terms_url: Some("/legal/terms".into()),
        privacy_url: Some("/legal/privacy".into()),
        signin_url: Some("/auth/login".into()),
        ..Default::default()
    })
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
