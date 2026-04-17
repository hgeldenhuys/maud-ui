//! `settings::profile` — edit name, email, avatar, bio on a settings page.
//!
//! Two stacked sections:
//!   1. Profile details — avatar + form fields + save
//!   2. Danger zone — destructive actions (delete account) with a
//!      clearly separated visual treatment
//!
//! Composes: `card`, `input`, `textarea`, `button`, `label`.
//!
//! ## Example
//!
//! ```no_run
//! use maud_ui::blocks::settings::profile;
//!
//! profile::render(profile::Props {
//!     action: "/settings/profile".into(),
//!     name: "Sofia Davis".into(),
//!     email: "sofia@acme.com".into(),
//!     avatar_initials: "SD".into(),
//!     bio: "Head of Platform at Acme.".into(),
//!     delete_action: Some("/settings/account/delete".into()),
//!     ..Default::default()
//! });
//! ```

use maud::{html, Markup};

use crate::primitives::{button, card, input, textarea};

/// Props for the settings profile block.
#[derive(Clone, Debug)]
pub struct Props {
    /// POST target for the profile form. Submits fields: `name`,
    /// `email`, `bio`. For the avatar upload, use a separate
    /// multipart endpoint — this block shows the current avatar but
    /// doesn't include a file input (decoupled so the form can be a
    /// regular POST).
    pub action: String,

    /// Current display name.
    pub name: String,

    /// Current email address.
    pub email: String,

    /// Two-letter initials fallback for the avatar when there's no
    /// image — matches the shell::sidebar pattern so shells stay
    /// consistent.
    pub avatar_initials: String,

    /// Current bio (optional long-form about text).
    pub bio: String,

    /// Avatar upload URL. `None` hides the "Change" button.
    pub avatar_upload_action: Option<String>,

    /// Remove-avatar URL. `None` hides the "Remove" button.
    pub avatar_remove_action: Option<String>,

    /// Success notice shown at the top (e.g. "Profile updated").
    pub success: Option<String>,

    /// Error banner shown at the top.
    pub error: Option<String>,

    /// Delete-account POST target. When set, renders a "Danger zone"
    /// card at the bottom with a destructive button. `None` hides the
    /// section entirely.
    pub delete_action: Option<String>,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            action: "/settings/profile".into(),
            name: String::new(),
            email: String::new(),
            avatar_initials: String::new(),
            bio: String::new(),
            avatar_upload_action: None,
            avatar_remove_action: None,
            success: None,
            error: None,
            delete_action: None,
        }
    }
}

/// Render the settings profile block.
pub fn render(props: Props) -> Markup {
    html! {
        div class="mui-block mui-block--settings" {
            @if let Some(msg) = &props.success {
                div class="mui-block--settings__notice mui-block--settings__notice--success" {
                    (msg)
                }
            }
            @if let Some(msg) = &props.error {
                div class="mui-block--settings__notice mui-block--settings__notice--danger" {
                    (msg)
                }
            }

            (card::render(card::Props {
                title: Some("Profile".into()),
                description: Some("Update how you appear across the app.".into()),
                children: html! {
                    form action=(props.action) method="post"
                         class="mui-block--settings__form" {
                        // Avatar row
                        div class="mui-block--settings__avatar-row" {
                            span class="mui-block--settings__avatar" aria-hidden="true" {
                                (props.avatar_initials)
                            }
                            div class="mui-block--settings__avatar-actions" {
                                @if let Some(url) = &props.avatar_upload_action {
                                    form action=(url) method="post" enctype="multipart/form-data"
                                         style="display:inline-flex;margin:0;gap:0.5rem;" {
                                        (button::render(button::Props {
                                            label: "Change".into(),
                                            variant: button::Variant::Outline,
                                            size: button::Size::Sm,
                                            button_type: "button",
                                            ..Default::default()
                                        }))
                                    }
                                }
                                @if let Some(url) = &props.avatar_remove_action {
                                    form action=(url) method="post"
                                         style="display:inline-flex;margin:0;" {
                                        (button::render(button::Props {
                                            label: "Remove".into(),
                                            variant: button::Variant::Ghost,
                                            size: button::Size::Sm,
                                            button_type: "submit",
                                            ..Default::default()
                                        }))
                                    }
                                }
                            }
                        }

                        // Name
                        div class="mui-block--settings__field" {
                            label for="mui-block-settings-name"
                                  class="mui-block--settings__label" { "Name" }
                            (input::render(input::Props {
                                name: "name".into(),
                                id: "mui-block-settings-name".into(),
                                input_type: crate::primitives::input::InputType::Text,
                                value: props.name.clone(),
                                required: true,
                                ..Default::default()
                            }))
                        }

                        // Email
                        div class="mui-block--settings__field" {
                            label for="mui-block-settings-email"
                                  class="mui-block--settings__label" { "Email" }
                            (input::render(input::Props {
                                name: "email".into(),
                                id: "mui-block-settings-email".into(),
                                input_type: crate::primitives::input::InputType::Email,
                                value: props.email.clone(),
                                required: true,
                                ..Default::default()
                            }))
                            span class="mui-block--settings__hint" {
                                "You\u{2019}ll be asked to verify a new email."
                            }
                        }

                        // Bio
                        div class="mui-block--settings__field" {
                            label for="mui-block-settings-bio"
                                  class="mui-block--settings__label" { "Bio" }
                            (textarea::render(textarea::Props {
                                name: "bio".into(),
                                id: "mui-block-settings-bio".into(),
                                value: props.bio.clone(),
                                placeholder: "A short line about yourself.".into(),
                                rows: 3u32,
                                ..Default::default()
                            }))
                        }

                        div class="mui-block--settings__actions" {
                            (button::render(button::Props {
                                label: "Save changes".into(),
                                variant: button::Variant::Primary,
                                size: button::Size::Md,
                                button_type: "submit",
                                ..Default::default()
                            }))
                        }
                    }
                },
                ..Default::default()
            }))

            // Danger zone
            @if let Some(url) = &props.delete_action {
                div class="mui-block--settings__danger" {
                    (card::render(card::Props {
                        title: Some("Delete account".into()),
                        description: Some("Permanently delete your account and all associated data. This can\u{2019}t be undone.".into()),
                        children: html! {
                            form action=(url) method="post" {
                                (button::render(button::Props {
                                    label: "Delete account".into(),
                                    variant: button::Variant::Danger,
                                    size: button::Size::Md,
                                    button_type: "submit",
                                    ..Default::default()
                                }))
                            }
                        },
                        ..Default::default()
                    }))
                }
            }
        }
    }
}

/// Realistic filled-in preview for the showcase.
pub fn preview() -> Markup {
    render(Props {
        action: "/settings/profile".into(),
        name: "Sofia Davis".into(),
        email: "sofia@acme.com".into(),
        avatar_initials: "SD".into(),
        bio: "Head of Platform at Acme. Previously at Stripe and Linear.".into(),
        avatar_upload_action: Some("/settings/avatar".into()),
        avatar_remove_action: Some("/settings/avatar/remove".into()),
        delete_action: Some("/settings/account/delete".into()),
        ..Default::default()
    })
}
