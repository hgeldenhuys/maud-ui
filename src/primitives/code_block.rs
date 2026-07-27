//! Code block — monospace pre-formatted code with a header (language + filename)
//! and an optional copy-to-clipboard button.
//!
//! Built-in syntax highlighting is emitted as `<span class="mui-code__tok
//! mui-code__tok--<kind>">…</span>` spans when [`Props::language`] is one of
//! `rust`/`rs`, `bash`/`sh`/`shell`, `typescript`/`ts`/`tsx`/`javascript`/`js`/
//! `jsx`, `json`, or `css`.
//! Token classes are styled in `css/components/code_block.css` using theme-aware
//! CSS vars (`--mui-code-*`) that flip between dark and `[data-theme="light"]`.
//!
//! ANY OTHER VALUE renders the source unhighlighted, wrapped in a
//! `<span data-mui-highlight="unsupported" data-language="…">` marker. The
//! marker exists because the failure is otherwise invisible: flat grey code
//! sitting next to coloured code reads as a broken highlighter, not as an
//! uncovered language. If a snippet looks dead, grep the rendered HTML for
//! `data-mui-highlight` before suspecting the stylesheet.
//!
//! Callers that want an external highlighter (syntect, shiki rendered
//! server-side, etc.) keep the escape hatch via [`Props::pre_rendered`], which
//! takes precedence over the built-in tokenizer.
use maud::{html, Markup, PreEscaped};

/// Code block rendering properties.
#[derive(Debug, Clone)]
pub struct Props {
    /// Raw source — used when `pre_rendered` is `None`. Highlighted per
    /// [`Props::language`] when the language is recognised.
    pub code: String,
    /// Optional pre-rendered HTML (e.g. from a syntax-highlighter). Takes
    /// precedence over `code` and the built-in highlighter when set.
    pub pre_rendered: Option<String>,
    /// Language label shown in the header (e.g. "rust", "bash"). When set to a
    /// recognised token, drives the built-in highlighter; unknown values fall
    /// back to plain-text rendering.
    pub language: Option<String>,
    /// Optional filename shown next to the language.
    pub filename: Option<String>,
    /// Whether to render the copy button (default true).
    pub show_copy: bool,
    /// Optional max-height (CSS) — enables vertical scrolling when set.
    pub max_height: Option<String>,
}

impl Default for Props {
    /// Hand-written rather than derived because `show_copy` must default to
    /// `true`: the field has always been documented as "default true", but a
    /// derived `Default` gave it `bool::default()` — `false` — so every
    /// `..Default::default()` construction silently dropped the copy button.
    /// Safe to correct now; this primitive was registered nowhere until 0.4.0,
    /// so nothing depended on the wrong value.
    fn default() -> Self {
        Self {
            code: String::new(),
            pre_rendered: None,
            language: None,
            filename: None,
            show_copy: true,
            max_height: None,
        }
    }
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
                    } @else if let Some(lang) = props.language.as_ref() {
                        (highlight(&props.code, lang))
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
                code: "curl -X POST https://api.example.com/v1/sessions \\\n  -H 'x-api-key: sk_live_…' \\\n  -d '{\"prompt\":\"hi\"}'".into(),
                language: Some("bash".into()),
                show_copy: true,
                ..Default::default()
            }))
        }
    }
}

// ---------------------------------------------------------------------------
// Built-in syntax highlighter — regex-free, ~300 LOC, no external deps.
//
// Design: for each supported language we iterate byte-by-byte and greedily
// match simple lexical classes (string, number, comment, identifier). An
// identifier is then classified against a small keyword / type table. This is
// deliberately coarse — a showcase-grade highlighter, not a full parser. It
// will, for example, mis-tag a `String` that appears inside a macro's string
// literal, but the literal branch always wins so the common case is correct.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tok {
    Text,
    Keyword,
    StringLit,
    Number,
    Comment,
    Type,
    Attribute,
    Lifetime,
    Flag,
    Variable,
    Boolean,
    Null,
    Punct,
}

impl Tok {
    fn class(self) -> Option<&'static str> {
        match self {
            Tok::Text => None,
            Tok::Keyword => Some("keyword"),
            Tok::StringLit => Some("string"),
            Tok::Number => Some("number"),
            Tok::Comment => Some("comment"),
            Tok::Type => Some("type"),
            Tok::Attribute => Some("attribute"),
            Tok::Lifetime => Some("lifetime"),
            Tok::Flag => Some("flag"),
            Tok::Variable => Some("variable"),
            Tok::Boolean => Some("boolean"),
            Tok::Null => Some("null"),
            Tok::Punct => Some("punct"),
        }
    }
}

