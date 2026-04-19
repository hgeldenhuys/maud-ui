//! Field component — form control wrapper with label, description, and error
//!
//! Two usage modes:
//! 1. Canonical: `render(Props)` — one-shot renderer, back-compat with existing call sites.
//! 2. Compose: build up fields using helper fns (`label`, `description`, `error`, `group`,
//!    `fieldset`, `legend`, `content`, `separator`, `title`) for structural flexibility.
use maud::{html, Markup};

/// Layout orientation for a Field.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Orientation {
    /// Stack label, control, description, and error vertically. Default.
    #[default]
    Vertical,
    /// Two-column grid: label on the left, control + meta on the right.
    Horizontal,
    /// Vertical on narrow viewports, horizontal at ≥ 640px.
    Responsive,
}

impl Orientation {
    /// Returns the class suffix for this orientation (or an empty string for the default).
    pub fn as_class(&self) -> &'static str {
        match self {
            Orientation::Vertical => "",
            Orientation::Horizontal => "horizontal",
            Orientation::Responsive => "responsive",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Props {
    pub label: String,
    pub id: String,
    pub description: Option<String>,
    /// Legacy single-error field. Still supported; prefer `errors` for multiple messages.
    pub error: Option<String>,
    /// Multiple validation errors. Rendered after `error` (if set).
    pub errors: Vec<String>,
    pub required: bool,
    pub orientation: Orientation,
    pub children: Markup,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            label: "Label".to_string(),
            id: "field-1".to_string(),
            description: None,
            error: None,
            errors: Vec::new(),
            required: false,
            orientation: Orientation::Vertical,
            children: html! { input.mui-input type="text"; },
        }
    }
}

pub fn render(props: Props) -> Markup {
    let invalid = props.error.is_some() || !props.errors.is_empty();
    let invalid_class = if invalid { " mui-field--invalid" } else { "" };
    let orientation_suffix = props.orientation.as_class();
    let orientation_class = if orientation_suffix.is_empty() {
        String::new()
    } else {
        format!(" mui-field--{orientation_suffix}")
    };

    html! {
        div
            class=(format!("mui-field{invalid_class}{orientation_class}"))
            data-invalid=[if invalid { Some("true") } else { None }]
        {
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
            @for (idx, err) in props.errors.iter().enumerate() {
                p.mui-field__error id=(format!("{}-err-{}", props.id, idx)) role="alert" { (err) }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Helper fns — for composing fields without the monolithic `render(Props)`.
// ---------------------------------------------------------------------------

/// A `<label>` bound to a control via `for`.
pub fn label(for_id: &str, text: &str) -> Markup {
    html! {
        label.mui-field__label for=(for_id) { (text) }
    }
}

/// A `<p>` styled as a field description / helper text.
pub fn description(text: &str) -> Markup {
    html! {
        p.mui-field__description { (text) }
    }
}

/// A `<p role="alert">` styled as a field error.
pub fn error(text: &str) -> Markup {
    html! {
        p.mui-field__error role="alert" { (text) }
    }
}

/// Vertical stack of fields — use to group related controls without a fieldset border.
pub fn group(children: Markup) -> Markup {
    html! {
        div.mui-field-group { (children) }
    }
}

/// A `<legend>` element for use inside a `<fieldset>`.
pub fn legend(text: &str) -> Markup {
    html! {
        legend.mui-field__legend { (text) }
    }
}

/// A `<fieldset>` with `role="group"` for accessibility, wrapping a `<legend>` + children.
pub fn fieldset(legend_text: &str, children: Markup) -> Markup {
    html! {
        fieldset.mui-fieldset role="group" {
            legend { (legend_text) }
            (children)
        }
    }
}

/// Slot for the control itself — lets you separate the control from label/description.
pub fn content(children: Markup) -> Markup {
    html! {
        div.mui-field__content { (children) }
    }
}

/// A visual `<hr>` separator for use inside a field group.
pub fn separator() -> Markup {
    html! {
        hr.mui-field__separator;
    }
}

/// A section title for use above a group of fields.
pub fn title(text: &str) -> Markup {
    html! {
        h3.mui-field__title { (text) }
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
                        ..Default::default()
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
                        ..Default::default()
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
                        ..Default::default()
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
                        ..Default::default()
                    }))
                }
            }

            // Multiple errors + data-invalid demo
            section {
                h2 { "Multiple Errors" }
                p.mui-showcase__caption { "A field with multiple validation errors rendered via the errors Vec. Root carries data-invalid for CSS and form-framework hooks." }
                div style="max-width:24rem;" {
                    (render(Props {
                        label: "Password".to_string(),
                        id: "demo-multi-err".to_string(),
                        description: Some("Passwords must meet all requirements.".to_string()),
                        error: None,
                        errors: vec![
                            "Must contain at least one uppercase letter.".to_string(),
                            "Must contain at least one digit.".to_string(),
                            "Must be at least 12 characters long.".to_string(),
                        ],
                        required: true,
                        children: html! {
                            input.mui-input type="password" id="demo-multi-err" name="password" value="weak" aria-invalid="true";
                        },
                        ..Default::default()
                    }))
                }
            }

            // Horizontal orientation
            section {
                h2 { "Horizontal Orientation" }
                p.mui-showcase__caption { "Label on the left, control on the right — good for dense forms and settings panels." }
                div style="max-width:32rem;display:flex;flex-direction:column;gap:0.75rem;" {
                    (render(Props {
                        label: "Display Name".to_string(),
                        id: "demo-hz-name".to_string(),
                        orientation: Orientation::Horizontal,
                        children: html! {
                            input.mui-input type="text" id="demo-hz-name" name="display_name" placeholder="Jane Smith";
                        },
                        ..Default::default()
                    }))
                    (render(Props {
                        label: "Email".to_string(),
                        id: "demo-hz-email".to_string(),
                        orientation: Orientation::Horizontal,
                        description: Some("Primary contact address.".to_string()),
                        children: html! {
                            input.mui-input type="email" id="demo-hz-email" name="email" placeholder="jane@example.com";
                        },
                        ..Default::default()
                    }))
                }
            }

            // Composed helpers: fieldset + group + title + separator
            section {
                h2 { "Composed Fieldset" }
                p.mui-showcase__caption { "Built from helper fns (fieldset, legend, group, title, separator, label, description, content) rather than the monolithic render(Props)." }
                div style="max-width:28rem;" {
                    (fieldset("Account", group(html! {
                        (title("Profile"))
                        div.mui-field {
                            (label("demo-cp-name", "Display Name"))
                            (content(html! {
                                input.mui-input type="text" id="demo-cp-name" name="name" placeholder="Jane";
                            }))
                            (description("How you'll appear to others."))
                        }
                        div.mui-field {
                            (label("demo-cp-handle", "Handle"))
                            (content(html! {
                                input.mui-input type="text" id="demo-cp-handle" name="handle" placeholder="@jane";
                            }))
                        }
                        (separator())
                        (title("Security"))
                        div.mui-field {
                            (label("demo-cp-pwd", "Password"))
                            (content(html! {
                                input.mui-input type="password" id="demo-cp-pwd" name="password";
                            }))
                            (error("Password is too short."))
                        }
                    })))
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
                            ..Default::default()
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
                            ..Default::default()
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
                            ..Default::default()
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
                            ..Default::default()
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
                            ..Default::default()
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
                            ..Default::default()
                        }))
                    }
                }
            }
        }
    }
}
