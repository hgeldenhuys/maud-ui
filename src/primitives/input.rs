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
            // Types row
            div {
                p.mui-showcase__caption { "Types" }
                div.mui-showcase__row {
                    (render(Props {
                        name: "name".into(),
                        input_type: InputType::Text,
                        placeholder: "Your name".into(),
                        ..Default::default()
                    }))
                    (render(Props {
                        name: "email".into(),
                        input_type: InputType::Email,
                        placeholder: "you@example.com".into(),
                        ..Default::default()
                    }))
                    (render(Props {
                        name: "password".into(),
                        input_type: InputType::Password,
                        placeholder: "••••••".into(),
                        ..Default::default()
                    }))
                    (render(Props {
                        name: "url".into(),
                        input_type: InputType::Url,
                        placeholder: "https://...".into(),
                        ..Default::default()
                    }))
                    (render(Props {
                        name: "tel".into(),
                        input_type: InputType::Tel,
                        placeholder: "+1 555 0100".into(),
                        ..Default::default()
                    }))
                    (render(Props {
                        name: "search".into(),
                        input_type: InputType::Search,
                        placeholder: "Search…".into(),
                        ..Default::default()
                    }))
                    (render(Props {
                        name: "number".into(),
                        input_type: InputType::Number,
                        placeholder: "42".into(),
                        ..Default::default()
                    }))
                }
            }

            // States row
            div {
                p.mui-showcase__caption { "States" }
                div.mui-showcase__row {
                    (render(Props {
                        name: "default".into(),
                        placeholder: "Default".into(),
                        ..Default::default()
                    }))
                    (render(Props {
                        name: "with-value".into(),
                        value: "With value".into(),
                        ..Default::default()
                    }))
                    (render(Props {
                        name: "invalid".into(),
                        invalid: true,
                        placeholder: "Invalid email".into(),
                        ..Default::default()
                    }))
                    (render(Props {
                        name: "disabled".into(),
                        disabled: true,
                        placeholder: "Disabled".into(),
                        ..Default::default()
                    }))
                    (render(Props {
                        name: "readonly".into(),
                        readonly: true,
                        value: "Read-only".into(),
                        ..Default::default()
                    }))
                }
            }
        }
    }
}
