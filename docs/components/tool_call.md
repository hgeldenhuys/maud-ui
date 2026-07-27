# ToolCall

Collapsible representation of an AI agent's tool invocation (Edit, Write, Bash, Read, Grep, etc.). Renders a compact trigger row — glyph, name, summary, status pill — with an expandable body holding the arguments and result panes.

## Import

```rust
use maud_ui::primitives::tool_call::{self, Kind, Status, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::tool_call;

html! {
    (tool_call::render(tool_call::Props {
        id: "edit-1".into(),
        kind: tool_call::Kind::Edit,
        name: "Edit".into(),
        summary: "src/primitives/message.rs".into(),
        status: tool_call::Status::Success,
        args: Some(html! { pre { "old_string: \"role: Role::Default\"\nnew_string: \"role: Role::Assistant\"" } }),
        result: Some(html! { pre { "Applied edit — 1 replacement." } }),
        open: true,
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | `""` | Unique DOM id — used to wire the details disclosure |
| `kind` | `Kind` | `Kind::Custom` | Tool family (drives accent colour + glyph) |
| `name` | `String` | `""` | Tool name (e.g. "Edit", "Bash") |
| `summary` | `String` | `""` | One-line summary shown in the trigger (e.g. file path, command) |
| `status` | `Status` | `Status::Success` | Current execution status |
| `args` | `Option<Markup>` | `None` | Optional args panel — rendered in the expanded body |
| `result` | `Option<Markup>` | `None` | Optional result panel — rendered in the expanded body |
| `open` | `bool` | `false` | Initial open state (default false — collapsed) |

`Props` derives `Default`, so defaults above come from `#[derive(Default)]` on `Props`, `Kind`, and `Status` (each enum marks its default variant with `#[default]`) — there is no hand-written `impl Default` block.

## Variants / Enums

### Kind

Tool families, each with its own accent colour and glyph.

- `Custom` (default) — glyph `•`, class `mui-tool-call--custom`
- `Edit` — glyph `✎`, class `mui-tool-call--edit`
- `Write` — glyph `➕`, class `mui-tool-call--write`
- `Read` — glyph `📖`, class `mui-tool-call--read`
- `Bash` — glyph `›_`, class `mui-tool-call--bash`
- `Grep` — glyph `🔍`, class `mui-tool-call--grep`
- `Glob` — glyph `✱`, class `mui-tool-call--glob`
- `Task` — glyph `❖`, class `mui-tool-call--task`
- `Agent` — glyph `◆`, class `mui-tool-call--agent`
- `Search` — glyph `🔎`, class `mui-tool-call--search`

### Status

Execution state — drives the status pill colour and label.

- `Success` (default) — class `mui-tool-call__status--success`, label "done"
- `Running` — class `mui-tool-call__status--running`, label "running"
- `Error` — class `mui-tool-call__status--error`, label "error"
- `Pending` — class `mui-tool-call__status--pending`, label "queued"

## Helper Functions

None. `Kind` and `Status` each carry private `class()`/`glyph()` and `class()`/`label()` methods, but neither is marked `pub`, so they are not part of the public API. The only `pub fn`s in the module are `render` and `showcase`, both excluded from this section by convention.

## Accessibility

- The trigger is a native `button type="button"`, so it is focusable and keyboard-operable (Enter/Space) without extra script.
- The trigger carries `aria-expanded` (reflecting `props.open`, toggled by the inline `onclick` handler) and `aria-controls` pointing at the body's `id` — the standard disclosure-button pattern.
- The body `div` sets its `id` to match `aria-controls` and toggles the boolean `hidden` attribute in lockstep with `aria-expanded` (both in the initial render via `hidden[!props.open]` and in the `onclick` script at runtime).
- The glyph and chevron spans are both marked `aria-hidden="true"`, so decorative icons are not read out.
- Status is conveyed by a visible text label ("done" / "running" / "error" / "queued") inside the pill, not by colour alone — so the state is readable without colour perception.
- There is no `aria-live` region anywhere in the markup, so a status change (e.g. `Running` → `Success` on a re-render) is not automatically announced to a screen reader unless something else moves focus to it.

## Related

CodeBlock (for rendering args/result as code), Message (the chat bubble a tool call typically nests inside), Diff (for edit-style results), StreamingCursor (for an in-flight response alongside a `Running` tool call)

## Shadcn reference

No shadcn/ui equivalent — this is an agent/tool-invocation primitive with no upstream shadcn component to reference.
