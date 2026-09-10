//! Named confirmation with a native POST form. No browser-native confirm prompt.
use crate::primitives::{dialog, form};
use maud::{html, Markup};

#[derive(Clone, Debug)]
pub struct Props {
    pub id: String,
    /// Name the consequence, e.g. "Check in Leila Morgan?"
    pub title: String,
    /// Human context: guest, room, dates or amount.
    pub message: String,
    pub confirm_label: String,
    pub cancel_label: String,
    pub pending_label: String,
    /// Native POST destination; application owns validation, authorization and CSRF.
    pub action: String,
    pub hidden_fields: Vec<(String, String)>,
    pub danger: bool,
}
impl Default for Props {
    fn default() -> Self {
        Self {
            id: "confirm-action".into(),
            title: "Save these changes?".into(),
            message: "Review the details before saving.".into(),
            confirm_label: "Save changes".into(),
            cancel_label: "Keep editing".into(),
            pending_label: "Saving…".into(),
            action: String::new(),
            hidden_fields: vec![],
            danger: false,
        }
    }
}
pub fn trigger(id: &str, label: &str) -> Markup {
    dialog::trigger(id, label)
}
pub fn render(props: Props) -> Markup {
    let form_id = format!("{}-form", props.id);
    dialog::render(dialog::Props {
        id: props.id.clone(),
        title: props.title,
        description: Some(props.message),
        show_close_button: false,
        size: dialog::Size::Sm,
        children: form::render(form::Props {
            id: Some(form_id),
            action: Some(props.action),
            feedback: true,
            pending_label: props.pending_label,
            children: html! {
                @for (name, value) in props.hidden_fields { input type="hidden" name=(name) value=(value); }
                (super::notice::region(&format!("{}-form-result", props.id)))
                div class="mui-confirm__actions" {
                    button type="button" class="mui-btn mui-btn--outline" data-mui-close autofocus { (props.cancel_label) }
                    button type="submit" class=(if props.danger { "mui-btn mui-btn--danger" } else { "mui-btn mui-btn--primary" }) { (props.confirm_label) }
                }
            },
            ..Default::default()
        }),
        ..Default::default()
    })
}
pub fn preview() -> Markup {
    super::super::kit_preview(|prefix| {
        let id = format!("{prefix}-confirm-checkin");
        html! {
            div data-mui="confirm-demo" {
            (trigger(&id, "Review check-in"))
            (render(Props { id, title: "Check in Leila Morgan?".into(), message: "Garden suite · 10–11 September. The booking will be marked as checked in.".into(), confirm_label: "Check in guest".into(), cancel_label: "Keep booking".into(), action: "/blocks/feedback-confirm".into(), ..Default::default() }))
            }
        }
    })
}
