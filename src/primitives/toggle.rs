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
            // Text-editor toolbar context
            section {
                h2 { "Text Editor Toolbar" }
                p.mui-showcase__caption { "Default variant — formatting controls" }
                div.mui-showcase__row {
                    (render(Props {
                        label: "B".to_string(),
                        pressed: true,
                        id: "toolbar-bold".to_string(),
                        ..Default::default()
                    }))
                    (render(Props {
                        label: "I".to_string(),
                        pressed: false,
                        id: "toolbar-italic".to_string(),
                        ..Default::default()
                    }))
                    (render(Props {
                        label: "U".to_string(),
                        pressed: false,
                        id: "toolbar-underline".to_string(),
                        ..Default::default()
                    }))
                }
                p.mui-showcase__caption { "Outline variant" }
                div.mui-showcase__row {
                    (render(Props {
                        label: "B".to_string(),
                        pressed: true,
                        variant: Variant::Outline,
                        id: "toolbar-outline-bold".to_string(),
                        ..Default::default()
                    }))
                    (render(Props {
                        label: "I".to_string(),
                        pressed: false,
                        variant: Variant::Outline,
                        id: "toolbar-outline-italic".to_string(),
                        ..Default::default()
                    }))
                    (render(Props {
                        label: "U".to_string(),
                        pressed: false,
                        variant: Variant::Outline,
                        id: "toolbar-outline-underline".to_string(),
                        ..Default::default()
                    }))
                }
            }

            // Sizes
            section {
                h2 { "Sizes" }
                div.mui-showcase__row {
                    span.mui-showcase__label { "md" }
                    (render(Props {
                        label: "B".to_string(),
                        pressed: true,
                        id: "size-md-b".to_string(),
                        ..Default::default()
                    }))
                    (render(Props {
                        label: "I".to_string(),
                        pressed: false,
                        id: "size-md-i".to_string(),
                        ..Default::default()
                    }))
                }
                div.mui-showcase__row {
                    span.mui-showcase__label { "sm" }
                    (render(Props {
                        label: "B".to_string(),
                        pressed: true,
                        size: Size::Sm,
                        id: "size-sm-b".to_string(),
                        ..Default::default()
                    }))
                    (render(Props {
                        label: "I".to_string(),
                        pressed: false,
                        size: Size::Sm,
                        id: "size-sm-i".to_string(),
                        ..Default::default()
                    }))
                }
            }

            // Disabled
            section {
                h2 { "Disabled" }
                div.mui-showcase__row {
                    (render(Props {
                        label: "Off".to_string(),
                        disabled: true,
                        id: "toggle-dis-off".to_string(),
                        ..Default::default()
                    }))
                    (render(Props {
                        label: "On".to_string(),
                        pressed: true,
                        disabled: true,
                        id: "toggle-dis-on".to_string(),
                        ..Default::default()
                    }))
                }
            }
        }
    }
}
