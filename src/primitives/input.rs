//! Input component — text, email, password, and other input types.

use maud::{html, Markup};

/// Input type variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputType {
    Text,
    Email,
    Password,
    Url,
    Tel,
    Search,
    Number,
}

impl InputType {
    fn html_type(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Email => "email",
            Self::Password => "password",
            Self::Url => "url",
            Self::Tel => "tel",
            Self::Search => "search",
            Self::Number => "number",
        }
    }
}

/// Input rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// HTML name attribute for form submission
    pub name: String,
    /// Input type (text, email, password, etc.)
    pub input_type: InputType,
    /// Placeholder text displayed when empty
    pub placeholder: String,
    /// Current value
    pub value: String,
    /// HTML id attribute for label linkage
    pub id: String,
    /// Whether the input is disabled
    pub disabled: bool,
    /// Whether the input is required
    pub required: bool,
    /// Whether the input is in an invalid state
    pub invalid: bool,
    /// Whether the input is readonly
    pub readonly: bool,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            name: String::new(),
            input_type: InputType::Text,
            placeholder: String::new(),
            value: String::new(),
            id: String::new(),
            disabled: false,
            required: false,
            invalid: false,
            readonly: false,
        }
    }
}

/// Render a single input with the given properties
pub fn render(props: Props) -> Markup {
    use maud::PreEscaped;

    let input_type = props.input_type.html_type();
    let mut html_str = format!(
        r#"<input class="mui-input" type="{}" name="{}" id="{}" placeholder="{}" value="{}""#,
        input_type,
        &props.name,
        &props.id,
        &props.placeholder,
        &props.value
    );

    if props.required {
        html_str.push_str(" required");
    }
    if props.disabled {
        html_str.push_str(" disabled");
    }
    if props.readonly {
        html_str.push_str(" readonly");
    }
    if props.invalid {
        html_str.push_str(r#" aria-invalid="true""#);
    }

    html_str.push_str(" />");

    PreEscaped(html_str).into()
}

/// Showcase all input types and states
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            // Realistic form section
            section {
                h2 { "Profile" }
                p.mui-showcase__caption { "A typical sign-up form using text, email, and password inputs." }
                div style="display:flex;flex-direction:column;gap:0.75rem;max-width:24rem;" {
                    label style="display:flex;flex-direction:column;gap:0.25rem;font-size:0.875rem;font-weight:500;" {
                        "Full Name"
                        (render(Props {
                            name: "fullname".into(),
                            id: "demo-fullname".into(),
                            input_type: InputType::Text,
                            placeholder: "Jane Smith".into(),
                            required: true,
                            ..Default::default()
                        }))
                    }
                    label style="display:flex;flex-direction:column;gap:0.25rem;font-size:0.875rem;font-weight:500;" {
                        "Email Address"
                        (render(Props {
                            name: "email".into(),
                            id: "demo-email".into(),
                            input_type: InputType::Email,
                            placeholder: "jane@example.com".into(),
                            required: true,
                            ..Default::default()
                        }))
                    }
                    label style="display:flex;flex-direction:column;gap:0.25rem;font-size:0.875rem;font-weight:500;" {
                        "Password"
                        (render(Props {
                            name: "password".into(),
                            id: "demo-password".into(),
                            input_type: InputType::Password,
                            placeholder: "At least 8 characters".into(),
                            required: true,
                            ..Default::default()
                        }))
                    }
                }
            }

            // All input types reference
            section {
                h2 { "Input Types" }
                p.mui-showcase__caption { "Each HTML input type rendered with a contextual placeholder." }
                div style="display:grid;grid-template-columns:repeat(auto-fill,minmax(14rem,1fr));gap:0.75rem;" {
                    div style="display:flex;flex-direction:column;gap:0.25rem;" {
                        span style="font-size:0.75rem;color:var(--mui-muted-fg,#888);" { "Text" }
                        (render(Props {
                            name: "type-text".into(),
                            input_type: InputType::Text,
                            placeholder: "Your name".into(),
                            ..Default::default()
                        }))
                    }
                    div style="display:flex;flex-direction:column;gap:0.25rem;" {
                        span style="font-size:0.75rem;color:var(--mui-muted-fg,#888);" { "Email" }
                        (render(Props {
                            name: "type-email".into(),
                            input_type: InputType::Email,
                            placeholder: "you@example.com".into(),
                            ..Default::default()
                        }))
                    }
                    div style="display:flex;flex-direction:column;gap:0.25rem;" {
                        span style="font-size:0.75rem;color:var(--mui-muted-fg,#888);" { "Password" }
                        (render(Props {
                            name: "type-password".into(),
                            input_type: InputType::Password,
                            placeholder: "Secret".into(),
                            ..Default::default()
                        }))
                    }
                    div style="display:flex;flex-direction:column;gap:0.25rem;" {
                        span style="font-size:0.75rem;color:var(--mui-muted-fg,#888);" { "URL" }
                        (render(Props {
                            name: "type-url".into(),
                            input_type: InputType::Url,
                            placeholder: "https://example.com".into(),
                            ..Default::default()
                        }))
                    }
                    div style="display:flex;flex-direction:column;gap:0.25rem;" {
                        span style="font-size:0.75rem;color:var(--mui-muted-fg,#888);" { "Phone" }
                        (render(Props {
                            name: "type-tel".into(),
                            input_type: InputType::Tel,
                            placeholder: "+1 (555) 000-0100".into(),
                            ..Default::default()
                        }))
                    }
                    div style="display:flex;flex-direction:column;gap:0.25rem;" {
                        span style="font-size:0.75rem;color:var(--mui-muted-fg,#888);" { "Search" }
                        (render(Props {
                            name: "type-search".into(),
                            input_type: InputType::Search,
                            placeholder: "Search articles...".into(),
                            ..Default::default()
                        }))
                    }
                    div style="display:flex;flex-direction:column;gap:0.25rem;" {
                        span style="font-size:0.75rem;color:var(--mui-muted-fg,#888);" { "Number" }
                        (render(Props {
                            name: "type-number".into(),
                            input_type: InputType::Number,
                            placeholder: "42".into(),
                            ..Default::default()
                        }))
                    }
                }
            }

            // States
            section {
                h2 { "States" }
                p.mui-showcase__caption { "Default, populated, invalid, disabled, and read-only." }
                div style="display:flex;flex-direction:column;gap:0.75rem;max-width:24rem;" {
                    div style="display:flex;flex-direction:column;gap:0.25rem;" {
                        span style="font-size:0.75rem;color:var(--mui-muted-fg,#888);" { "Default" }
                        (render(Props {
                            name: "state-default".into(),
                            placeholder: "Enter a value...".into(),
                            ..Default::default()
                        }))
                    }
                    div style="display:flex;flex-direction:column;gap:0.25rem;" {
                        label for="state-value" style="font-size:0.75rem;color:var(--mui-muted-fg,#888);" { "With value" }
                        (render(Props {
                            name: "state-value".into(),
                            id: "state-value".into(),
                            value: "jane@example.com".into(),
                            input_type: InputType::Email,
                            ..Default::default()
                        }))
                    }
                    div style="display:flex;flex-direction:column;gap:0.25rem;" {
                        label for="state-invalid" style="font-size:0.75rem;color:var(--mui-muted-fg,#888);" { "Invalid" }
                        (render(Props {
                            name: "state-invalid".into(),
                            id: "state-invalid".into(),
                            invalid: true,
                            value: "not-an-email".into(),
                            input_type: InputType::Email,
                            ..Default::default()
                        }))
                        span style="font-size:0.75rem;color:var(--mui-destructive,#ef4444);" { "Please enter a valid email address." }
                    }
                    div style="display:flex;flex-direction:column;gap:0.25rem;" {
                        label for="state-disabled" style="font-size:0.75rem;color:var(--mui-muted-fg,#888);" { "Disabled" }
                        (render(Props {
                            name: "state-disabled".into(),
                            id: "state-disabled".into(),
                            disabled: true,
                            value: "Cannot edit".into(),
                            ..Default::default()
                        }))
                    }
                    div style="display:flex;flex-direction:column;gap:0.25rem;" {
                        label for="state-readonly" style="font-size:0.75rem;color:var(--mui-muted-fg,#888);" { "Read-only" }
                        (render(Props {
                            name: "state-readonly".into(),
                            id: "state-readonly".into(),
                            readonly: true,
                            value: "user-9a3f2b".into(),
                            ..Default::default()
                        }))
                    }
                }
            }
        }
    }
}
