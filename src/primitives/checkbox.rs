//! Checkbox component — maud-ui (shadcn/ui-grade)

use maud::{html, Markup};

#[derive(Clone, Debug)]
pub struct Props {
    pub name: String,
    pub value: String,
    pub label: String,
    pub description: Option<String>,
    pub id: String,
    pub checked: bool,
    pub indeterminate: bool,
    pub disabled: bool,
    pub required: bool,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            name: "checkbox".to_string(),
            value: "on".to_string(),
            label: "Checkbox".to_string(),
            description: None,
            id: "checkbox".to_string(),
            checked: false,
            indeterminate: false,
            disabled: false,
            required: false,
        }
    }
}

pub fn render(props: Props) -> Markup {
    let disabled_class = if props.disabled {
        " mui-checkbox--disabled"
    } else {
        ""
    };

    let indeterminate_class = if props.indeterminate {
        " mui-checkbox__indicator--indeterminate"
    } else {
        ""
    };

    let has_desc = props.description.is_some();

    html! {
        label class=(format!("mui-checkbox{}", disabled_class)) for=(props.id.clone()) {
            @if props.checked {
                @if props.disabled {
                    @if props.required {
                        input type="checkbox" class="mui-checkbox__input" id=(props.id) name=(props.name) value=(props.value) checked disabled required;
                    } @else {
                        input type="checkbox" class="mui-checkbox__input" id=(props.id) name=(props.name) value=(props.value) checked disabled;
                    }
                } @else {
                    @if props.required {
                        input type="checkbox" class="mui-checkbox__input" id=(props.id) name=(props.name) value=(props.value) checked required;
                    } @else {
                        input type="checkbox" class="mui-checkbox__input" id=(props.id) name=(props.name) value=(props.value) checked;
                    }
                }
            } @else {
                @if props.disabled {
                    @if props.required {
                        input type="checkbox" class="mui-checkbox__input" id=(props.id) name=(props.name) value=(props.value) disabled required;
                    } @else {
                        input type="checkbox" class="mui-checkbox__input" id=(props.id) name=(props.name) value=(props.value) disabled;
                    }
                } @else {
                    @if props.required {
                        input type="checkbox" class="mui-checkbox__input" id=(props.id) name=(props.name) value=(props.value) required;
                    } @else {
                        input type="checkbox" class="mui-checkbox__input" id=(props.id) name=(props.name) value=(props.value);
                    }
                }
            }

            span class=(format!("mui-checkbox__indicator{}", indeterminate_class)) aria-hidden="true" {}

            @if has_desc {
                span class="mui-checkbox__text" {
                    span class="mui-checkbox__label" { (props.label) }
                    @if let Some(desc) = props.description {
                        span class="mui-checkbox__description" { (desc) }
                    }
                }
            } @else {
                span class="mui-checkbox__label" { (props.label) }
            }
        }
    }
}

pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            section {
                h2 { "Checkbox" }
                div style="display: flex; flex-direction: column; gap: 0.75rem;" {
                    (render(Props {
                        name: "cb-unchecked".to_string(),
                        value: "on".to_string(),
                        label: "Unchecked".to_string(),
                        description: None,
                        id: "cb-unchecked".to_string(),
                        checked: false,
                        indeterminate: false,
                        disabled: false,
                        required: false,
                    }))
                    (render(Props {
                        name: "cb-checked".to_string(),
                        value: "on".to_string(),
                        label: "Checked".to_string(),
                        description: None,
                        id: "cb-checked".to_string(),
                        checked: true,
                        indeterminate: false,
                        disabled: false,
                        required: false,
                    }))
                    (render(Props {
                        name: "cb-indeterminate".to_string(),
                        value: "on".to_string(),
                        label: "Indeterminate".to_string(),
                        description: None,
                        id: "cb-indeterminate".to_string(),
                        checked: false,
                        indeterminate: true,
                        disabled: false,
                        required: false,
                    }))
                    (render(Props {
                        name: "cb-disabled".to_string(),
                        value: "on".to_string(),
                        label: "Disabled".to_string(),
                        description: None,
                        id: "cb-disabled".to_string(),
                        checked: false,
                        indeterminate: false,
                        disabled: true,
                        required: false,
                    }))
                    (render(Props {
                        name: "cb-disabled-checked".to_string(),
                        value: "on".to_string(),
                        label: "Disabled + Checked".to_string(),
                        description: None,
                        id: "cb-disabled-checked".to_string(),
                        checked: true,
                        indeterminate: false,
                        disabled: true,
                        required: false,
                    }))
                    (render(Props {
                        name: "cb-with-desc".to_string(),
                        value: "on".to_string(),
                        label: "Accept terms and conditions".to_string(),
                        description: Some("You agree to our Terms of Service and Privacy Policy.".to_string()),
                        id: "cb-with-desc".to_string(),
                        checked: false,
                        indeterminate: false,
                        disabled: false,
                        required: false,
                    }))
                }
            }
        }
    }
}
