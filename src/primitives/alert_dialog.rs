//! Alert dialog component — modal dialog role="alertdialog" that cannot be closed by ESC or backdrop click.
//! Unlike Dialog, there is no close button or backdrop dismiss — the user must choose an explicit action.
use crate::primitives::button::Variant as ButtonVariant;
use maud::{html, Markup, PreEscaped};

/// Size variants for the alert dialog container. Mirrors shadcn's sizing tokens.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Size {
    /// Default width — `max-width: 32rem` (shadcn default).
    Default,
    /// Compact width — `max-width: 24rem`.
    Sm,
}

impl Size {
    fn modifier_class(self) -> Option<&'static str> {
        match self {
            Size::Default => None,
            Size::Sm => Some("mui-alert-dialog--sm"),
        }
    }
}

/// Alert dialog rendering properties
#[derive(Clone, Debug)]
pub struct Props {
    /// Unique identifier for the alert dialog
    pub id: String,
    /// Alert dialog title
    pub title: String,
    /// Optional description text displayed below title
    pub description: Option<String>,
    /// Optional media slot (icon/image) rendered above the header.
    /// Matches shadcn's `AlertDialogMedia`.
    pub media: Option<Markup>,
    /// Markup content displayed in alert dialog body (optional extra content)
    pub children: Markup,
    /// Footer markup — action buttons (Cancel + destructive action)
    pub footer: Option<Markup>,
    /// Initial open state (default false)
    pub open: bool,
    /// Size variant — `Default` (32rem) or `Sm` (24rem, compact).
    pub size: Size,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            id: "alert-dialog".to_string(),
            title: "Confirm".to_string(),
            description: None,
            media: None,
            children: html! {},
            footer: None,
            open: false,
            size: Size::Default,
        }
    }
}

/// Render an alert dialog trigger button that opens the dialog with the given target_id
pub fn trigger(target_id: &str, label: &str, variant: &str) -> Markup {
    html! {
        button type="button"
            class=(format!("mui-btn mui-btn--{} mui-btn--md", variant))
            data-mui="alert-dialog-trigger"
            data-target=(target_id)
        {
            (label)
        }
    }
}

/// Render a media slot wrapper. Callers pass an icon/image as `children`;
/// the wrapper centers it above the dialog header.
///
/// Usage:
/// ```ignore
/// media(html! { svg { /* ... */ } })
/// ```
pub fn media(children: Markup) -> Markup {
    html! {
        div class="mui-alert-dialog__media" {
            (children)
        }
    }
}

/// Map a `ButtonVariant` to its `mui-btn--*` modifier class. Keep in sync
/// with `crate::primitives::button::Variant::class_name`.
fn variant_class(variant: ButtonVariant) -> &'static str {
    match variant {
        ButtonVariant::Default => "mui-btn--default",
        ButtonVariant::Primary => "mui-btn--primary",
        ButtonVariant::Secondary => "mui-btn--secondary",
        ButtonVariant::Outline => "mui-btn--outline",
        ButtonVariant::Ghost => "mui-btn--ghost",
        ButtonVariant::Danger => "mui-btn--danger",
        ButtonVariant::Link => "mui-btn--link",
    }
}

/// Render a primary action button for the alert dialog footer. Clicking it
/// closes the dialog (via `data-mui-close`). Use `ButtonVariant::Danger` for
/// destructive actions, `ButtonVariant::Primary` for constructive.
///
/// Emits a button that matches `button::render` output shape (same classes,
/// same md size) but with `data-mui-close` attached so the alert-dialog
/// controller dismisses the dialog on click.
pub fn action(label: &str, variant: ButtonVariant) -> Markup {
    let class = format!("mui-btn {} mui-btn--md", variant_class(variant));
    html! {
        button type="button" class=(class) data-mui-close {
            (label)
        }
    }
}

/// Render a secondary "cancel" button for the alert dialog footer. Always
/// `Ghost` variant + `data-mui-close` to dismiss without taking action.
pub fn cancel(label: &str) -> Markup {
    action(label, ButtonVariant::Ghost)
}

/// Render an alert dialog with the given properties
pub fn render(props: Props) -> Markup {
    let title_id = format!("{}-title", props.id);
    let desc_id = format!("{}-desc", props.id);
    let has_desc = props.description.is_some();

    let class = match props.size.modifier_class() {
        Some(modifier) => format!("mui-alert-dialog {}", modifier),
        None => "mui-alert-dialog".to_string(),
    };

    html! {
        dialog class=(class)
            id=(props.id)
            data-mui="alert-dialog"
            role="alertdialog"
            aria-labelledby=(title_id)
            aria-describedby=[if has_desc { Some(desc_id.as_str()) } else { None }]
            open[props.open]
        {
            @if let Some(m) = props.media {
                (m)
            }
            div class="mui-alert-dialog__header" {
                h2 class="mui-alert-dialog__title" id=(title_id) {
                    (props.title)
                }
            }
            @if let Some(desc) = props.description {
                p class="mui-alert-dialog__description" id=(desc_id) {
                    (desc)
                }
            }
            div class="mui-alert-dialog__body" {
                (props.children)
            }
            @if let Some(footer) = props.footer {
                div class="mui-alert-dialog__footer" {
                    (footer)
                }
            }
        }
    }
}

/// Inline SVG alert-triangle icon (48x48, stroke=currentColor) for media demos.
fn icon_alert_triangle() -> Markup {
    PreEscaped(r#"<svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"/><path d="M12 9v4"/><path d="M12 17h.01"/></svg>"#.to_string())
}

/// Showcase all alert dialog use cases
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            // Delete Account — destructive confirmation
            {
                (trigger("demo-alert-delete", "Delete account", "danger"))
            }
            {
                (render(Props {
                    id: "demo-alert-delete".to_string(),
                    title: "Delete Account".to_string(),
                    description: Some("This action cannot be undone. Your account, projects, and all associated data will be permanently deleted.".to_string()),
                    children: html! {},
                    footer: Some(html! {
                        button class="mui-btn mui-btn--ghost mui-btn--md" data-mui-close { "Cancel" }
                        button class="mui-btn mui-btn--danger mui-btn--md" data-mui-close { "Delete account" }
                    }),
                    ..Default::default()
                }))
            }

            // Discard Changes — non-destructive confirmation
            {
                (trigger("demo-alert-discard", "Discard changes", "default"))
            }
            {
                (render(Props {
                    id: "demo-alert-discard".to_string(),
                    title: "Discard Changes".to_string(),
                    description: Some("Your unsaved changes will be lost. This cannot be recovered.".to_string()),
                    children: html! {},
                    footer: Some(html! {
                        button class="mui-btn mui-btn--ghost mui-btn--md" data-mui-close { "Keep editing" }
                        button class="mui-btn mui-btn--primary mui-btn--md" data-mui-close { "Discard" }
                    }),
                    ..Default::default()
                }))
            }

            // Compact (Size::Sm) with media icon + helper-built footer
            {
                (trigger("demo-alert-compact", "Revoke session", "outline"))
            }
            {
                (render(Props {
                    id: "demo-alert-compact".to_string(),
                    title: "Revoke session?".to_string(),
                    description: Some("You will be signed out of this device.".to_string()),
                    media: Some(media(icon_alert_triangle())),
                    size: Size::Sm,
                    footer: Some(html! {
                        (cancel("Keep signed in"))
                        (action("Revoke", ButtonVariant::Danger))
                    }),
                    ..Default::default()
                }))
            }
        }
    }
}
