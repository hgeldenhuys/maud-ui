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
    File,
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
            Self::File => "file",
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
    /// Optional aria-describedby value linking to descriptive text (e.g. hint or error message)
    pub aria_describedby: Option<String>,
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
            aria_describedby: None,
        }
    }
}

/// Render a single input with the given properties.
///
/// All attribute values are auto-escaped by maud's `html!` macro, so
/// user-controlled `name`, `placeholder`, `value`, `id`, and
/// `aria_describedby` strings cannot break out of their attributes.
pub fn render(props: Props) -> Markup {
    let input_type = props.input_type.html_type();
    let is_file = matches!(props.input_type, InputType::File);
    let class = if is_file {
        "mui-input mui-input--file"
    } else {
        "mui-input"
    };

    html! {
        input
            class=(class)
            type=(input_type)
            name=(props.name)
            id=(props.id)
            placeholder=(props.placeholder)
            value=(props.value)
            required[props.required]
            disabled[props.disabled]
            readonly[props.readonly]
            aria-invalid=[props.invalid.then_some("true")]
            aria-describedby=[props.aria_describedby.as_deref()]
            {}
    }
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
                            aria_describedby: Some("demo-password-hint".into()),
                            ..Default::default()
                        }))
                        p id="demo-password-hint" style="font-size:0.75rem;color:var(--mui-muted-fg,#888);margin:0;" {
                            "Use at least 8 characters with a mix of letters and numbers."
                        }
                    }
                    label style="display:flex;flex-direction:column;gap:0.25rem;font-size:0.875rem;font-weight:500;" {
                        "Profile Picture"
                        (render(Props {
                            name: "avatar".into(),
                            id: "demo-avatar".into(),
                            input_type: InputType::File,
                            aria_describedby: Some("demo-avatar-hint".into()),
                            ..Default::default()
                        }))
                        p id="demo-avatar-hint" style="font-size:0.75rem;color:var(--mui-muted-fg,#888);margin:0;" {
                            "PNG or JPG, up to 2MB."
                        }
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
                    div style="display:flex;flex-direction:column;gap:0.25rem;" {
                        span style="font-size:0.75rem;color:var(--mui-muted-fg,#888);" { "File" }
                        (render(Props {
                            name: "type-file".into(),
                            input_type: InputType::File,
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
                            aria_describedby: Some("state-invalid-error".into()),
                            ..Default::default()
                        }))
                        span id="state-invalid-error" style="font-size:0.75rem;color:var(--mui-destructive,#ef4444);" { "Please enter a valid email address." }
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify maud's html! macro escapes attribute values, closing the XSS hole
    /// that the prior `format!`-based render() had.
    #[test]
    fn render_escapes_attribute_values() {
        let malicious = r#""><script>alert(1)</script>"#;
        let markup = render(Props {
            name: malicious.into(),
            value: malicious.into(),
            placeholder: malicious.into(),
            ..Default::default()
        });
        let html = markup.into_string();

        // The raw, un-escaped form must never appear in output.
        assert!(
            !html.contains("<script>alert(1)</script>"),
            "unescaped script tag leaked into attribute output: {html}"
        );
        // The escaped form should appear (maud uses &quot; for " and &lt;/&gt; for angle brackets).
        assert!(
            html.contains("&lt;script&gt;alert(1)&lt;/script&gt;"),
            "expected escaped <script> sequence in output, got: {html}"
        );
        assert!(
            html.contains("&quot;"),
            "expected escaped double-quote in output, got: {html}"
        );
    }

    #[test]
    fn aria_describedby_emitted_only_when_some() {
        let without = render(Props {
            name: "x".into(),
            ..Default::default()
        })
        .into_string();
        assert!(
            !without.contains("aria-describedby"),
            "aria-describedby leaked when None: {without}"
        );

        let with = render(Props {
            name: "x".into(),
            aria_describedby: Some("hint-id".into()),
            ..Default::default()
        })
        .into_string();
        assert!(
            with.contains(r#"aria-describedby="hint-id""#),
            "aria-describedby missing when Some: {with}"
        );
    }

    #[test]
    fn file_variant_renders_type_file() {
        let html = render(Props {
            name: "avatar".into(),
            input_type: InputType::File,
            ..Default::default()
        })
        .into_string();
        assert!(html.contains(r#"type="file""#));
        assert!(html.contains("mui-input--file"));
    }
}
