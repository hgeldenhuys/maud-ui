//! Checkbox component — maud-ui Wave 1

use maud::{html, Markup};

#[derive(Clone, Debug)]
pub struct Props {
    pub name: String,
    pub value: String,
    pub label: String,
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

    let aria_checked = if props.indeterminate {
        "mixed"
    } else if props.checked {
        "true"
    } else {
        "false"
    };

    html! {
        label class=(format!("mui-checkbox{}", disabled_class)) for=(props.id.clone()) {
            @if props.checked {
                @if props.disabled {
                    @if props.required {
                        input type="checkbox" class="mui-checkbox__input" id=(props.id) name=(props.name) value=(props.value) aria-checked=(aria_checked) checked disabled required;
                    } @else {
                        input type="checkbox" class="mui-checkbox__input" id=(props.id) name=(props.name) value=(props.value) aria-checked=(aria_checked) checked disabled;
                    }
                } @else {
                    @if props.required {
                        input type="checkbox" class="mui-checkbox__input" id=(props.id) name=(props.name) value=(props.value) aria-checked=(aria_checked) checked required;
                    } @else {
                        input type="checkbox" class="mui-checkbox__input" id=(props.id) name=(props.name) value=(props.value) aria-checked=(aria_checked) checked;
                    }
                }
            } @else {
                @if props.disabled {
                    @if props.required {
                        input type="checkbox" class="mui-checkbox__input" id=(props.id) name=(props.name) value=(props.value) aria-checked=(aria_checked) disabled required;
                    } @else {
                        input type="checkbox" class="mui-checkbox__input" id=(props.id) name=(props.name) value=(props.value) aria-checked=(aria_checked) disabled;
                    }
                } @else {
                    @if props.required {
                        input type="checkbox" class="mui-checkbox__input" id=(props.id) name=(props.name) value=(props.value) aria-checked=(aria_checked) required;
                    } @else {
                        input type="checkbox" class="mui-checkbox__input" id=(props.id) name=(props.name) value=(props.value) aria-checked=(aria_checked);
                    }
                }
            }

            span class=(format!("mui-checkbox__indicator{}", indeterminate_class)) aria-hidden="true";

            span class="mui-checkbox__label" {
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
                        name: "cb-unchecked".to_string(),
                        value: "on".to_string(),
                        label: "Unchecked".to_string(),
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
                        id: "cb-disabled-checked".to_string(),
                        checked: true,
                        indeterminate: false,
                        disabled: true,
                        required: false,
                    }))
                }
            }
        }
    }
}
