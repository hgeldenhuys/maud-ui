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

#[derive(Debug, Clone)]
pub struct Props {
    pub name: String,
    pub label: String,
    pub options: Vec<RadioOption>,
    pub selected: Option<String>,
    pub orientation: Orientation,
    pub disabled: bool,
}

pub fn render(props: Props) -> Markup {
    let orientation_class = props.orientation.class_name();
    html! {
        fieldset class=(format!("mui-radio-group {}", orientation_class)) role="radiogroup" aria-label=(props.label.clone()) {
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
                    }))
                }
            }
        }
    }
}