fn highlight(source: &str, language: &str) -> Markup {
    let spans: Vec<(Tok, &str)> = match language.to_ascii_lowercase().as_str() {
        "rust" | "rs" => tokenize_rust(source),
        "bash" | "sh" | "shell" => tokenize_bash(source),
        "typescript" | "ts" | "tsx" | "javascript" | "js" | "jsx" => tokenize_ts(source),
        "json" => tokenize_json(source),
        "css" => tokenize_css(source),
        // Unsupported language. The snippet still renders — losing the code
        // would be far worse than losing its colour — but it renders FLAT, and
        // flat grey code next to highlighted code reads as "the highlighter is
        // broken" rather than "this language isn't covered". That ambiguity
        // cost a real diagnosis: a `css` snippet on the landing page rendered
        // dead and looked like a styling bug.
        //
        // So the fallback is no longer silent. The marker names the language
        // that was asked for, which is what you need to know to either add a
        // tokenizer or fix a typo'd label.
        _ => {
            return html! {
                span data-mui-highlight="unsupported" data-language=(language) { (source) }
            }
        }
    };
    html! {
        @for (tok, text) in &spans {
            @match tok.class() {
                Some(cls) => span class=(format!("mui-code__tok mui-code__tok--{cls}")) { (text) },
                None => (text),
            }
        }
    }
}

// --- Tiny byte-level cursor helpers ---------------------------------------

fn is_ident_start(b: u8) -> bool {
    b == b'_' || b.is_ascii_alphabetic()
}
fn is_ident_cont(b: u8) -> bool {
    b == b'_' || b.is_ascii_alphanumeric()
}

/// Greedy string-literal scanner. Handles backslash escapes. Returns the byte
/// length consumed including both quote chars; caller ensures `src[0]` is
/// the opening quote.
fn scan_string(src: &[u8], quote: u8) -> usize {
    let mut i = 1;
    while i < src.len() {
        let b = src[i];
        if b == b'\\' && i + 1 < src.len() {
            i += 2;
            continue;
        }
        if b == quote {
            return i + 1;
        }
        // strings in our target langs are single-line; if we hit a newline
        // before the close, stop so we don't swallow the rest of the file.
        if b == b'\n' {
            return i;
        }
        i += 1;
    }
    src.len()
}

/// Scan a numeric literal (int, float, hex, bin, with optional type suffix).
fn scan_number(src: &[u8]) -> usize {
    let mut i = 0;
    if src.len() >= 2 && src[0] == b'0' && matches!(src[1], b'x' | b'X' | b'b' | b'B' | b'o' | b'O')
    {
        i = 2;
        while i < src.len() && (src[i].is_ascii_alphanumeric() || src[i] == b'_') {
            i += 1;
        }
        return i;
    }
    while i < src.len() && (src[i].is_ascii_digit() || src[i] == b'_') {
        i += 1;
    }
    if i < src.len() && src[i] == b'.' && i + 1 < src.len() && src[i + 1].is_ascii_digit() {
        i += 1;
        while i < src.len() && (src[i].is_ascii_digit() || src[i] == b'_') {
            i += 1;
        }
    }
    // optional exponent + type suffix (i32, u64, f64, n for bigint…)
    while i < src.len() && (src[i].is_ascii_alphanumeric() || src[i] == b'_') {
        i += 1;
    }
    i
}

/// Scan `//` to end-of-line or `/* … */` block comment. `src[0..=1]` is the
/// opener.
fn scan_line_comment(src: &[u8]) -> usize {
    let mut i = 0;
    while i < src.len() && src[i] != b'\n' {
        i += 1;
    }
    i
}
fn scan_block_comment(src: &[u8]) -> usize {
    let mut i = 2;
    while i + 1 < src.len() {
        if src[i] == b'*' && src[i + 1] == b'/' {
            return i + 2;
        }
        i += 1;
    }
    src.len()
}

fn scan_ident(src: &[u8]) -> usize {
    let mut i = 0;
    while i < src.len() && is_ident_cont(src[i]) {
        i += 1;
    }
    i
}

fn is_punct(b: u8) -> bool {
    matches!(
        b,
        b'{' | b'}'
            | b'('
            | b')'
            | b'['
            | b']'
            | b';'
            | b','
            | b':'
            | b'.'
            | b'='
            | b'+'
            | b'-'
            | b'*'
            | b'/'
            | b'%'
            | b'<'
            | b'>'
            | b'!'
            | b'?'
            | b'&'
            | b'|'
            | b'^'
            | b'~'
            | b'@'
    )
}

// --- Rust ------------------------------------------------------------------

