//! Dialog component — modal dialog using native <dialog> element with focus trap and ESC/backdrop close.
use maud::{html, Markup};

/// Dialog size variants
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Size {
    /// Default size (max-width 32rem, padded)
    #[default]
    Default,
    /// Compact size (max-width 28rem, reduced padding)
    Sm,
}

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
    /// Whether to render the close button in the top-right (default true)
    pub show_close_button: bool,
    /// Dialog size variant (default Default)
    pub size: Size,
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
            show_close_button: true,
            size: Size::Default,
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
    let dialog_class = match props.size {
        Size::Default => "mui-dialog",
        Size::Sm => "mui-dialog mui-dialog--sm",
    };

    html! {
        dialog class=(dialog_class)
            id=(props.id)
            data-mui="dialog"
            aria-labelledby=(title_id)
            aria-describedby=[if has_desc { Some(desc_id.as_str()) } else { None }]
            aria-modal="true"
            open[props.open]
        {
            // Close button — absolute positioned, outside header flow
            @if props.show_close_button {
                (close_button("Close"))
            }

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
            // Edit Profile dialog
            {
                (trigger("demo-dialog-edit-profile", "Edit Profile"))
            }
            {
                (render(Props {
                    id: "demo-dialog-edit-profile".to_string(),
                    title: "Edit Profile".to_string(),
                    description: Some("Update your personal information below.".to_string()),
                    children: html! {
                        div style="display:flex;align-items:center;gap:1rem;margin-bottom:1rem;" {
                            div style="width:3rem;height:3rem;border-radius:50%;background:var(--mui-muted);display:flex;align-items:center;justify-content:center;font-size:1.25rem;flex-shrink:0;" {
                                "JD"
                            }
                            p style="font-size:0.875rem;color:var(--mui-muted-fg);" {
                                "Upload a new avatar from your device."
                            }
                        }
                        div class="mui-field" {
                            label class="mui-label" { "Name" }
                            input class="mui-input" type="text" value="Jane Doe" {}
                        }
                        div class="mui-field" {
                            label class="mui-label" { "Email" }
                            input class="mui-input" type="email" value="jane@example.com" {}
                        }
                    },
                    footer: Some(html! {
                        button class="mui-btn mui-btn--secondary mui-btn--md" data-mui-close { "Cancel" }
                        button class="mui-btn mui-btn--primary mui-btn--md" { "Save" }
                    }),
                    ..Default::default()
                }))
            }

            // Share Document dialog
            {
                (trigger("demo-dialog-share-doc", "Share Document"))
            }
            {
                (render(Props {
                    id: "demo-dialog-share-doc".to_string(),
                    title: "Share Document".to_string(),
                    description: Some("Invite a collaborator by email address.".to_string()),
                    children: html! {
                        div class="mui-field" {
                            label class="mui-label" { "Email address" }
                            input class="mui-input" type="email" placeholder="collaborator@company.com" {}
                        }
                        div class="mui-field" {
                            label class="mui-label" { "Permission" }
                            select class="mui-select__trigger" style="width:100%;" {
                                option value="viewer" { "Viewer" }
                                option value="editor" { "Editor" }
                            }
                        }
                    },
                    footer: Some(html! {
                        button class="mui-btn mui-btn--secondary mui-btn--md" data-mui-close { "Cancel" }
                        button class="mui-btn mui-btn--primary mui-btn--md" { "Send invite" }
                    }),
                    ..Default::default()
                }))
            }

            // Compact (Sm) dialog — demonstrates size: Sm
            {
                (trigger("demo-dialog-compact", "Compact Dialog"))
            }
            {
                (render(Props {
                    id: "demo-dialog-compact".to_string(),
                    title: "Quick Action".to_string(),
                    description: Some("A compact dialog for short interactions.".to_string()),
                    children: html! {
                        p style="font-size:0.875rem;color:var(--mui-text-muted);" {
                            "This dialog uses the Sm size variant with reduced padding."
                        }
                    },
                    footer: Some(html! {
                        button class="mui-btn mui-btn--primary mui-btn--md" data-mui-close { "OK" }
                    }),
                    size: Size::Sm,
                    ..Default::default()
                }))
            }

            // No-close-button dialog — force a footer-only decision
            {
                (trigger("demo-dialog-forced-choice", "Forced Choice"))
            }
            {
                (render(Props {
                    id: "demo-dialog-forced-choice".to_string(),
                    title: "Confirm Deletion".to_string(),
                    description: Some("This cannot be undone. Make a choice below.".to_string()),
                    children: html! {
                        p style="font-size:0.875rem;color:var(--mui-text-muted);" {
                            "The top-right close button is hidden — the user must use the footer buttons."
                        }
                    },
                    footer: Some(html! {
                        button class="mui-btn mui-btn--secondary mui-btn--md" data-mui-close { "Cancel" }
                        button class="mui-btn mui-btn--primary mui-btn--md" data-mui-close { "Delete" }
                    }),
                    show_close_button: false,
                    ..Default::default()
                }))
            }
        }
    }
}
