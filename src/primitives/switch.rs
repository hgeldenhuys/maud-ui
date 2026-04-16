//! Switch component — maud-ui Wave 2
use maud::{html, Markup};

#[derive(Clone, Debug)]
pub struct Props {
    pub name: String,
    pub id: String,
    pub label: String,
    pub checked: bool,
    pub disabled: bool,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            name: "switch".to_string(),
            id: "switch".to_string(),
            label: "Toggle".to_string(),
            checked: false,
            disabled: false,
        }
    }
}

pub fn render(props: Props) -> Markup {
    let aria_checked = if props.checked { "true" } else { "false" };
    let value = if props.checked { "true" } else { "false" };

    html! {
        span class="mui-switch-wrap" {
            @if props.disabled {
                button type="button" class="mui-switch" role="switch"
                    aria-checked=(aria_checked)
                    id=(props.id.clone())
                    data-mui="switch"
                    data-name=(props.name.clone())
                    disabled {
                    span class="mui-switch__thumb" aria-hidden="true";
                }
            } @else {
                button type="button" class="mui-switch" role="switch"
                    aria-checked=(aria_checked)
                    id=(props.id.clone())
                    data-mui="switch"
                    data-name=(props.name.clone()) {
                    span class="mui-switch__thumb" aria-hidden="true";
                }
            }
            input type="hidden" name=(props.name.clone()) value=(value)
                class="mui-switch__value" aria-hidden="true";
            label for=(props.id) class="mui-switch__label" {
                (props.label)
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
                    }))
                    (render(Props {
                        name: "demo-on".to_string(),
                        id: "demo-on".to_string(),
                        label: "On".to_string(),
                        checked: true,
                        disabled: false,
                    }))
                    (render(Props {
                        name: "demo-disabled".to_string(),
                        id: "demo-disabled".to_string(),
                        label: "Disabled".to_string(),
                        checked: false,
                        disabled: true,
                    }))
                    (render(Props {
                        name: "demo-locked".to_string(),
                        id: "demo-locked".to_string(),
                        label: "Locked on".to_string(),
                        checked: true,
                        disabled: true,
                    }))
                }
            }
        }
    }
}
