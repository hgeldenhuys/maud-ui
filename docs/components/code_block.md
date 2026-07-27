# CodeBlock

A monospace, pre-formatted code display primitive with an optional header (language label + filename) and a copy-to-clipboard button. Ships a small built-in syntax highlighter for a handful of common languages, with an escape hatch (`pre_rendered`) for HTML produced by an external highlighter.

## Import

```rust
use maud_ui::primitives::code_block::{self, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::code_block;

html! {
    (code_block::render(code_block::Props {
        code: "fn main() {\n    println!(\"hello, maud-ui!\");\n}".into(),
        language: Some("rust".into()),
        filename: Some("src/main.rs".into()),
        show_copy: true,
        ..Default::default()
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `code` | `String` | `String::new()` (`""`) | Raw source — used when `pre_rendered` is `None`. Highlighted per `language` when the language is recognised. |
| `pre_rendered` | `Option<String>` | `None` | Optional pre-rendered HTML (e.g. from a syntax-highlighter such as syntect or server-rendered shiki). Takes precedence over `code` and the built-in highlighter when set. |
| `language` | `Option<String>` | `None` | Language label shown in the header (e.g. `"rust"`, `"bash"`). When set to a recognised token, drives the built-in highlighter; unknown values fall back to plain-text rendering. |
| `filename` | `Option<String>` | `None` | Optional filename shown next to the language. |
| `show_copy` | `bool` | `true` | Whether to render the copy button. |
| `max_height` | `Option<String>` | `None` | Optional max-height (CSS) — enables vertical scrolling when set. |

**Note on `show_copy`'s default:** this field is `true` by default, and `Props` has a hand-written `impl Default` rather than a derived one specifically to make that so. Until 0.4.0 the `Default` was derived, which gave `bool::default()` — `false` — while the field's own doc comment claimed "default true"; every `..Default::default()` construction silently dropped the copy button. Corrected in 0.4.0, when this primitive was first registered.

**Header visibility:** the header row (language/filename/copy button) only renders when at least one of `language`, `filename` is `Some(_)`, or `show_copy` is `true` (`has_header = props.language.is_some() || props.filename.is_some() || show_copy`). Otherwise `render` emits a bare `<pre><code>` with no header.

**Language selection & built-in highlighting:** `language` is matched case-insensitively (`to_ascii_lowercase()`) against:

| Tokens | Highlighter |
|--------|-------------|
| `rust`, `rs` | `tokenize_rust` |
| `bash`, `sh`, `shell` | `tokenize_bash` |
| `typescript`, `ts`, `tsx`, `javascript`, `js`, `jsx` | `tokenize_ts` (one shared tokenizer for the whole TS/JS family) |
| `json` | `tokenize_json` |

Any other value (or `language: None`) skips the tokenizer entirely and renders `code` as plain, unhighlighted text. `pre_rendered`, when `Some`, bypasses the built-in highlighter regardless of `language`.

## Variants / Enums

None. `code_block` has no `pub` enum — the module does define a private `enum Tok` (`Text`, `Keyword`, `StringLit`, `Number`, `Comment`, `Type`, `Attribute`, `Lifetime`, `Flag`, `Variable`, `Boolean`, `Null`, `Punct`) used internally to classify highlighter spans into `mui-code__tok--<kind>` CSS classes, but it is not part of the public API and cannot be constructed or matched on by callers.

## Helper Functions

None. Every function in the module besides `render` and `showcase` — `highlight`, `tokenize_rust`, `tokenize_bash`, `tokenize_ts`, `tokenize_json`, `scan_string`, `scan_number`, `scan_line_comment`, `scan_block_comment`, `scan_ident`, `is_punct`, `is_ident_start`, `is_ident_cont`, `next_char_len`, and `Tok::class` — is private (module-internal, no `pub`) and is not callable from outside `code_block`.

## Accessibility

- The copy button is a real `<button type="button">` (not a `div`/`span`), so it is natively focusable and keyboard-operable (Enter/Space) without extra wiring, and carries `aria-label="Copy code"` for screen-reader users since its visible label is just "Copy".
- On click, the button's own `onclick` handler reads `pre.innerText` from the sibling `<pre>` inside the same `.mui-code-block`, calls `navigator.clipboard.writeText`, and swaps its text content to "Copied" for 1200ms before reverting — but this swap has no `aria-live` region, so screen readers are not guaranteed to announce the "Copied" confirmation.
- Code is marked up as a plain `<pre><code>` pair — standard semantics for preformatted code, no extra `role` is set.
- The outer wrapper carries `data-mui="code-block"`, a component-identification hook, not an ARIA attribute.
- Syntax-highlight spans (`mui-code__tok mui-code__tok--<kind>`) are presentational only — no `aria-*` or `role` is attached to them, so they do not add or remove anything from how the code is announced.
- Language and filename are rendered as plain visible text (`span.mui-code-block__lang`, `span.mui-code-block__file`) with no `aria-label`/`aria-description` binding them to the code region.

## Related

Message, Diff, ToolCall, StreamingCursor — CodeBlock is part of the AI-chat/agent primitive family and commonly appears embedded inside Message bodies, Diff views, and ToolCall output panels.

## Shadcn reference

No shadcn equivalent — `code_block` is a custom AI-chat/agent-family primitive with no corresponding component in shadcn/ui.
