//! Toggle component — maud-ui Wave 2
use maud::{html, Markup};

#[derive(Clone, Debug)]
pub struct Props {
    pub label: String,
    pub pressed: bool,
    pub disabled: bool,
    pub id: String,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            label: "Toggle".to_string(),
            pressed: false,
            disabled: false,
            id: "toggle".to_string(),
        }
    }
}

pub fn render(props: Props) -> Markup {
    let aria_pressed = if props.pressed { "true" } else { "false" };

    html! {
        @if props.disabled {
            button type="button" class="mui-toggle"
                aria-pressed=(aria_pressed)
                id=(props.id.clone())
                data-mui="toggle"
                disabled {
                (props.label)
            }
        } @else {
            button type="button" class="mui-toggle"
                aria-pressed=(aria_pressed)
                id=(props.id.clone())
                data-mui="toggle" {
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
                        label: "B".to_string(),
                        pressed: false,
                        disabled: false,
                        id: "toggle-b".to_string(),
                    }))
                    (render(Props {
                        label: "I".to_string(),
                        pressed: true,
                        disabled: false,
                        id: "toggle-i".to_string(),
                    }))
                    (render(Props {
                        label: "U".to_string(),
                        pressed: true,
                        disabled: false,
                        id: "toggle-u".to_string(),
                    }))
                }
            }
            section {
                h2 { "Disabled" }
                div.mui-showcase__row {
                    (render(Props {
                        label: "Disabled".to_string(),
                        pressed: false,
                        disabled: true,
                        id: "toggle-disabled-1".to_string(),
                    }))
                    (render(Props {
                        label: "Disabled".to_string(),
                        pressed: true,
                        disabled: true,
                        id: "toggle-disabled-2".to_string(),
                    }))
                }
            }
        }
    }
}
