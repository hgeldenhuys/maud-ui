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
    /// Explicit `aria-invalid="true"` emission. OR'd with `invalid` so either
    /// field turns on the attribute; exposed separately so form frameworks
    /// can set just the ARIA bit without triggering invalid styling.
    pub aria_invalid: bool,
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
            rows: 3,
            id: String::new(),
            disabled: false,
            required: false,
            invalid: false,
            aria_invalid: false,
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
    if props.invalid || props.aria_invalid {
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
            // Feedback form with character counter
            section {
                h2 { "Share your feedback" }
                p.mui-showcase__caption { "Tell us what worked and what didn't. We read every response." }
                div style="display:flex;flex-direction:column;gap:0.5rem;max-width:28rem;" {
                    label for="feedback-message" style="font-size:0.875rem;font-weight:500;" { "Your feedback" }
                    (render(Props {
                        name: "feedback".into(),
                        id: "feedback-message".into(),
                        placeholder: "What's on your mind?".into(),
                        rows: 5,
                        ..Default::default()
                    }))
                    div style="display:flex;justify-content:space-between;font-size:0.75rem;color:var(--mui-text-muted);" {
                        span { "Min 20 characters" }
                        span { "0 / 500" }
                    }
                }
            }

            // Bio field
            section {
                h2 { "Profile" }
                p.mui-showcase__caption { "Shown on your public profile and attribution lines." }
                div style="display:flex;flex-direction:column;gap:0.5rem;max-width:28rem;" {
                    label for="profile-bio" style="font-size:0.875rem;font-weight:500;" { "Bio" }
                    (render(Props {
                        name: "bio".into(),
                        id: "profile-bio".into(),
                        placeholder: "Write a short intro about yourself".into(),
                        rows: 4,
                        ..Default::default()
                    }))
                    p style="font-size:0.75rem;color:var(--mui-text-muted);margin:0;" {
                        "Tip: mention where you work, what you build, and where folks can find you."
                    }
                }
            }

            // Invalid (aria_invalid) — form framework flagged field
            section {
                h2 { "Invalid (aria)" }
                p.mui-showcase__caption { "Form framework set aria_invalid without styling as hard error." }
                div style="display:flex;flex-direction:column;gap:0.5rem;max-width:28rem;" {
                    label for="invalid-notes" style="font-size:0.875rem;font-weight:500;" { "Release notes" }
                    (render(Props {
                        name: "release-notes".into(),
                        id: "invalid-notes".into(),
                        placeholder: "Summarise what changed".into(),
                        aria_invalid: true,
                        ..Default::default()
                    }))
                    p style="font-size:0.75rem;color:var(--mui-text-muted);margin:0;" {
                        "Screen readers will announce this field as invalid."
                    }
                }
            }

            // Admin notes — read-only
            section {
                h2 { "Admin notes" }
                p.mui-showcase__caption { "Read-only. Changes require a support ticket." }
                div style="display:flex;flex-direction:column;gap:0.5rem;max-width:28rem;" {
                    label for="admin-notes" style="font-size:0.875rem;font-weight:500;color:var(--mui-text-muted);" {
                        "Admin notes \u{2014} read only"
                    }
                    (render(Props {
                        name: "admin-notes".into(),
                        id: "admin-notes".into(),
                        value: "Account flagged for manual review on 2026-04-10 by Sofia M. (billing). Reason: chargeback window open until 2026-05-10. Do not issue refunds without approval from #finance-ops.".into(),
                        readonly: true,
                        rows: 4,
                        ..Default::default()
                    }))
                    p style="font-size:0.75rem;color:var(--mui-text-muted);margin:0;" {
                        "Last updated by Sofia M. \u{00B7} 6 days ago"
                    }
                }
            }
        }
    }
}
