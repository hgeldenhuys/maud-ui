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

/// Unordered list (disc markers, 1.5rem indent)
pub fn list_ul(items: Vec<Markup>) -> Markup {
    html! {
        ul.mui-typography-list."mui-typography-list--ul" {
            @for item in &items {
                li { (item) }
            }
        }
    }
}

/// Ordered list (decimal markers, 1.5rem indent)
pub fn list_ol(items: Vec<Markup>) -> Markup {
    html! {
        ol.mui-typography-list."mui-typography-list--ol" {
            @for item in &items {
                li { (item) }
            }
        }
    }
}

/// Large text (1.125rem, semibold) — shadcn `<div class="text-lg font-semibold">` equivalent
pub fn large(text: &str) -> Markup {
    html! { div.mui-typography-large { (text) } }
}

/// Small text (0.75rem, medium) — shadcn `<small>` equivalent
pub fn small(text: &str) -> Markup {
    html! { small.mui-typography-small { (text) } }
}

/// Table wrapper (horizontal scroll for narrow viewports) — wrap a `<table>` in this
pub fn table(children: Markup) -> Markup {
    html! {
        div.mui-typography-table-wrapper { (children) }
    }
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
                    (large("Large text for emphasis or section intros."))
                    (muted("Muted text for secondary information, timestamps, or helper copy."))
                    (small("Small text for fine print, captions, or metadata."))
                }
            }

            div {
                p.mui-showcase__caption { "Lists" }
                div.mui-showcase__column style="gap:0.75rem" {
                    (list_ul(vec![
                        html! { "First bullet item" },
                        html! { "Second bullet item" },
                        html! { "Third bullet item" },
                    ]))
                    (list_ol(vec![
                        html! { "Step one" },
                        html! { "Step two" },
                        html! { "Step three" },
                    ]))
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

            div {
                p.mui-showcase__caption { "Table" }
                (table(html! {
                    table style="border-collapse:collapse;width:100%" {
                        thead {
                            tr {
                                th style="text-align:left;padding:0.5rem;border-bottom:1px solid var(--mui-border)" { "Name" }
                                th style="text-align:left;padding:0.5rem;border-bottom:1px solid var(--mui-border)" { "Role" }
                                th style="text-align:left;padding:0.5rem;border-bottom:1px solid var(--mui-border)" { "Status" }
                            }
                        }
                        tbody {
                            tr {
                                td style="padding:0.5rem" { "Ada Lovelace" }
                                td style="padding:0.5rem" { "Mathematician" }
                                td style="padding:0.5rem" { "Active" }
                            }
                            tr {
                                td style="padding:0.5rem" { "Alan Turing" }
                                td style="padding:0.5rem" { "Cryptanalyst" }
                                td style="padding:0.5rem" { "Active" }
                            }
                        }
                    }
                }))
            }
        }
    }
}
