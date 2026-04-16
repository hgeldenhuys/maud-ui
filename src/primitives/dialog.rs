//! Dialog component — modal dialog using native <dialog> element with focus trap and ESC/backdrop close.
use maud::{html, Markup};

/// Dialog rendering properties
#[derive(Clone, Debug)]
pub struct Props {
    /// Unique identifier for the dialog (used by trigger to open it)
    pub id: String,
    /// Dialog title
    pub title: String,
    /// Optional description text displayed below title
    pub description: Option<String>,
    /// Markup content displayed in dialog body
    pub children: Markup,
    /// Optional footer markup (buttons/actions, rendered right-aligned)
    pub footer: Option<Markup>,
    /// Initial open state (default false; if true renders with open attribute for SSR)
    pub open: bool,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            id: "dialog".to_string(),
            title: "Dialog".to_string(),
            description: None,
            children: html! {},
            footer: None,
            open: false,
        }
    }
}

/// Render a dialog trigger button that opens the dialog with the given target_id
pub fn trigger(target_id: &str, label: &str) -> Markup {
    html! {
        button type="button"
            class="mui-btn mui-btn--default mui-btn--md"
            data-mui="dialog-trigger"
            data-target=(target_id)
        {
            (label)
        }
    }
}

/// Render a close button for use inside the dialog (positioned absolute via CSS)
pub fn close_button(label: &str) -> Markup {
    html! {
        button type="button"
            class="mui-dialog__close"
            data-mui-close
            aria-label=(label)
        {
            "\u{00d7}"
        }
    }
}

/// Render a dialog with the given properties
pub fn render(props: Props) -> Markup {
    let title_id = format!("{}-title", props.id);
    let desc_id = format!("{}-desc", props.id);
    let has_desc = props.description.is_some();

    html! {
        dialog class="mui-dialog"
            id=(props.id)
            data-mui="dialog"
            aria-labelledby=(title_id)
            aria-describedby=[if has_desc { Some(desc_id.as_str()) } else { None }]
            open[props.open]
        {
            // Close button — absolute positioned, outside header flow
            (close_button("Close"))

            div class="mui-dialog__header" {
                h2 class="mui-dialog__title" id=(title_id) {
                    (props.title)
                }
            }
            @if let Some(desc) = props.description {
                p class="mui-dialog__description" id=(desc_id) {
                    (desc)
                }
            }
            div class="mui-dialog__body" {
                (props.children)
            }
            @if let Some(footer) = props.footer {
                div class="mui-dialog__footer" {
                    (footer)
                }
            }
        }
    }
}

/// Showcase all dialog use cases
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            // Dialog with basic content and form
            {
                (trigger("demo-dialog-1", "Open dialog"))
            }
            {
                (render(Props {
                    id: "demo-dialog-1".to_string(),
                    title: "Edit profile".to_string(),
                    description: Some("Make changes to your profile here.".to_string()),
                    children: html! {
                        div class="mui-field" {
                            label class="mui-label" { "Name" }
                            input class="mui-input" type="text" placeholder="Your name" {}
                        }
                        div class="mui-field" {
                            label class="mui-label" { "Email" }
                            input class="mui-input" type="email" placeholder="your@email.com" {}
                        }
                    },
                    footer: Some(html! {
                        button class="mui-btn mui-btn--secondary mui-btn--md" data-mui-close { "Cancel" }
                        button class="mui-btn mui-btn--primary mui-btn--md" { "Save changes" }
                    }),
                    open: false,
                }))
            }
        }
    }
}