const RUST_KEYWORDS: &[&str] = &[
    "as",
    "async",
    "await",
    "break",
    "const",
    "continue",
    "crate",
    "dyn",
    "else",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "return",
    "self",
    "Self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "unsafe",
    "use",
    "where",
    "while",
    "box",
    "macro_rules",
    "union",
];
const RUST_TYPES: &[&str] = &[
    "bool",
    "char",
    "str",
    "String",
    "u8",
    "u16",
    "u32",
    "u64",
    "u128",
    "usize",
    "i8",
    "i16",
    "i32",
    "i64",
    "i128",
    "isize",
    "f32",
    "f64",
    "Vec",
    "Option",
    "Result",
    "Box",
    "Rc",
    "Arc",
    "RefCell",
    "Cell",
    "Mutex",
    "RwLock",
    "HashMap",
    "HashSet",
    "BTreeMap",
    "BTreeSet",
    "Markup",
    "PreEscaped",
    "Props",
];

fn tokenize_rust(src: &str) -> Vec<(Tok, &str)> {
    let bytes = src.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        // whitespace → passthrough
        if b.is_ascii_whitespace() {
            let start = i;
            while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            out.push((Tok::Text, &src[start..i]));
            continue;
        }
        // line + block comments
        if b == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'/' {
            let n = scan_line_comment(&bytes[i..]);
            out.push((Tok::Comment, &src[i..i + n]));
            i += n;
            continue;
        }
        if b == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'*' {
            let n = scan_block_comment(&bytes[i..]);
            out.push((Tok::Comment, &src[i..i + n]));
            i += n;
            continue;
        }
        // attribute `#[...]` or `#![...]`
        if b == b'#' && i + 1 < bytes.len() && (bytes[i + 1] == b'[' || bytes[i + 1] == b'!') {
            let start = i;
            // find matching `]` on the same or following lines
            let mut depth = 0;
            while i < bytes.len() {
                if bytes[i] == b'[' {
                    depth += 1;
                }
                if bytes[i] == b']' {
                    depth -= 1;
                    if depth == 0 {
                        i += 1;
                        break;
                    }
                }
                i += 1;
            }
            out.push((Tok::Attribute, &src[start..i]));
            continue;
        }
        // lifetime `'a` vs char literal `'x'` / `'\n'`. A lifetime is `'`
        // followed by an ident-start char and then ident-cont chars that do
        // NOT close with another `'`. Peek to disambiguate without consuming.
        if b == b'\'' {
            if i + 1 < bytes.len() && is_ident_start(bytes[i + 1]) {
                // find end of ident
                let mut j = i + 1;
                while j < bytes.len() && is_ident_cont(bytes[j]) {
                    j += 1;
                }
                // if the next char closes the quote → it's a char literal
                // like `'x'`; otherwise it's a lifetime.
                if j < bytes.len() && bytes[j] == b'\'' {
                    let n = scan_string(&bytes[i..], b'\'');
                    out.push((Tok::StringLit, &src[i..i + n]));
                    i += n;
                } else {
                    out.push((Tok::Lifetime, &src[i..j]));
                    i = j;
                }
                continue;
            }
            // escape or wide char literal like `'\n'` — full string scan
            let n = scan_string(&bytes[i..], b'\'');
            out.push((Tok::StringLit, &src[i..i + n]));
            i += n;
            continue;
        }
        // strings `"..."` + raw `r"..."` / `r#"..."#`
        if b == b'"' {
            let n = scan_string(&bytes[i..], b'"');
            out.push((Tok::StringLit, &src[i..i + n]));
            i += n;
            continue;
        }
        if b == b'r' && i + 1 < bytes.len() && (bytes[i + 1] == b'"' || bytes[i + 1] == b'#') {
            // raw string: r"..." or r#"..."# (up to 6 hashes)
            let start = i;
            i += 1;
            let mut hashes = 0;
            while i < bytes.len() && bytes[i] == b'#' {
                hashes += 1;
                i += 1;
            }
            if i < bytes.len() && bytes[i] == b'"' {
                i += 1;
                while i < bytes.len() {
                    if bytes[i] == b'"' {
                        let mut j = i + 1;
                        let mut got = 0;
                        while j < bytes.len() && bytes[j] == b'#' && got < hashes {
                            j += 1;
                            got += 1;
                        }
                        if got == hashes {
                            i = j;
                            break;
                        }
                    }
                    i += 1;
                }
                out.push((Tok::StringLit, &src[start..i]));
                continue;
            }
            // wasn't actually a raw string — rewind to treat as ident
            i = start;
        }
        // numbers
        if b.is_ascii_digit() {
            let n = scan_number(&bytes[i..]);
            out.push((Tok::Number, &src[i..i + n]));
            i += n;
            continue;
        }
        // identifiers + keywords + types + macros
        if is_ident_start(b) {
            let n = scan_ident(&bytes[i..]);
            let word = &src[i..i + n];
            let tok = if RUST_KEYWORDS.contains(&word) {
                Tok::Keyword
            } else if RUST_TYPES.contains(&word)
                || word
                    .chars()
                    .next()
                    .map(|c| c.is_ascii_uppercase())
                    .unwrap_or(false)
            {
                Tok::Type
            } else {
                Tok::Text
            };
            out.push((tok, word));
            i += n;
            continue;
        }
        if is_punct(b) {
            out.push((Tok::Punct, &src[i..i + 1]));
            i += 1;
            continue;
        }
        // anything else (non-ASCII, stray bytes) → passthrough one char
        let ch_len = next_char_len(&bytes[i..]);
        out.push((Tok::Text, &src[i..i + ch_len]));
        i += ch_len;
    }
    out
}

