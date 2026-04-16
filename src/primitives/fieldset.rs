//! Fieldset component — groups related form controls with a legend
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
                        div style="display: flex; flex-direction: column; gap: 0.75rem;" {
                            div {
                                label { "First Name" }
                                input.mui-input type="text" placeholder="John";
                            }
                            div {
                                label { "Last Name" }
                                input.mui-input type="text" placeholder="Doe";
                            }
                        }
                    },
                }))
            }

            section {
                h2 { "Disabled Fieldset" }
                (render(Props {
                    legend: "Disabled Group".to_string(),
                    disabled: true,
                    children: html! {
                        div style="display: flex; flex-direction: column; gap: 0.75rem;" {
                            div {
                                label { "Email" }
                                input.mui-input type="email" placeholder="you@example.com" disabled;
                            }
                            div {
                                label { "Phone" }
                                input.mui-input type="tel" placeholder="+1 (555) 000-0000" disabled;
                            }
                        }
                    },
                }))
            }
        }
    }
}
