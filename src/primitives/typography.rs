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

/// Section eyebrow — a small uppercase, letter-spaced heading that labels a
/// band of content without competing with the page's own h1/h2 scale. Still a
/// real `<h2>`, so it keeps the document outline intact.
pub fn eyebrow(text: &str) -> Markup {
    html! { h2.mui-eyebrow { (text) } }
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

/// Heading level the first level of a prose blob is demoted to.
///
/// A markdown body normally starts at `#` / `<h1>`. Dropped into a page
/// verbatim that is a page-level heading, and a page rendering a handful of
/// them has no usable outline — one real surface produced 189 `<h1>` elements
/// this way. 4 leaves room for the host page's own h1 → h2 → h3.
pub const PROSE_HEADING_BASE: u8 = 4;

/// Prose wrapper for a server-rendered HTML blob (rendered markdown, a CMS
/// field, a changelog body). The per-element helpers above can't reach inside
/// markup you didn't build element by element; this styles the descendants
/// instead — headings, paragraphs, links, lists, `code`, and `pre`.
///
/// Headings in the blob are demoted to start at [`PROSE_HEADING_BASE`], so
/// content the page did not author cannot inject page-level headings. Use
/// [`prose_at`] to choose a different base.
pub fn prose(children: Markup) -> Markup {
    prose_at(children, PROSE_HEADING_BASE)
}

/// [`prose`] with an explicit heading base — pass the level the blob's top
/// heading should become, given where the block sits in the page outline.
pub fn prose_at(children: Markup, heading_base: u8) -> Markup {
    let demoted = demote_headings(&children.into_string(), heading_base);
    html! {
        div.mui-prose { (maud::PreEscaped(demoted)) }
    }
}

/// Shift every `<hN>` in an HTML blob so level 1 becomes `base`, clamped at 6.
///
/// Rewrites tag names only — attributes, content, and everything else pass
/// through untouched. Escaped text (`&lt;h1&gt;` inside a code block) can't
/// match, so code samples are safe.
fn demote_headings(html: &str, base: u8) -> String {
    let base = base.clamp(1, 6);
    if base == 1 {
        return html.to_string();
    }
    let mut out = String::with_capacity(html.len());
    let mut rest = html;
    // Every split point below lands on an ASCII byte, so the slicing stays on
    // UTF-8 boundaries regardless of what the blob contains.
    while let Some(pos) = rest.find('<') {
        out.push_str(&rest[..pos]);
        let tail = &rest[pos..];
        let b = tail.as_bytes();
        let closing = b.get(1) == Some(&b'/');
        let n = if closing { 2 } else { 1 };
        let is_heading = matches!(b.get(n), Some(b'h' | b'H'))
            && matches!(b.get(n + 1), Some(b'1'..=b'6'))
            // Guard against <hr>, <html>, <h1x>: the level must end the name.
            && matches!(b.get(n + 2), Some(b'>' | b' ' | b'\t' | b'\n' | b'\r' | b'/'));
        if is_heading {
            let level = b[n + 1] - b'0';
            let demoted = (base + level - 1).min(6);
            out.push('<');
            if closing {
                out.push('/');
            }
            out.push('h');
            out.push(char::from(b'0' + demoted));
            rest = &tail[n + 2..];
        } else {
            out.push('<');
            rest = &tail[1..];
        }
    }
    out.push_str(rest);
    out
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
                p.mui-showcase__caption { "Eyebrow" }
                (eyebrow("Recent activity"))
            }

            div {
                p.mui-showcase__caption { "Prose — a server-rendered HTML blob" }
                (prose(maud::PreEscaped(
                    "<h2>Release 0.3.0</h2>\
                     <p>Adds the <code>prose</code> wrapper for markdown bodies \
                     rendered outside the component tree.</p>\
                     <ul><li>Badge hues</li><li>Status dots</li></ul>\
                     <pre><code>cargo add maud-ui</code></pre>".to_string()
                )))
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