// --- Bash ------------------------------------------------------------------

const BASH_KEYWORDS: &[&str] = &[
    "if", "then", "else", "elif", "fi", "for", "while", "do", "done", "case", "esac", "in",
    "function", "return", "local", "export", "readonly", "unset", "declare", "echo", "printf",
    "read", "set", "shift", "source", "exit", "trap",
];

fn tokenize_bash(src: &str) -> Vec<(Tok, &str)> {
    let bytes = src.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b.is_ascii_whitespace() {
            let start = i;
            while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            out.push((Tok::Text, &src[start..i]));
            continue;
        }
        // `#` line comment. Avoid misclassifying `#!/bin/bash` shebangs only at
        // start-of-line (good enough).
        if b == b'#' {
            let n = scan_line_comment(&bytes[i..]);
            out.push((Tok::Comment, &src[i..i + n]));
            i += n;
            continue;
        }
        if b == b'"' || b == b'\'' {
            let n = scan_string(&bytes[i..], b);
            out.push((Tok::StringLit, &src[i..i + n]));
            i += n;
            continue;
        }
        // $VAR / ${VAR}
        if b == b'$' && i + 1 < bytes.len() {
            let start = i;
            i += 1;
            if bytes[i] == b'{' {
                while i < bytes.len() && bytes[i] != b'}' {
                    i += 1;
                }
                if i < bytes.len() {
                    i += 1;
                }
            } else if is_ident_start(bytes[i]) {
                i += scan_ident(&bytes[i..]);
            }
            out.push((Tok::Variable, &src[start..i]));
            continue;
        }
        // --flag or -f (word starting with `-` after whitespace)
        if b == b'-'
            && (out
                .last()
                .map(|(_, s)| s.ends_with(char::is_whitespace))
                .unwrap_or(true))
            && i + 1 < bytes.len()
            && (bytes[i + 1] == b'-' || is_ident_start(bytes[i + 1]))
        {
            let start = i;
            i += 1;
            while i < bytes.len()
                && (is_ident_cont(bytes[i]) || bytes[i] == b'-' || bytes[i] == b'_')
            {
                i += 1;
            }
            out.push((Tok::Flag, &src[start..i]));
            continue;
        }
        if b.is_ascii_digit() {
            let n = scan_number(&bytes[i..]);
            out.push((Tok::Number, &src[i..i + n]));
            i += n;
            continue;
        }
        if is_ident_start(b) {
            let n = scan_ident(&bytes[i..]);
            let word = &src[i..i + n];
            let tok = if BASH_KEYWORDS.contains(&word) {
                Tok::Keyword
            } else {
                Tok::Text
            };
            out.push((tok, word));
            i += n;
            continue;
        }
        if is_punct(b) {
            out.push((Tok::Punct, &src[i..i + 1]));
            i += 1;
            continue;
        }
        let ch_len = next_char_len(&bytes[i..]);
        out.push((Tok::Text, &src[i..i + ch_len]));
        i += ch_len;
    }
    out
}

// --- TypeScript / JavaScript ----------------------------------------------

