//! Button component — maud-ui Wave 1

use maud::{html, Markup, PreEscaped};

/// Inline SVG plus icon (16x16, stroke=currentColor) for use in leading_icon.
fn icon_plus() -> Markup {
    PreEscaped(r#"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 5v14"/><path d="M5 12h14"/></svg>"#.to_string())
}

/// Inline SVG GitHub icon (16x16, stroke=currentColor).
fn icon_github() -> Markup {
    PreEscaped(r#"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.4-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4"/><path d="M9 18c-4.51 2-5-2-7-2"/></svg>"#.to_string())
}

/// Inline SVG loader spinner (16x16, stroke=currentColor) — self-animates via `.mui-spin` class.
fn icon_spinner() -> Markup {
    PreEscaped(r#"<svg xmlns="http://www.w3.org/2000/svg" class="mui-spin" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>"#.to_string())
}

#[derive(Clone, Debug)]
pub struct Props {
    pub label: String,
    pub variant: Variant,
    pub size: Size,
    pub disabled: bool,
    pub button_type: &'static str,
    /// Optional leading icon (SVG markup). Use `stroke="currentColor"` so it
    /// inherits the button's text color — emoji characters do NOT inherit
    /// color and will render in OS system colors.
    pub leading_icon: Option<Markup>,
    /// aria-label override. Required for icon-only buttons (where `label` is
    /// empty) so screen readers announce the button's purpose.
    pub aria_label: Option<String>,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            label: "Button".to_string(),
            variant: Variant::Default,
            size: Size::Md,
            disabled: false,
            button_type: "button",
            leading_icon: None,
            aria_label: None,
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
    let disabled_attr = if props.disabled { "true" } else { "false" };

    let class = format!(
        "mui-btn {} {}",
        props.variant.class_name(),
        props.size.class_name()
    );

    html! {
        @if let Some(label) = &props.aria_label {
            button class=(class) type=(props.button_type) aria-disabled=(disabled_attr) aria-label=(label) {
                @if let Some(icon) = &props.leading_icon {
                    span.mui-btn__icon aria-hidden="true" { (icon) }
                }
                (props.label)
            }
        } @else {
            button class=(class) type=(props.button_type) aria-disabled=(disabled_attr) {
                @if let Some(icon) = &props.leading_icon {
                    span.mui-btn__icon aria-hidden="true" { (icon) }
                }
                (props.label)
            }
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
                        leading_icon: None,
                        aria_label: None,
                    }))
                    (render(Props {
                        label: "Continue to billing".to_string(),
                        variant: Variant::Primary,
                        size: Size::Md,
                        disabled: false,
                        button_type: "button",
                        leading_icon: None,
                        aria_label: None,
                    }))
                    (render(Props {
                        label: "Cancel".to_string(),
                        variant: Variant::Outline,
                        size: Size::Md,
                        disabled: false,
                        button_type: "button",
                        leading_icon: None,
                        aria_label: None,
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
                        leading_icon: None,
                        aria_label: None,
                    }))
                    (render(Props {
                        label: "Revoke API key".to_string(),
                        variant: Variant::Danger,
                        size: Size::Sm,
                        disabled: false,
                        button_type: "button",
                        leading_icon: None,
                        aria_label: None,
                    }))
                }
            }
            section {
                h2 { "Loading state" }
                p.mui-showcase__caption { "Disabled + spinner icon while awaiting a response." }
                div.mui-showcase__row {
                    (render(Props {
                        label: "Signing in\u{2026}".to_string(),
                        variant: Variant::Primary,
                        size: Size::Md,
                        disabled: true,
                        button_type: "button",
                        leading_icon: Some(icon_spinner()),
                        aria_label: None,
                    }))
                    (render(Props {
                        label: "Deploying\u{2026}".to_string(),
                        variant: Variant::Secondary,
                        size: Size::Md,
                        disabled: true,
                        button_type: "button",
                        leading_icon: Some(icon_spinner()),
                        aria_label: None,
                    }))
                }
            }
            section {
                h2 { "Icon + text" }
                p.mui-showcase__caption { "Leading glyph for recognition at a glance." }
                div.mui-showcase__row {
                    (render(Props {
                        label: "Invite teammate".to_string(),
                        variant: Variant::Primary,
                        size: Size::Md,
                        disabled: false,
                        button_type: "button",
                        leading_icon: Some(icon_plus()),
                        aria_label: None,
                    }))
                    (render(Props {
                        label: "GitHub".to_string(),
                        variant: Variant::Outline,
                        size: Size::Md,
                        disabled: false,
                        button_type: "button",
                        leading_icon: Some(icon_github()),
                        aria_label: None,
                    }))
                    (render(Props {
                        label: String::new(),
                        variant: Variant::Outline,
                        size: Size::Icon,
                        disabled: false,
                        button_type: "button",
                        leading_icon: Some(icon_plus()),
                        aria_label: Some("Add item".to_string()),
                    }))
                }
            }
        }
    }
}
