//! Code block — monospace pre-formatted code with a header (language + filename)
//! and an optional copy-to-clipboard button.
//!
//! Rendered without syntax highlighting — callers that want highlighting should
//! pre-render HTML and pass via [`Props::pre_rendered`].
use maud::{html, Markup, PreEscaped};

/// Code block rendering properties.
#[derive(Debug, Clone, Default)]
pub struct Props {
    /// Raw source — used when `pre_rendered` is `None`.
    pub code: String,
    /// Optional pre-rendered HTML (e.g. from a syntax-highlighter). Takes
    /// precedence over `code` when set.
    pub pre_rendered: Option<String>,
    /// Language label shown in the header (e.g. "rust", "bash").
    pub language: Option<String>,
    /// Optional filename shown next to the language.
    pub filename: Option<String>,
    /// Whether to render the copy button (default true).
    pub show_copy: bool,
    /// Optional max-height (CSS) — enables vertical scrolling when set.
    pub max_height: Option<String>,
}

/// Render a single code block.
pub fn render(props: Props) -> Markup {
    let show_copy = props.show_copy || matches!(props.show_copy, true);
    let has_header = props.language.is_some() || props.filename.is_some() || show_copy;
    let body_style = props
        .max_height
        .as_ref()
        .map(|h| format!("max-height: {h}; overflow: auto;"));

    html! {
        div class="mui-code-block" data-mui="code-block" {
            @if has_header {
                div class="mui-code-block__header" {
                    div class="mui-code-block__meta" {
                        @if let Some(lang) = props.language.as_ref() {
                            span class="mui-code-block__lang" { (lang) }
                        }
                        @if let Some(file) = props.filename.as_ref() {
                            span class="mui-code-block__file" { (file) }
                        }
                    }
                    @if show_copy {
                        button type="button"
                            class="mui-code-block__copy"
                            data-mui-copy="this"
                            aria-label="Copy code"
                            onclick=(r#"(function(btn){const pre=btn.closest('.mui-code-block').querySelector('pre');navigator.clipboard.writeText(pre.innerText).then(()=>{const o=btn.textContent;btn.textContent='Copied';setTimeout(()=>btn.textContent=o,1200);});})(this)"#)
                        {
                            "Copy"
                        }
                    }
                }
            }
            pre class="mui-code-block__pre" style=[body_style.as_deref()] {
                code {
                    @if let Some(rendered) = props.pre_rendered.as_ref() {
                        (PreEscaped(rendered.clone()))
                    } @else {
                        (props.code)
                    }
                }
            }
        }
    }
}

/// Showcase of the code block across a few common shapes.
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__column style="max-width: 44rem;" {
            (render(Props {
                code: "fn main() {\n    println!(\"hello, maud-ui!\");\n}".into(),
                language: Some("rust".into()),
                filename: Some("src/main.rs".into()),
                show_copy: true,
                ..Default::default()
            }))
            (render(Props {
                code: "curl -X POST https://api.kapable.dev/v1/sessions \\\n  -H 'x-api-key: sk_live_…' \\\n  -d '{\"prompt\":\"hi\"}'".into(),
                language: Some("bash".into()),
                show_copy: true,
                ..Default::default()
            }))
        }
    }
}
