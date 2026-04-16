//! Alert dialog component — modal dialog role="alertdialog" that cannot be closed by ESC or backdrop click.
//! Unlike Dialog, there is no close button or backdrop dismiss — the user must choose an explicit action.
use maud::{html, Markup};

/// Alert dialog rendering properties
#[derive(Clone, Debug)]
pub struct Props {
    /// Unique identifier for the alert dialog
    pub id: String,
    /// Alert dialog title
    pub title: String,
    /// Optional description text displayed below title
    pub description: Option<String>,
    /// Markup content displayed in alert dialog body (optional extra content)
    pub children: Markup,
    /// Footer markup — action buttons (Cancel + destructive action)
    pub footer: Option<Markup>,
    /// Initial open state (default false)
    pub open: bool,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            id: "alert-dialog".to_string(),
            title: "Confirm".to_string(),
            description: None,
            children: html! {},
            footer: None,
            open: false,
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

/// Render an alert dialog with the given properties
pub fn render(props: Props) -> Markup {
    let title_id = format!("{}-title", props.id);
    let desc_id = format!("{}-desc", props.id);
    let has_desc = props.description.is_some();

    html! {
        dialog class="mui-alert-dialog"
            id=(props.id)
            data-mui="alert-dialog"
            role="alertdialog"
            aria-labelledby=(title_id)
            aria-describedby=[if has_desc { Some(desc_id.as_str()) } else { None }]
            open[props.open]
        {
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

/// Showcase all alert dialog use cases
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            // Destructive action alert dialog
            {
                (trigger("demo-alert-1", "Delete account", "danger"))
            }
            {
                (render(Props {
                    id: "demo-alert-1".to_string(),
                    title: "Are you absolutely sure?".to_string(),
                    description: Some("This action cannot be undone. This will permanently delete your account and remove your data from our servers.".to_string()),
                    children: html! {},
                    footer: Some(html! {
                        button class="mui-btn mui-btn--ghost mui-btn--md" data-mui-close { "Cancel" }
                        button class="mui-btn mui-btn--danger mui-btn--md" data-mui-close { "Yes, delete account" }
                    }),
                    open: false,
                }))
            }
            // Confirmation alert dialog
            {
                (trigger("demo-alert-2", "Discard changes", "default"))
            }
            {
                (render(Props {
                    id: "demo-alert-2".to_string(),
                    title: "Discard changes?".to_string(),
                    description: Some("You have unsaved changes. Are you sure you want to discard them?".to_string()),
                    children: html! {},
                    footer: Some(html! {
                        button class="mui-btn mui-btn--ghost mui-btn--md" data-mui-close { "Cancel" }
                        button class="mui-btn mui-btn--primary mui-btn--md" data-mui-close { "Continue" }
                    }),
                    open: false,
                }))
            }
        }
    }
}
