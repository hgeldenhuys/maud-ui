//! Tabs component — maud-ui Wave 2

use maud::{html, Markup};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Variant {
    #[default]
    Default,
    Line,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ActivationMode {
    #[default]
    Automatic,
    Manual,
}

#[derive(Clone, Debug)]
pub struct Tab {
    pub id: String,
    pub label: String,
    pub content: Markup,
    pub disabled: bool,
}

impl Default for Tab {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            content: html! {},
            disabled: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Props {
    pub tabs: Vec<Tab>,
    pub default_active: usize,
    pub aria_label: String,
    pub orientation: Orientation,
    pub variant: Variant,
    pub activation_mode: ActivationMode,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            tabs: vec![],
            default_active: 0,
            aria_label: "Tabs".to_string(),
            orientation: Orientation::default(),
            variant: Variant::default(),
            activation_mode: ActivationMode::default(),
        }
    }
}

pub fn render(props: Props) -> Markup {
    let root_class = match props.orientation {
        Orientation::Horizontal => "mui-tabs",
        Orientation::Vertical => "mui-tabs mui-tabs--vertical",
    };
    let list_class = match props.variant {
        Variant::Default => "mui-tabs__list",
        Variant::Line => "mui-tabs__list mui-tabs__list--line",
    };
    let activation_mode = match props.activation_mode {
        ActivationMode::Automatic => "automatic",
        ActivationMode::Manual => "manual",
    };
    let is_vertical = matches!(props.orientation, Orientation::Vertical);

    html! {
        div class=(root_class) data-mui="tabs" data-activation-mode=(activation_mode) {
            div class=(list_class)
                role="tablist"
                aria-label=(props.aria_label)
                aria-orientation=[if is_vertical { Some("vertical") } else { None }]
            {
                @for (i, tab) in props.tabs.iter().enumerate() {
                    @let is_active = i == props.default_active && !tab.disabled;
                    @let tabindex = if is_active { "0" } else { "-1" };
                    button type="button"
                        class="mui-tabs__trigger"
                        role="tab"
                        id=(format!("{}-trigger", tab.id))
                        aria-controls=(format!("{}-panel", tab.id))
                        aria-selected=(if is_active { "true" } else { "false" })
                        aria-disabled=[if tab.disabled { Some("true") } else { None }]
                        disabled[tab.disabled]
                        tabindex=(tabindex) {
                        (tab.label)
                    }
                }
            }
            div class="mui-tabs__panels" {
                @for (i, tab) in props.tabs.iter().enumerate() {
                    @let is_active = i == props.default_active && !tab.disabled;
                    @let hidden = !is_active;
                    div class="mui-tabs__panel"
                        role="tabpanel"
                        id=(format!("{}-panel", tab.id))
                        aria-labelledby=(format!("{}-trigger", tab.id))
                        tabindex=[if !hidden { Some("0") } else { None }]
                        hidden[hidden]
                    {
                        (tab.content)
                    }
                }
            }
        }
    }
}

