//! Toggle component — two-state button (pressed/unpressed)
use maud::{html, Markup};

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Size {
    #[default]
    Md,
    Sm,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Variant {
    #[default]
    Default,
    Outline,
}

#[derive(Clone, Debug)]
pub struct Props {
    pub label: String,
    pub pressed: bool,
    pub disabled: bool,
    pub id: String,
    pub size: Size,
    pub variant: Variant,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            label: "Toggle".to_string(),
            pressed: false,
            disabled: false,
            id: "toggle".to_string(),
            size: Size::Md,
            variant: Variant::Default,
        }
    }
}

pub fn render(props: Props) -> Markup {
    let aria_pressed = if props.pressed { "true" } else { "false" };
    let size_cls = match props.size {
        Size::Md => "mui-toggle--md",
        Size::Sm => "mui-toggle--sm",
    };
    let variant_cls = match props.variant {
        Variant::Default => "",
        Variant::Outline => "mui-toggle--outline",
    };

    html! {
        button type="button"
            class={"mui-toggle " (size_cls) " " (variant_cls)}
            aria-pressed=(aria_pressed)
            id=(props.id)
            data-mui="toggle"
            disabled[props.disabled]
        {
            (props.label)
        }
    }
}

pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            section {
                h2 { "Default variant" }
                div.mui-showcase__row {
                    (render(Props {
                        label: "B".to_string(),
                        pressed: false,
                        id: "toggle-b".to_string(),
                        ..Default::default()
                    }))
                    (render(Props {
                        label: "I".to_string(),
                        pressed: true,
                        id: "toggle-i".to_string(),
                        ..Default::default()
                    }))
                    (render(Props {
                        label: "U".to_string(),
                        pressed: true,
                        id: "toggle-u".to_string(),
                        ..Default::default()
                    }))
                }
            }
            section {
                h2 { "Outline variant" }
                div.mui-showcase__row {
                    (render(Props {
                        label: "B".to_string(),
                        pressed: false,
                        variant: Variant::Outline,
                        id: "toggle-outline-b".to_string(),
                        ..Default::default()
                    }))
                    (render(Props {
                        label: "I".to_string(),
                        pressed: true,
                        variant: Variant::Outline,
                        id: "toggle-outline-i".to_string(),
                        ..Default::default()
                    }))
                }
            }
            section {
                h2 { "Small size" }
                div.mui-showcase__row {
                    (render(Props {
                        label: "On".to_string(),
                        pressed: true,
                        size: Size::Sm,
                        id: "toggle-sm-on".to_string(),
                        ..Default::default()
                    }))
                    (render(Props {
                        label: "Off".to_string(),
                        size: Size::Sm,
                        id: "toggle-sm-off".to_string(),
                        ..Default::default()
                    }))
                }
            }
            section {
                h2 { "Disabled" }
                div.mui-showcase__row {
                    (render(Props {
                        label: "Disabled".to_string(),
                        disabled: true,
                        id: "toggle-disabled-1".to_string(),
                        ..Default::default()
                    }))
                    (render(Props {
                        label: "Disabled".to_string(),
                        pressed: true,
                        disabled: true,
                        id: "toggle-disabled-2".to_string(),
                        ..Default::default()
                    }))
                }
            }
        }
    }
}
