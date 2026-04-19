//! Toast component — transient notifications with dismiss and auto-dismiss.
//!
//! **Note:** shadcn deprecated the original `toast` primitive in favour of
//! `sonner` (positioned viewport + richer API). For new call sites prefer
//! [`super::sonner`], which re-exports this module's [`Props`] / [`Variant`]
//! / [`render`] and adds a `Position` enum for viewport placement. This
//! module is retained for back-compat.

use maud::{html, Markup};

/// Toast variant enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Info,
    Success,
    Warning,
    Danger,
}

impl Variant {
    fn class(&self) -> &'static str {
        match self {
            Self::Info => "mui-toast--info",
            Self::Success => "mui-toast--success",
            Self::Warning => "mui-toast--warning",
            Self::Danger => "mui-toast--danger",
        }
    }

    fn role(&self) -> &'static str {
        match self {
            Self::Danger => "alert",
            _ => "status",
        }
    }

    fn aria_live(&self) -> &'static str {
        match self {
            Self::Danger => "assertive",
            _ => "polite",
        }
    }
}

/// Toast rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// Title text displayed in the toast
    pub title: String,
    /// Optional description text
    pub description: Option<String>,
    /// Visual variant (color scheme)
    pub variant: Variant,
    /// Auto-dismiss duration in milliseconds (0 = no auto-dismiss)
    pub duration_ms: u32,
    /// Unique identifier for the toast
    pub id: String,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: None,
            variant: Variant::Info,
            duration_ms: 5000,
            id: String::new(),
        }
    }
}

/// Render a single toast with the given properties
pub fn render(props: Props) -> Markup {
    let role = props.variant.role();
    let aria_live = props.variant.aria_live();

    html! {
        div class={"mui-toast " (props.variant.class())}
            id=(props.id)
            role=(role)
            aria-live=(aria_live)
            data-mui="toast"
            data-duration=(props.duration_ms.to_string())
        {
            div.mui-toast__title {
                (props.title)
            }
            @if let Some(desc) = props.description {
                div.mui-toast__description {
                    (desc)
                }
            }
            button type="button" class="mui-toast__close" aria-label="Dismiss" {
                "×"
            }
        }
    }
}

/// Render a ToastAction slot — an inline button callers can embed next to a
/// toast (e.g. "Undo", "View"). `onclick` is emitted verbatim as the button's
/// `onclick` attribute, so callers may pass arbitrary JS (e.g.
/// `"MaudUI.toastDismiss('my-toast-id')"`).
pub fn action(label: &str, onclick: &str) -> Markup {
    html! {
        button type="button" class="mui-toast__action" onclick=(onclick) { (label) }
    }
}

/// Render the viewport container for imperative toasts
pub fn viewport() -> Markup {
    html! {
        div id="mui-toast-viewport" class="mui-toast-viewport" aria-live="polite" {}
    }
}

/// Showcase all toast variants and use cases
pub fn showcase() -> Markup {
    html! {
        (viewport())

        div.mui-showcase__grid {
            // Static toasts section
            div {
                p.mui-showcase__caption { "Static toasts (inline)" }
                div style="position: static;" {
                    (render(Props {
                        title: "Profile updated".into(),
                        description: Some("Your display name and avatar have been saved.".into()),
                        variant: Variant::Info,
                        duration_ms: 5000,
                        id: "toast-info-static".into(),
                    }))
                    (render(Props {
                        title: "Invoice sent".into(),
                        description: Some("Invoice #1042 has been emailed to the client.".into()),
                        variant: Variant::Success,
                        duration_ms: 5000,
                        id: "toast-success-static".into(),
                    }))
                    (render(Props {
                        title: "Session expiring in 5 minutes".into(),
                        description: Some("Save your work now to avoid losing unsaved changes.".into()),
                        variant: Variant::Warning,
                        duration_ms: 5000,
                        id: "toast-warning-static".into(),
                    }))
                    (render(Props {
                        title: "Payment failed".into(),
                        description: Some("Please check your card details and try again.".into()),
                        variant: Variant::Danger,
                        duration_ms: 5000,
                        id: "toast-danger-static".into(),
                    }))
                }
            }

            // Imperative API section
            div {
                p.mui-showcase__caption { "Imperative (click to dispatch)" }

                p.mui-showcase__caption { "Title only" }
                div.mui-showcase__row {
                    button type="button"
                        class="mui-btn mui-btn--default mui-btn--md"
                        onclick="MaudUI.toast({variant:'info', title:'Profile updated', duration_ms:5000})"
                    {
                        "Show info toast"
                    }
                    button type="button"
                        class="mui-btn mui-btn--default mui-btn--md"
                        onclick="MaudUI.toast({variant:'success', title:'Changes saved', duration_ms:5000})"
                    {
                        "Show success toast"
                    }
                    button type="button"
                        class="mui-btn mui-btn--default mui-btn--md"
                        onclick="MaudUI.toast({variant:'warning', title:'Session expiring soon', duration_ms:5000})"
                    {
                        "Show warning toast"
                    }
                    button type="button"
                        class="mui-btn mui-btn--default mui-btn--md"
                        onclick="MaudUI.toast({variant:'danger', title:'Upload failed', duration_ms:5000})"
                    {
                        "Show error toast"
                    }
                }

                p.mui-showcase__caption { "With description" }
                div.mui-showcase__row {
                    button type="button"
                        class="mui-btn mui-btn--default mui-btn--md"
                        onclick="MaudUI.toast({variant:'info', title:'New comment', description:'Alex replied to your thread in #design.', duration_ms:5000})"
                    {
                        "Info with description"
                    }
                    button type="button"
                        class="mui-btn mui-btn--default mui-btn--md"
                        onclick="MaudUI.toast({variant:'success', title:'Deployment complete', description:'v2.4.1 is now live in production.', duration_ms:5000})"
                    {
                        "Success with description"
                    }
                    button type="button"
                        class="mui-btn mui-btn--default mui-btn--md"
                        onclick="MaudUI.toast({variant:'warning', title:'API rate limit', description:'You have 12 requests remaining this minute.', duration_ms:5000})"
                    {
                        "Warning with description"
                    }
                    button type="button"
                        class="mui-btn mui-btn--default mui-btn--md"
                        onclick="MaudUI.toast({variant:'danger', title:'Payment failed', description:'Please check your card details and try again.', duration_ms:5000})"
                    {
                        "Error with description"
                    }
                }

                p.mui-showcase__caption { "With action slot" }
                div style="position: static;" {
                    div class="mui-toast mui-toast--info" role="status" aria-live="polite" {
                        div.mui-toast__title { "Message archived" }
                        div.mui-toast__description { "You can restore it from the archive view." }
                        (action("Undo", "MaudUI.toast({variant:'success', title:'Restored', duration_ms:3000})"))
                        button type="button" class="mui-toast__close" aria-label="Dismiss" { "×" }
                    }
                }
            }
        }
    }
}
