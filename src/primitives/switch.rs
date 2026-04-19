//! Switch component — maud-ui Wave 2
use maud::{html, Markup};

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Size {
    #[default]
    Default,
    Sm,
}

#[derive(Clone, Debug)]
pub struct Props {
    pub name: String,
    pub id: String,
    pub label: String,
    pub checked: bool,
    pub disabled: bool,
    /// Explicit accessible name. Required when `label` is empty (e.g., when
    /// the switch sits next to an external label or description block).
    /// Falls back to `label` if not set.
    pub aria_label: Option<String>,
    /// Visual size variant. `Default` = 2.75rem × 1.5rem, `Sm` = 2rem × 1.125rem.
    pub size: Size,
    /// Propagates to the hidden `<input>` as `aria-invalid="true"` when set.
    pub aria_invalid: bool,
    /// Propagates to the hidden `<input>` as `required` when set.
    pub required: bool,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            name: "switch".to_string(),
            id: "switch".to_string(),
            label: "Toggle".to_string(),
            checked: false,
            disabled: false,
            aria_label: None,
            size: Size::Default,
            aria_invalid: false,
            required: false,
        }
    }
}

pub fn render(props: Props) -> Markup {
    let aria_checked = if props.checked { "true" } else { "false" };
    let value = if props.checked { "true" } else { "false" };
    let accessible_name = props
        .aria_label
        .clone()
        .unwrap_or_else(|| props.label.clone());
    let size_cls = match props.size {
        Size::Default => "",
        Size::Sm => "mui-switch--sm",
    };
    let switch_class = if size_cls.is_empty() {
        "mui-switch".to_string()
    } else {
        format!("mui-switch {}", size_cls)
    };

    html! {
        span class="mui-switch-wrap" {
            @if props.disabled {
                button type="button" class=(switch_class.clone()) role="switch"
                    aria-checked=(aria_checked)
                    aria-label=(accessible_name)
                    id=(props.id.clone())
                    data-mui="switch"
                    data-name=(props.name.clone())
                    disabled {
                    span class="mui-switch__thumb" aria-hidden="true";
                }
            } @else {
                button type="button" class=(switch_class.clone()) role="switch"
                    aria-checked=(aria_checked)
                    aria-label=(accessible_name)
                    id=(props.id.clone())
                    data-mui="switch"
                    data-name=(props.name.clone()) {
                    span class="mui-switch__thumb" aria-hidden="true";
                }
            }
            input type="hidden" name=(props.name.clone()) value=(value)
                class="mui-switch__value" aria-hidden="true"
                aria-invalid=[if props.aria_invalid { Some("true") } else { None }]
                required[props.required];
            @if !props.label.is_empty() {
                label for=(props.id) class="mui-switch__label" {
                    (props.label)
                }
            }
        }
    }
}

pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            // Realistic settings panel
            section {
                h2 { "Notification Settings" }
                div style="display:flex;flex-direction:column;gap:1rem;max-width:28rem;" {
                    // Marketing emails — off
                    div style="display:flex;align-items:flex-start;justify-content:space-between;gap:1rem;" {
                        div {
                            label for="sw-marketing" style="font-size:0.875rem;font-weight:500;color:var(--mui-text);display:block;" {
                                "Marketing emails"
                            }
                            span style="font-size:0.8125rem;color:var(--mui-text-muted);" {
                                "Receive emails about new products, features, and more."
                            }
                        }
                        (render(Props {
                            name: "marketing".to_string(),
                            id: "sw-marketing".to_string(),
                            label: String::new(),
                            checked: false,
                            disabled: false,
                            aria_label: Some("Marketing emails".to_string()),
                            ..Default::default()
                        }))
                    }
                    // Push notifications — on
                    div style="display:flex;align-items:flex-start;justify-content:space-between;gap:1rem;" {
                        div {
                            label for="sw-push" style="font-size:0.875rem;font-weight:500;color:var(--mui-text);display:block;" {
                                "Push notifications"
                            }
                            span style="font-size:0.8125rem;color:var(--mui-text-muted);" {
                                "Receive notifications directly on your device."
                            }
                        }
                        (render(Props {
                            name: "push".to_string(),
                            id: "sw-push".to_string(),
                            label: String::new(),
                            checked: true,
                            disabled: false,
                            aria_label: Some("Push notifications".to_string()),
                            ..Default::default()
                        }))
                    }
                    // Airplane mode — disabled
                    div style="display:flex;align-items:flex-start;justify-content:space-between;gap:1rem;opacity:0.6;" {
                        div {
                            label for="sw-airplane" style="font-size:0.875rem;font-weight:500;color:var(--mui-text);display:block;" {
                                "Airplane mode"
                            }
                            span style="font-size:0.8125rem;color:var(--mui-text-muted);" {
                                "Managed by your organization."
                            }
                        }
                        (render(Props {
                            name: "airplane".to_string(),
                            id: "sw-airplane".to_string(),
                            label: String::new(),
                            checked: false,
                            disabled: true,
                            aria_label: Some("Airplane mode".to_string()),
                            ..Default::default()
                        }))
                    }
                }
            }

            // Simple inline states
            section {
                h2 { "States" }
                div.mui-showcase__row {
                    (render(Props {
                        name: "demo-off".to_string(),
                        id: "demo-off".to_string(),
                        label: "Off".to_string(),
                        checked: false,
                        disabled: false,
                        aria_label: None,
                        ..Default::default()
                    }))
                    (render(Props {
                        name: "demo-on".to_string(),
                        id: "demo-on".to_string(),
                        label: "On".to_string(),
                        checked: true,
                        disabled: false,
                        aria_label: None,
                        ..Default::default()
                    }))
                    (render(Props {
                        name: "demo-disabled".to_string(),
                        id: "demo-disabled".to_string(),
                        label: "Disabled".to_string(),
                        checked: false,
                        disabled: true,
                        aria_label: None,
                        ..Default::default()
                    }))
                    (render(Props {
                        name: "demo-locked".to_string(),
                        id: "demo-locked".to_string(),
                        label: "Locked on".to_string(),
                        checked: true,
                        disabled: true,
                        aria_label: None,
                        ..Default::default()
                    }))
                }
            }

            // Size variant — Sm
            section {
                h2 { "Sizes" }
                p.mui-showcase__caption { "Default and compact (sm) variants." }
                div.mui-showcase__row {
                    (render(Props {
                        name: "size-default".to_string(),
                        id: "size-default".to_string(),
                        label: "Default".to_string(),
                        checked: true,
                        ..Default::default()
                    }))
                    (render(Props {
                        name: "size-sm".to_string(),
                        id: "size-sm".to_string(),
                        label: "Compact".to_string(),
                        checked: true,
                        size: Size::Sm,
                        ..Default::default()
                    }))
                }
            }

            // Form integration — required + aria-invalid
            section {
                h2 { "Form Integration" }
                p.mui-showcase__caption { "Required and invalid states propagate to the hidden input." }
                div.mui-showcase__row {
                    (render(Props {
                        name: "terms".to_string(),
                        id: "sw-terms".to_string(),
                        label: "Accept terms (required)".to_string(),
                        checked: false,
                        required: true,
                        ..Default::default()
                    }))
                    (render(Props {
                        name: "consent".to_string(),
                        id: "sw-consent".to_string(),
                        label: "Consent (invalid)".to_string(),
                        checked: false,
                        aria_invalid: true,
                        ..Default::default()
                    }))
                }
            }
        }
    }
}
