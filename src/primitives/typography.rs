//! Typography component — text utility components for consistent typographic styling.

use maud::{html, Markup};

/// Heading 1 (2rem, bold)
pub fn h1(text: &str) -> Markup {
    html! { h1.mui-h1 { (text) } }
}

/// Heading 2 (1.5rem, semi-bold)
pub fn h2(text: &str) -> Markup {
    html! { h2.mui-h2 { (text) } }
}

/// Heading 3 (1.25rem, semi-bold)
pub fn h3(text: &str) -> Markup {
    html! { h3.mui-h3 { (text) } }
}

/// Heading 4 (1.125rem, semi-bold)
pub fn h4(text: &str) -> Markup {
    html! { h4.mui-h4 { (text) } }
}

/// Paragraph (0.875rem, standard)
pub fn p(text: &str) -> Markup {
    html! { p.mui-p { (text) } }
}

/// Lead text (large intro, 1.125rem, muted)
pub fn lead(text: &str) -> Markup {
    html! { p.mui-lead { (text) } }
}

/// Muted text (0.875rem, muted color)
pub fn muted(text: &str) -> Markup {
    html! { span.mui-muted { (text) } }
}

/// Inline code (monospace, small)
pub fn code_inline(text: &str) -> Markup {
    html! { code.mui-code { (text) } }
}

/// Blockquote (italic, muted, left border)
pub fn blockquote(text: &str) -> Markup {
    html! { blockquote.mui-blockquote { (text) } }
}

/// Showcase all typography elements
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            div {
                p.mui-showcase__caption { "Headings" }
                div {
                    (h1("Heading 1"))
                    (h2("Heading 2"))
                    (h3("Heading 3"))
                    (h4("Heading 4"))
                }
            }

            div {
                p.mui-showcase__caption { "Body text" }
                div {
                    (p("Paragraph text at standard size."))
                    (lead("Lead text for introductions and emphasis."))
                    (muted("Muted text for secondary information."))
                }
            }

            div {
                p.mui-showcase__caption { "Code and quotes" }
                div {
                    p {
                        "Use " (code_inline("code_inline")) " for keywords."
                    }
                    (blockquote("A great blockquote can inspire action and reflection."))
                }
            }
        }
    }
}
