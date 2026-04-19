//! RadioGroup component.
use maud::{html, Markup};

#[derive(Debug, Clone)]
pub struct RadioOption {
    pub value: String,
    pub label: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

impl Orientation {
    fn class_name(&self) -> &'static str {
        match self {
            Orientation::Vertical => "mui-radio-group--vertical",
            Orientation::Horizontal => "mui-radio-group--horizontal",
        }
    }
}

/// Density variant for a RadioGroup — mirrors the shadcn Base UI "size" axis.
#[derive(Debug, Clone, Copy, Default)]
pub enum Variant {
    /// Default spacing.
    #[default]
    Default,
    /// More breathing room between options; bigger hit targets.
    Comfortable,
    /// Tighter rows for dense admin UIs.
    Compact,
}

impl Variant {
    fn class_name(&self) -> Option<&'static str> {
        match self {
            Variant::Default => None,
            Variant::Comfortable => Some("mui-radio-group--comfortable"),
            Variant::Compact => Some("mui-radio-group--compact"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Props {
    pub name: String,
    pub label: String,
    pub options: Vec<RadioOption>,
    pub selected: Option<String>,
    pub orientation: Orientation,
    pub disabled: bool,
    /// Marks each underlying `<input type="radio">` as required so native
    /// form validation blocks submit until one option is chosen.
    pub required: bool,
    /// Density variant — see [`Variant`].
    pub variant: Variant,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            name: String::new(),
            label: String::new(),
            options: Vec::new(),
            selected: None,
            orientation: Orientation::Vertical,
            disabled: false,
            required: false,
            variant: Variant::Default,
        }
    }
}

pub fn render(props: Props) -> Markup {
    let orientation_class = props.orientation.class_name();
    let variant_class = props.variant.class_name().unwrap_or("");
    let mut class = format!("mui-radio-group {orientation_class}");
    if !variant_class.is_empty() {
        class.push(' ');
        class.push_str(variant_class);
    }
    html! {
        fieldset class=(class) {
            legend class="mui-radio-group__legend" { (props.label) }
            @for opt in props.options {
                label class="mui-radio" for=(format!("{}-{}", props.name, opt.value)) {
                    input
                        type="radio"
                        class="mui-radio__input"
                        id=(format!("{}-{}", props.name, opt.value))
                        name=(props.name.clone())
                        value=(opt.value.clone())
                        checked[props.selected.as_ref() == Some(&opt.value)]
                        disabled[props.disabled]
                        required[props.required]
                    ;
                    span class="mui-radio__indicator" aria-hidden="true" {}
                    @if let Some(desc) = opt.description {
                        span class="mui-radio__text" {
                            span class="mui-radio__label" { (opt.label) }
                            span class="mui-radio__description" { (desc) }
                        }
                    } @else {
                        span class="mui-radio__label" { (opt.label) }
                    }
                }
            }
        }
    }
}

pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            section {
                h2 { "Vertical (default)" }
                div.mui-showcase__row {
                    (render(Props {
                        name: "plan".to_string(),
                        label: "Select a plan".to_string(),
                        options: vec![
                            RadioOption { value: "free".to_string(), label: "Free".to_string(), description: Some("Up to 3 projects".to_string()) },
                            RadioOption { value: "pro".to_string(), label: "Pro".to_string(), description: Some("Unlimited projects".to_string()) },
                            RadioOption { value: "team".to_string(), label: "Team".to_string(), description: Some("Unlimited projects + collaboration".to_string()) },
                        ],
                        selected: Some("pro".to_string()),
                        orientation: Orientation::Vertical,
                        disabled: false,
                        ..Default::default()
                    }))
                }
            }
            section {
                h2 { "Horizontal" }
                div.mui-showcase__row {
                    (render(Props {
                        name: "size".to_string(),
                        label: "Size".to_string(),
                        options: vec![
                            RadioOption { value: "s".to_string(), label: "S".to_string(), description: None },
                            RadioOption { value: "m".to_string(), label: "M".to_string(), description: None },
                            RadioOption { value: "l".to_string(), label: "L".to_string(), description: None },
                            RadioOption { value: "xl".to_string(), label: "XL".to_string(), description: None },
                        ],
                        selected: None,
                        orientation: Orientation::Horizontal,
                        disabled: false,
                        ..Default::default()
                    }))
                }
            }
            section {
                h2 { "Disabled" }
                div.mui-showcase__row {
                    (render(Props {
                        name: "notify".to_string(),
                        label: "Notifications".to_string(),
                        options: vec![
                            RadioOption { value: "all".to_string(), label: "All".to_string(), description: None },
                            RadioOption { value: "mentions".to_string(), label: "Mentions only".to_string(), description: None },
                            RadioOption { value: "none".to_string(), label: "None".to_string(), description: None },
                        ],
                        selected: Some("mentions".to_string()),
                        orientation: Orientation::Vertical,
                        disabled: true,
                        ..Default::default()
                    }))
                }
            }
            section {
                h2 { "Required + Comfortable" }
                p.mui-showcase__caption { "required: true propagates to each input; variant: Comfortable opens spacing." }
                div.mui-showcase__row {
                    (render(Props {
                        name: "billing".to_string(),
                        label: "Billing cadence".to_string(),
                        options: vec![
                            RadioOption { value: "monthly".to_string(), label: "Monthly".to_string(), description: Some("Cancel anytime".to_string()) },
                            RadioOption { value: "annual".to_string(), label: "Annual".to_string(), description: Some("Two months free".to_string()) },
                        ],
                        selected: None,
                        required: true,
                        variant: Variant::Comfortable,
                        ..Default::default()
                    }))
                }
            }
            section {
                h2 { "Compact" }
                p.mui-showcase__caption { "variant: Compact — tighter rows for admin/settings panels." }
                div.mui-showcase__row {
                    (render(Props {
                        name: "priority".to_string(),
                        label: "Priority".to_string(),
                        options: vec![
                            RadioOption { value: "low".to_string(), label: "Low".to_string(), description: None },
                            RadioOption { value: "normal".to_string(), label: "Normal".to_string(), description: None },
                            RadioOption { value: "high".to_string(), label: "High".to_string(), description: None },
                        ],
                        selected: Some("normal".to_string()),
                        variant: Variant::Compact,
                        ..Default::default()
                    }))
                }
            }
        }
    }
}
