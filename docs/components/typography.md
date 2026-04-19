# Typography

Text utility components for consistent typographic styling across headings, body text, lists, and code.

## Description

Typography provides a set of semantic and styled text components. Each function renders an appropriately semantic HTML element with maud-ui CSS classes for consistent sizing, weight, color, and spacing. Components range from headings (h1–h4) to body text (p, lead, muted, large, small), lists (ordered/unordered), code (inline and block), blockquote, and table wrapper.

## Import

```rust
use maud_ui::primitives::typography;
```

## Example

```rust
use maud::html;
use maud_ui::primitives::typography;

html! {
    (typography::h1("Page Title"))
    (typography::lead("An introductory paragraph with larger, lighter text."))
    (typography::p("Regular paragraph text goes here."))
    (typography::muted("This is muted secondary text."))
    (typography::code_inline("const x = 42"))
    (typography::list_ul(vec![
        html! { "Item one" },
        html! { "Item two" },
    ]))
}
```

## Functions

### Headings

#### h1(text: &str) -> Markup
Renders `<h1 class="mui-h1">`. Large primary heading (2rem, bold).

#### h2(text: &str) -> Markup
Renders `<h2 class="mui-h2">`. Secondary heading (1.5rem, semi-bold).

#### h3(text: &str) -> Markup
Renders `<h3 class="mui-h3">`. Tertiary heading (1.25rem, semi-bold).

#### h4(text: &str) -> Markup
Renders `<h4 class="mui-h4">`. Quaternary heading (1.125rem, semi-bold).

### Body Text

#### p(text: &str) -> Markup
Renders `<p class="mui-p">`. Standard paragraph text (0.875rem).

#### lead(text: &str) -> Markup
Renders `<p class="mui-lead">`. Large intro text (1.125rem, muted color, used for opening paragraphs).

#### large(text: &str) -> Markup
Renders `<div class="mui-typography-large">`. Large text for emphasis (1.125rem, semibold).

#### small(text: &str) -> Markup
Renders `<small class="mui-typography-small">`. Small text for fine print, captions (0.75rem, medium weight).

#### muted(text: &str) -> Markup
Renders `<span class="mui-muted">`. Muted secondary text (0.875rem, muted color, no block).

### Lists

#### list_ul(items: Vec<Markup>) -> Markup
Renders `<ul class="mui-typography-list mui-typography-list--ul">` with `<li>` children. Unordered list with disc markers and 1.5rem indent.

```rust
typography::list_ul(vec![
    html! { "First item" },
    html! { "Second item" },
    html! { "Third item" },
])
```

#### list_ol(items: Vec<Markup>) -> Markup
Renders `<ol class="mui-typography-list mui-typography-list--ol">` with `<li>` children. Ordered list with decimal markers and 1.5rem indent.

```rust
typography::list_ol(vec![
    html! { "Step one" },
    html! { "Step two" },
    html! { "Step three" },
])
```

### Code

#### code_inline(text: &str) -> Markup
Renders `<code class="mui-code">`. Inline monospace code (small, gray background, no block). Suitable for keywords, variable names, function calls.

```rust
html! {
    p {
        "Install with " (typography::code_inline("bun install")) "."
    }
}
```

### Blockquote

#### blockquote(text: &str) -> Markup
Renders `<blockquote class="mui-blockquote">`. Styled quote with italic text, muted color, and left border.

```rust
typography::blockquote("Design is how it works, not just how it looks.")
```

### Table

#### table(children: Markup) -> Markup
Renders `<div class="mui-typography-table-wrapper">` containing the provided markup. Enables horizontal scrolling on narrow viewports for wide `<table>` elements.

```rust
typography::table(html! {
    table {
        thead { tr { th { "Col 1" } th { "Col 2" } } }
        tbody { tr { td { "Value 1" } td { "Value 2" } } }
    }
})
```

## Variants

### Heading Hierarchy
Complete heading levels from h1 down to h4.

```rust
(typography::h1("Main Title"))
(typography::h2("Section Title"))
(typography::h3("Subsection"))
(typography::h4("Sub-subsection"))
```

### Lead Paragraph
Large introductory text that draws the eye.

```rust
(typography::lead("This is your opening paragraph, visually larger and lighter than body text."))
(typography::p("The rest of the article goes here in standard paragraph text."))
```

### Emphasized Large Text
Large text for callouts or section intros.

```rust
(typography::large("Important update: System will be down for maintenance."))
```

### Muted Supporting Text
Secondary information in gray.

```rust
html! {
    p { (typography::muted("Updated 2 hours ago")) }
}
```

### Small Fine Print
Captions, metadata, legal text.

```rust
(typography::small("© 2026 Your Company. All rights reserved."))
```

### Inline Code
Monospace for keywords and variable names.

```rust
html! {
    p {
        "To start development, run " (typography::code_inline("bun dev")) "."
    }
}
```

### Blockquote
Semantic quote with visual styling.

```rust
(typography::blockquote("The best way to predict the future is to invent it."))
```

### Unordered List
Bullet-point list.

```rust
(typography::list_ul(vec![
    html! { "Feature one" },
    html! { "Feature two" },
    html! { "Feature three" },
]))
```

### Ordered List
Numbered list for steps or sequences.

```rust
(typography::list_ol(vec![
    html! { "Register an account" },
    html! { "Create your first project" },
    html! { "Deploy to production" },
]))
```

### Responsive Table Wrapper
Tables that scroll horizontally on mobile.

```rust
(typography::table(html! {
    table {
        thead {
            tr {
                th { "Name" }
                th { "Role" }
                th { "Email" }
            }
        }
        tbody {
            tr { td { "Alice" } td { "Engineer" } td { "alice@example.com" } }
            tr { td { "Bob" } td { "Designer" } td { "bob@example.com" } }
        }
    }
}))
```

## Accessibility

- **Semantic HTML:** All components use appropriate semantic elements (`<h1>`–`<h4>`, `<p>`, `<blockquote>`, `<ul>`, `<ol>`, `<code>`).
- **Heading Hierarchy:** Use h1 for page title, h2 for sections, h3/h4 for subsections. Never skip levels.
- **List Semantics:** Use `list_ul()` and `list_ol()` for actual lists; don't style divs as lists.
- **Code Readability:** Inline code is sufficiently contrasted from body text for readability.
- **Link Underlines:** When linking inside paragraphs, ensure links are underlined or otherwise distinguished.

## Related

- [Button](/docs/components/button.md) — Action elements often paired with typography
- [Card](/docs/components/card.md) — Typography inside card layouts
- [Badge](/docs/components/badge.md) — Labels and status indicators
- [Input](/docs/components/input.md) — Form labels and hints typically use typography

## Shadcn Reference

Maud-ui Typography provides headings (h1–h4), body text (p, lead, large, small, muted), lists (list_ul, list_ol), code (code_inline), blockquote, and table wrapper—covering shadcn's text utility patterns.
