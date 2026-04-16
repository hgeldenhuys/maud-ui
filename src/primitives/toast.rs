//! Toast component — transient notifications with dismiss and auto-dismiss.

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
                        title: "Info notification".into(),
                        description: Some("This is an informational toast message.".into()),
                        variant: Variant::Info,
                        duration_ms: 5000,
                        id: "toast-info-static".into(),
                    }))
                    (render(Props {
                        title: "Success! Changes saved".into(),
                        description: Some("Your changes have been saved successfully.".into()),
                        variant: Variant::Success,
                        duration_ms: 5000,
                        id: "toast-success-static".into(),
                    }))
                    (render(Props {
                        title: "Warning: low disk space".into(),
                        description: Some("You have less than 1GB of storage remaining.".into()),
                        variant: Variant::Warning,
                        duration_ms: 5000,
                        id: "toast-warning-static".into(),
                    }))
                    (render(Props {
                        title: "Error: network connection failed".into(),
                        description: Some("Please check your internet connection and try again.".into()),
                        variant: Variant::Danger,
                        duration_ms: 5000,
                        id: "toast-danger-static".into(),
                    }))
                }
            }

            // Imperative API section
            div {
                p.mui-showcase__caption { "Imperative (click to dispatch)" }
                div.mui-showcase__row {
                    button type="button"
                        class="mui-btn mui-btn--primary mui-btn--md"
                        onclick="MaudUI.toast({variant:'info', title:'Info', description:'This is an info toast.', duration_ms:5000})"
                    {
                        "Show info"
                    }
                    button type="button"
                        class="mui-btn mui-btn--primary mui-btn--md"
                        onclick="MaudUI.toast({variant:'success', title:'Saved!', description:'Your changes have been saved.', duration_ms:5000})"
                    {
                        "Show success"
                    }
                    button type="button"
                        class="mui-btn mui-btn--primary mui-btn--md"
                        onclick="MaudUI.toast({variant:'warning', title:'Warning', description:'Please review your input.', duration_ms:5000})"
                    {
                        "Show warning"
                    }
                    button type="button"
                        class="mui-btn mui-btn--primary mui-btn--md"
                        onclick="MaudUI.toast({variant:'danger', title:'Error', description:'An unexpected error occurred.', duration_ms:5000})"
                    {
                        "Show error"
                    }
                }
            }
        }
    }
}