const TS_KEYWORDS: &[&str] = &[
    "abstract",
    "as",
    "async",
    "await",
    "break",
    "case",
    "catch",
    "class",
    "const",
    "continue",
    "debugger",
    "declare",
    "default",
    "delete",
    "do",
    "else",
    "enum",
    "export",
    "extends",
    "finally",
    "for",
    "from",
    "function",
    "get",
    "if",
    "implements",
    "import",
    "in",
    "instanceof",
    "interface",
    "is",
    "keyof",
    "let",
    "namespace",
    "new",
    "of",
    "package",
    "private",
    "protected",
    "public",
    "readonly",
    "return",
    "set",
    "static",
    "super",
    "switch",
    "this",
    "throw",
    "try",
    "type",
    "typeof",
    "var",
    "void",
    "while",
    "with",
    "yield",
];
const TS_TYPES: &[&str] = &[
    "string",
    "number",
    "boolean",
    "object",
    "any",
    "unknown",
    "never",
    "undefined",
    "null",
    "true",
    "false",
    "Promise",
    "Array",
    "Record",
    "Partial",
    "Readonly",
    "Pick",
    "Omit",
    "Date",
    "Map",
    "Set",
];

fn tokenize_ts(src: &str) -> Vec<(Tok, &str)> {
    let bytes = src.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b.is_ascii_whitespace() {
            let start = i;
            while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            out.push((Tok::Text, &src[start..i]));
            continue;
        }
        if b == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'/' {
            let n = scan_line_comment(&bytes[i..]);
            out.push((Tok::Comment, &src[i..i + n]));
            i += n;
            continue;
        }
        if b == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'*' {
            let n = scan_block_comment(&bytes[i..]);
            out.push((Tok::Comment, &src[i..i + n]));
            i += n;
            continue;
        }
        if b == b'"' || b == b'\'' || b == b'`' {
            let n = scan_string(&bytes[i..], b);
            out.push((Tok::StringLit, &src[i..i + n]));
            i += n;
            continue;
        }
        if b.is_ascii_digit() {
            let n = scan_number(&bytes[i..]);
            out.push((Tok::Number, &src[i..i + n]));
            i += n;
            continue;
        }
        if is_ident_start(b) {
            let n = scan_ident(&bytes[i..]);
            let word = &src[i..i + n];
            let is_capitalised = word
                .chars()
                .next()
                .map(|c| c.is_ascii_uppercase())
                .unwrap_or(false);
            let tok = if TS_KEYWORDS.contains(&word) {
                Tok::Keyword
            } else if TS_TYPES.contains(&word) || is_capitalised {
                Tok::Type
            } else {
                Tok::Text
            };
            out.push((tok, word));
            i += n;
            continue;
        }
        if is_punct(b) {
            out.push((Tok::Punct, &src[i..i + 1]));
            i += 1;
            continue;
        }
        let ch_len = next_char_len(&bytes[i..]);
        out.push((Tok::Text, &src[i..i + ch_len]));
        i += ch_len;
    }
    out
}

// --- JSON ------------------------------------------------------------------

