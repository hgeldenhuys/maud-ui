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
            // Primary demo: realistic "Create Account" form
            section {
                h2 { "Create Account" }
                p.mui-showcase__caption { "A realistic registration form showing label, description, error, and required field patterns together." }
                div style="display:flex;flex-direction:column;gap:1rem;max-width:24rem;" {
                    (render(Props {
                        label: "Full Name".to_string(),
                        id: "demo-ca-name".to_string(),
                        description: None,
                        error: None,
                        required: true,
                        children: html! {
                            input.mui-input type="text" id="demo-ca-name" name="fullname" placeholder="Jane Smith" required;
                        },
                    }))
                    (render(Props {
                        label: "Email".to_string(),
                        id: "demo-ca-email".to_string(),
                        description: Some("We'll never share your email with anyone.".to_string()),
                        error: None,
                        required: true,
                        children: html! {
                            input.mui-input type="email" id="demo-ca-email" name="email" placeholder="jane@example.com" required aria-describedby="demo-ca-email-desc";
                        },
                    }))
                    (render(Props {
                        label: "Password".to_string(),
                        id: "demo-ca-password".to_string(),
                        description: Some("Use 8 or more characters with a mix of letters and numbers.".to_string()),
                        error: Some("Must be at least 8 characters.".to_string()),
                        required: true,
                        children: html! {
                            input.mui-input type="password" id="demo-ca-password" name="password" value="short" required aria-invalid="true" aria-describedby="demo-ca-password-desc demo-ca-password-err";
                        },
                    }))
                    (render(Props {
                        label: "Bio".to_string(),
                        id: "demo-ca-bio".to_string(),
                        description: Some("A brief introduction for your public profile.".to_string()),
                        error: None,
                        required: false,
                        children: html! {
                            textarea.mui-textarea id="demo-ca-bio" name="bio" rows="3" placeholder="Tell us about yourself..." aria-describedby="demo-ca-bio-desc" {}
                        },
                    }))
                }
            }

            // Anatomy: individual field features
            section {
                h2 { "Field Anatomy" }
                p.mui-showcase__caption { "Each feature of the Field component shown in isolation." }
                div style="display:grid;grid-template-columns:repeat(auto-fill,minmax(16rem,1fr));gap:1.5rem;" {
                    div {
                        h3 style="font-size:0.875rem;margin-bottom:0.5rem;" { "Basic" }
                        (render(Props {
                            label: "Username".to_string(),
                            id: "demo-a-basic".to_string(),
                            description: None,
                            error: None,
                            required: false,
                            children: html! {
                                input.mui-input type="text" id="demo-a-basic" name="username" placeholder="johndoe";
                            },
                        }))
                    }
                    div {
                        h3 style="font-size:0.875rem;margin-bottom:0.5rem;" { "Required" }
                        (render(Props {
                            label: "Email Address".to_string(),
                            id: "demo-a-required".to_string(),
                            description: None,
                            error: None,
                            required: true,
                            children: html! {
                                input.mui-input type="email" id="demo-a-required" name="email" placeholder="you@example.com" required;
                            },
                        }))
                    }
                    div {
                        h3 style="font-size:0.875rem;margin-bottom:0.5rem;" { "With Description" }
                        (render(Props {
                            label: "Phone".to_string(),
                            id: "demo-a-desc".to_string(),
                            description: Some("Include country code for international numbers.".to_string()),
                            error: None,
                            required: false,
                            children: html! {
                                input.mui-input type="tel" id="demo-a-desc" name="phone" placeholder="+1 (555) 000-0100" aria-describedby="demo-a-desc-desc";
                            },
                        }))
                    }
                    div {
                        h3 style="font-size:0.875rem;margin-bottom:0.5rem;" { "With Error" }
                        (render(Props {
                            label: "Website".to_string(),
                            id: "demo-a-error".to_string(),
                            description: None,
                            error: Some("Please enter a valid URL starting with https://.".to_string()),
                            required: false,
                            children: html! {
                                input.mui-input type="url" id="demo-a-error" name="website" value="not-a-url" aria-invalid="true" aria-describedby="demo-a-error-err";
                            },
                        }))
                    }
                    div {
                        h3 style="font-size:0.875rem;margin-bottom:0.5rem;" { "Description + Error" }
                        (render(Props {
                            label: "Invite Code".to_string(),
                            id: "demo-a-both".to_string(),
                            description: Some("Found in your invitation email.".to_string()),
                            error: Some("This code has already been used.".to_string()),
                            required: true,
                            children: html! {
                                input.mui-input type="text" id="demo-a-both" name="invite" value="USED-CODE-123" aria-invalid="true" aria-describedby="demo-a-both-desc demo-a-both-err";
                            },
                        }))
                    }
                    div {
                        h3 style="font-size:0.875rem;margin-bottom:0.5rem;" { "Textarea Field" }
                        (render(Props {
                            label: "Notes".to_string(),
                            id: "demo-a-textarea".to_string(),
                            description: Some("Any additional context (optional).".to_string()),
                            error: None,
                            required: false,
                            children: html! {
                                textarea.mui-textarea id="demo-a-textarea" name="notes" rows="3" placeholder="Add notes..." aria-describedby="demo-a-textarea-desc" {}
                            },
                        }))
                    }
                }
            }
        }
    }
}
