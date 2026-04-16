//! Field component — form control wrapper with label, description, and error
use maud::{html, Markup};

#[derive(Clone, Debug)]
pub struct Props {
    pub label: String,
    pub id: String,
    pub description: Option<String>,
    pub error: Option<String>,
    pub required: bool,
    pub children: Markup,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            label: "Label".to_string(),
            id: "field-1".to_string(),
            description: None,
            error: None,
            required: false,
            children: html! { input.mui-input type="text"; },
        }
    }
}

pub fn render(props: Props) -> Markup {
    let invalid_class = if props.error.is_some() {
        " mui-field--invalid"
    } else {
        ""
    };

    html! {
        div class=(format!("mui-field{invalid_class}")) {
            label.mui-field__label for=(props.id) {
                (props.label)
                @if props.required {
                    span.mui-field__required aria-label="required" { "*" }
                }
            }
            (props.children)
            @if let Some(desc) = &props.description {
                p.mui-field__description id=(format!("{}-desc", props.id)) { (desc) }
            }
            @if let Some(err) = &props.error {
                p.mui-field__error id=(format!("{}-err", props.id)) role="alert" { (err) }
            }
        }
    }
}

pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            section {
                h2 { "Basic Field" }
                (render(Props {
                    label: "Email".to_string(),
                    id: "demo-f-1".to_string(),
                    description: None,
                    error: None,
                    required: false,
                    children: html! {
                        input.mui-input type="text" id="demo-f-1" name="email" placeholder="you@example.com" aria-describedby="demo-f-1-desc";
                    },
                }))
            }

            section {
                h2 { "With Description" }
                (render(Props {
                    label: "Username".to_string(),
                    id: "demo-f-2".to_string(),
                    description: Some("Letters, numbers, underscores only.".to_string()),
                    error: None,
                    required: false,
                    children: html! {
                        input.mui-input type="text" id="demo-f-2" name="username" placeholder="john_doe" aria-describedby="demo-f-2-desc";
                    },
                }))
            }

            section {
                h2 { "With Error" }
                (render(Props {
                    label: "Password".to_string(),
                    id: "demo-f-3".to_string(),
                    description: None,
                    error: Some("Must be at least 8 characters.".to_string()),
                    required: false,
                    children: html! {
                        input.mui-input type="password" id="demo-f-3" name="password" aria-invalid="true" aria-describedby="demo-f-3-err";
                    },
                }))
            }

            section {
                h2 { "Required Field" }
                (render(Props {
                    label: "Full Name".to_string(),
                    id: "demo-f-4".to_string(),
                    description: None,
                    error: None,
                    required: true,
                    children: html! {
                        input.mui-input type="text" id="demo-f-4" name="fullname" placeholder="John Doe" aria-describedby="demo-f-4-desc";
                    },
                }))
            }

            section {
                h2 { "Combined Form" }
                div style="display: flex; flex-direction: column; gap: 1rem; max-width: 24rem;" {
                    (render(Props {
                        label: "Email".to_string(),
                        id: "demo-f-5".to_string(),
                        description: Some("We will never share your email.".to_string()),
                        error: None,
                        required: true,
                        children: html! {
                            input.mui-input type="email" id="demo-f-5" name="email" placeholder="you@example.com" aria-describedby="demo-f-5-desc";
                        },
                    }))
                    (render(Props {
                        label: "Password".to_string(),
                        id: "demo-f-6".to_string(),
                        description: Some("At least 8 characters with a number.".to_string()),
                        error: Some("Password is too short.".to_string()),
                        required: true,
                        children: html! {
                            input.mui-input type="password" id="demo-f-6" name="password" aria-invalid="true" aria-describedby="demo-f-6-desc demo-f-6-err";
                        },
                    }))
                    (render(Props {
                        label: "Bio".to_string(),
                        id: "demo-f-7".to_string(),
                        description: Some("Brief description for your profile.".to_string()),
                        error: None,
                        required: false,
                        children: html! {
                            textarea.mui-textarea id="demo-f-7" name="bio" rows="3" placeholder="Tell us about yourself..." aria-describedby="demo-f-7-desc" {}
                        },
                    }))
                }
            }
        }
    }
}