pub fn showcase() -> Markup {
    use crate::primitives::{button, input, label};

    let tabs = vec![
        Tab {
            id: "account".to_string(),
            label: "Account".to_string(),
            content: html! {
                div style="padding:1rem 0;" {
                    p style="font-size:0.875rem;color:var(--mui-text-muted);margin:0 0 1rem;" {
                        "Make changes to your account here. Click save when you\u{2019}re done."
                    }
                    div style="display:flex;flex-direction:column;gap:0.75rem;max-width:24rem;" {
                        div class="mui-field" {
                            (label::render(label::Props {
                                text: "Name".into(),
                                html_for: Some("tab-account-name".into()),
                                ..Default::default()
                            }))
                            (input::render(input::Props {
                                name: "name".into(),
                                id: "tab-account-name".into(),
                                input_type: input::InputType::Text,
                                value: "Pedro Duarte".into(),
                                ..Default::default()
                            }))
                        }
                        div class="mui-field" {
                            (label::render(label::Props {
                                text: "Email".into(),
                                html_for: Some("tab-account-email".into()),
                                ..Default::default()
                            }))
                            (input::render(input::Props {
                                name: "email".into(),
                                id: "tab-account-email".into(),
                                input_type: input::InputType::Email,
                                value: "pedro@example.com".into(),
                                ..Default::default()
                            }))
                        }
                        div style="display:flex;justify-content:flex-end;margin-top:0.5rem;" {
                            (button::render(button::Props {
                                label: "Save changes".into(),
                                variant: button::Variant::Primary,
                                size: button::Size::Md,
                                ..Default::default()
                            }))
                        }
                    }
                }
            },
            disabled: false,
        },
        Tab {
            id: "password".to_string(),
            label: "Password".to_string(),
            content: html! {
                div style="padding:1rem 0;" {
                    p style="font-size:0.875rem;color:var(--mui-text-muted);margin:0 0 1rem;" {
                        "Change your password here. After saving, you\u{2019}ll be logged out."
                    }
                    div style="display:flex;flex-direction:column;gap:0.75rem;max-width:24rem;" {
                        div class="mui-field" {
                            (label::render(label::Props {
                                text: "Current password".into(),
                                html_for: Some("tab-pw-current".into()),
                                ..Default::default()
                            }))
                            (input::render(input::Props {
                                name: "current_password".into(),
                                id: "tab-pw-current".into(),
                                input_type: input::InputType::Password,
                                placeholder: "Enter current password".into(),
                                ..Default::default()
                            }))
                        }
                        div class="mui-field" {
                            (label::render(label::Props {
                                text: "New password".into(),
                                html_for: Some("tab-pw-new".into()),
                                ..Default::default()
                            }))
                            (input::render(input::Props {
                                name: "new_password".into(),
                                id: "tab-pw-new".into(),
                                input_type: input::InputType::Password,
                                placeholder: "Enter new password".into(),
                                ..Default::default()
                            }))
                        }
                        div style="display:flex;justify-content:flex-end;margin-top:0.5rem;" {
                            (button::render(button::Props {
                                label: "Change password".into(),
                                variant: button::Variant::Primary,
                                size: button::Size::Md,
                                ..Default::default()
                            }))
                        }
                    }
                }
            },
            disabled: false,
        },
        Tab {
            id: "team".to_string(),
            label: "Team".to_string(),
            content: html! {
                div style="padding:1rem 0;" {
                    p style="font-size:0.875rem;color:var(--mui-text-muted);margin:0 0 1rem;" {
                        "Invite your team members to collaborate."
                    }
                    div style="display:flex;flex-direction:column;gap:0.75rem;max-width:28rem;" {
                        @for (name, email, role) in [
                            ("Sofia Davis", "sofia@example.com", "Owner"),
                            ("Jackson Lee", "jackson@example.com", "Member"),
                            ("Isabella Nguyen", "isabella@example.com", "Member"),
                        ] {
                            div style="display:flex;align-items:center;justify-content:space-between;padding:0.5rem 0;border-bottom:1px solid var(--mui-border,#e5e7eb);" {
                                div {
                                    p style="font-size:0.875rem;font-weight:500;margin:0;" { (name) }
                                    p style="font-size:0.8125rem;color:var(--mui-text-muted);margin:0.125rem 0 0;" { (email) }
                                }
                                span style="font-size:0.75rem;color:var(--mui-text-muted);padding:0.25rem 0.5rem;border:1px solid var(--mui-border,#e5e7eb);border-radius:0.375rem;" {
                                    (role)
                                }
                            }
                        }
                    }
                }
            },
            disabled: false,
        },
    ];

    let default_tabs = tabs.clone();
    let line_tabs = tabs.clone();

    let mut vertical_tabs = tabs.clone();
    // Mark one tab disabled in the vertical demo
    if let Some(last) = vertical_tabs.last_mut() {
        last.disabled = true;
    }

    html! {
        div style="display:flex;flex-direction:column;gap:2.5rem;" {
            section {
                h4 style="font-size:0.875rem;font-weight:600;margin:0 0 0.75rem;color:var(--mui-text);" {
                    "Default (horizontal, segmented)"
                }
                (render(Props {
                    tabs: default_tabs,
                    default_active: 0,
                    aria_label: "Account settings".to_string(),
                    ..Default::default()
                }))
            }

            section {
                h4 style="font-size:0.875rem;font-weight:600;margin:0 0 0.75rem;color:var(--mui-text);" {
                    "Line variant (underline)"
                }
                (render(Props {
                    tabs: line_tabs,
                    default_active: 0,
                    aria_label: "Account settings (line)".to_string(),
                    variant: Variant::Line,
                    ..Default::default()
                }))
            }

            section {
                h4 style="font-size:0.875rem;font-weight:600;margin:0 0 0.75rem;color:var(--mui-text);" {
                    "Vertical orientation + disabled tab + manual activation"
                }
                (render(Props {
                    tabs: vertical_tabs,
                    default_active: 0,
                    aria_label: "Account settings (vertical)".to_string(),
                    orientation: Orientation::Vertical,
                    activation_mode: ActivationMode::Manual,
                    ..Default::default()
                }))
            }
        }
    }
}