/// CSS tokenizer.
///
/// Added because this library's entire argument is its custom properties, and
/// every snippet that showed one rendered as flat grey text — `css` fell
/// through to the unhighlighted branch. A token system you cannot read is a
/// poor advertisement for a token system.
///
/// Two pieces of context decide how an identifier is tagged, so both are
/// tracked as we go:
///
///   - `blocks`: a stack of "is this a declaration block?". A `{` opens a
///     declaration block normally, but NOT after an at-rule — the body of
///     `@media { … }` holds more selectors, not properties. Without this,
///     every selector nested in a media query got tagged as a property.
///   - `in_value`: set by `:` inside a declaration block, cleared by `;` `{`
///     `}`. It is what separates `color` (property) from `red` (value), and
///     it is deliberately NOT set by `:` at selector level so `a:hover` stays
///     a selector.
///
/// Coarse by design, like its siblings above: it is a showcase highlighter,
/// not a CSS parser.
fn tokenize_css(src: &str) -> Vec<(Tok, &str)> {
    let bytes = src.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    // Stack of block kinds; `true` = declarations live here.
    let mut blocks: Vec<bool> = Vec::new();
    let mut in_value = false;
    let mut pending_at_rule = false;

    let in_decl = |blocks: &Vec<bool>| *blocks.last().unwrap_or(&false);

    while i < bytes.len() {
        let b = bytes[i];

        if b.is_ascii_whitespace() {
            let start = i;
            while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            out.push((Tok::Text, &src[start..i]));
            continue;
        }

        // Comments. Unterminated ones run to EOF rather than panicking.
        if b == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'*' {
            let start = i;
            i += 2;
            while i + 1 < bytes.len() && !(bytes[i] == b'*' && bytes[i + 1] == b'/') {
                i += 1;
            }
            i = (i + 2).min(bytes.len());
            out.push((Tok::Comment, &src[start..i]));
            continue;
        }

        if b == b'"' || b == b'\'' {
            let n = scan_string(&bytes[i..], b);
            out.push((Tok::StringLit, &src[i..i + n]));
            i += n;
            continue;
        }

        // At-rule: @media, @import, @supports…
        if b == b'@' {
            let n = 1 + scan_css_ident(&bytes[i + 1..]);
            out.push((Tok::Keyword, &src[i..i + n]));
            i += n;
            pending_at_rule = true;
            continue;
        }

        // Custom property — `--mui-accent`. The single most important thing on
        // screen for this library, so it gets its own colour rather than
        // sharing the generic property tint.
        if b == b'-' && i + 1 < bytes.len() && bytes[i + 1] == b'-' {
            let n = 2 + scan_css_ident(&bytes[i + 2..]);
            out.push((Tok::Variable, &src[i..i + n]));
            i += n;
            continue;
        }

        // `#` is a hex colour in a value position and an id selector outside
        // one. Same character, two meanings, and only the context separates
        // them.
        if b == b'#' {
            let n = 1 + scan_css_ident(&bytes[i + 1..]);
            let tok = if in_value { Tok::Number } else { Tok::Type };
            out.push((tok, &src[i..i + n]));
            i += n;
            continue;
        }

        // Class selector / decimal-leading number.
        if b == b'.' {
            if i + 1 < bytes.len() && bytes[i + 1].is_ascii_digit() {
                let start = i;
                i += 1 + scan_css_number_tail(&bytes[i + 1..]);
                out.push((Tok::Number, &src[start..i]));
                continue;
            }
            let n = 1 + scan_css_ident(&bytes[i + 1..]);
            if n > 1 {
                out.push((Tok::Type, &src[i..i + n]));
                i += n;
                continue;
            }
        }

        if b.is_ascii_digit() {
            let start = i;
            i += scan_css_number_tail(&bytes[i..]);
            out.push((Tok::Number, &src[start..i]));
            continue;
        }

        if is_ident_start(b) || b == b'-' {
            let n = scan_css_ident(&bytes[i..]);
            let word = &src[i..i + n];
            let next_non_space = bytes[i + n..]
                .iter()
                .find(|c| !c.is_ascii_whitespace())
                .copied();
            let tok = if next_non_space == Some(b'(') {
                // A function: var(), calc(), rgb(), clamp()…
                Tok::Type
            } else if in_decl(&blocks) && !in_value {
                Tok::Attribute
            } else if in_decl(&blocks) {
                Tok::Text
            } else {
                // Selector level: element names, and the keywords inside an
                // at-rule prelude.
                Tok::Type
            };
            out.push((tok, word));
            i += n;
            continue;
        }

        match b {
            b'{' => {
                blocks.push(!pending_at_rule);
                pending_at_rule = false;
                in_value = false;
            }
            b'}' => {
                blocks.pop();
                in_value = false;
            }
            b';' => {
                in_value = false;
                pending_at_rule = false;
            }
            b':' => {
                if in_decl(&blocks) {
                    in_value = true;
                }
            }
            _ => {}
        }

        if is_punct(b) || b == b'%' {
            out.push((Tok::Punct, &src[i..i + 1]));
            i += 1;
            continue;
        }

        let ch_len = next_char_len(&bytes[i..]);
        out.push((Tok::Text, &src[i..i + ch_len]));
        i += ch_len;
    }
    out
}

/// CSS identifiers may contain hyphens, which `scan_ident` (alphanumeric plus
/// underscore) stops at — it would split `border-radius` into three tokens.
fn scan_css_ident(bytes: &[u8]) -> usize {
    let mut n = 0;
    while n < bytes.len() && (is_ident_cont(bytes[n]) || bytes[n] == b'-') {
        n += 1;
    }
    n
}

/// A CSS number carries its unit: `0.75rem`, `100%`, `2px`, `1e3`. Keeping the
/// unit inside the number token stops `rem` being re-read as a property name.
fn scan_css_number_tail(bytes: &[u8]) -> usize {
    let mut n = 0;
    while n < bytes.len() && (bytes[n].is_ascii_digit() || bytes[n] == b'.') {
        n += 1;
    }
    while n < bytes.len() && bytes[n].is_ascii_alphabetic() {
        n += 1;
    }
    if n < bytes.len() && bytes[n] == b'%' {
        n += 1;
    }
    n
}

