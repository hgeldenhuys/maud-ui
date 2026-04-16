//! Fieldset component — groups related form controls with a legend
use crate::primitives::field;
use maud::{html, Markup};

#[derive(Clone, Debug)]
pub struct Props {
    pub legend: String,
    pub disabled: bool,
    pub children: Markup,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            legend: "Fieldset".to_string(),
            disabled: false,
            children: html! {},
        }
    }
}

pub fn render(props: Props) -> Markup {
    html! {
        fieldset.mui-fieldset disabled[props.disabled] {
            legend.mui-fieldset__legend { (props.legend) }
            (props.children)
        }
    }
}

pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            section {
                h2 { "Profile Fieldset" }
                (render(Props {
                    legend: "Profile".to_string(),
                    disabled: false,
                    children: html! {
                        (field::render(field::Props {
                            label: "First Name".to_string(),
                            id: "fs-fname".to_string(),
                            description: None,
                            error: None,
                            required: true,
                            children: html! {
                                input.mui-input type="text" id="fs-fname" placeholder="John";
                            },
                        }))
                        (field::render(field::Props {
                            label: "Last Name".to_string(),
                            id: "fs-lname".to_string(),
                            description: None,
                            error: None,
                            required: true,
                            children: html! {
                                input.mui-input type="text" id="fs-lname" placeholder="Doe";
                            },
                        }))
                    },
                }))
            }

            section {
                h2 { "Disabled Fieldset" }
                (render(Props {
                    legend: "Disabled Group".to_string(),
                    disabled: true,
                    children: html! {
                        (field::render(field::Props {
                            label: "Email".to_string(),
                            id: "fs-email".to_string(),
                            description: None,
                            error: None,
                            required: false,
                            children: html! {
                                input.mui-input type="email" id="fs-email" placeholder="you@example.com" disabled;
                            },
                        }))
                        (field::render(field::Props {
                            label: "Phone".to_string(),
                            id: "fs-phone".to_string(),
                            description: None,
                            error: None,
                            required: false,
                            children: html! {
                                input.mui-input type="tel" id="fs-phone" placeholder="+1 (555) 000-0000" disabled;
                            },
                        }))
                    },
                }))
            }
        }
    }
}
