//! Radio component — single radio button in a group

use maud::{html, Markup, PreEscaped};

#[derive(Clone, Debug)]
pub struct Props {
    pub name: String,
    pub value: String,
    pub label: String,
    pub id: String,
    pub checked: bool,
    pub disabled: bool,
    pub required: bool,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            name: "radio".to_string(),
            value: "option".to_string(),
            label: "Option".to_string(),
            id: "radio-default".to_string(),
            checked: false,
            disabled: false,
            required: false,
        }
    }
}

pub fn render(props: Props) -> Markup {
    let disabled_class = if props.disabled {
        " mui-radio--disabled"
    } else {
        ""
    };
    let class = format!("mui-radio{}", disabled_class);

    let checked_attr = if props.checked { " checked" } else { "" };
    let disabled_attr = if props.disabled { " disabled" } else { "" };
    let required_attr = if props.required { " required" } else { "" };

    let input_html = format!(
        r#"<input type="radio" class="mui-radio__input" id="{}" name="{}" value="{}"{}{}{} />"#,
        html_escape(&props.id),
        html_escape(&props.name),
        html_escape(&props.value),
        checked_attr,
        disabled_attr,
        required_attr,
    );

    html! {
        label class=(class) for=(props.id) {
            (PreEscaped(input_html))
            span class="mui-radio__indicator" aria-hidden="true" {}
            span class="mui-radio__label" { (props.label) }
        }
    }
}

fn html_escape(s: &str) -> String {
    s.chars()
        .flat_map(|c| match c {
            '&' => "&amp;".chars().collect::<Vec<_>>(),
            '<' => "&lt;".chars().collect::<Vec<_>>(),
            '>' => "&gt;".chars().collect::<Vec<_>>(),
            '"' => "&quot;".chars().collect::<Vec<_>>(),
            '\'' => "&#39;".chars().collect::<Vec<_>>(),
            c => vec![c],
        })
        .collect()
}

pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            section {
                h2 { "Plan" }
                div.mui-showcase__row {
                    (render(Props {
                        name: "plan".to_string(),
                        value: "free".to_string(),
                        label: "Free".to_string(),
                        id: "demo-radio-plan-free".to_string(),
                        checked: true,
                        disabled: false,
                        required: false,
                    }))
                }
                div.mui-showcase__row {
                    (render(Props {
                        name: "plan".to_string(),
                        value: "pro".to_string(),
                        label: "Pro".to_string(),
                        id: "demo-radio-plan-pro".to_string(),
                        checked: false,
                        disabled: false,
                        required: false,
                    }))
                }
                div.mui-showcase__row {
                    (render(Props {
                        name: "plan".to_string(),
                        value: "team".to_string(),
                        label: "Team".to_string(),
                        id: "demo-radio-plan-team".to_string(),
                        checked: false,
                        disabled: false,
                        required: false,
                    }))
                }
            }
            section {
                h2 { "States" }
                div.mui-showcase__row {
                    (render(Props {
                        name: "state-unchecked".to_string(),
                        value: "unchecked".to_string(),
                        label: "Unchecked".to_string(),
                        id: "demo-radio-state-unchecked".to_string(),
                        checked: false,
                        disabled: false,
                        required: false,
                    }))
                    (render(Props {
                        name: "state-checked".to_string(),
                        value: "checked".to_string(),
                        label: "Checked".to_string(),
                        id: "demo-radio-state-checked".to_string(),
                        checked: true,
                        disabled: false,
                        required: false,
                    }))
                    (render(Props {
                        name: "state-disabled-unchecked".to_string(),
                        value: "disabled-unchecked".to_string(),
                        label: "Disabled unchecked".to_string(),
                        id: "demo-radio-state-disabled-unchecked".to_string(),
                        checked: false,
                        disabled: true,
                        required: false,
                    }))
                    (render(Props {
                        name: "state-disabled-checked".to_string(),
                        value: "disabled-checked".to_string(),
                        label: "Disabled checked".to_string(),
                        id: "demo-radio-state-disabled-checked".to_string(),
                        checked: true,
                        disabled: true,
                        required: false,
                    }))
                }
            }
        }
    }
}
