//! Textarea component — multi-line text input field.

use maud::{html, Markup, PreEscaped};

/// Resize behavior for textarea
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resize {
    None,
    Vertical,
    Horizontal,
    Both,
}

impl Resize {
    fn class(&self) -> &'static str {
        match self {
            Self::None => "mui-textarea--resize-none",
            Self::Vertical => "mui-textarea--resize-vertical",
            Self::Horizontal => "mui-textarea--resize-horizontal",
            Self::Both => "mui-textarea--resize-both",
        }
    }
}

/// Textarea rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// Form field name
    pub name: String,
    /// Placeholder text
    pub placeholder: String,
    /// Text content inside textarea
    pub value: String,
    /// Number of rows (default 4)
    pub rows: u32,
    /// Unique identifier
    pub id: String,
    /// Whether field is disabled
    pub disabled: bool,
    /// Whether field is required
    pub required: bool,
    /// Whether field shows invalid state
    pub invalid: bool,
    /// Whether field is read-only
    pub readonly: bool,
    /// Resize behavior
    pub resize: Resize,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            name: String::new(),
            placeholder: String::new(),
            value: String::new(),
            rows: 4,
            id: String::new(),
            disabled: false,
            required: false,
            invalid: false,
            readonly: false,
            resize: Resize::Vertical,
        }
    }
}

/// Render a single textarea with the given properties
pub fn render(props: Props) -> Markup {
    let mut attrs = String::new();

    if props.required {
        attrs.push_str(" required");
    }
    if props.disabled {
        attrs.push_str(" disabled");
    }
    if props.readonly {
        attrs.push_str(" readonly");
    }
    if props.invalid {
        attrs.push_str(r#" aria-invalid="true""#);
    }

    let class = format!("mui-textarea {}", props.resize.class());
    let html_string = format!(
        r#"<textarea class="{}" name="{}" id="{}" placeholder="{}" rows="{}"{}>{}</textarea>"#,
        escape_html(&class),
        escape_html(&props.name),
        escape_html(&props.id),
        escape_html(&props.placeholder),
        props.rows,
        attrs,
        escape_html(&props.value)
    );

    PreEscaped(html_string)
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Showcase all textarea variants and use cases
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            // Realistic feedback form
            section {
                h2 { "Feedback Form" }
                p.mui-showcase__caption { "A textarea in a realistic message context." }
                div style="display:flex;flex-direction:column;gap:0.75rem;max-width:28rem;" {
                    label style="display:flex;flex-direction:column;gap:0.25rem;font-size:0.875rem;font-weight:500;" {
                        "Your Message"
                        (render(Props {
                            name: "message".into(),
                            id: "demo-message".into(),
                            placeholder: "Tell us what you think...".into(),
                            rows: 5,
                            ..Default::default()
                        }))
                    }
                }
            }

            // States
            section {
                h2 { "States" }
                p.mui-showcase__caption { "Default, populated, invalid, disabled, and read-only." }
                div style="display:flex;flex-direction:column;gap:1rem;max-width:28rem;" {
                    div style="display:flex;flex-direction:column;gap:0.25rem;" {
                        span style="font-size:0.75rem;color:var(--mui-muted-fg,#888);" { "Default" }
                        (render(Props {
                            name: "state-default".into(),
                            placeholder: "Start typing...".into(),
                            rows: 3,
                            ..Default::default()
                        }))
                    }
                    div style="display:flex;flex-direction:column;gap:0.25rem;" {
                        span style="font-size:0.75rem;color:var(--mui-muted-fg,#888);" { "With content" }
                        (render(Props {
                            name: "state-value".into(),
                            value: "The onboarding flow was smooth and intuitive. One suggestion: adding a progress indicator on the setup wizard would help users know how many steps remain.".into(),
                            rows: 3,
                            ..Default::default()
                        }))
                    }
                    div style="display:flex;flex-direction:column;gap:0.25rem;" {
                        span style="font-size:0.75rem;color:var(--mui-muted-fg,#888);" { "Invalid" }
                        (render(Props {
                            name: "state-invalid".into(),
                            invalid: true,
                            value: "Hi".into(),
                            rows: 3,
                            ..Default::default()
                        }))
                        span style="font-size:0.75rem;color:var(--mui-destructive,#ef4444);" { "Message must be at least 20 characters." }
                    }
                    div style="display:flex;flex-direction:column;gap:0.25rem;" {
                        span style="font-size:0.75rem;color:var(--mui-muted-fg,#888);" { "Disabled" }
                        (render(Props {
                            name: "state-disabled".into(),
                            disabled: true,
                            value: "Submissions are currently closed.".into(),
                            rows: 3,
                            ..Default::default()
                        }))
                    }
                    div style="display:flex;flex-direction:column;gap:0.25rem;" {
                        span style="font-size:0.75rem;color:var(--mui-muted-fg,#888);" { "Read-only" }
                        (render(Props {
                            name: "state-readonly".into(),
                            readonly: true,
                            value: "This response was submitted on 2026-04-10 and cannot be edited.".into(),
                            rows: 3,
                            ..Default::default()
                        }))
                    }
                }
            }

            // Resize variants
            section {
                h2 { "Resize Behavior" }
                p.mui-showcase__caption { "Control how users can resize the textarea." }
                div style="display:grid;grid-template-columns:repeat(auto-fill,minmax(14rem,1fr));gap:1rem;" {
                    div style="display:flex;flex-direction:column;gap:0.25rem;" {
                        span style="font-size:0.75rem;color:var(--mui-muted-fg,#888);" { "Vertical (default)" }
                        (render(Props {
                            name: "resize-vertical".into(),
                            resize: Resize::Vertical,
                            placeholder: "Drag to resize vertically...".into(),
                            rows: 3,
                            ..Default::default()
                        }))
                    }
                    div style="display:flex;flex-direction:column;gap:0.25rem;" {
                        span style="font-size:0.75rem;color:var(--mui-muted-fg,#888);" { "Horizontal" }
                        (render(Props {
                            name: "resize-horizontal".into(),
                            resize: Resize::Horizontal,
                            placeholder: "Drag to resize horizontally...".into(),
                            rows: 3,
                            ..Default::default()
                        }))
                    }
                    div style="display:flex;flex-direction:column;gap:0.25rem;" {
                        span style="font-size:0.75rem;color:var(--mui-muted-fg,#888);" { "Both" }
                        (render(Props {
                            name: "resize-both".into(),
                            resize: Resize::Both,
                            placeholder: "Drag any direction...".into(),
                            rows: 3,
                            ..Default::default()
                        }))
                    }
                    div style="display:flex;flex-direction:column;gap:0.25rem;" {
                        span style="font-size:0.75rem;color:var(--mui-muted-fg,#888);" { "None" }
                        (render(Props {
                            name: "resize-none".into(),
                            resize: Resize::None,
                            placeholder: "Fixed size, no resize handle...".into(),
                            rows: 3,
                            ..Default::default()
                        }))
                    }
                }
            }
        }
    }
}
