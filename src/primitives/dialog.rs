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
            aria-controls=(target_id) aria-haspopup="dialog"
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
                        div style="display:flex;align-items:center;gap: var(--mui-space-lg);margin-bottom:var(--mui-space-lg);" {
                            div data-mui-type="h3" style="width:3rem;height:3rem;border-radius: var(--mui-radius-full);background:var(--mui-bg-input);display:flex;align-items:center;justify-content:center;flex-shrink:0;" {
                                "JD"
                            }
                            p data-mui-type="small" style="color:var(--mui-text-muted);" {
                                "Upload a new avatar from your device."
                            }
                        }
                        div class="mui-field" {
                            label class="mui-label" for="demo-dialog-profile-name" { "Name" }
                            input id="demo-dialog-profile-name" class="mui-input" type="text" value="Jane Doe" {}
                        }
                        div class="mui-field" {
                            label class="mui-label" for="demo-dialog-profile-email" { "Email" }
                            input id="demo-dialog-profile-email" class="mui-input" type="email" value="jane@example.com" {}
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
                            label class="mui-label" for="demo-dialog-share-email" { "Email address" }
                            input id="demo-dialog-share-email" class="mui-input" type="email" placeholder="collaborator@company.com" {}
                        }
                        div class="mui-field" {
                            label class="mui-label" for="demo-dialog-share-permission" { "Permission" }
                            select id="demo-dialog-share-permission" class="mui-select__trigger" style="width:100%;" {
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
                        p data-mui-type="small" style="color:var(--mui-text-muted);" {
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

            {
                (trigger("demo-dialog-review", "Review a long document"))
                (render(Props {
                    id: "demo-dialog-review".into(),
                    title: "Review the arrival instructions before sharing with your guests".into(),
                    description: Some("The title leaves space for Close; only the document body scrolls.".into()),
                    children: html! {
                        @for section in 1..=12 {
                            h3 { "Arrival detail " (section) }
                            p { "Confirm the guest name, arrival time and contact details. Share the access instructions with everyone on the reservation and keep a copy available at the front desk." }
                        }
                    },
                    footer: Some(html! { button type="button" class="mui-btn mui-btn--primary mui-btn--md" data-mui-close { "Done reviewing" } }),
                    ..Default::default()
                }))
            }

            // A footer-only variant still supports Escape and cancellation.
            {
                (trigger("demo-dialog-forced-choice", "Footer actions"))
            }
            {
                (render(Props {
                    id: "demo-dialog-forced-choice".to_string(),
                    title: "Delete the shared document and its comments?".to_string(),
                    description: Some("Review this action before deleting. You can cancel or press Escape.".to_string()),
                    children: html! {
                        p data-mui-type="small" style="color:var(--mui-text-muted);" {
                            "The close icon is optional. Escape and the Cancel action still dismiss this dialog."
                        }
                    },
                    footer: Some(html! {
                        button class="mui-btn mui-btn--secondary mui-btn--md" data-mui-close { "Cancel" }
                        button class="mui-btn mui-btn--danger mui-btn--md" data-mui-close { "Delete document" }
                    }),
                    show_close_button: false,
                    ..Default::default()
                }))
            }
        }
    }
}
