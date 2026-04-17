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
                h2 { "Form actions" }
                p.mui-showcase__caption { "Primary/secondary pairing for settings, onboarding, checkout." }
                div.mui-showcase__row {
                    (render(Props {
                        label: "Save changes".to_string(),
                        variant: Variant::Primary,
                        size: Size::Md,
                        disabled: false,
                        button_type: "submit",
                    }))
                    (render(Props {
                        label: "Continue to billing".to_string(),
                        variant: Variant::Primary,
                        size: Size::Md,
                        disabled: false,
                        button_type: "button",
                    }))
                    (render(Props {
                        label: "Cancel".to_string(),
                        variant: Variant::Outline,
                        size: Size::Md,
                        disabled: false,
                        button_type: "button",
                    }))
                }
            }
            section {
                h2 { "Destructive" }
                p.mui-showcase__caption { "Irreversible actions — only after a confirm dialog." }
                div.mui-showcase__row {
                    (render(Props {
                        label: "Delete account".to_string(),
                        variant: Variant::Danger,
                        size: Size::Md,
                        disabled: false,
                        button_type: "button",
                    }))
                    (render(Props {
                        label: "Revoke API key".to_string(),
                        variant: Variant::Danger,
                        size: Size::Sm,
                        disabled: false,
                        button_type: "button",
                    }))
                }
            }
            section {
                h2 { "Loading state" }
                p.mui-showcase__caption { "Disabled + spinner label while awaiting a response." }
                div.mui-showcase__row {
                    (render(Props {
                        label: "\u{27f3}  Signing in\u{2026}".to_string(),
                        variant: Variant::Primary,
                        size: Size::Md,
                        disabled: true,
                        button_type: "button",
                    }))
                    (render(Props {
                        label: "\u{27f3}  Deploying\u{2026}".to_string(),
                        variant: Variant::Secondary,
                        size: Size::Md,
                        disabled: true,
                        button_type: "button",
                    }))
                }
            }
            section {
                h2 { "Icon + text" }
                p.mui-showcase__caption { "Leading glyph for recognition at a glance." }
                div.mui-showcase__row {
                    (render(Props {
                        label: "\u{2795}  Invite teammate".to_string(),
                        variant: Variant::Primary,
                        size: Size::Md,
                        disabled: false,
                        button_type: "button",
                    }))
                    (render(Props {
                        label: "\u{f09b}  GitHub".to_string(),
                        variant: Variant::Outline,
                        size: Size::Md,
                        disabled: false,
                        button_type: "button",
                    }))
                    (render(Props {
                        label: "+".to_string(),
                        variant: Variant::Outline,
                        size: Size::Icon,
                        disabled: false,
                        button_type: "button",
                    }))
                }
            }
        }
    }
}
