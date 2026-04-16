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
                div.mui-showcase__column style="gap:0.5rem" {
                    (h1("The quick brown fox"))
                    (h2("The quick brown fox"))
                    (h3("The quick brown fox"))
                    (h4("The quick brown fox"))
                }
            }

            div {
                p.mui-showcase__caption { "Body text" }
                div.mui-showcase__column style="gap:0.75rem" {
                    (lead("A lead paragraph is great for intros. It's visually larger and lighter than body text, drawing the eye without shouting."))
                    (p("Regular paragraph text. This is the standard body copy size used throughout the interface for readable content at comfortable line lengths."))
                    (muted("Muted text for secondary information, timestamps, or helper copy."))
                }
            }

            div {
                p.mui-showcase__caption { "Code" }
                div.mui-showcase__column style="gap:0.75rem" {
                    p.mui-p {
                        "Install dependencies with " (code_inline("bun install")) " then run " (code_inline("bun dev")) " to start."
                    }
                }
            }

            div {
                p.mui-showcase__caption { "Blockquote" }
                (blockquote("Design is not just what it looks like and feels like. Design is how it works."))
            }
        }
    }
}
