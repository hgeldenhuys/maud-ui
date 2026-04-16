//! Alert dialog component — modal dialog role="alertdialog" that cannot be closed by ESC or backdrop click.
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
    /// Markup content displayed in alert dialog body (typically buttons)
    pub children: Markup,
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
            open: false,
        }
    }
}

/// Render an alert dialog trigger button that opens the dialog with the given target_id
pub fn trigger(target_id: &str, label: &str, variant: &str) -> Markup {
    html! {
        button type="button"
            class=(format!("mui-btn {} mui-btn--md", format!("mui-btn--{}", variant)))
            data-mui="alert-dialog-trigger"
            data-target=(target_id)
        {
            (label)
        }
    }
}

/// Render a close button for use inside the alert dialog
pub fn close_button(label: &str) -> Markup {
    html! {
        button type="button"
            class="mui-dialog__close"
            data-mui-close
            aria-label=(label)
        {
            "×"
        }
    }
}

/// Render an alert dialog with the given properties
pub fn render(props: Props) -> Markup {
    let title_id = format!("{}-title", props.id);
    let desc_id = format!("{}-desc", props.id);
    let aria_describedby = if props.description.is_some() { desc_id.clone() } else { String::new() };

    if aria_describedby.is_empty() {
        html! {
            dialog class="mui-alert-dialog"
                id=(props.id)
                data-mui="alert-dialog"
                role="alertdialog"
                aria-labelledby=(title_id)
                open[props.open]
            {
                div class="mui-dialog__header" {
                    h2 class="mui-dialog__title" id=(title_id) {
                        (props.title)
                    }
                    (close_button("Close"))
                }
                @if let Some(desc) = props.description {
                    p class="mui-dialog__description" id=(desc_id) {
                        (desc)
                    }
                }
                div class="mui-dialog__body" {
                    (props.children)
                }
            }
        }
    } else {
        html! {
            dialog class="mui-alert-dialog"
                id=(props.id)
                data-mui="alert-dialog"
                role="alertdialog"
                aria-labelledby=(title_id)
                aria-describedby=(aria_describedby)
                open[props.open]
            {
                div class="mui-dialog__header" {
                    h2 class="mui-dialog__title" id=(title_id) {
                        (props.title)
                    }
                    (close_button("Close"))
                }
                @if let Some(desc) = props.description {
                    p class="mui-dialog__description" id=(desc_id) {
                        (desc)
                    }
                }
                div class="mui-dialog__body" {
                    (props.children)
                }
            }
        }
    }
}

/// Showcase all alert dialog use cases
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            // Alert dialog with danger buttons
            {
                (trigger("demo-alert-1", "Delete account", "danger"))
            }
            {
                (render(Props {
                    id: "demo-alert-1".to_string(),
                    title: "Are you sure?".to_string(),
                    description: Some("This action cannot be undone.".to_string()),
                    children: html! {
                        div style="display: flex; gap: 0.75rem; margin-top: 1.5rem; justify-content: flex-end;" {
                            button class="mui-btn mui-btn--secondary" data-mui-close { "Cancel" }
                            button class="mui-btn mui-btn--danger" data-mui-close { "Delete" }
                        }
                    },
                    open: false,
                }))
            }
        }
    }
}
