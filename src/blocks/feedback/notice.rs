//! Persistent, human-first feedback. Diagnostics never replace the main message.
use crate::blocks::action::Link;
use maud::{html, Markup};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Tone {
    #[default]
    Info,
    Success,
    Error,
}
impl Tone {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Success => "success",
            Self::Error => "error",
        }
    }
}
#[derive(Clone, Debug)]
pub struct Props {
    pub id: Option<String>,
    /// A complete human sentence, e.g. "The guest could not be saved."
    pub message: String,
    /// Helpful next step or record context, before technical details.
    pub description: Option<String>,
    /// Optional diagnostics, behind a native Details disclosure.
    pub details: Option<String>,
    pub tone: Tone,
    pub action: Option<Link>,
}
impl Default for Props {
    fn default() -> Self {
        Self {
            id: None,
            message: "Changes saved.".into(),
            description: None,
            details: None,
            tone: Tone::Info,
            action: None,
        }
    }
}
pub fn render(props: Props) -> Markup {
    let message = if props.message.trim().is_empty() {
        if props.tone == Tone::Error {
            "Your changes could not be saved."
        } else {
            "Changes saved."
        }
    } else {
        &props.message
    };
    html! {
        div class="mui-notice" id=[props.id.as_deref()] data-tone=(props.tone.as_str())
            role=(if props.tone == Tone::Error { "alert" } else { "status" }) aria-atomic="true" tabindex="-1" {
            p class="mui-notice__message" { (message) }
            @if let Some(description) = props.description { p class="mui-notice__description" { (description) } }
            @if let Some(details) = props.details {
                details class="mui-notice__details" { summary { "Details" } pre { (details) } }
            }
            @if let Some(action) = props.action { a class="mui-notice__action" href=(action.href) { (action.label) } }
        }
    }
}
/// An initially empty result region for `MaudUI.formFeedback` or `MaudUI.notice`.
pub fn region(id: &str) -> Markup {
    html! { div id=(id) class="mui-notice" data-mui-form-result role="status" aria-atomic="true" tabindex="-1" hidden {} }
}
pub fn preview() -> Markup {
    super::super::kit_preview(|prefix| {
        html! {
            div class="mui-stack" {
                (render(Props { id: Some(format!("{prefix}-saved")), message: "Guest saved.".into(), description: Some("Leila Morgan is ready for a booking.".into()), tone: Tone::Success, ..Default::default() }))
                (render(Props { id: Some(format!("{prefix}-failed")), message: "The payment could not be recorded.".into(), description: Some("Ask a manager to record this payment. Your entries are still here.".into()), details: Some("Payment creation is unavailable for this role. Reference: N6-403.".into()), tone: Tone::Error, ..Default::default() }))
            }
        }
    })
}
