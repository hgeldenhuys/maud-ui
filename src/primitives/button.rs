//! Button component — maud-ui Wave 1

use maud::{html, Markup};

#[derive(Clone, Debug)]
pub struct Props {
    pub label: String,
    pub variant: Variant,
    pub size: Size,
    pub disabled: bool,
    pub button_type: &'static str,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            label: "Button".to_string(),
            variant: Variant::Default,
            size: Size::Md,
            disabled: false,
            button_type: "button",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Variant {
    Default,
    Primary,
    Secondary,
    Outline,
    Ghost,
    Danger,
    Link,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Size {
    Sm,
    Md,
    Lg,
    Icon,
}

impl Variant {
    fn class_name(self) -> &'static str {
        match self {
            Variant::Default => "mui-btn--default",
            Variant::Primary => "mui-btn--primary",
            Variant::Secondary => "mui-btn--secondary",
            Variant::Outline => "mui-btn--outline",
            Variant::Ghost => "mui-btn--ghost",
            Variant::Danger => "mui-btn--danger",
            Variant::Link => "mui-btn--link",
        }
    }
}

impl Size {
    fn class_name(self) -> &'static str {
        match self {
            Size::Sm => "mui-btn--sm",
            Size::Md => "mui-btn--md",
            Size::Lg => "mui-btn--lg",
            Size::Icon => "mui-btn--icon",
        }
    }
}

pub fn render(props: Props) -> Markup {
    let disabled_attr = if props.disabled {
        "true"
    } else {
        "false"
    };

    let class = format!(
        "mui-btn {} {}",
        props.variant.class_name(),
        props.size.class_name()
    );

    html! {
        button class=(class) type=(props.button_type) aria-disabled=(disabled_attr) {
            (props.label)
        }
    }
}

pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            section {
                h2 { "Variants" }
                div.mui-showcase__row {
                    (render(Props {
                        label: "Default".to_string(),
                        variant: Variant::Default,
                        size: Size::Md,
                        disabled: false,
                        button_type: "button",
                    }))
                    (render(Props {
                        label: "Primary".to_string(),
                        variant: Variant::Primary,
                        size: Size::Md,
                        disabled: false,
                        button_type: "button",
                    }))
                    (render(Props {
                        label: "Secondary".to_string(),
                        variant: Variant::Secondary,
                        size: Size::Md,
                        disabled: false,
                        button_type: "button",
                    }))
                    (render(Props {
                        label: "Outline".to_string(),
                        variant: Variant::Outline,
                        size: Size::Md,
                        disabled: false,
                        button_type: "button",
                    }))
                    (render(Props {
                        label: "Ghost".to_string(),
                        variant: Variant::Ghost,
                        size: Size::Md,
                        disabled: false,
                        button_type: "button",
                    }))
                    (render(Props {
                        label: "Danger".to_string(),
                        variant: Variant::Danger,
                        size: Size::Md,
                        disabled: false,
                        button_type: "button",
                    }))
                    (render(Props {
                        label: "Link".to_string(),
                        variant: Variant::Link,
                        size: Size::Md,
                        disabled: false,
                        button_type: "button",
                    }))
                }
            }
            section {
                h2 { "Sizes" }
                div.mui-showcase__row {
                    (render(Props {
                        label: "Sm".to_string(),
                        variant: Variant::Primary,
                        size: Size::Sm,
                        disabled: false,
                        button_type: "button",
                    }))
                    (render(Props {
                        label: "Md".to_string(),
                        variant: Variant::Primary,
                        size: Size::Md,
                        disabled: false,
                        button_type: "button",
                    }))
                    (render(Props {
                        label: "Lg".to_string(),
                        variant: Variant::Primary,
                        size: Size::Lg,
                        disabled: false,
                        button_type: "button",
                    }))
                    (render(Props {
                        label: "+".to_string(),
                        variant: Variant::Primary,
                        size: Size::Icon,
                        disabled: false,
                        button_type: "button",
                    }))
                }
            }
            section {
                h2 { "Disabled" }
                div.mui-showcase__row {
                    (render(Props {
                        label: "Primary Disabled".to_string(),
                        variant: Variant::Primary,
                        size: Size::Md,
                        disabled: true,
                        button_type: "button",
                    }))
                    (render(Props {
                        label: "Outline Disabled".to_string(),
                        variant: Variant::Outline,
                        size: Size::Md,
                        disabled: true,
                        button_type: "button",
                    }))
                }
            }
        }
    }
}
