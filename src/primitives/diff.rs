//! Unified diff viewer — renders line-level changes with line numbers,
//! ± counts, and hunk headers. No syntax highlighting — the renderer
//! focuses on accurate layout.

use maud::{html, Markup};

/// Kind of line in a diff — drives row colouring.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineKind {
    Context,
    Add,
    Remove,
    Hunk,
}

impl LineKind {
    fn class(&self) -> &'static str {
        match self {
            Self::Context => "mui-diff__line--context",
            Self::Add => "mui-diff__line--add",
            Self::Remove => "mui-diff__line--remove",
            Self::Hunk => "mui-diff__line--hunk",
        }
    }

    fn sigil(&self) -> &'static str {
        match self {
            Self::Context => " ",
            Self::Add => "+",
            Self::Remove => "-",
            Self::Hunk => "@",
        }
    }

    /// Text announced to assistive tech in place of the (aria-hidden) sigil and
    /// the row tint. `None` for context lines — they are the baseline, and
    /// prefixing every unchanged line with "Unchanged" would bury the two
    /// states that actually matter.
    fn announcement(&self) -> Option<&'static str> {
        match self {
            Self::Add => Some("Added: "),
            Self::Remove => Some("Removed: "),
            Self::Context | Self::Hunk => None,
        }
    }
}

/// A single line inside a diff.
#[derive(Debug, Clone)]
pub struct Line {
    pub kind: LineKind,
    /// Line number in the original file (None for added lines).
    pub old_line_no: Option<u32>,
    /// Line number in the new file (None for removed lines).
    pub new_line_no: Option<u32>,
    pub text: String,
}

/// Diff rendering properties.
#[derive(Debug, Clone, Default)]
pub struct Props {
    /// File path shown in the header (e.g. "src/main.rs").
    pub file_path: Option<String>,
    /// Lines in display order (usually: hunk, then context+add+remove per hunk).
    pub lines: Vec<Line>,
    /// Whether to show line-number columns.
    pub show_line_numbers: bool,
    /// Optional `added` count override — when None, counted from `lines`.
    pub added: Option<u32>,
    /// Optional `removed` count override.
    pub removed: Option<u32>,
}

fn count(lines: &[Line]) -> (u32, u32) {
    let mut adds = 0u32;
    let mut removes = 0u32;
    for l in lines {
        match l.kind {
            LineKind::Add => adds += 1,
            LineKind::Remove => removes += 1,
            _ => {}
        }
    }
    (adds, removes)
}

/// Render a diff view.
pub fn render(props: Props) -> Markup {
    let (adds, removes) = count(&props.lines);
    let total_added = props.added.unwrap_or(adds);
    let total_removed = props.removed.unwrap_or(removes);

    html! {
        div class="mui-diff" data-mui="diff" {
            div class="mui-diff__header" {
                @if let Some(path) = props.file_path.as_ref() {
                    span class="mui-diff__file" { (path) }
                }
                div class="mui-diff__stats" {
                    span class="mui-diff__added" { "+" (total_added) }
                    span class="mui-diff__removed" { "\u{2212}" (total_removed) }
                }
            }
            div class="mui-diff__body" role="table" aria-label="diff" {
                @for line in &props.lines {
                    div class={"mui-diff__line " (line.kind.class())} role="row" {
                        @if matches!(line.kind, LineKind::Hunk) {
                            span class="mui-diff__text" role="cell" { (line.text) }
                        } @else {
                            @if props.show_line_numbers {
                                span class="mui-diff__lineno mui-diff__lineno--old" role="cell" {
                                    @if let Some(n) = line.old_line_no { (n) }
                                }
                                span class="mui-diff__lineno mui-diff__lineno--new" role="cell" {
                                    @if let Some(n) = line.new_line_no { (n) }
                                }
                            }
                            span class="mui-diff__sigil" aria-hidden="true" { (line.kind.sigil()) }
                            span class="mui-diff__text" role="cell" {
                                // The +/- sigil is aria-hidden and the row tint
                                // is colour, so without this a screen reader
                                // gets the line text and no way to tell an
                                // addition from a deletion — which is the only
                                // thing a diff exists to convey. Lives INSIDE
                                // the cell: content sitting in a role="row" but
                                // outside a cell is not reliably announced.
                                @if let Some(state) = line.kind.announcement() {
                                    span class="mui-visually-hidden" { (state) }
                                }
                                (line.text)
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Showcase of a small diff.
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__column style="max-width: 44rem;" {
            (render(Props {
                file_path: Some("src/primitives/message.rs".into()),
                show_line_numbers: true,
                lines: vec![
                    Line { kind: LineKind::Hunk, old_line_no: None, new_line_no: None, text: "@@ -17,5 +17,6 @@".into() },
                    Line { kind: LineKind::Context, old_line_no: Some(17), new_line_no: Some(17), text: "pub struct Props {".into() },
                    Line { kind: LineKind::Remove, old_line_no: Some(18), new_line_no: None, text: "    pub role: String,".into() },
                    Line { kind: LineKind::Add, old_line_no: None, new_line_no: Some(18), text: "    pub role: Role,".into() },
                    Line { kind: LineKind::Add, old_line_no: None, new_line_no: Some(19), text: "    pub is_live: bool,".into() },
                    Line { kind: LineKind::Context, old_line_no: Some(19), new_line_no: Some(20), text: "    pub body: Markup,".into() },
                    Line { kind: LineKind::Context, old_line_no: Some(20), new_line_no: Some(21), text: "}".into() },
                ],
                ..Default::default()
            }))
        }
    }
}