fn tokenize_json(src: &str) -> Vec<(Tok, &str)> {
    let bytes = src.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b.is_ascii_whitespace() {
            let start = i;
            while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            out.push((Tok::Text, &src[start..i]));
            continue;
        }
        if b == b'"' {
            let n = scan_string(&bytes[i..], b'"');
            out.push((Tok::StringLit, &src[i..i + n]));
            i += n;
            continue;
        }
        if b.is_ascii_digit() || (b == b'-' && i + 1 < bytes.len() && bytes[i + 1].is_ascii_digit())
        {
            let start = i;
            if b == b'-' {
                i += 1;
            }
            i += scan_number(&bytes[i..]);
            out.push((Tok::Number, &src[start..i]));
            continue;
        }
        if is_ident_start(b) {
            let n = scan_ident(&bytes[i..]);
            let word = &src[i..i + n];
            let tok = match word {
                "true" | "false" => Tok::Boolean,
                "null" => Tok::Null,
                _ => Tok::Text,
            };
            out.push((tok, word));
            i += n;
            continue;
        }
        if is_punct(b) {
            out.push((Tok::Punct, &src[i..i + 1]));
            i += 1;
            continue;
        }
        let ch_len = next_char_len(&bytes[i..]);
        out.push((Tok::Text, &src[i..i + ch_len]));
        i += ch_len;
    }
    out
}

