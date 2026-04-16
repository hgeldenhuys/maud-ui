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
            section {
                h2 { "States" }
                div.mui-showcase__row {
                    (render(Props {
                        name: "demo-sw-1".to_string(),
                        id: "demo-sw-1".to_string(),
                        label: "Airplane mode".to_string(),
                        checked: false,
                        disabled: false,
                    }))
                    (render(Props {
                        name: "demo-sw-2".to_string(),
                        id: "demo-sw-2".to_string(),
                        label: "Notifications".to_string(),
                        checked: true,
                        disabled: false,
                    }))
                    (render(Props {
                        name: "demo-sw-3".to_string(),
                        id: "demo-sw-3".to_string(),
                        label: "Offline mode".to_string(),
                        checked: false,
                        disabled: true,
                    }))
                }
            }
        }
    }
}
