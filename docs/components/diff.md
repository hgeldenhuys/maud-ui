# Diff

A unified diff viewer that renders line-level changes with line numbers, ± counts, and hunk headers. It has no syntax highlighting — the renderer focuses on accurate layout, not language-aware styling.

## Import

```rust
use maud_ui::primitives::diff::{self, LineKind, Line, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::diff;

html! {
    (diff::render(diff::Props {
        file_path: Some("src/primitives/message.rs".into()),
        show_line_numbers: true,
        lines: vec![
            diff::Line { kind: diff::LineKind::Hunk, old_line_no: None, new_line_no: None, text: "@@ -17,5 +17,6 @@".into() },
            diff::Line { kind: diff::LineKind::Context, old_line_no: Some(17), new_line_no: Some(17), text: "pub struct Props {".into() },
            diff::Line { kind: diff::LineKind::Remove, old_line_no: Some(18), new_line_no: None, text: "    pub role: String,".into() },
            diff::Line { kind: diff::LineKind::Add, old_line_no: None, new_line_no: Some(18), text: "    pub role: Role,".into() },
        ],
        ..Default::default()
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `file_path` | `Option<String>` | `None` | File path shown in the header (e.g. "src/main.rs"). |
| `lines` | `Vec<Line>` | `vec![]` (empty) | Lines in display order (usually: hunk, then context+add+remove per hunk). |
| `show_line_numbers` | `bool` | `false` | Whether to show line-number columns. |
| `added` | `Option<u32>` | `None` | Optional `added` count override — when `None`, counted from `lines`. |
| `removed` | `Option<u32>` | `None` | Optional `removed` count override. |

`Props` derives `Default`, so the values above come straight from `#[derive(Default)]` (`Option` → `None`, `Vec` → empty, `bool` → `false`) — every field above can be omitted via `..Default::default()`, as in the example.

### Line

`Line` derives only `Debug, Clone` (no `Default`), so every field below is required — there is no `..Default::default()` shortcut when constructing one.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `kind` | `LineKind` | *(required — no default)* | Kind of line in the diff; drives row colouring. |
| `old_line_no` | `Option<u32>` | *(required — no default)* | Line number in the original file (`None` for added lines). |
| `new_line_no` | `Option<u32>` | *(required — no default)* | Line number in the new file (`None` for removed lines). |
| `text` | `String` | *(required — no default)* | The line's text content, rendered verbatim into `.mui-diff__text`. |

## Variants / Enums

### LineKind

- `Context` — an unchanged line. Row class `mui-diff__line--context`; sigil `" "` (a single space).
- `Add` — an added line. Row class `mui-diff__line--add`; sigil `"+"`.
- `Remove` — a removed line. Row class `mui-diff__line--remove`; sigil `"-"` (a plain hyphen-minus — note this differs from the `\u{2212}` true minus sign used for the header's removed count).
- `Hunk` — a hunk header line (e.g. `@@ -17,5 +17,6 @@`). Row class `mui-diff__line--hunk`; `render()` special-cases this kind — it skips the line-number columns and the sigil span entirely and shows only the raw `text`, so the hunk marker must already be present in that text.

`class()` and `sigil()` are methods on `LineKind` but are **not** `pub` — they're private helpers `render()` uses internally, not part of the public API (see Helper Functions).

## Helper Functions

None. Besides `render` and `showcase`, `diff.rs` exports no other `pub fn`. The module's only other functions — the free function `count(lines: &[Line]) -> (u32, u32)` and `LineKind`'s `class`/`sigil` methods — are all private (no `pub` keyword) and exist purely to support `render()`.

## Accessibility

The diff body is `role="table"` with `aria-label="diff"`; each line is `role="row"` and its spans are `role="cell"` — so screen readers get generic table/row/cell semantics, but nothing that names an individual row as "added" or "removed."

Add/Remove state reaches every user through three channels. Sighted users get colour (the background tint `css/components/diff.css` applies to `.mui-diff__line--add`/`--remove`) and shape (a literal `"+"`/`"-"` sigil in `.mui-diff__sigil`). That sigil is marked `aria-hidden="true"`, so assistive tech instead receives a visually-hidden `"Added: "` / `"Removed: "` prefix emitted **inside** the row's `role="cell"` text span — placement matters, because content sitting in a `role="row"` but outside a cell is not reliably announced.

Context lines carry no announcement by design: they are the baseline, and prefixing every unchanged line with "Unchanged" would bury the two states that actually matter.

Until 0.4.0 the sigil was the only non-colour channel and it was `aria-hidden`, so screen-reader users got the line text with no way to tell an addition from a deletion — the one thing a diff exists to convey. `tests/render_tests.rs::conversation_0_4_0` guards the fix.

One gap remains: line-number cells (`old_line_no`/`new_line_no`) are plain, unlabelled cells, with nothing but visual position and CSS class to say which column is "old" and which is "new".

Hunk rows are the one exception where content stays fully accessible regardless of colour or ARIA: they render only their `text` (typically the `@@ ... @@` header itself), so a screen reader reads the same boundary marker a sighted user sees.

## Related

`code_block` (renders literal source in a monospace panel diff often sits alongside), `message` (chat-turn container a diff is commonly nested inside), `tool_call` (adjacent panel for an agent's tool invocation, often paired with a diff of the file it edited), `streaming_cursor` (appended while a diff is still being generated line-by-line)

## Shadcn reference

No shadcn/ui equivalent — shadcn ships no diff-view primitive, so there's nothing to link here.