/// Return the UTF-8 byte length of the character starting at `bytes[0]`. Used
/// as a safe fallback so we never split a multi-byte codepoint when emitting
/// `&str` slices.
fn next_char_len(bytes: &[u8]) -> usize {
    if bytes.is_empty() {
        return 0;
    }
    match bytes[0] {
        // ASCII (0..=0x7F) and stray continuation bytes (0x80..=0xBF) both
        // advance by one byte. Continuation bytes only appear if earlier
        // UTF-8 parsing got out of sync — we advance safely instead of
        // panicking.
        b if b < 0xC0 => 1,
        b if b < 0xE0 => 2,
        b if b < 0xF0 => 3,
        _ => 4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn classes(spans: &[(Tok, &str)]) -> Vec<&'static str> {
        spans
            .iter()
            .filter_map(|(t, _)| t.class())
            .collect::<Vec<_>>()
    }

    /// Finds the token kind a given piece of source text was tagged with.
    fn kind_of<'a>(spans: &'a [(Tok, &str)], needle: &str) -> Option<&'static str> {
        spans
            .iter()
            .find(|(_, text)| *text == needle)
            .and_then(|(t, _)| t.class())
    }

    #[test]
    fn css_separates_properties_from_values_and_selectors() {
        let src = ".card {\n  color: red;\n  border-radius: 0.5rem;\n}";
        let toks = tokenize_css(src);
        assert_eq!(kind_of(&toks, ".card"), Some("type"), "selector");
        assert_eq!(kind_of(&toks, "color"), Some("attribute"), "property name");
        // `red` sits after the colon, so it must NOT be tagged as a property.
        assert_eq!(kind_of(&toks, "red"), None, "value");
        assert_eq!(
            kind_of(&toks, "border-radius"),
            Some("attribute"),
            "hyphenated property must stay one token"
        );
        assert_eq!(kind_of(&toks, "0.5rem"), Some("number"), "number keeps unit");
    }

    #[test]
    fn css_tags_custom_properties_and_functions() {
        let src = ":root { --mui-accent: #2563eb; }\n.b { color: var(--mui-accent); }";
        let toks = tokenize_css(src);
        assert_eq!(kind_of(&toks, "--mui-accent"), Some("variable"));
        assert_eq!(kind_of(&toks, "var"), Some("type"), "function call");
        assert_eq!(kind_of(&toks, "#2563eb"), Some("number"), "hex in a value");
    }

    /// The bug this tokenizer's block stack exists to prevent: inside
    /// `@media { … }` the nested `.card` is a SELECTOR, but a naive
    /// "any brace opens a declaration block" rule tags it as a property name.
    #[test]
    fn css_selectors_nested_in_an_at_rule_are_not_properties() {
        let src = "@media (min-width: 40rem) {\n  .card { color: red; }\n}";
        let toks = tokenize_css(src);
        assert_eq!(kind_of(&toks, "@media"), Some("keyword"));
        assert_eq!(
            kind_of(&toks, ".card"),
            Some("type"),
            "a selector inside @media must not be tagged as a property"
        );
        assert_eq!(kind_of(&toks, "color"), Some("attribute"));
    }

    /// `#` is a hex colour in a value and an id selector outside one.
    #[test]
    fn css_hash_is_context_sensitive() {
        let toks = tokenize_css("#main { color: #fff; }");
        assert_eq!(kind_of(&toks, "#main"), Some("type"), "id selector");
        assert_eq!(kind_of(&toks, "#fff"), Some("number"), "hex colour");
    }

    /// Tokenizers must never lose or reorder source. Reassembling every span
    /// has to reproduce the input byte for byte.
    #[test]
    fn css_tokenizer_is_lossless() {
        let src = "/* c */\n@media (min-width: 40rem) {\n  .a::after { content: \"x\"; width: 50%; }\n}\n";
        let joined: String = tokenize_css(src).iter().map(|(_, t)| *t).collect();
        assert_eq!(joined, src, "tokenizer dropped or reordered source");
    }

    /// An unsupported language must still render its source, and must say so.
    #[test]
    fn unsupported_language_renders_source_with_a_marker() {
        let html = highlight("SELECT 1;", "sql").into_string();
        assert!(html.contains("SELECT 1;"), "source must survive");
        assert!(
            html.contains(r#"data-mui-highlight="unsupported""#),
            "an uncovered language must be inspectable, not silently flat"
        );
        assert!(html.contains(r#"data-language="sql""#));
    }

    #[test]
    fn rust_tags_keywords_types_strings_numbers_comments() {
        let src = "// hi\nfn main() { let x: u32 = 42; println!(\"hi\"); }";
        let toks = tokenize_rust(src);
        let cls = classes(&toks);
        assert!(cls.contains(&"comment"));
        assert!(cls.contains(&"keyword"));
        assert!(cls.contains(&"type"));
        assert!(cls.contains(&"number"));
        assert!(cls.contains(&"string"));
    }

    #[test]
    fn rust_lifetime_vs_char_literal() {
        let src = "fn f<'a>(c: char) { let _ = 'x'; }";
        let toks = tokenize_rust(src);
        assert!(toks.iter().any(|(t, s)| *t == Tok::Lifetime && *s == "'a"));
        assert!(toks
            .iter()
            .any(|(t, s)| *t == Tok::StringLit && *s == "'x'"));
    }

    #[test]
    fn rust_attribute_is_one_span() {
        let src = "#[derive(Debug)] struct S;";
        let toks = tokenize_rust(src);
        assert!(toks
            .iter()
            .any(|(t, s)| *t == Tok::Attribute && s.starts_with("#[") && s.ends_with(']')));
    }

    #[test]
    fn bash_flag_variable_comment_string() {
        // `$HOME` outside any quote so the variable scanner sees it. $VARs
        // inside a double-quoted string are covered by the string class, by
        // design (keeps the tokenizer trivial).
        let src = "# deploy\ncd $HOME && curl --fail -X POST \"$URL\" -H 'x-api-key: k'";
        let toks = tokenize_bash(src);
        let cls = classes(&toks);
        assert!(cls.contains(&"comment"));
        assert!(cls.contains(&"flag"));
        assert!(cls.contains(&"variable"));
        assert!(cls.contains(&"string"));
    }

    #[test]
    fn ts_keyword_type_string_number() {
        let src = "export const n: number = 1; /* ok */ const s: string = \"a\";";
        let toks = tokenize_ts(src);
        let cls = classes(&toks);
        assert!(cls.contains(&"keyword"));
        assert!(cls.contains(&"type"));
        assert!(cls.contains(&"string"));
        assert!(cls.contains(&"number"));
        assert!(cls.contains(&"comment"));
    }

    #[test]
    fn json_boolean_null_number_string() {
        let src = r#"{"a": true, "b": null, "c": 3.14, "d": "hi"}"#;
        let toks = tokenize_json(src);
        let cls = classes(&toks);
        assert!(cls.contains(&"boolean"));
        assert!(cls.contains(&"null"));
        assert!(cls.contains(&"number"));
        assert!(cls.contains(&"string"));
    }

    #[test]
    fn unknown_language_passes_through_unhighlighted() {
        // Exercises the match-arm for unknown languages. It used to return the
        // source completely bare; it now wraps it in an inert marker span.
        //
        // The assertion was deliberately loosened rather than the behaviour
        // reverted: bare output made an uncovered language indistinguishable
        // from a broken stylesheet, which is exactly how a flat `css` snippet
        // on the landing page got misread. What actually matters here — and
        // what this still pins — is that NO token classes are applied and the
        // source survives intact.
        let out = highlight("totally plain", "cobol").into_string();
        assert!(out.contains("totally plain"), "source must survive");
        assert!(
            !out.contains("mui-code__tok"),
            "an unknown language must not get token classes"
        );
    }

    #[test]
    fn non_ascii_is_not_split() {
        // A cheeky emoji inside a Rust source snippet. Must not panic on char
        // boundaries — next_char_len() guards us.
        let src = "// 🚀 launch\nfn go() {}";
        let toks = tokenize_rust(src);
        assert!(!toks.is_empty());
    }
}
